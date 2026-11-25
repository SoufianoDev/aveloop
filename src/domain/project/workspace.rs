use std::path::{Component, Path, PathBuf};

use thiserror::Error;

use super::model::Project;

/// The user's opened context. Holds no language, build system, or compiler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    roots: Vec<PathBuf>,
    exclusions: Vec<PathBuf>,
    projects: Vec<Project>,
}

impl Workspace {
    /// Roots must be absolute; nested roots collapse into their ancestor. Exclusions are
    /// project-relative and may not escape a root.
    pub fn new(roots: Vec<PathBuf>, exclusions: Vec<PathBuf>) -> Result<Self, WorkspaceError> {
        if roots.is_empty() {
            return Err(WorkspaceError::NoRoots);
        }
        for root in &roots {
            if !root.is_absolute() {
                return Err(WorkspaceError::RelativeRoot {
                    root: root.display().to_string(),
                });
            }
        }
        for exclusion in &exclusions {
            if exclusion.is_absolute() {
                return Err(WorkspaceError::AbsoluteExclusion {
                    exclusion: exclusion.display().to_string(),
                });
            }
            if exclusion
                .components()
                .any(|component| matches!(component, Component::ParentDir))
            {
                return Err(WorkspaceError::EscapingExclusion {
                    exclusion: exclusion.display().to_string(),
                });
            }
        }

        let mut unique: Vec<PathBuf> = Vec::new();
        for root in roots {
            if !unique.contains(&root) {
                unique.push(root);
            }
        }
        let collapsed: Vec<PathBuf> = unique
            .iter()
            .enumerate()
            .filter(|(index, root)| {
                !unique
                    .iter()
                    .enumerate()
                    .any(|(other_index, other)| other_index != *index && root.starts_with(other))
            })
            .map(|(_, root)| root.clone())
            .collect();

        Ok(Self {
            roots: collapsed,
            exclusions,
            projects: Vec::new(),
        })
    }

    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }

    pub fn exclusions(&self) -> &[PathBuf] {
        &self.exclusions
    }

    pub fn projects(&self) -> &[Project] {
        &self.projects
    }

    pub fn add_project(&mut self, project: Project) -> Result<(), WorkspaceError> {
        if !self.roots.iter().any(|root| project.root.starts_with(root)) {
            return Err(WorkspaceError::ProjectOutsideRoots {
                root: project.root.display().to_string(),
            });
        }
        if self
            .projects
            .iter()
            .any(|existing| existing.root == project.root)
        {
            return Err(WorkspaceError::DuplicateProjectRoot {
                root: project.root.display().to_string(),
            });
        }
        self.projects.push(project);
        Ok(())
    }

    /// Nearest-ancestor association: the most specific project root wins.
    pub fn project_for(&self, path: &Path) -> Option<&Project> {
        self.projects
            .iter()
            .filter(|project| path.starts_with(&project.root))
            .max_by_key(|project| project.root.components().count())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceError {
    #[error("a workspace needs at least one root directory")]
    NoRoots,
    #[error("workspace root `{root}` must be an absolute path")]
    RelativeRoot { root: String },
    #[error("workspace exclusion `{exclusion}` is absolute; exclusions are relative to a root")]
    AbsoluteExclusion { exclusion: String },
    #[error("workspace exclusion `{exclusion}` escapes its root; remove `..` components")]
    EscapingExclusion { exclusion: String },
    #[error("project root `{root}` is not inside any workspace root")]
    ProjectOutsideRoots { root: String },
    #[error("two projects claim the root `{root}`")]
    DuplicateProjectRoot { root: String },
}
