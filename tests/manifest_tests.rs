use aveloop::domain::{Language, Manifest, ManifestError, SUPPORTED_FORMAT_VERSION};

const MINIMAL: &str = r#"
version = 1
name = "demo"
language = "c"
entry = "src/main.c"
"#;

#[test]
fn parses_minimal_manifest() {
    let manifest = Manifest::parse(MINIMAL).expect("valid manifest");

    assert_eq!(manifest.version, SUPPORTED_FORMAT_VERSION);
    assert_eq!(manifest.name, "demo");
    assert_eq!(manifest.language, Language::C);
    assert_eq!(manifest.entry.to_str(), Some("src/main.c"));
    assert!(manifest.sources.is_empty());
    assert!(manifest.include_dirs.is_empty());
    assert!(manifest.working_directory.is_none());
}

#[test]
fn parses_full_manifest() {
    let manifest = Manifest::parse(
        r#"
version = 1
name = "demo"
language = "cpp"
entry = "src/main.cpp"
sources = ["src/other.cpp"]
include_dirs = ["include"]
defines = ["DEBUG=1"]
compiler_args = ["-O2"]
linker_args = ["-lm"]
program_args = ["--fast"]
working_directory = "build"
"#,
    )
    .expect("valid manifest");

    assert_eq!(manifest.language, Language::Cpp);
    assert_eq!(manifest.sources.len(), 1);
    assert_eq!(manifest.include_dirs.len(), 1);
    assert_eq!(manifest.defines, ["DEBUG=1"]);
    assert_eq!(manifest.compiler_args, ["-O2"]);
    assert_eq!(manifest.linker_args, ["-lm"]);
    assert_eq!(manifest.program_args, ["--fast"]);
    assert_eq!(
        manifest
            .working_directory
            .as_deref()
            .and_then(|dir| dir.to_str()),
        Some("build")
    );
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
fn rejects_absolute_paths() {
    let absolute = std::env::temp_dir().join("main.c");
    let escaped = absolute.display().to_string().replace('\\', "\\\\");
    let source = MINIMAL.replace("\"src/main.c\"", &format!("\"{escaped}\""));

    let error = Manifest::parse(&source).unwrap_err();

    assert_eq!(
        error,
        ManifestError::AbsolutePath {
            field: "entry",
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
            field: "entry",
            value: "../main.c".into()
        }
    );
}

#[test]
fn rejects_unknown_keys() {
    let error = Manifest::parse(&format!("{MINIMAL}\ntelemetry = true")).unwrap_err();

    assert!(matches!(error, ManifestError::Invalid(_)));
    assert!(error.to_string().contains("telemetry"));
}

#[test]
fn rejects_unknown_language() {
    let error =
        Manifest::parse(&MINIMAL.replace("language = \"c\"", "language = \"rust\"")).unwrap_err();

    assert!(matches!(error, ManifestError::Invalid(_)));
    assert!(error.to_string().contains("rust"));
}
