//! This module contains filesystem operations for workspace management.
//! It provides [`WorkspaceRepository`] to open and create workspace directories.

use std::fs;
use std::path::Path;

use crate::domain::workspace::{Workspace, WorkspaceName, WorkspacePath};
use crate::error::AppError;

pub struct WorkspaceRepository;

impl WorkspaceRepository {
    pub fn open_workspace(path: &Path) -> Result<Workspace, AppError> {
        if !path.exists() {
            return Err(AppError::WorkspaceNotFound);
        }

        if !path.is_dir() {
            return Err(AppError::WorkspaceCannotBeOpened);
        }

        let folder_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or(AppError::WorkspaceCannotBeOpened)?;

        let name = WorkspaceName::new(folder_name)?;
        let workspace_path = WorkspacePath::new(path);

        Ok(Workspace::new(name, workspace_path))
    }

    pub fn create_workspace(
        parent_path: &Path,
        name: WorkspaceName,
    ) -> Result<Workspace, AppError> {
        if !parent_path.exists() {
            return Err(AppError::WorkspaceParentNotFound);
        }

        if !parent_path.is_dir() {
            return Err(AppError::WorkspaceParentCannotBeOpened);
        }

        let target_path = parent_path.join(name.as_str());

        if target_path.exists() {
            return Err(AppError::WorkspaceAlreadyExists);
        }

        fs::create_dir(&target_path)?;

        Self::open_workspace(&target_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_open_existing_workspace_success() {
        let dir = tempdir().unwrap();
        let ws_dir = dir.path().join("my_workspace");
        fs::create_dir(&ws_dir).unwrap();

        let result = WorkspaceRepository::open_workspace(&ws_dir);
        assert!(result.is_ok());

        let workspace = result.unwrap();
        assert_eq!(workspace.name().as_str(), "my_workspace");
        assert_eq!(workspace.path().as_path(), ws_dir);
    }

    #[test]
    fn test_open_workspace_not_found() {
        let dir = tempdir().unwrap();
        let non_existent = dir.path().join("ghost_workspace");

        let result = WorkspaceRepository::open_workspace(&non_existent);
        assert!(matches!(result, Err(AppError::WorkspaceNotFound)));
    }

    #[test]
    fn test_open_workspace_when_path_is_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        fs::write(&file_path, "content").unwrap();

        let result = WorkspaceRepository::open_workspace(&file_path);
        assert!(matches!(result, Err(AppError::WorkspaceCannotBeOpened)));
    }

    #[test]
    fn test_create_workspace_success() {
        let parent_dir = tempdir().unwrap();
        let name = WorkspaceName::new("New Project").unwrap();

        let result = WorkspaceRepository::create_workspace(parent_dir.path(), name.clone());
        assert!(result.is_ok());

        let workspace = result.unwrap();
        let expected_path = parent_dir.path().join("New Project");

        assert_eq!(workspace.name(), &name);
        assert_eq!(workspace.path().as_path(), expected_path);
        assert!(expected_path.is_dir());
    }

    #[test]
    fn test_create_workspace_parent_not_found() {
        let parent_dir = tempdir().unwrap();
        let non_existent_parent = parent_dir.path().join("missing_parent");
        let name = WorkspaceName::new("New Project").unwrap();

        let result = WorkspaceRepository::create_workspace(&non_existent_parent, name);
        assert!(matches!(result, Err(AppError::WorkspaceParentNotFound)));
    }

    #[test]
    fn test_create_workspace_parent_is_file() {
        let parent_dir = tempdir().unwrap();
        let file_parent = parent_dir.path().join("file.txt");
        fs::write(&file_parent, "data").unwrap();

        let name = WorkspaceName::new("New Project").unwrap();

        let result = WorkspaceRepository::create_workspace(&file_parent, name);
        assert!(matches!(
            result,
            Err(AppError::WorkspaceParentCannotBeOpened)
        ));
    }

    #[test]
    fn test_create_workspace_already_exists() {
        let parent_dir = tempdir().unwrap();
        let name = WorkspaceName::new("Existing Workspace").unwrap();

        fs::create_dir(parent_dir.path().join("Existing Workspace")).unwrap();

        let result = WorkspaceRepository::create_workspace(parent_dir.path(), name);
        assert!(matches!(result, Err(AppError::WorkspaceAlreadyExists)));
    }
}
