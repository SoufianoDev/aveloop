use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;

pub const SUPPORTED_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub version: u32,
    pub name: String,
    pub language: Language,
    pub entry: PathBuf,
    #[serde(default)]
    pub sources: Vec<PathBuf>,
    #[serde(default)]
    pub include_dirs: Vec<PathBuf>,
    #[serde(default)]
    pub defines: Vec<String>,
    #[serde(default)]
    pub compiler_args: Vec<String>,
    #[serde(default)]
    pub linker_args: Vec<String>,
    #[serde(default)]
    pub program_args: Vec<String>,
    #[serde(default)]
    pub working_directory: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    C,
    Cpp,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestError {
    #[error("ave.toml is invalid: {0}")]
    Invalid(String),
    #[error(
        "ave.toml declares format version {found}, but this build supports version {supported}; update AveLoop or set `version = {supported}`"
    )]
    UnsupportedVersion { found: u32, supported: u32 },
    #[error(
        "`{field}` path `{value}` is absolute; project paths must be relative to the project root"
    )]
    AbsolutePath { field: &'static str, value: String },
    #[error("`{field}` path `{value}` escapes the project root; remove `..` components")]
    EscapingPath { field: &'static str, value: String },
}

impl Manifest {
    pub fn parse(source: &str) -> Result<Self, ManifestError> {
        #[derive(Deserialize)]
        struct VersionProbe {
            version: u32,
        }

        let probe: VersionProbe =
            toml::from_str(source).map_err(|error| ManifestError::Invalid(error.to_string()))?;
        if probe.version != SUPPORTED_FORMAT_VERSION {
            return Err(ManifestError::UnsupportedVersion {
                found: probe.version,
                supported: SUPPORTED_FORMAT_VERSION,
            });
        }

        let manifest: Self =
            toml::from_str(source).map_err(|error| ManifestError::Invalid(error.to_string()))?;
        manifest.validate_paths()?;
        Ok(manifest)
    }

    fn validate_paths(&self) -> Result<(), ManifestError> {
        let mut paths = vec![("entry", self.entry.as_path())];
        paths.extend(self.sources.iter().map(|path| ("sources", path.as_path())));
        paths.extend(
            self.include_dirs
                .iter()
                .map(|path| ("include_dirs", path.as_path())),
        );
        if let Some(dir) = &self.working_directory {
            paths.push(("working_directory", dir.as_path()));
        }

        for (field, path) in paths {
            validate_project_path(field, path)?;
        }
        Ok(())
    }
}

fn validate_project_path(field: &'static str, path: &Path) -> Result<(), ManifestError> {
    if path.is_absolute() {
        return Err(ManifestError::AbsolutePath {
            field,
            value: path.display().to_string(),
        });
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(ManifestError::EscapingPath {
            field,
            value: path.display().to_string(),
        });
    }
    Ok(())
}
