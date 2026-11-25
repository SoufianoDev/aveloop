use std::fmt;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;

use super::language::{LanguageId, LanguageIdentity, LanguageRegistry};
use super::manifest::ConfigMode;

/// A descriptor file kind. Recognition never implies integration or build support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DescriptorKind {
    AveToml,
    CMakeLists,
    Makefile,
    CompileCommands,
    CargoManifest,
    GoModule,
}

impl DescriptorKind {
    pub const ALL: [Self; 6] = [
        Self::AveToml,
        Self::CMakeLists,
        Self::Makefile,
        Self::CompileCommands,
        Self::CargoManifest,
        Self::GoModule,
    ];

    pub fn from_file_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.file_name() == name)
    }

    pub fn file_name(&self) -> &'static str {
        match self {
            Self::AveToml => "ave.toml",
            Self::CMakeLists => "CMakeLists.txt",
            Self::Makefile => "Makefile",
            Self::CompileCommands => "compile_commands.json",
            Self::CargoManifest => "Cargo.toml",
            Self::GoModule => "go.mod",
        }
    }

    pub fn is_native(&self) -> bool {
        !matches!(self, Self::AveToml)
    }
}

impl fmt::Display for DescriptorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.file_name())
    }
}

/// How far AveLoop can go with one descriptor. A strict progression, reported per descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DescriptorStatus {
    /// The file was identified at its path; its model is not read.
    Recognized,
    /// AveLoop can read the descriptor's project and target model.
    Integrated,
    /// AveLoop can execute a build through the descriptor.
    BuildCapable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptorAttachment {
    pub kind: DescriptorKind,
    /// Relative to the project root.
    pub path: PathBuf,
    /// Set only for `ave.toml`, whose role is explicit.
    pub mode: Option<ConfigMode>,
    pub status: DescriptorStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetKind {
    #[default]
    Executable,
    Library,
    Test,
    AuxiliaryTool,
}

/// A concrete build or execution unit. Never derived from directory layout alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    pub name: String,
    pub kind: TargetKind,
    /// Exactly one descriptor attachment owns a target.
    pub owner: DescriptorKind,
    pub entry: Option<PathBuf>,
    /// Project-relative paths. A member may use a different language than its siblings.
    pub sources: Vec<PathBuf>,
    /// Resolves ambiguous sources for this target only; never a project-wide language.
    pub language_hint: Option<LanguageId>,
    pub include_dirs: Vec<PathBuf>,
    pub defines: Vec<String>,
    pub compiler_args: Vec<String>,
    pub linker_args: Vec<String>,
    pub program_args: Vec<String>,
    pub working_directory: Option<PathBuf>,
    pub depends_on: Vec<String>,
}

/// A file with its own language identity. Target membership lives on `Target::sources`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceUnit {
    pub path: PathBuf,
    pub language: LanguageIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    /// Absolute. Identity is the workspace plus this path.
    pub root: PathBuf,
    /// The role of `ave.toml` in this project, absent when there is none.
    pub config_mode: Option<ConfigMode>,
    pub selected_target: Option<String>,
    pub attachments: Vec<DescriptorAttachment>,
    pub targets: Vec<Target>,
    pub sources: Vec<SourceUnit>,
}

impl Project {
    pub fn target(&self, name: &str) -> Option<&Target> {
        self.targets.iter().find(|target| target.name == name)
    }

    pub fn source(&self, path: &Path) -> Option<&SourceUnit> {
        self.sources.iter().find(|unit| unit.path == path)
    }

    pub fn targets_for_source(&self, path: &Path) -> Vec<&Target> {
        self.targets
            .iter()
            .filter(|target| target.sources.iter().any(|source| source == path))
            .collect()
    }

    /// Language participation is derived from target sources, never stored on the project.
    pub fn target_languages(&self, name: &str) -> Vec<LanguageId> {
        let Some(target) = self.target(name) else {
            return Vec::new();
        };
        let mut languages: Vec<LanguageId> = target
            .sources
            .iter()
            .filter_map(|path| self.source(path))
            .filter_map(|unit| match &unit.language {
                LanguageIdentity::Explicit(id) | LanguageIdentity::Registered(id) => {
                    Some(id.clone())
                }
                LanguageIdentity::Ambiguous(_) => target.language_hint.clone(),
                LanguageIdentity::Unknown => None,
            })
            .collect();
        languages.sort();
        languages.dedup();
        languages
    }

    pub fn validate(&self, registry: &LanguageRegistry) -> Result<(), ProjectError> {
        if !self.root.is_absolute() {
            return Err(ProjectError::RelativeRoot {
                root: self.root.display().to_string(),
            });
        }
        if self.name.trim().is_empty() {
            return Err(ProjectError::EmptyName);
        }

        if self.config_mode == Some(ConfigMode::Standalone) {
            if let Some(attachment) = self.attachments.iter().find(|attachment| {
                attachment.kind.is_native() && attachment.status != DescriptorStatus::Recognized
            }) {
                return Err(ProjectError::StandaloneShadowsNative {
                    descriptor: attachment.kind.to_string(),
                });
            }
        }

        for (index, target) in self.targets.iter().enumerate() {
            if target.name.trim().is_empty() {
                return Err(ProjectError::EmptyTargetName);
            }
            if self.config_mode == Some(ConfigMode::Overlay) {
                return Err(ProjectError::OverlayDefinesTargets {
                    name: target.name.clone(),
                });
            }
            if self.targets[..index]
                .iter()
                .any(|other| other.name == target.name)
            {
                return Err(ProjectError::DuplicateTarget {
                    name: target.name.clone(),
                });
            }
            if !self
                .attachments
                .iter()
                .any(|attachment| attachment.kind == target.owner)
            {
                return Err(ProjectError::MissingOwner {
                    target: target.name.clone(),
                    owner: target.owner.to_string(),
                });
            }
            for source in &target.sources {
                if self.source(source).is_none() {
                    return Err(ProjectError::UnknownSource {
                        target: target.name.clone(),
                        path: source.display().to_string(),
                    });
                }
            }
            for dependency in &target.depends_on {
                if dependency == &target.name {
                    return Err(ProjectError::SelfDependency {
                        target: target.name.clone(),
                    });
                }
                if self.target(dependency).is_none() {
                    return Err(ProjectError::UnknownDependency {
                        target: target.name.clone(),
                        dependency: dependency.clone(),
                    });
                }
            }
            if let Some(hint) = &target.language_hint {
                if !registry.supports(hint) {
                    return Err(ProjectError::UnregisteredLanguage {
                        id: hint.to_string(),
                    });
                }
            }
        }

        for unit in &self.sources {
            if let LanguageIdentity::Explicit(id) = &unit.language {
                if !registry.supports(id) {
                    return Err(ProjectError::UnregisteredLanguage { id: id.to_string() });
                }
            }
        }

        if let Some(selected) = &self.selected_target {
            // An overlay may select a native target that Alpha cannot enumerate yet.
            if !self.targets.is_empty() && self.target(selected).is_none() {
                return Err(ProjectError::UnknownSelectedTarget {
                    name: selected.clone(),
                });
            }
        }

        Ok(())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProjectError {
    #[error("project root `{root}` must be an absolute path")]
    RelativeRoot { root: String },
    #[error("project name must not be empty")]
    EmptyName,
    #[error("target name must not be empty")]
    EmptyTargetName,
    #[error("target `{name}` is declared more than once")]
    DuplicateTarget { name: String },
    #[error("target `{target}` declares owner `{owner}`, but the project has no such descriptor")]
    MissingOwner { target: String, owner: String },
    #[error("target `{target}` lists `{path}`, which is not a project source")]
    UnknownSource { target: String, path: String },
    #[error("target `{target}` depends on `{dependency}`, which is not a project target")]
    UnknownDependency { target: String, dependency: String },
    #[error("target `{target}` depends on itself")]
    SelfDependency { target: String },
    #[error(
        "`mode = \"overlay\"` cannot define targets; move target `{name}` to `mode = \"standalone\"`"
    )]
    OverlayDefinesTargets { name: String },
    #[error(
        "standalone mode would shadow the integrated `{descriptor}` descriptor; use `mode = \"overlay\"` instead"
    )]
    StandaloneShadowsNative { descriptor: String },
    #[error("language `{id}` is not registered; register the integration or use a supported id")]
    UnregisteredLanguage { id: String },
    #[error("`{id}` is not a valid language id")]
    InvalidLanguageId { id: String },
    #[error("selected target `{name}` is not a project target")]
    UnknownSelectedTarget { name: String },
}
