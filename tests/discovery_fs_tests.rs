//! The `std::fs` adapter against a real directory tree.

use std::path::PathBuf;

use aveloop::application::discovery::open_workspace;
use aveloop::application::ports::ProjectWriter;
use aveloop::domain::{ConfigMode, DescriptorKind, DescriptorStatus, LanguageRegistry};
use aveloop::infrastructure::{FileSystemSource, FileSystemWriter};

#[test]
fn discovers_a_real_directory_and_skips_build_output() {
    let root = std::env::temp_dir().join(format!("aveloop-discovery-fs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let app = root.join("app");
    std::fs::create_dir_all(app.join("src")).expect("create src");
    std::fs::create_dir_all(app.join("target")).expect("create target");
    std::fs::write(app.join("CMakeLists.txt"), "project(app)").expect("write cmake");
    std::fs::write(app.join("ave.toml"), "version = 1\nmode = \"overlay\"\n")
        .expect("write manifest");
    std::fs::write(app.join("src/main.c"), "int main(void) { return 0; }").expect("write source");
    std::fs::write(app.join("target/CMakeLists.txt"), "project(build)").expect("write build file");

    let discovery = open_workspace(
        &FileSystemSource,
        &LanguageRegistry::alpha(),
        vec![root.clone()],
        Vec::new(),
    )
    .expect("opens");

    let projects = discovery.workspace.projects();
    assert_eq!(projects.len(), 1);
    let project = &projects[0];
    assert_eq!(project.name, "app");
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
        ]
    );
    assert_eq!(
        project
            .sources
            .iter()
            .map(|unit| unit.path.to_path_buf())
            .collect::<Vec<_>>(),
        [PathBuf::from("src/main.c")]
    );
    assert!(discovery.problems.is_empty());
    assert!(discovery.incomplete.is_none());

    std::fs::remove_dir_all(&root).expect("cleanup");
}

#[test]
fn writes_a_source_file_through_the_port() {
    let root = std::env::temp_dir().join(format!("aveloop-writer-fs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("create root");
    let file = root.join("main.c");

    FileSystemWriter
        .write_text(&file, "int main(void) { return 1; }")
        .expect("writes");
    assert_eq!(
        std::fs::read_to_string(&file).expect("reads back"),
        "int main(void) { return 1; }"
    );

    let error = FileSystemWriter
        .write_text(&root, "not a file")
        .expect_err("writing over a directory fails");
    assert_eq!(error.path, root);

    std::fs::remove_dir_all(&root).expect("cleanup");
}
