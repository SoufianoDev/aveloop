use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::domain::{
    ConfigMode, DescriptorAttachment, DescriptorKind, DescriptorStatus, LanguageIdentity,
    LanguageRegistry, Manifest, Project, SourceUnit, Workspace, WorkspaceError,
};

use super::ports::{EntryKind, ProjectSource};

/// Never scanned, at any depth. User exclusions extend this list.
pub const DEFAULT_EXCLUSIONS: [&str; 11] = [
    ".git",
    ".hg",
    ".svn",
    "build",
    "out",
    "target",
    "node_modules",
    "vendor",
    "dist",
    ".venv",
    "__pycache__",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiscoveryBounds {
    pub max_depth: usize,
    pub max_entries: usize,
}

impl Default for DiscoveryBounds {
    fn default() -> Self {
        Self {
            max_depth: 64,
            max_entries: 50_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Discovery {
    pub workspace: Workspace,
    pub problems: Vec<Problem>,
    /// Set when a bound stopped the scan: the workspace is incomplete, never silently truncated.
    pub incomplete: Option<Incomplete>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Problem {
    InvalidManifest {
        path: PathBuf,
        message: String,
    },
    InvalidProject {
        root: PathBuf,
        message: String,
    },
    Unreadable {
        path: PathBuf,
        message: String,
    },
    SkippedSymlink {
        path: PathBuf,
    },
    /// Two projects claim the same source. Both candidates are reported; neither wins.
    ConflictingSource {
        path: PathBuf,
        claimants: Vec<PathBuf>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Incomplete {
    pub root: PathBuf,
    pub reason: Bound,
    pub visited: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound {
    Depth,
    Entries,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DiscoveryError {
    #[error(transparent)]
    Workspace(#[from] WorkspaceError),
    #[error("workspace root `{root}` could not be read: {message}")]
    RootUnreadable { root: String, message: String },
}

/// Opens a workspace. No manifest is required, and nothing is written.
pub fn open_workspace(
    source: &dyn ProjectSource,
    registry: &LanguageRegistry,
    roots: Vec<PathBuf>,
    exclusions: Vec<PathBuf>,
) -> Result<Discovery, DiscoveryError> {
    open_workspace_with(
        source,
        registry,
        roots,
        exclusions,
        DiscoveryBounds::default(),
    )
}

pub fn open_workspace_with(
    source: &dyn ProjectSource,
    registry: &LanguageRegistry,
    roots: Vec<PathBuf>,
    exclusions: Vec<PathBuf>,
    bounds: DiscoveryBounds,
) -> Result<Discovery, DiscoveryError> {
    let mut canonical = Vec::with_capacity(roots.len());
    for root in roots {
        let resolved =
            source
                .canonicalize(&root)
                .map_err(|error| DiscoveryError::RootUnreadable {
                    root: root.display().to_string(),
                    message: error.message,
                })?;
        canonical.push(resolved);
    }

    let mut workspace = Workspace::new(canonical, exclusions)?;
    let scan = Scanner::new(source, &workspace, bounds).scan();

    let mut problems = scan.problems;
    let mut sources_by_project: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    for file in scan.files {
        match nearest_candidate(&scan.candidates, &file) {
            Some(root) => sources_by_project.entry(root).or_default().push(file),
            None => workspace.add_unassociated(file),
        }
    }

    let mut claims: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    for candidate in scan.candidates {
        let mut sources = sources_by_project
            .remove(&candidate.root)
            .unwrap_or_default();
        sources.sort();
        let project = build_project(source, registry, &candidate, &sources, &mut problems);
        for target in &project.targets {
            for path in &target.sources {
                claims
                    .entry(project.root.join(path))
                    .or_default()
                    .push(project.root.clone());
            }
        }
        workspace.add_project(project)?;
    }

    for (path, mut claimants) in claims {
        claimants.sort();
        claimants.dedup();
        if claimants.len() > 1 {
            problems.push(Problem::ConflictingSource { path, claimants });
        }
    }

    Ok(Discovery {
        workspace,
        problems,
        incomplete: scan.incomplete,
    })
}

pub fn open_project(
    source: &dyn ProjectSource,
    registry: &LanguageRegistry,
    root: PathBuf,
) -> Result<Discovery, DiscoveryError> {
    open_workspace(source, registry, vec![root], Vec::new())
}

fn nearest_candidate(candidates: &[Candidate], file: &Path) -> Option<PathBuf> {
    candidates
        .iter()
        .filter(|candidate| file.starts_with(&candidate.root))
        .max_by_key(|candidate| candidate.root.components().count())
        .map(|candidate| candidate.root.clone())
}

fn recognized(kind: DescriptorKind) -> DescriptorAttachment {
    DescriptorAttachment {
        kind,
        path: PathBuf::from(kind.file_name()),
        mode: None,
        status: DescriptorStatus::Recognized,
    }
}

fn build_project(
    source: &dyn ProjectSource,
    registry: &LanguageRegistry,
    candidate: &Candidate,
    sources: &[PathBuf],
    problems: &mut Vec<Problem>,
) -> Project {
    let mut attachments = Vec::new();
    let mut targets = Vec::new();
    let mut config_mode = None;
    let mut selected_target = None;
    let mut declared_name = None;
    let mut overrides = BTreeMap::new();

    if candidate.attachments.contains(&DescriptorKind::AveToml) {
        let path = candidate.root.join(DescriptorKind::AveToml.file_name());
        let manifest = match source.read_text(&path) {
            Ok(text) => match Manifest::parse(&text) {
                Ok(manifest) => Some(manifest),
                Err(error) => {
                    problems.push(Problem::InvalidManifest {
                        path: path.clone(),
                        message: error.to_string(),
                    });
                    None
                }
            },
            Err(error) => {
                problems.push(Problem::Unreadable {
                    path: error.path,
                    message: error.message,
                });
                None
            }
        };
        match manifest {
            Some(manifest) => {
                config_mode = Some(manifest.mode);
                selected_target = manifest.selected_target.clone();
                declared_name = manifest.name.clone().filter(|name| !name.trim().is_empty());
                attachments.push(DescriptorAttachment {
                    kind: DescriptorKind::AveToml,
                    path: PathBuf::from(DescriptorKind::AveToml.file_name()),
                    mode: Some(manifest.mode),
                    status: match manifest.mode {
                        ConfigMode::Overlay => DescriptorStatus::Integrated,
                        ConfigMode::Standalone => DescriptorStatus::BuildCapable,
                    },
                });
                match manifest.project(&candidate.root, registry) {
                    Ok(project) => {
                        targets = project.targets;
                        // Parsing already validated the manifest that produced this project.
                        overrides = manifest.language_overrides().unwrap_or_default();
                    }
                    Err(error) => problems.push(Problem::InvalidProject {
                        root: candidate.root.clone(),
                        message: error.to_string(),
                    }),
                }
            }
            None => attachments.push(recognized(DescriptorKind::AveToml)),
        }
    }

    for kind in &candidate.attachments {
        if *kind != DescriptorKind::AveToml {
            attachments.push(recognized(*kind));
        }
    }

    let name = declared_name.unwrap_or_else(|| {
        candidate
            .root
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| candidate.root.display().to_string())
    });

    let units: Vec<SourceUnit> = sources
        .iter()
        .map(|path| {
            let relative = path
                .strip_prefix(&candidate.root)
                .unwrap_or(path)
                .to_path_buf();
            // shortcut: overrides match exact project-relative paths; globs follow later.
            let explicit = overrides.get(&relative).cloned();
            let language = LanguageIdentity::determine(&relative, explicit, registry);
            SourceUnit {
                path: relative,
                language,
            }
        })
        .collect();

    let project = Project {
        name,
        root: candidate.root.clone(),
        config_mode,
        selected_target,
        attachments,
        targets,
        sources: units,
    };

    if let Err(error) = project.validate(registry) {
        problems.push(Problem::InvalidProject {
            root: candidate.root.clone(),
            message: error.to_string(),
        });
    }
    project
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Candidate {
    root: PathBuf,
    attachments: Vec<DescriptorKind>,
}

struct Scan {
    candidates: Vec<Candidate>,
    files: Vec<PathBuf>,
    problems: Vec<Problem>,
    incomplete: Option<Incomplete>,
}

struct Scanner<'a> {
    source: &'a dyn ProjectSource,
    roots: Vec<PathBuf>,
    exclusions: Vec<PathBuf>,
    bounds: DiscoveryBounds,
    visited: usize,
    candidates: Vec<Candidate>,
    files: Vec<PathBuf>,
    problems: Vec<Problem>,
    incomplete: Option<Incomplete>,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a dyn ProjectSource, workspace: &Workspace, bounds: DiscoveryBounds) -> Self {
        Self {
            source,
            roots: workspace.roots().to_vec(),
            exclusions: workspace.exclusions().to_vec(),
            bounds,
            visited: 0,
            candidates: Vec::new(),
            files: Vec::new(),
            problems: Vec::new(),
            incomplete: None,
        }
    }

    fn scan(mut self) -> Scan {
        for root in self.roots.clone() {
            if self.incomplete.is_some() {
                break;
            }
            self.walk(&root, &root, 0);
        }
        Scan {
            candidates: self.candidates,
            files: self.files,
            problems: self.problems,
            incomplete: self.incomplete,
        }
    }

    fn walk(&mut self, root: &Path, directory: &Path, depth: usize) {
        if depth > self.bounds.max_depth {
            self.stop(root, Bound::Depth);
            return;
        }

        let mut entries = match self.source.entries(directory) {
            Ok(entries) => entries,
            Err(error) => {
                self.problems.push(Problem::Unreadable {
                    path: error.path,
                    message: error.message,
                });
                return;
            }
        };
        entries.sort_by(|left, right| left.path.cmp(&right.path));

        self.visited += entries.len();
        if self.visited > self.bounds.max_entries {
            self.stop(root, Bound::Entries);
            return;
        }

        let attachments: Vec<DescriptorKind> = entries
            .iter()
            .filter(|entry| entry.kind == EntryKind::File)
            .filter_map(|entry| entry.path.file_name().and_then(|name| name.to_str()))
            .filter_map(DescriptorKind::from_file_name)
            .collect();
        if !attachments.is_empty() {
            self.candidates.push(Candidate {
                root: directory.to_path_buf(),
                attachments,
            });
        }

        for entry in entries {
            match entry.kind {
                EntryKind::Symlink => self
                    .problems
                    .push(Problem::SkippedSymlink { path: entry.path }),
                EntryKind::File => {
                    // A descriptor is configuration, not a source unit.
                    let is_descriptor = entry
                        .path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .and_then(DescriptorKind::from_file_name)
                        .is_some();
                    if !is_descriptor {
                        self.files.push(entry.path);
                    }
                }
                EntryKind::Directory => {
                    if self.is_excluded(root, &entry.path) {
                        continue;
                    }
                    self.walk(root, &entry.path, depth + 1);
                    if self.incomplete.is_some() {
                        return;
                    }
                }
            }
        }
    }

    fn stop(&mut self, root: &Path, reason: Bound) {
        self.incomplete = Some(Incomplete {
            root: root.to_path_buf(),
            reason,
            visited: self.visited,
        });
    }

    fn is_excluded(&self, root: &Path, path: &Path) -> bool {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if DEFAULT_EXCLUSIONS.contains(&name) {
            return true;
        }
        let relative = path.strip_prefix(root).unwrap_or(path);
        self.exclusions
            .iter()
            .any(|exclusion| relative.starts_with(exclusion))
    }
}
