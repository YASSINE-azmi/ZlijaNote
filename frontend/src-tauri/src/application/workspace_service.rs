//! WorkspaceService coordinates Workspace domain rules, WorkspaceRepository operations,
//! and AppConfigStore updates. It does not access Tauri APIs directly.

use std::path::{Path, PathBuf};

use crate::domain::workspace::{Workspace, WorkspaceName};
use crate::error::AppError;
use crate::infrastructure::app_config_store::AppConfigStore;
use crate::infrastructure::workspace_repository::WorkspaceRepository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateWorkspaceInput {
    pub name: String,
    pub parent_path: PathBuf,
}

pub struct WorkspaceService;

impl WorkspaceService {
    pub fn create_workspace(
        input: CreateWorkspaceInput,
        config_dir: &Path,
    ) -> Result<Workspace, AppError> {
        let workspace_name = WorkspaceName::new(input.name)?;

        let workspace = WorkspaceRepository::create_workspace(&input.parent_path, workspace_name)?;

        Self::update_app_config(config_dir, workspace.path().as_path())?;

        Ok(workspace)
    }

    pub fn open_workspace(workspace_path: &Path, config_dir: &Path) -> Result<Workspace, AppError> {
        let workspace = WorkspaceRepository::open_workspace(workspace_path)?;

        Self::update_app_config(config_dir, workspace.path().as_path())?;

        Ok(workspace)
    }

    fn update_app_config(config_dir: &Path, workspace_path: &Path) -> Result<(), AppError> {
        let mut config = AppConfigStore::load(config_dir)?;
        config.set_last_opened_workspace(workspace_path.to_path_buf());
        AppConfigStore::save(config_dir, &config)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_create_workspace_success_updates_config() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();

        let input = CreateWorkspaceInput {
            name: "Personal Notes".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };

        let workspace = WorkspaceService::create_workspace(input, config_temp.path()).unwrap();

        assert_eq!(workspace.name().as_str(), "Personal Notes");

        let expected_path = parent_temp.path().join("Personal Notes");
        assert_eq!(workspace.path().as_path(), expected_path);
        assert!(expected_path.is_dir());

        let config = AppConfigStore::load(config_temp.path()).unwrap();
        assert_eq!(config.last_opened_workspace, Some(expected_path.clone()));
        assert_eq!(config.recent_workspaces, vec![expected_path]);
    }

    #[test]
    fn test_create_workspace_trims_name_and_updates_config() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();

        let input = CreateWorkspaceInput {
            name: "  Personal Notes  ".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };

        let workspace = WorkspaceService::create_workspace(input, config_temp.path()).unwrap();
        let expected_path = parent_temp.path().join("Personal Notes");

        assert_eq!(workspace.name().as_str(), "Personal Notes");
        assert_eq!(workspace.path().as_path(), expected_path);
        assert!(expected_path.is_dir());

        let config = AppConfigStore::load(config_temp.path()).unwrap();
        assert_eq!(config.last_opened_workspace, Some(expected_path));
    }

    #[test]
    fn test_create_workspace_empty_name_returns_error_and_no_side_effects() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();

        let input = CreateWorkspaceInput {
            name: "    ".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };

        let res = WorkspaceService::create_workspace(input, config_temp.path());

        assert!(matches!(res, Err(AppError::WorkspaceNameIsEmpty)));
        assert_eq!(fs::read_dir(parent_temp.path()).unwrap().count(), 0);
        assert!(!config_temp.path().join("app_config.json").exists());
    }

    #[test]
    fn test_create_workspace_unsafe_name_returns_error_and_no_side_effects() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();

        let input = CreateWorkspaceInput {
            name: "../outside".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };

        let res = WorkspaceService::create_workspace(input, config_temp.path());

        assert!(matches!(res, Err(AppError::WorkspaceNameIsUnsafe)));
        assert_eq!(fs::read_dir(parent_temp.path()).unwrap().count(), 0);
        assert!(!config_temp.path().join("app_config.json").exists());
    }

    #[test]
    fn test_create_workspace_parent_not_found_has_no_side_effects() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();
        let missing_parent = parent_temp.path().join("missing_dir");

        let input = CreateWorkspaceInput {
            name: "Personal Notes".to_string(),
            parent_path: missing_parent,
        };

        let res = WorkspaceService::create_workspace(input, config_temp.path());

        assert!(matches!(res, Err(AppError::WorkspaceParentNotFound)));
        assert!(!config_temp.path().join("app_config.json").exists());
    }

    #[test]
    fn test_create_workspace_parent_is_file_has_no_side_effects() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();
        let file_parent = parent_temp.path().join("file.txt");
        fs::write(&file_parent, "content").unwrap();

        let input = CreateWorkspaceInput {
            name: "Personal Notes".to_string(),
            parent_path: file_parent,
        };

        let res = WorkspaceService::create_workspace(input, config_temp.path());

        assert!(matches!(res, Err(AppError::WorkspaceParentCannotBeOpened)));
        assert!(!config_temp.path().join("app_config.json").exists());
    }

    #[test]
    fn test_create_workspace_duplicate_returns_error_and_preserves_config() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();

        let input = CreateWorkspaceInput {
            name: "Personal Notes".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };

        WorkspaceService::create_workspace(input.clone(), config_temp.path()).unwrap();

        let res = WorkspaceService::create_workspace(input, config_temp.path());
        assert!(matches!(res, Err(AppError::WorkspaceAlreadyExists)));

        let config = AppConfigStore::load(config_temp.path()).unwrap();
        assert_eq!(config.recent_workspaces.len(), 1);
    }

    #[test]
    fn test_open_workspace_success_updates_config() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();

        let input = CreateWorkspaceInput {
            name: "Work Notes".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };
        let created_ws = WorkspaceService::create_workspace(input, config_temp.path()).unwrap();

        let opened_ws =
            WorkspaceService::open_workspace(created_ws.path().as_path(), config_temp.path())
                .unwrap();

        assert_eq!(opened_ws.name(), created_ws.name());
        assert_eq!(opened_ws.path(), created_ws.path());

        let config = AppConfigStore::load(config_temp.path()).unwrap();
        assert_eq!(
            config.last_opened_workspace,
            Some(created_ws.path().as_path().to_path_buf())
        );
        assert_eq!(config.recent_workspaces.len(), 1);
    }

    #[test]
    fn test_open_workspace_not_found_has_no_side_effects() {
        let config_temp = tempdir().unwrap();
        let missing = config_temp.path().join("Ghost Workspace");

        let res = WorkspaceService::open_workspace(&missing, config_temp.path());

        assert!(matches!(res, Err(AppError::WorkspaceNotFound)));
        assert!(!config_temp.path().join("app_config.json").exists());
    }

    #[test]
    fn test_open_workspace_is_file_has_no_side_effects() {
        let config_temp = tempdir().unwrap();
        let file_path = config_temp.path().join("workspace.txt");
        fs::write(&file_path, "not a dir").unwrap();

        let res = WorkspaceService::open_workspace(&file_path, config_temp.path());

        assert!(matches!(res, Err(AppError::WorkspaceCannotBeOpened)));
        assert!(!config_temp.path().join("app_config.json").exists());
    }

    #[test]
    fn test_create_workspace_fails_config_but_creates_folder_successfully() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();
        let invalid_config_dir = config_temp.path().join("invalid_file.txt");
        fs::write(&invalid_config_dir, "content").unwrap();

        let input = CreateWorkspaceInput {
            name: "Partial Notes".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };

        let res = WorkspaceService::create_workspace(input, &invalid_config_dir);
        assert!(matches!(res, Err(AppError::AppConfigCannotBeRead)));

        let expected_ws_path = parent_temp.path().join("Partial Notes");
        assert!(expected_ws_path.is_dir());
        assert!(expected_ws_path.join(".zlija").is_dir());
    }

    #[test]
    fn test_open_workspace_fails_config_but_workspace_is_valid() {
        let parent_temp = tempdir().unwrap();
        let valid_config_temp = tempdir().unwrap();

        let input = CreateWorkspaceInput {
            name: "Existing Notes".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };
        let created_ws =
            WorkspaceService::create_workspace(input, valid_config_temp.path()).unwrap();

        let invalid_config_dir = valid_config_temp.path().join("invalid_file.txt");
        fs::write(&invalid_config_dir, "content").unwrap();

        let res =
            WorkspaceService::open_workspace(created_ws.path().as_path(), &invalid_config_dir);
        assert!(matches!(res, Err(_)));

        assert!(created_ws.path().as_path().is_dir());
    }

    #[test]
    fn test_workspace_service_updates_recent_order_correctly() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();

        let input_a = CreateWorkspaceInput {
            name: "Workspace A".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };
        let ws_a = WorkspaceService::create_workspace(input_a, config_temp.path()).unwrap();
        let path_a = ws_a.path().as_path().to_path_buf();

        let input_b = CreateWorkspaceInput {
            name: "Workspace B".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };
        let ws_b = WorkspaceService::create_workspace(input_b, config_temp.path()).unwrap();
        let path_b = ws_b.path().as_path().to_path_buf();

        let config_after_creates = AppConfigStore::load(config_temp.path()).unwrap();
        assert_eq!(
            config_after_creates.last_opened_workspace,
            Some(path_b.clone())
        );
        assert_eq!(
            config_after_creates.recent_workspaces,
            vec![path_b.clone(), path_a.clone()]
        );

        WorkspaceService::open_workspace(&path_a, config_temp.path()).unwrap();

        let config_after_open = AppConfigStore::load(config_temp.path()).unwrap();
        assert_eq!(
            config_after_open.last_opened_workspace,
            Some(path_a.clone())
        );
        assert_eq!(config_after_open.recent_workspaces, vec![path_a, path_b]);
    }
}
