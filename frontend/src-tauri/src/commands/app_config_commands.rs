use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::application::app_config::AppConfig;
use crate::infrastructure::app_config_store::AppConfigStore;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigResponse {
    pub schema_version: u32,
    pub last_opened_workspace: Option<String>,
    pub recent_workspaces: Vec<String>,
}

impl From<AppConfig> for AppConfigResponse {
    fn from(config: AppConfig) -> Self {
        Self {
            schema_version: config.schema_version,
            last_opened_workspace: config
                .last_opened_workspace
                .map(|p| p.to_string_lossy().into_owned()),
            recent_workspaces: config
                .recent_workspaces
                .into_iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
        }
    }
}

#[tauri::command]
pub fn load_app_config(app: AppHandle) -> Result<AppConfigResponse, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to resolve app config directory: {}", e))?;

    let config = AppConfigStore::load(&config_dir)
        .map_err(|e| format!("Failed to load app config: {}", e))?;

    Ok(config.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_app_config_to_response_dto_conversion() {
        let app_config = AppConfig {
            schema_version: 1,
            last_opened_workspace: Some(PathBuf::from("/fake/path/Notes")),
            recent_workspaces: vec![
                PathBuf::from("/fake/path/Notes"),
                PathBuf::from("/fake/path/Work"),
            ],
        };

        let response: AppConfigResponse = app_config.into();

        assert_eq!(response.schema_version, 1);
        assert_eq!(
            response.last_opened_workspace,
            Some("/fake/path/Notes".to_string())
        );
        assert_eq!(
            response.recent_workspaces,
            vec![
                "/fake/path/Notes".to_string(),
                "/fake/path/Work".to_string()
            ]
        );
    }

    #[test]
    fn test_app_config_response_serializes_to_camel_case_json() {
        let response = AppConfigResponse {
            schema_version: 1,
            last_opened_workspace: Some("/fake/path/Notes".to_string()),
            recent_workspaces: vec![
                "/fake/path/Notes".to_string(),
                "/fake/path/Work".to_string(),
            ],
        };

        let json_value = serde_json::to_value(&response).expect("Failed to serialize to JSON");

        assert_eq!(json_value["schemaVersion"], 1);
        assert_eq!(json_value["lastOpenedWorkspace"], "/fake/path/Notes");
        assert_eq!(json_value["recentWorkspaces"][0], "/fake/path/Notes");
        assert_eq!(json_value["recentWorkspaces"][1], "/fake/path/Work");

        assert!(json_value.get("schema_version").is_none());
        assert!(json_value.get("last_opened_workspace").is_none());
        assert!(json_value.get("recent_workspaces").is_none());
    }
}
