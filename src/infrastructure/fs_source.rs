//! Adapters for the application ports. Only this layer touches the OS.

use std::path::{Path, PathBuf};

use crate::application::ports::{DirEntry, EntryKind, ProjectSource, ProjectWriter, SourceError};

/// Read-only `std::fs` view used by project discovery.
#[derive(Debug, Default, Clone, Copy)]
pub struct FileSystemSource;

impl ProjectSource for FileSystemSource {
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, SourceError> {
        std::fs::canonicalize(path).map_err(|error| SourceError::new(path, error.to_string()))
    }

    fn entries(&self, directory: &Path) -> Result<Vec<DirEntry>, SourceError> {
        let read = std::fs::read_dir(directory)
            .map_err(|error| SourceError::new(directory, error.to_string()))?;
        let mut entries = Vec::new();
        for entry in read {
            let entry = entry.map_err(|error| SourceError::new(directory, error.to_string()))?;
            let path = entry.path();
            let file_type = entry
                .file_type()
                .map_err(|error| SourceError::new(&path, error.to_string()))?;
            let kind = if file_type.is_symlink() {
                EntryKind::Symlink
            } else if file_type.is_dir() {
                EntryKind::Directory
            } else {
                EntryKind::File
            };
            entries.push(DirEntry::new(path, kind));
        }
        Ok(entries)
    }

    fn read_text(&self, file: &Path) -> Result<String, SourceError> {
        std::fs::read_to_string(file).map_err(|error| SourceError::new(file, error.to_string()))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FileSystemWriter;

impl ProjectWriter for FileSystemWriter {
    fn write_text(&self, file: &Path, text: &str) -> Result<(), SourceError> {
        std::fs::write(file, text).map_err(|error| SourceError::new(file, error.to_string()))
    }
}
