//! Project discovery: association, descriptor status, exclusions, bounds, and conflicts.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aveloop::application::discovery::{
    open_project, open_workspace, open_workspace_with, Bound, DiscoveryBounds, Problem,
    DEFAULT_EXCLUSIONS,
};
use aveloop::application::ports::{DirEntry, EntryKind, ProjectSource, SourceError};
use aveloop::domain::{
    ConfigMode, DescriptorKind, DescriptorStatus, LanguageId, LanguageIdentity, LanguageRegistry,
};

#[derive(Default)]
struct FakeSource {
    dirs: BTreeMap<PathBuf, Vec<DirEntry>>,
    files: BTreeMap<PathBuf, String>,
}

impl FakeSource {
    fn new() -> Self {
        Self::default()
    }

    fn file(mut self, path: &str, contents: &str) -> Self {
        let path = p(path);
        self.register(&path, EntryKind::File);
        self.files.insert(path, contents.to_string());
        self
    }

    fn dir(mut self, path: &str) -> Self {
        self.register(&p(path), EntryKind::Directory);
        self
    }

    fn symlink(mut self, path: &str) -> Self {
        self.register(&p(path), EntryKind::Symlink);
        self
    }

    /// Registers the path in its parent, and every ancestor as a directory.
    fn register(&mut self, path: &Path, kind: EntryKind) {
        let mut current = path.to_path_buf();
        let mut current_kind = kind;
        while let Some(parent) = current.parent() {
            if parent == Path::new("/") || parent.as_os_str().is_empty() {
                break;
            }
            let siblings = self.dirs.entry(parent.to_path_buf()).or_default();
            if !siblings.iter().any(|entry| entry.path == current) {
                siblings.push(DirEntry::new(current.clone(), current_kind));
            }
            current = parent.to_path_buf();
            current_kind = EntryKind::Directory;
        }
    }
}

impl ProjectSource for FakeSource {
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, SourceError> {
        Ok(path.to_path_buf())
    }

    fn entries(&self, directory: &Path) -> Result<Vec<DirEntry>, SourceError> {
        self.dirs
            .get(directory)
            .cloned()
            .ok_or_else(|| SourceError::new(directory, "not a directory"))
    }

    fn read_text(&self, file: &Path) -> Result<String, SourceError> {
        self.files
            .get(file)
            .cloned()
            .ok_or_else(|| SourceError::new(file, "not a file"))
    }
}

fn registry() -> LanguageRegistry {
    LanguageRegistry::alpha()
}

/// The fake tree's logical names, translated to a platform-absolute path.
fn p(path: &str) -> PathBuf {
    let relative = path.trim_start_matches('/');
    if cfg!(windows) {
        PathBuf::from("C:\\").join(relative.replace('/', "\\"))
    } else {
        Path::new("/").join(relative)
    }
}

fn root() -> Vec<PathBuf> {
    vec![p("/ws")]
}

fn standalone(name: &str, target: &str, sources: &str) -> String {
    format!(
        "version = 1\nmode = \"standalone\"\nname = \"{name}\"\n\n[[target]]\nname = \"{target}\"\nsources = [{sources}]\n"
    )
}

#[test]
fn opens_a_workspace_without_any_manifest() {
    let source = FakeSource::new()
        .file("/ws/notes.md", "# notes")
        .file("/ws/src/main.c", "int main(void) { return 0; }");

    let discovery = open_workspace(&source, &registry(), root(), Vec::new()).expect("opens");

    assert!(discovery.workspace.projects().is_empty());
    assert_eq!(
        discovery.workspace.unassociated(),
        [p("/ws/notes.md"), p("/ws/src/main.c")]
    );
    assert!(discovery.problems.is_empty());
    assert!(discovery.incomplete.is_none());
}

#[test]
fn associates_files_with_the_nearest_project_root() {
    let source = FakeSource::new()
        .file(
            "/ws/app/ave.toml",
            &standalone("app", "app", "\"src/main.c\""),
        )
        .file("/ws/app/src/main.c", "int main(void) { return 0; }")
        .file("/ws/app/README.md", "# app")
        .file("/ws/loose.c", "int loose;");

    let discovery = open_workspace(&source, &registry(), root(), Vec::new()).expect("opens");

    let projects = discovery.workspace.projects();
    assert_eq!(projects.len(), 1);
    let project = &projects[0];
    assert_eq!(project.name, "app");
    assert_eq!(project.root, p("/ws/app"));
    assert_eq!(project.config_mode, Some(ConfigMode::Standalone));
    assert_eq!(
        project
            .sources
            .iter()
            .map(|unit| unit.path.clone())
            .collect::<Vec<_>>(),
        [PathBuf::from("README.md"), PathBuf::from("src/main.c")]
    );
    assert!(project.target("app").is_some());
    assert_eq!(discovery.workspace.unassociated(), [p("/ws/loose.c")]);
    assert!(discovery.problems.is_empty());
}

#[test]
fn reports_one_project_with_every_descriptor_and_its_status() {
    let source = FakeSource::new()
        .file(
            "/ws/engine/ave.toml",
            "version = 1\nmode = \"overlay\"\n\n[languages]\n\"src/main.c\" = \"cpp\"\n",
        )
        .file("/ws/engine/CMakeLists.txt", "project(engine)")
        .file("/ws/engine/Makefile", "all:")
        .file("/ws/engine/src/main.c", "int main() { return 0; }")
        .file("/ws/engine/src/engine.h", "int engine();");

    let discovery = open_workspace(&source, &registry(), root(), Vec::new()).expect("opens");

    let projects = discovery.workspace.projects();
    assert_eq!(projects.len(), 1);
    let project = &projects[0];
    assert_eq!(project.config_mode, Some(ConfigMode::Overlay));
    assert_eq!(
        project
            .attachments
            .iter()
            .map(|attachment| (attachment.kind, attachment.status))
            .collect::<Vec<_>>(),
        [
            (DescriptorKind::AveToml, DescriptorStatus::Integrated),
            (DescriptorKind::CMakeLists, DescriptorStatus::Recognized),
            (DescriptorKind::Makefile, DescriptorStatus::Recognized),
        ]
    );
    assert_eq!(
        project
            .source(Path::new("src/main.c"))
            .map(|unit| &unit.language),
        Some(&LanguageIdentity::Explicit(
            LanguageId::new("cpp").expect("valid id")
        ))
    );
    assert!(project.targets.is_empty());
    assert!(discovery.problems.is_empty());
}

#[test]
fn standalone_mode_defines_targets_through_discovery() {
    let source = FakeSource::new()
        .file(
            "/ws/app/ave.toml",
            &standalone("app", "server", "\"src/main.c\", \"src/net.c\""),
        )
        .file("/ws/app/src/main.c", "int main(void) { return 0; }")
        .file("/ws/app/src/net.c", "void net(void) {}");

    let discovery = open_workspace(&source, &registry(), root(), Vec::new()).expect("opens");

    let project = &discovery.workspace.projects()[0];
    assert_eq!(
        project.attachments[0].status,
        DescriptorStatus::BuildCapable
    );
    assert_eq!(project.attachments[0].mode, Some(ConfigMode::Standalone));
    let target = project.target("server").expect("target discovered");
    assert_eq!(target.sources.len(), 2);
    assert!(discovery.problems.is_empty());
}

#[test]
fn reports_an_invalid_manifest_without_dropping_the_project() {
    let source = FakeSource::new()
        .file("/ws/app/ave.toml", "version = 2\nmode = \"standalone\"\n")
        .file("/ws/app/src/main.c", "int main(void) { return 0; }");

    let discovery = open_workspace(&source, &registry(), root(), Vec::new()).expect("opens");

    let project = &discovery.workspace.projects()[0];
    assert_eq!(project.config_mode, None);
    assert_eq!(project.attachments[0].status, DescriptorStatus::Recognized);
    assert!(project.targets.is_empty());
    assert_eq!(project.sources.len(), 1);
    assert!(discovery
        .problems
        .iter()
        .any(|problem| matches!(problem, Problem::InvalidManifest { .. })));
}

#[test]
fn a_nested_descriptor_becomes_a_child_project() {
    let source = FakeSource::new()
        .file("/ws/CMakeLists.txt", "project(top)")
        .file("/ws/src/top.c", "int top;")
        .file("/ws/native/ave.toml", "version = 1\nmode = \"overlay\"\n")
        .file("/ws/native/src/main.c", "int main(void) { return 0; }");

    let discovery = open_workspace(&source, &registry(), root(), Vec::new()).expect("opens");

    let projects = discovery.workspace.projects();
    assert_eq!(projects.len(), 2);
    assert_eq!(projects[0].root, p("/ws"));
    assert_eq!(projects[1].root, p("/ws/native"));
    assert_eq!(projects[0].sources[0].path, PathBuf::from("src/top.c"));
    assert_eq!(projects[1].sources[0].path, PathBuf::from("src/main.c"));
    assert!(discovery.workspace.unassociated().is_empty());
}

#[test]
fn skips_default_excluded_directories() {
    let source = FakeSource::new()
        .file("/ws/ave.toml", "version = 1\nmode = \"overlay\"\n")
        .file("/ws/main.c", "int main(void) { return 0; }")
        .file("/ws/target/ave.toml", "version = 1\nmode = \"overlay\"\n")
        .file("/ws/.git/CMakeLists.txt", "project(git)")
        .file("/ws/node_modules/pkg/CMakeLists.txt", "project(pkg)")
        .file("/ws/node_modules/pkg/index.js", "module.exports = {};");

    let discovery = open_workspace(&source, &registry(), root(), Vec::new()).expect("opens");

    assert_eq!(discovery.workspace.projects().len(), 1);
    assert_eq!(discovery.workspace.projects()[0].sources.len(), 1);
    assert!(discovery.workspace.unassociated().is_empty());
    assert!(DEFAULT_EXCLUSIONS.contains(&"node_modules"));
}

#[test]
fn honors_user_exclusions() {
    let source = FakeSource::new()
        .file("/ws/ave.toml", "version = 1\nmode = \"overlay\"\n")
        .file("/ws/generated/api.c", "int api;")
        .file("/ws/main.c", "int main(void) { return 0; }");

    let discovery = open_workspace(
        &source,
        &registry(),
        root(),
        vec![PathBuf::from("generated")],
    )
    .expect("opens");

    assert_eq!(discovery.workspace.projects()[0].sources.len(), 1);
    assert!(discovery.workspace.unassociated().is_empty());
}

#[test]
fn reports_skipped_symlinks() {
    let source = FakeSource::new()
        .file("/ws/ave.toml", "version = 1\nmode = \"overlay\"\n")
        .symlink("/ws/linked")
        .file("/ws/main.c", "int main(void) { return 0; }");

    let discovery = open_workspace(&source, &registry(), root(), Vec::new()).expect("opens");

    assert!(discovery.problems.contains(&Problem::SkippedSymlink {
        path: p("/ws/linked")
    }));
}

#[test]
fn reports_unreadable_directories() {
    let source = FakeSource::new().dir("/ws/broken");

    let discovery = open_workspace(&source, &registry(), root(), Vec::new()).expect("opens");

    assert!(discovery.problems.iter().any(|problem| matches!(
        problem,
        Problem::Unreadable { path, .. } if *path == p("/ws/broken")
    )));
}

#[test]
fn reports_incomplete_discovery_at_the_depth_bound() {
    let source = FakeSource::new()
        .file("/ws/CMakeLists.txt", "project(top)")
        .file("/ws/a/b/main.c", "int main(void) { return 0; }");

    let discovery = open_workspace_with(
        &source,
        &registry(),
        root(),
        Vec::new(),
        DiscoveryBounds {
            max_depth: 1,
            max_entries: 100,
        },
    )
    .expect("opens");

    let incomplete = discovery.incomplete.expect("depth bound reported");
    assert_eq!(incomplete.reason, Bound::Depth);
    assert_eq!(incomplete.root, p("/ws"));
}

#[test]
fn reports_incomplete_discovery_at_the_entry_bound() {
    let source = FakeSource::new()
        .file("/ws/CMakeLists.txt", "project(top)")
        .file("/ws/main.c", "int main(void) { return 0; }");

    let discovery = open_workspace_with(
        &source,
        &registry(),
        root(),
        Vec::new(),
        DiscoveryBounds {
            max_depth: 64,
            max_entries: 1,
        },
    )
    .expect("opens");

    let incomplete = discovery.incomplete.expect("entry bound reported");
    assert_eq!(incomplete.reason, Bound::Entries);
    assert!(discovery.workspace.projects().is_empty());
}

#[test]
fn reports_conflicting_source_claims_with_both_candidates() {
    let source = FakeSource::new()
        .file(
            "/ws/ave.toml",
            &standalone("outer", "outer-app", "\"native/src/main.c\""),
        )
        .file(
            "/ws/native/ave.toml",
            &standalone("inner", "inner-app", "\"src/main.c\""),
        )
        .file("/ws/native/src/main.c", "int main(void) { return 0; }");

    let discovery = open_workspace(&source, &registry(), root(), Vec::new()).expect("opens");

    let conflict = discovery
        .problems
        .iter()
        .find_map(|problem| match problem {
            Problem::ConflictingSource { path, claimants } => {
                Some((path.clone(), claimants.clone()))
            }
            _ => None,
        })
        .expect("conflict reported");
    assert_eq!(conflict.0, p("/ws/native/src/main.c"));
    assert_eq!(conflict.1, [p("/ws"), p("/ws/native")]);
}

#[test]
fn open_project_is_a_single_root_workspace() {
    let source = FakeSource::new()
        .file("/ws/app/ave.toml", &standalone("app", "app", "\"main.c\""))
        .file("/ws/app/main.c", "int main(void) { return 0; }")
        .file("/ws/elsewhere/main.c", "int other;");

    let discovery = open_project(&source, &registry(), p("/ws/app")).expect("opens");

    assert_eq!(discovery.workspace.projects().len(), 1);
    assert!(discovery.workspace.unassociated().is_empty());
}

#[test]
fn rejects_a_relative_root() {
    let source = FakeSource::new().file("/ws/main.c", "int main(void) { return 0; }");

    let error = open_workspace(
        &source,
        &registry(),
        vec![PathBuf::from("relative/path")],
        Vec::new(),
    )
    .expect_err("relative root rejected");

    assert!(matches!(
        error,
        aveloop::application::discovery::DiscoveryError::Workspace(_)
    ));
}
