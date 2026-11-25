use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    pub path: PathBuf,
    pub kind: EntryKind,
}

impl DirEntry {
    pub fn new(path: impl Into<PathBuf>, kind: EntryKind) -> Self {
        Self {
            path: path.into(),
            kind,
        }
    }
}

/// Read-only view of the file system. Discovery never writes through it.
pub trait ProjectSource {
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, SourceError>;

    fn entries(&self, directory: &Path) -> Result<Vec<DirEntry>, SourceError>;

    fn read_text(&self, file: &Path) -> Result<String, SourceError>;
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("`{path}`: {message}")]
pub struct SourceError {
    pub path: PathBuf,
    pub message: String,
}

impl SourceError {
    pub fn new(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}
