use std::path::PathBuf;

use aveloop::domain::{
    CapabilityStatus, ConfigMode, DescriptorAttachment, DescriptorKind, DescriptorStatus,
    ExtensionResolution, LanguageCapabilities, LanguageId, LanguageIdentity, LanguageIntegration,
    LanguageRegistry, Project, ProjectError, RegistryError, SourceUnit, Target, TargetKind,
    Workspace, WorkspaceError,
};

fn c() -> LanguageId {
    LanguageId::new("c").unwrap()
}

fn cpp() -> LanguageId {
    LanguageId::new("cpp").unwrap()
}

fn root(name: &str) -> PathBuf {
    std::env::temp_dir().join(name)
}

fn source(path: &str, language: LanguageIdentity) -> SourceUnit {
    SourceUnit {
        path: path.into(),
        language,
    }
}

fn target(name: &str, sources: &[&str], depends_on: &[&str]) -> Target {
    Target {
        name: name.into(),
        kind: TargetKind::Executable,
        owner: DescriptorKind::AveToml,
        entry: None,
        sources: sources.iter().map(PathBuf::from).collect(),
        language_hint: None,
        include_dirs: Vec::new(),
        defines: Vec::new(),
        compiler_args: Vec::new(),
        linker_args: Vec::new(),
        program_args: Vec::new(),
        working_directory: None,
        depends_on: depends_on.iter().map(|name| name.to_string()).collect(),
    }
}

fn attachment(kind: DescriptorKind, status: DescriptorStatus) -> DescriptorAttachment {
    DescriptorAttachment {
        kind,
        path: PathBuf::from(kind.file_name()),
        mode: None,
        status,
    }
}

fn project(config_mode: Option<ConfigMode>, attachments: Vec<DescriptorAttachment>) -> Project {
    Project {
        name: "engine".into(),
        root: root("aveloop-project-model-tests"),
        config_mode,
        selected_target: None,
        attachments,
        targets: Vec::new(),
        sources: Vec::new(),
    }
}

#[test]
fn alpha_registry_declares_capabilities_individually() {
    let registry = LanguageRegistry::alpha();
    let c_integration = registry.integration("c").expect("C is registered");

    assert_eq!(
        c_integration.capabilities.recognition,
        CapabilityStatus::Supported
    );
    assert_eq!(
        c_integration.capabilities.diagnostics,
        CapabilityStatus::Partial
    );
    assert_eq!(
        c_integration.capabilities.lsp,
        CapabilityStatus::Unavailable
    );
    assert_eq!(
        c_integration.capabilities.visualization,
        CapabilityStatus::Supported
    );
    assert!(registry.integration("rust").is_none());
    assert_eq!(
        registry.resolve_extension("rs"),
        ExtensionResolution::Unknown
    );
    assert_eq!(
        registry.resolve_extension(".h"),
        ExtensionResolution::Ambiguous(vec![c(), cpp()])
    );
    assert_eq!(
        registry.resolve_extension("hpp"),
        ExtensionResolution::Single(cpp())
    );
}

#[test]
fn registry_accepts_new_integration_without_model_change() {
    let mut registry = LanguageRegistry::alpha();
    registry
        .register(LanguageIntegration {
            id: LanguageId::new("rust").unwrap(),
            display_name: "Rust".into(),
            extensions: vec!["rs".into()],
            capabilities: LanguageCapabilities {
                recognition: CapabilityStatus::Supported,
                ..LanguageCapabilities::default()
            },
        })
        .expect("a new language id is a registration, not a model change");

    assert_eq!(
        registry.resolve_extension("rs"),
        ExtensionResolution::Single(LanguageId::new("rust").unwrap())
    );
    assert_eq!(
        registry.register(LanguageIntegration {
            id: LanguageId::new("rust").unwrap(),
            display_name: "Rust".into(),
            extensions: vec!["rs".into()],
            capabilities: LanguageCapabilities::default(),
        }),
        Err(RegistryError::DuplicateId("rust".into()))
    );
}

#[test]
fn project_validation_accepts_source_outside_every_target() {
    let registry = LanguageRegistry::alpha();
    let mut project = project(
        None,
        vec![attachment(
            DescriptorKind::CMakeLists,
            DescriptorStatus::Recognized,
        )],
    );
    project
        .sources
        .push(source("src/main.c", LanguageIdentity::Registered(c())));

    project
        .validate(&registry)
        .expect("a source may belong to no target");
    assert!(project
        .targets_for_source(&PathBuf::from("src/main.c"))
        .is_empty());
    assert_eq!(
        project.target_languages("missing"),
        Vec::<LanguageId>::new()
    );
}

#[test]
fn project_validation_rejects_target_with_unknown_source() {
    let registry = LanguageRegistry::alpha();
    let mut project = project(
        Some(ConfigMode::Standalone),
        vec![attachment(
            DescriptorKind::AveToml,
            DescriptorStatus::BuildCapable,
        )],
    );
    project.targets.push(target("app", &["src/ghost.c"], &[]));

    assert_eq!(
        project.validate(&registry),
        Err(ProjectError::UnknownSource {
            target: "app".into(),
            path: "src/ghost.c".into()
        })
    );
}

#[test]
fn project_validation_rejects_overlay_that_defines_targets() {
    let registry = LanguageRegistry::alpha();
    let mut project = project(
        Some(ConfigMode::Overlay),
        vec![attachment(
            DescriptorKind::AveToml,
            DescriptorStatus::Integrated,
        )],
    );
    project.targets.push(target("app", &[], &[]));

    assert_eq!(
        project.validate(&registry),
        Err(ProjectError::OverlayDefinesTargets { name: "app".into() })
    );
}

#[test]
fn project_validation_rejects_standalone_shadowing_integrated_descriptor() {
    let registry = LanguageRegistry::alpha();
    let project = project(
        Some(ConfigMode::Standalone),
        vec![
            attachment(DescriptorKind::CMakeLists, DescriptorStatus::Integrated),
            attachment(DescriptorKind::AveToml, DescriptorStatus::BuildCapable),
        ],
    );

    assert_eq!(
        project.validate(&registry),
        Err(ProjectError::StandaloneShadowsNative {
            descriptor: "CMakeLists.txt".into()
        })
    );
}

#[test]
fn project_validation_allows_standalone_over_recognized_only_descriptor() {
    let registry = LanguageRegistry::alpha();
    let project = project(
        Some(ConfigMode::Standalone),
        vec![
            attachment(DescriptorKind::CMakeLists, DescriptorStatus::Recognized),
            attachment(DescriptorKind::AveToml, DescriptorStatus::BuildCapable),
        ],
    );

    project
        .validate(&registry)
        .expect("a recognized-only descriptor owns no targets to shadow");
}

#[test]
fn project_validation_rejects_self_dependency() {
    let registry = LanguageRegistry::alpha();
    let mut project = project(
        Some(ConfigMode::Standalone),
        vec![attachment(
            DescriptorKind::AveToml,
            DescriptorStatus::BuildCapable,
        )],
    );
    project.targets.push(target("app", &[], &["app"]));

    assert_eq!(
        project.validate(&registry),
        Err(ProjectError::SelfDependency {
            target: "app".into()
        })
    );
}

#[test]
fn workspace_rejects_relative_root() {
    assert_eq!(
        Workspace::new(vec![PathBuf::from("relative/dir")], Vec::new()),
        Err(WorkspaceError::RelativeRoot {
            root: "relative/dir".into()
        })
    );
    assert_eq!(
        Workspace::new(Vec::new(), Vec::new()),
        Err(WorkspaceError::NoRoots)
    );
}

#[test]
fn workspace_collapses_duplicate_and_nested_roots() {
    let workspace = Workspace::new(
        vec![
            root("aveloop-ws"),
            root("aveloop-ws"),
            root("aveloop-ws/native"),
            root("aveloop-ws-other"),
        ],
        Vec::new(),
    )
    .expect("valid roots");

    assert_eq!(
        workspace.roots(),
        [root("aveloop-ws"), root("aveloop-ws-other")]
    );
}

#[test]
fn workspace_rejects_escaping_exclusions() {
    assert_eq!(
        Workspace::new(
            vec![root("aveloop-ws")],
            vec![PathBuf::from("../elsewhere")]
        ),
        Err(WorkspaceError::EscapingExclusion {
            exclusion: "../elsewhere".into()
        })
    );
}

#[test]
fn workspace_rejects_project_outside_its_roots() {
    let mut workspace = Workspace::new(vec![root("aveloop-ws")], Vec::new()).expect("valid roots");
    let mut project = project(None, vec![]);
    project.root = root("somewhere-else");

    assert_eq!(
        workspace.add_project(project),
        Err(WorkspaceError::ProjectOutsideRoots {
            root: root("somewhere-else").display().to_string()
        })
    );
}

#[test]
fn workspace_resolves_the_nearest_project() {
    let mut workspace = Workspace::new(vec![root("aveloop-ws")], Vec::new()).expect("valid roots");

    let mut outer = project(None, vec![]);
    outer.name = "outer".into();
    outer.root = root("aveloop-ws");
    workspace.add_project(outer).expect("inside root");

    let mut inner = project(None, vec![]);
    inner.name = "inner".into();
    inner.root = root("aveloop-ws/native");
    workspace.add_project(inner).expect("inside root");

    let resolved = workspace
        .project_for(&root("aveloop-ws/native/src/main.c"))
        .expect("nearest project");
    assert_eq!(resolved.name, "inner");
    assert_eq!(
        workspace
            .project_for(&root("aveloop-ws/src/main.c"))
            .map(|project| project.name.as_str()),
        Some("outer")
    );
}
