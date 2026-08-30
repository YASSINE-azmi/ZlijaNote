//! This module contains the domain model for the workspace.
//! The main entity is [`Workspace`].

use crate::error::AppError;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceName(String);

impl WorkspaceName {
    pub fn new(name: impl Into<String>) -> Result<Self, AppError> {
        let raw_name = name.into();
        let trimmed = raw_name.trim();

        if trimmed.is_empty() {
            return Err(AppError::WorkspaceNameIsEmpty);
        }

        if trimmed == "." || trimmed == ".." {
            return Err(AppError::WorkspaceNameIsUnsafe);
        }

        if trimmed.chars().any(|c| {
            c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
        }) {
            return Err(AppError::WorkspaceNameIsUnsafe);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for WorkspaceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePath(PathBuf);

impl WorkspacePath {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    name: WorkspaceName,
    path: WorkspacePath,
}

impl Workspace {
    pub fn new(name: WorkspaceName, path: WorkspacePath) -> Self {
        Self { name, path }
    }

    pub fn name(&self) -> &WorkspaceName {
        &self.name
    }

    pub fn path(&self) -> &WorkspacePath {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_workspace_names() {
        assert!(WorkspaceName::new("Zlija Workspace").is_ok());
        assert!(WorkspaceName::new("مساحة عمل Rust").is_ok());

        let name = WorkspaceName::new("  My Workspace  ").unwrap();
        assert_eq!(name.as_str(), "My Workspace");
    }

    #[test]
    fn test_invalid_empty_workspace_names() {
        assert!(matches!(
            WorkspaceName::new(""),
            Err(AppError::WorkspaceNameIsEmpty)
        ));
        assert!(matches!(
            WorkspaceName::new("     "),
            Err(AppError::WorkspaceNameIsEmpty)
        ));
    }

    #[test]
    fn test_invalid_unsafe_workspace_names() {
        assert!(matches!(
            WorkspaceName::new("."),
            Err(AppError::WorkspaceNameIsUnsafe)
        ));
        assert!(matches!(
            WorkspaceName::new(".."),
            Err(AppError::WorkspaceNameIsUnsafe)
        ));
        assert!(matches!(
            WorkspaceName::new("workspace/name"),
            Err(AppError::WorkspaceNameIsUnsafe)
        ));
        assert!(matches!(
            WorkspaceName::new("workspace\\name"),
            Err(AppError::WorkspaceNameIsUnsafe)
        ));
        assert!(matches!(
            WorkspaceName::new("workspace:name"),
            Err(AppError::WorkspaceNameIsUnsafe)
        ));
        assert!(matches!(
            WorkspaceName::new("workspace\nname"),
            Err(AppError::WorkspaceNameIsUnsafe)
        ));
    }

    #[test]
    fn test_workspace_constructor() {
        let name = WorkspaceName::new("Main Workspace").unwrap();
        let path = WorkspacePath::new("/some/path/Main Workspace");
        let workspace = Workspace::new(name.clone(), path.clone());

        assert_eq!(workspace.name, name);
        assert_eq!(workspace.path, path);
        assert_eq!(
            workspace.path.as_path(),
            Path::new("/some/path/Main Workspace")
        );
    }
}
