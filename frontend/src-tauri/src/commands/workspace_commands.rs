use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::application::workspace_service::{CreateWorkspaceInput, WorkspaceService};
use crate::domain::workspace::Workspace;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub parent_path: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OpenWorkspaceRequest {
    pub workspace_path: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceResponse {
    pub name: String,
    pub path: String,
}

impl From<Workspace> for WorkspaceResponse {
    fn from(workspace: Workspace) -> Self {
        Self {
            name: workspace.name().as_str().to_string(),
            path: workspace.path().as_path().to_string_lossy().into_owned(),
        }
    }
}

#[tauri::command]
pub fn create_workspace(
    app: AppHandle,
    request: CreateWorkspaceRequest,
) -> Result<WorkspaceResponse, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to resolve app config directory: {}", e))?;

    let input = CreateWorkspaceInput {
        name: request.name,
        parent_path: PathBuf::from(request.parent_path),
    };

    let workspace = WorkspaceService::create_workspace(input, &config_dir)
        .map_err(|e| format!("Failed to create workspace: {}", e))?;

    Ok(workspace.into())
}

#[tauri::command]
pub fn open_workspace(
    app: AppHandle,
    request: OpenWorkspaceRequest,
) -> Result<WorkspaceResponse, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to resolve app config directory: {}", e))?;

    let workspace_path = PathBuf::from(request.workspace_path);

    let workspace = WorkspaceService::open_workspace(&workspace_path, &config_dir)
        .map_err(|e| format!("Failed to open workspace: {}", e))?;

    Ok(workspace.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_workspace_request_deserializes_from_camel_case_json() {
        let json_data = r#"{
            "name": "Personal Notes",
            "parentPath": "/home/user/Documents"
        }"#;

        let request: CreateWorkspaceRequest =
            serde_json::from_str(json_data).expect("Failed to deserialize valid JSON");

        assert_eq!(request.name, "Personal Notes");
        assert_eq!(request.parent_path, "/home/user/Documents");
    }

    #[test]
    fn test_invalid_create_request_missing_parent_path_fails_deserialization() {
        let invalid_json_data = r#"{
            "name": "Personal Notes"
        }"#;

        let result: Result<CreateWorkspaceRequest, serde_json::Error> =
            serde_json::from_str(invalid_json_data);

        assert!(result.is_err());
    }

    #[test]
    fn test_open_workspace_request_deserializes_from_camel_case_json() {
        let json_data = r#"{
            "workspacePath": "/home/user/Documents/Personal Notes"
        }"#;

        let request: OpenWorkspaceRequest =
            serde_json::from_str(json_data).expect("Failed to deserialize valid JSON");

        assert_eq!(
            request.workspace_path,
            "/home/user/Documents/Personal Notes"
        );
    }

    #[test]
    fn test_workspace_service_result_converts_to_workspace_response() {
        let parent_temp = tempdir().unwrap();
        let config_temp = tempdir().unwrap();

        let input = CreateWorkspaceInput {
            name: "Work Notes".to_string(),
            parent_path: parent_temp.path().to_path_buf(),
        };

        let workspace = WorkspaceService::create_workspace(input, config_temp.path()).unwrap();
        let expected_path = workspace.path().as_path().to_string_lossy().into_owned();

        let response: WorkspaceResponse = workspace.into();

        assert_eq!(response.name, "Work Notes");
        assert_eq!(response.path, expected_path);
    }

    #[test]
    fn test_workspace_response_serializes_to_camel_case_json() {
        let response = WorkspaceResponse {
            name: "My Notes".to_string(),
            path: "/fake/path/My Notes".to_string(),
        };

        let json_value = serde_json::to_value(&response).expect("Failed to serialize to JSON");

        assert_eq!(json_value["name"], "My Notes");
        assert_eq!(json_value["path"], "/fake/path/My Notes");
    }
}
