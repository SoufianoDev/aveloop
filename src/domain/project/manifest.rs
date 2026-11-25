use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;

use super::language::{LanguageId, LanguageIdentity, LanguageRegistry};
use super::model::{
    DescriptorAttachment, DescriptorKind, DescriptorStatus, Project, ProjectError, SourceUnit,
    Target, TargetKind,
};

pub const SUPPORTED_FORMAT_VERSION: u32 = 1;

/// The role of an `ave.toml`. Explicit, never inferred from content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigMode {
    /// Adds settings over a native descriptor; cannot define or shadow targets.
    Overlay,
    /// Defines AveLoop-managed targets when no integrated native descriptor owns the project.
    Standalone,
}

/// The `ave.toml` document. Parsing validates the file; [`Manifest::project`] validates meaning.
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub version: u32,
    pub mode: ConfigMode,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub selected_target: Option<String>,
    #[serde(default, rename = "target")]
    pub targets: Vec<ManifestTarget>,
    /// Explicit language overrides, keyed by project-relative path.
    #[serde(default)]
    pub languages: BTreeMap<String, String>,
    #[serde(default)]
    pub exclude: Vec<PathBuf>,
    #[serde(flatten)]
    unknown: BTreeMap<String, toml::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestTarget {
    pub name: String,
    #[serde(default)]
    pub kind: TargetKind,
    #[serde(default)]
    pub entry: Option<PathBuf>,
    #[serde(default)]
    pub sources: Vec<PathBuf>,
    /// Resolves ambiguous sources for this target only.
    #[serde(default)]
    pub language: Option<String>,
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
    #[serde(default)]
    pub depends_on: Vec<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestError {
    #[error("ave.toml is invalid: {0}")]
    Invalid(String),
    #[error(
        "ave.toml declares format version {found}, but this build supports version {supported}; update AveLoop or set `version = {supported}`"
    )]
    UnsupportedVersion { found: u32, supported: u32 },
    #[error("ave.toml has no project-wide `language`; declare `language` on a `[[target]]` or override a file in `[languages]`")]
    ProjectLanguageNotAllowed,
    #[error("unknown key `{0}` in ave.toml")]
    UnknownKey(String),
    #[error(
        "`mode = \"overlay\"` cannot define targets (`{name}`); set `mode = \"standalone\"` when no integrated native descriptor owns the project"
    )]
    OverlayDefinesTargets { name: String },
    #[error("`{field}` is empty; remove it or give it a value")]
    EmptyField { field: String },
    #[error(
        "`{field}` path `{value}` is absolute; project paths must be relative to the project root"
    )]
    AbsolutePath { field: String, value: String },
    #[error("`{field}` path `{value}` escapes the project root; remove `..` components")]
    EscapingPath { field: String, value: String },
    #[error("{0}")]
    InvalidLanguageId(#[from] super::language::InvalidLanguageId),
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
        manifest.validate()?;
        Ok(manifest)
    }

    /// Builds the project this file describes. Paths stay relative to `root`.
    pub fn project(
        &self,
        root: impl Into<PathBuf>,
        registry: &LanguageRegistry,
    ) -> Result<Project, ProjectError> {
        let root = root.into();
        let name = self
            .name
            .clone()
            .unwrap_or_else(|| default_project_name(&root));

        let overrides: BTreeMap<String, LanguageId> = self
            .languages
            .iter()
            .map(|(pattern, id)| {
                LanguageId::new(id.clone())
                    .map(|id| (pattern.clone(), id))
                    .map_err(|error| ProjectError::InvalidLanguageId { id: error.value })
            })
            .collect::<Result<_, _>>()?;

        let targets: Vec<Target> = self
            .targets
            .iter()
            .map(ManifestTarget::to_target)
            .collect::<Result<_, _>>()?;

        let mut sources: Vec<SourceUnit> = Vec::new();
        for target in &targets {
            for path in &target.sources {
                if sources.iter().any(|unit| &unit.path == path) {
                    continue;
                }
                // shortcut: overrides match exact paths only; glob patterns arrive with discovery.
                let explicit = overrides.get(&path.to_string_lossy().to_string()).cloned();
                let language = LanguageIdentity::determine(path, explicit, registry);
                sources.push(SourceUnit {
                    path: path.clone(),
                    language,
                });
            }
        }

        let attachments = vec![DescriptorAttachment {
            kind: DescriptorKind::AveToml,
            path: PathBuf::from(DescriptorKind::AveToml.file_name()),
            mode: Some(self.mode),
            status: match self.mode {
                ConfigMode::Overlay => DescriptorStatus::Integrated,
                ConfigMode::Standalone => DescriptorStatus::BuildCapable,
            },
        }];

        let project = Project {
            name,
            root,
            config_mode: Some(self.mode),
            selected_target: self.selected_target.clone(),
            attachments,
            targets,
            sources,
        };
        project.validate(registry)?;
        Ok(project)
    }

    fn validate(&self) -> Result<(), ManifestError> {
        if self.unknown.contains_key("language") {
            return Err(ManifestError::ProjectLanguageNotAllowed);
        }
        if let Some(key) = self.unknown.keys().next() {
            return Err(ManifestError::UnknownKey(key.clone()));
        }
        if self.mode == ConfigMode::Overlay {
            if let Some(target) = self.targets.first() {
                return Err(ManifestError::OverlayDefinesTargets {
                    name: target.name.clone(),
                });
            }
        }

        for (pattern, language) in &self.languages {
            validate_project_path("languages", Path::new(pattern))?;
            LanguageId::new(language.clone())?;
        }
        for exclusion in &self.exclude {
            validate_project_path("exclude", exclusion)?;
        }
        for (index, target) in self.targets.iter().enumerate() {
            target.validate(index)?;
        }
        Ok(())
    }
}

impl ManifestTarget {
    fn validate(&self, index: usize) -> Result<(), ManifestError> {
        if self.name.trim().is_empty() {
            return Err(ManifestError::EmptyField {
                field: format!("targets[{index}].name"),
            });
        }
        for (field, path) in [
            ("entry", self.entry.as_ref()),
            ("working_directory", self.working_directory.as_ref()),
        ] {
            if let Some(path) = path {
                validate_project_path(&format!("targets[{index}].{field}"), path)?;
            }
        }
        for path in &self.sources {
            validate_project_path(&format!("targets[{index}].sources"), path)?;
        }
        for path in &self.include_dirs {
            validate_project_path(&format!("targets[{index}].include_dirs"), path)?;
        }
        for dependency in &self.depends_on {
            if dependency.trim().is_empty() {
                return Err(ManifestError::EmptyField {
                    field: format!("targets[{index}].depends_on"),
                });
            }
        }
        if let Some(language) = &self.language {
            LanguageId::new(language.clone())?;
        }
        Ok(())
    }

    fn to_target(&self) -> Result<Target, ProjectError> {
        let language_hint = self
            .language
            .as_ref()
            .map(|id| {
                LanguageId::new(id.clone())
                    .map_err(|error| ProjectError::InvalidLanguageId { id: error.value })
            })
            .transpose()?;
        Ok(Target {
            name: self.name.clone(),
            kind: self.kind,
            owner: DescriptorKind::AveToml,
            entry: self.entry.clone(),
            sources: self.sources.clone(),
            language_hint,
            include_dirs: self.include_dirs.clone(),
            defines: self.defines.clone(),
            compiler_args: self.compiler_args.clone(),
            linker_args: self.linker_args.clone(),
            program_args: self.program_args.clone(),
            working_directory: self.working_directory.clone(),
            depends_on: self.depends_on.clone(),
        })
    }
}

fn default_project_name(root: &Path) -> String {
    root.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| root.display().to_string())
}

fn validate_project_path(field: &str, path: &Path) -> Result<(), ManifestError> {
    if path.is_absolute() {
        return Err(ManifestError::AbsolutePath {
            field: field.to_string(),
            value: path.display().to_string(),
        });
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(ManifestError::EscapingPath {
            field: field.to_string(),
            value: path.display().to_string(),
        });
    }
    Ok(())
}
