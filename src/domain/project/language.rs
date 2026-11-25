use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use thiserror::Error;

pub const MAX_LANGUAGE_ID_LEN: usize = 32;

/// Extensible language identifier. Alpha registers `c` and `cpp`; the type does not.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageId(String);

impl LanguageId {
    pub fn new(id: impl Into<String>) -> Result<Self, InvalidLanguageId> {
        let value = id.into();
        let valid = !value.is_empty()
            && value.len() <= MAX_LANGUAGE_ID_LEN
            && value.chars().next().is_some_and(|c| c.is_ascii_lowercase())
            && value
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '-' | '+'));
        if valid {
            Ok(Self(value))
        } else {
            Err(InvalidLanguageId { value })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LanguageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error(
    "`{value}` is not a valid language id; use lowercase ASCII letters, digits, `-`, or `+` (max {MAX_LANGUAGE_ID_LEN} characters)"
)]
pub struct InvalidLanguageId {
    pub value: String,
}

/// What one source unit's language is, and how certain that is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageIdentity {
    /// Set by an explicit `ave.toml` override; wins over any extension mapping.
    Explicit(LanguageId),
    /// Derived from a registered extension mapping.
    Registered(LanguageId),
    /// A known extension owned by several languages, such as `.h`. Never guessed.
    Ambiguous(Vec<LanguageId>),
    /// No registered mapping. The file stays editable as plain text.
    Unknown,
}

impl LanguageIdentity {
    /// Resolves in order: explicit override, registered extension, ambiguous, unknown.
    pub fn determine(
        path: &Path,
        explicit: Option<LanguageId>,
        registry: &LanguageRegistry,
    ) -> Self {
        if let Some(id) = explicit {
            return Self::Explicit(id);
        }
        let extension = path.extension().and_then(|ext| ext.to_str());
        match extension.map(|ext| registry.resolve_extension(ext)) {
            Some(ExtensionResolution::Single(id)) => Self::Registered(id),
            Some(ExtensionResolution::Ambiguous(candidates)) => Self::Ambiguous(candidates),
            _ => Self::Unknown,
        }
    }

    /// The resolved language, absent for ambiguous and unknown identities.
    pub fn language(&self) -> Option<&LanguageId> {
        match self {
            Self::Explicit(id) | Self::Registered(id) => Some(id),
            Self::Ambiguous(_) | Self::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionResolution {
    Single(LanguageId),
    Ambiguous(Vec<LanguageId>),
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CapabilityStatus {
    #[default]
    Unavailable,
    Partial,
    Supported,
}

/// Declared one capability at a time so recognition never implies support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LanguageCapabilities {
    pub recognition: CapabilityStatus,
    pub highlighting: CapabilityStatus,
    pub parsing: CapabilityStatus,
    pub editing: CapabilityStatus,
    pub diagnostics: CapabilityStatus,
    pub lsp: CapabilityStatus,
    pub build_integration: CapabilityStatus,
    pub toolchain_discovery: CapabilityStatus,
    pub execution: CapabilityStatus,
    pub debugging: CapabilityStatus,
    pub runtime_observation: CapabilityStatus,
    pub visualization: CapabilityStatus,
}

impl LanguageCapabilities {
    /// Alpha's C17 and C++20 posture: everything except LSP, with compiler-only diagnostics.
    pub const ALPHA: Self = Self {
        recognition: CapabilityStatus::Supported,
        highlighting: CapabilityStatus::Supported,
        parsing: CapabilityStatus::Supported,
        editing: CapabilityStatus::Supported,
        diagnostics: CapabilityStatus::Partial,
        lsp: CapabilityStatus::Unavailable,
        build_integration: CapabilityStatus::Supported,
        toolchain_discovery: CapabilityStatus::Supported,
        execution: CapabilityStatus::Supported,
        debugging: CapabilityStatus::Supported,
        runtime_observation: CapabilityStatus::Supported,
        visualization: CapabilityStatus::Supported,
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageIntegration {
    pub id: LanguageId,
    pub display_name: String,
    pub extensions: Vec<String>,
    pub capabilities: LanguageCapabilities,
}

#[derive(Debug, Clone, Default)]
pub struct LanguageRegistry {
    integrations: Vec<LanguageIntegration>,
    ambiguous: BTreeMap<String, Vec<LanguageId>>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// The only integrations Alpha registers; everything else is an unknown extension.
    pub fn alpha() -> Self {
        let mut registry = Self::new();
        let c = LanguageId::new("c").expect("built-in language id");
        let cpp = LanguageId::new("cpp").expect("built-in language id");
        registry
            .register(LanguageIntegration {
                id: c.clone(),
                display_name: "C17".into(),
                extensions: vec!["c".into()],
                capabilities: LanguageCapabilities::ALPHA,
            })
            .expect("built-in integration");
        registry
            .register(LanguageIntegration {
                id: cpp.clone(),
                display_name: "C++20".into(),
                extensions: ["cpp", "cc", "cxx", "hpp", "hxx", "hh"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                capabilities: LanguageCapabilities::ALPHA,
            })
            .expect("built-in integration");
        registry
            .add_ambiguous_extension("h", vec![c, cpp])
            .expect("built-in ambiguity");
        registry
    }

    pub fn register(&mut self, integration: LanguageIntegration) -> Result<(), RegistryError> {
        if self.integration(integration.id.as_str()).is_some() {
            return Err(RegistryError::DuplicateId(integration.id.to_string()));
        }
        for extension in &integration.extensions {
            if let Some(owner) = self.extension_owner(extension) {
                return Err(RegistryError::DuplicateExtension {
                    extension: extension.clone(),
                    owner: owner.to_string(),
                });
            }
        }
        self.integrations.push(integration);
        Ok(())
    }

    pub fn add_ambiguous_extension(
        &mut self,
        extension: &str,
        candidates: Vec<LanguageId>,
    ) -> Result<(), RegistryError> {
        if let Some(owner) = self.extension_owner(extension) {
            return Err(RegistryError::DuplicateExtension {
                extension: extension.to_string(),
                owner: owner.to_string(),
            });
        }
        if self.ambiguous.contains_key(extension) {
            return Err(RegistryError::DuplicateExtension {
                extension: extension.to_string(),
                owner: "an ambiguous mapping".into(),
            });
        }
        if let Some(missing) = candidates
            .iter()
            .find(|id| self.integration(id.as_str()).is_none())
        {
            return Err(RegistryError::UnregisteredCandidate(missing.to_string()));
        }
        self.ambiguous
            .insert(extension.to_ascii_lowercase(), candidates);
        Ok(())
    }

    pub fn integration(&self, id: &str) -> Option<&LanguageIntegration> {
        self.integrations
            .iter()
            .find(|entry| entry.id.as_str() == id)
    }

    pub fn supports(&self, id: &LanguageId) -> bool {
        self.integration(id.as_str()).is_some()
    }

    pub fn integrations(&self) -> &[LanguageIntegration] {
        &self.integrations
    }

    pub fn resolve_extension(&self, extension: &str) -> ExtensionResolution {
        let extension = extension.trim_start_matches('.').to_ascii_lowercase();
        if let Some(candidates) = self.ambiguous.get(&extension) {
            return ExtensionResolution::Ambiguous(candidates.clone());
        }
        match self.extension_owner(&extension) {
            Some(id) => ExtensionResolution::Single(id.clone()),
            None => ExtensionResolution::Unknown,
        }
    }

    fn extension_owner(&self, extension: &str) -> Option<&LanguageId> {
        let extension = extension.trim_start_matches('.').to_ascii_lowercase();
        self.integrations
            .iter()
            .find(|entry| entry.extensions.iter().any(|owned| owned == &extension))
            .map(|entry| &entry.id)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RegistryError {
    #[error("language id `{0}` is already registered")]
    DuplicateId(String),
    #[error("extension `{extension}` is already claimed by `{owner}`")]
    DuplicateExtension { extension: String, owner: String },
    #[error("ambiguous extension candidate `{0}` is not registered")]
    UnregisteredCandidate(String),
}
