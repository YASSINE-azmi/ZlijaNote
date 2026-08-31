//! WorkspaceService coordinates Workspace domain rules and WorkspaceRepository operations.
//! It does not access filesystem or Tauri APIs directly.

use std::path::{Path, PathBuf};

use crate::domain::workspace::{Workspace, WorkspaceName};
use crate::error::AppError;
use crate::infrastructure::workspace_repository::WorkspaceRepository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateWorkspaceInput {
    pub name: String,
    pub parent_path: PathBuf,
}

pub struct WorkspaceService;

impl WorkspaceService {
    pub fn create_workspace(input: CreateWorkspaceInput) -> Result<Workspace, AppError> {
        let workspace_name = WorkspaceName::new(input.name)?;

        WorkspaceRepository::create_workspace(&input.parent_path, workspace_name)
    }

    pub fn open_workspace(workspace_path: &Path) -> Result<Workspace, AppError> {
        WorkspaceRepository::open_workspace(workspace_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_create_workspace_success() {
        let temp = tempdir().unwrap();
        let input = CreateWorkspaceInput {
            name: "Personal Notes".to_string(),
            parent_path: temp.path().to_path_buf(),
        };

        let workspace = WorkspaceService::create_workspace(input).unwrap();

        assert_eq!(workspace.name().as_str(), "Personal Notes");

        let expected_path = temp.path().join("Personal Notes");
        assert_eq!(workspace.path().as_path(), expected_path);
        assert!(expected_path.is_dir());
    }

    #[test]
    fn test_create_workspace_trims_name() {
        let temp = tempdir().unwrap();
        let input = CreateWorkspaceInput {
            name: "  Personal Notes  ".to_string(),
            parent_path: temp.path().to_path_buf(),
        };

        let workspace = WorkspaceService::create_workspace(input).unwrap();

        assert_eq!(workspace.name().as_str(), "Personal Notes");

        let expected_path = temp.path().join("Personal Notes");
        assert!(expected_path.is_dir());
    }

    #[test]
    fn test_create_workspace_empty_name_returns_error_and_no_side_effects() {
        let temp = tempdir().unwrap();
        let input = CreateWorkspaceInput {
            name: "    ".to_string(),
            parent_path: temp.path().to_path_buf(),
        };

        let res = WorkspaceService::create_workspace(input);

        assert!(matches!(res, Err(AppError::WorkspaceNameIsEmpty)));
        assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 0);
    }

    #[test]
    fn test_create_workspace_unsafe_name_returns_error_and_no_side_effects() {
        let temp = tempdir().unwrap();
        let input = CreateWorkspaceInput {
            name: "../outside".to_string(),
            parent_path: temp.path().to_path_buf(),
        };

        let res = WorkspaceService::create_workspace(input);

        assert!(matches!(res, Err(AppError::WorkspaceNameIsUnsafe)));
        assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 0);
    }

    #[test]
    fn test_create_workspace_parent_not_found() {
        let temp = tempdir().unwrap();
        let missing_parent = temp.path().join("missing_dir");

        let input = CreateWorkspaceInput {
            name: "Personal Notes".to_string(),
            parent_path: missing_parent,
        };

        let res = WorkspaceService::create_workspace(input);

        assert!(matches!(res, Err(AppError::WorkspaceParentNotFound)));
    }

    #[test]
    fn test_create_workspace_parent_is_file() {
        let temp = tempdir().unwrap();
        let file_parent = temp.path().join("file.txt");
        fs::write(&file_parent, "content").unwrap();

        let input = CreateWorkspaceInput {
            name: "Personal Notes".to_string(),
            parent_path: file_parent,
        };

        let res = WorkspaceService::create_workspace(input);

        assert!(matches!(res, Err(AppError::WorkspaceParentCannotBeOpened)));
    }

    #[test]
    fn test_create_workspace_duplicate_returns_error() {
        let temp = tempdir().unwrap();

        let input = CreateWorkspaceInput {
            name: "Personal Notes".to_string(),
            parent_path: temp.path().to_path_buf(),
        };

        WorkspaceService::create_workspace(input.clone()).unwrap();

        let res = WorkspaceService::create_workspace(input);

        assert!(matches!(res, Err(AppError::WorkspaceAlreadyExists)));
    }

    #[test]
    fn test_open_workspace_success() {
        let temp = tempdir().unwrap();

        let input = CreateWorkspaceInput {
            name: "Work Notes".to_string(),
            parent_path: temp.path().to_path_buf(),
        };
        let created_ws = WorkspaceService::create_workspace(input).unwrap();

        let opened_ws = WorkspaceService::open_workspace(created_ws.path().as_path()).unwrap();

        assert_eq!(opened_ws.name(), created_ws.name());
        assert_eq!(opened_ws.path(), created_ws.path());
    }

    #[test]
    fn test_open_workspace_not_found() {
        let temp = tempdir().unwrap();
        let missing = temp.path().join("Ghost Workspace");

        let res = WorkspaceService::open_workspace(&missing);

        assert!(matches!(res, Err(AppError::WorkspaceNotFound)));
    }

    #[test]
    fn test_open_workspace_is_file() {
        let temp = tempdir().unwrap();
        let file_path = temp.path().join("workspace.txt");
        fs::write(&file_path, "not a dir").unwrap();

        let res = WorkspaceService::open_workspace(&file_path);

        assert!(matches!(res, Err(AppError::WorkspaceCannotBeOpened)));
    }
}
