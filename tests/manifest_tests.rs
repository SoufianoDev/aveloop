use std::path::PathBuf;

use aveloop::domain::{
    ConfigMode, DescriptorStatus, LanguageId, LanguageIdentity, LanguageRegistry, Manifest,
    ManifestError, ProjectError, TargetKind, SUPPORTED_FORMAT_VERSION,
};

const MINIMAL: &str = r#"
version = 1
mode = "standalone"
name = "demo"

[[target]]
name = "app"
entry = "src/main.c"
sources = ["src/main.c"]
"#;

const FULL: &str = r#"
version = 1
mode = "standalone"
name = "engine"
selected_target = "app"
exclude = ["build", "target"]

[languages]
"src/engine.h" = "cpp"

[[target]]
name = "app"
kind = "executable"
entry = "src/main.cpp"
sources = ["src/main.cpp", "src/engine.c", "src/engine.h"]
language = "cpp"
include_dirs = ["include"]
defines = ["DEBUG=1"]
compiler_args = ["-O2"]
linker_args = ["-lm"]
program_args = ["--fast"]
working_directory = "build"

[[target]]
name = "tool"
kind = "auxiliary-tool"
sources = ["tools/gen.c"]
depends_on = ["app"]
"#;

fn root() -> PathBuf {
    std::env::temp_dir().join("aveloop-manifest-tests")
}

#[test]
fn parses_minimal_standalone_manifest() {
    let manifest = Manifest::parse(MINIMAL).expect("valid manifest");

    assert_eq!(manifest.version, SUPPORTED_FORMAT_VERSION);
    assert_eq!(manifest.mode, ConfigMode::Standalone);
    assert_eq!(manifest.name.as_deref(), Some("demo"));
    assert_eq!(manifest.targets.len(), 1);
    assert_eq!(manifest.targets[0].name, "app");
    assert_eq!(manifest.targets[0].kind, TargetKind::Executable);
    assert_eq!(
        manifest.targets[0]
            .entry
            .as_deref()
            .and_then(|path| path.to_str()),
        Some("src/main.c")
    );
    assert!(manifest.languages.is_empty());
    assert!(manifest.exclude.is_empty());
}

#[test]
fn parses_full_manifest_with_two_targets() {
    let manifest = Manifest::parse(FULL).expect("valid manifest");

    assert_eq!(manifest.targets.len(), 2);
    assert_eq!(manifest.targets[0].kind, TargetKind::Executable);
    assert_eq!(manifest.targets[1].kind, TargetKind::AuxiliaryTool);
    assert_eq!(manifest.targets[1].depends_on, ["app"]);
    assert_eq!(manifest.targets[0].defines, ["DEBUG=1"]);
    assert_eq!(manifest.targets[0].compiler_args, ["-O2"]);
    assert_eq!(manifest.targets[0].linker_args, ["-lm"]);
    assert_eq!(manifest.targets[0].program_args, ["--fast"]);
    assert_eq!(
        manifest.exclude,
        [PathBuf::from("build"), PathBuf::from("target")]
    );
    assert_eq!(manifest.languages.len(), 1);

    let project = manifest
        .project(root(), &LanguageRegistry::alpha())
        .expect("valid project");

    assert_eq!(project.attachments[0].kind.file_name(), "ave.toml");
    assert_eq!(
        project.attachments[0].status,
        DescriptorStatus::BuildCapable
    );
    assert_eq!(project.targets.len(), 2);
    assert_eq!(project.selected_target.as_deref(), Some("app"));
}

#[test]
fn parses_overlay_without_targets() {
    let manifest = Manifest::parse(
        r#"
version = 1
mode = "overlay"
name = "engine"
selected_target = "native-app"
"#,
    )
    .expect("valid overlay");

    assert_eq!(manifest.mode, ConfigMode::Overlay);
    assert!(manifest.targets.is_empty());

    let project = manifest
        .project(root(), &LanguageRegistry::alpha())
        .expect("valid project");
    assert_eq!(project.attachments[0].status, DescriptorStatus::Integrated);
    assert!(project.targets.is_empty());
}

#[test]
fn rejects_unsupported_version() {
    let error = Manifest::parse(&MINIMAL.replace("version = 1", "version = 2")).unwrap_err();

    assert_eq!(
        error,
        ManifestError::UnsupportedVersion {
            found: 2,
            supported: SUPPORTED_FORMAT_VERSION
        }
    );
    assert!(error.to_string().contains("supports version 1"));
}

#[test]
fn rejects_missing_mode() {
    let error = Manifest::parse(&MINIMAL.replace("mode = \"standalone\"\n", "")).unwrap_err();

    assert!(matches!(error, ManifestError::Invalid(_)));
    assert!(error.to_string().contains("mode"));
}

#[test]
fn rejects_absolute_paths() {
    let absolute = std::env::temp_dir().join("main.c");
    let escaped = absolute.display().to_string().replace('\\', "\\\\");
    let source = MINIMAL.replace("\"src/main.c\"", &format!("\"{escaped}\""));

    let error = Manifest::parse(&source).unwrap_err();

    assert_eq!(
        error,
        ManifestError::AbsolutePath {
            field: "targets[0].entry".into(),
            value: absolute.display().to_string()
        }
    );
    assert!(error.to_string().contains("relative to the project root"));
}

#[test]
fn rejects_paths_escaping_project_root() {
    let error = Manifest::parse(&MINIMAL.replace("\"src/main.c\"", "\"../main.c\"")).unwrap_err();

    assert_eq!(
        error,
        ManifestError::EscapingPath {
            field: "targets[0].entry".into(),
            value: "../main.c".into()
        }
    );
}

#[test]
fn rejects_unknown_keys() {
    let error = Manifest::parse(&MINIMAL.replace(
        "mode = \"standalone\"",
        "mode = \"standalone\"\ntelemetry = true",
    ))
    .unwrap_err();

    assert_eq!(error, ManifestError::UnknownKey("telemetry".into()));
    assert!(error.to_string().contains("telemetry"));
}

#[test]
fn rejects_unknown_target_keys() {
    let error = Manifest::parse(&MINIMAL.replace(
        "sources = [\"src/main.c\"]",
        "sources = [\"src/main.c\"]\ntelemetry = true",
    ))
    .unwrap_err();

    assert!(matches!(error, ManifestError::Invalid(_)));
    assert!(error.to_string().contains("telemetry"));
}

#[test]
fn rejects_project_level_language() {
    let error = Manifest::parse(&MINIMAL.replace("[[target]]", "language = \"c\"\n\n[[target]]"))
        .unwrap_err();

    assert_eq!(error, ManifestError::ProjectLanguageNotAllowed);
    assert!(error.to_string().contains("no project-wide `language`"));
}

#[test]
fn rejects_overlay_mode_with_targets() {
    let error = Manifest::parse(&MINIMAL.replace("mode = \"standalone\"", "mode = \"overlay\""))
        .unwrap_err();

    assert_eq!(
        error,
        ManifestError::OverlayDefinesTargets { name: "app".into() }
    );
    assert!(error.to_string().contains("standalone"));
}

#[test]
fn rejects_invalid_language_id_syntax() {
    let error =
        Manifest::parse(&MINIMAL.replace("sources = [\"src/main.c\"]", "language = \"C++\""))
            .unwrap_err();

    assert!(matches!(error, ManifestError::InvalidLanguageId(_)));
    assert!(error.to_string().contains("C++"));
}

#[test]
fn project_resolves_per_source_language_identity() {
    let registry = LanguageRegistry::alpha();
    let project = Manifest::parse(FULL)
        .expect("valid manifest")
        .project(root(), &registry)
        .expect("valid project");

    let explicit = project
        .source(&PathBuf::from("src/engine.h"))
        .expect("declared source");
    assert_eq!(
        explicit.language,
        LanguageIdentity::Explicit(LanguageId::new("cpp").unwrap())
    );

    let registered = project
        .source(&PathBuf::from("src/engine.c"))
        .expect("declared source");
    assert_eq!(
        registered.language,
        LanguageIdentity::Registered(LanguageId::new("c").unwrap())
    );
}

#[test]
fn derives_target_languages_from_membership() {
    let registry = LanguageRegistry::alpha();
    let project = Manifest::parse(
        r#"
version = 1
mode = "standalone"

[[target]]
name = "app"
sources = ["src/main.cpp", "src/engine.c"]

[[target]]
name = "tool"
sources = ["tools/gen.go"]
"#,
    )
    .expect("valid manifest")
    .project(root(), &registry)
    .expect("valid project");

    assert_eq!(
        project.target_languages("app"),
        [
            LanguageId::new("c").unwrap(),
            LanguageId::new("cpp").unwrap()
        ]
    );
    assert!(project.target_languages("tool").is_empty());
    assert_eq!(
        project
            .source(&PathBuf::from("tools/gen.go"))
            .unwrap()
            .language,
        LanguageIdentity::Unknown
    );
}

#[test]
fn rejects_unregistered_language_hint() {
    let registry = LanguageRegistry::alpha();
    let error = Manifest::parse(&FULL.replace("language = \"cpp\"", "language = \"rust\""))
        .expect("valid manifest")
        .project(root(), &registry)
        .unwrap_err();

    assert_eq!(
        error,
        ProjectError::UnregisteredLanguage { id: "rust".into() }
    );
    assert!(error.to_string().contains("not registered"));
}

#[test]
fn rejects_duplicate_target_names() {
    let registry = LanguageRegistry::alpha();
    let error = Manifest::parse(&format!(
        "{MINIMAL}\n[[target]]\nname = \"app\"\nsources = [\"src/other.c\"]"
    ))
    .expect("valid manifest")
    .project(root(), &registry)
    .unwrap_err();

    assert_eq!(error, ProjectError::DuplicateTarget { name: "app".into() });
}

#[test]
fn rejects_unknown_target_dependency() {
    let registry = LanguageRegistry::alpha();
    let error =
        Manifest::parse(&FULL.replace("depends_on = [\"app\"]", "depends_on = [\"ghost\"]"))
            .expect("valid manifest")
            .project(root(), &registry)
            .unwrap_err();

    assert_eq!(
        error,
        ProjectError::UnknownDependency {
            target: "tool".into(),
            dependency: "ghost".into()
        }
    );
}
