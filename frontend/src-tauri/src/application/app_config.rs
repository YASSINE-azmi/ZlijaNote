use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const MAX_RECENT_WORKSPACES: usize = 10;
pub const APP_CONFIG_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub schema_version: u32,
    pub last_opened_workspace: Option<PathBuf>,
    pub recent_workspaces: Vec<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            schema_version: APP_CONFIG_SCHEMA_VERSION,
            last_opened_workspace: None,
            recent_workspaces: Vec::new(),
        }
    }
}

impl AppConfig {
    pub fn set_last_opened_workspace(&mut self, path: PathBuf) {
        self.last_opened_workspace = Some(path.clone());

        self.recent_workspaces.retain(|p| p != &path);

        self.recent_workspaces.insert(0, path);

        if self.recent_workspaces.len() > MAX_RECENT_WORKSPACES {
            self.recent_workspaces.truncate(MAX_RECENT_WORKSPACES);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();

        assert_eq!(config.schema_version, 1);
        assert_eq!(config.last_opened_workspace, None);
        assert!(config.recent_workspaces.is_empty());
    }

    #[test]
    fn test_set_first_workspace() {
        let mut config = AppConfig::default();
        let path = PathBuf::from("/home/user/Documents/WorkspaceA");

        config.set_last_opened_workspace(path.clone());

        assert_eq!(config.last_opened_workspace, Some(path.clone()));
        assert_eq!(config.recent_workspaces.len(), 1);
        assert_eq!(config.recent_workspaces[0], path);
    }

    #[test]
    fn test_set_multiple_workspaces() {
        let mut config = AppConfig::default();
        let path_a = PathBuf::from("/A");
        let path_b = PathBuf::from("/B");
        let path_c = PathBuf::from("/C");

        config.set_last_opened_workspace(path_a.clone());
        config.set_last_opened_workspace(path_b.clone());
        config.set_last_opened_workspace(path_c.clone());

        assert_eq!(config.last_opened_workspace, Some(path_c.clone()));
        assert_eq!(config.recent_workspaces, vec![path_c, path_b, path_a]);
    }

    #[test]
    fn test_set_existing_workspace_moves_to_top() {
        let mut config = AppConfig::default();
        let path_a = PathBuf::from("/A");
        let path_b = PathBuf::from("/B");

        config.set_last_opened_workspace(path_a.clone());
        config.set_last_opened_workspace(path_b.clone());
        assert_eq!(
            config.recent_workspaces,
            vec![path_b.clone(), path_a.clone()]
        );

        config.set_last_opened_workspace(path_a.clone());

        assert_eq!(config.last_opened_workspace, Some(path_a.clone()));
        assert_eq!(config.recent_workspaces, vec![path_a, path_b]);
    }

    #[test]
    fn test_max_recent_workspaces() {
        let mut config = AppConfig::default();

        for i in 1..=12 {
            let path = PathBuf::from(format!("/workspace_{}", i));
            config.set_last_opened_workspace(path);
        }

        assert_eq!(config.recent_workspaces.len(), MAX_RECENT_WORKSPACES);

        assert_eq!(config.recent_workspaces[0], PathBuf::from("/workspace_12"));
        assert_eq!(config.recent_workspaces[9], PathBuf::from("/workspace_3"));

        assert!(!config
            .recent_workspaces
            .contains(&PathBuf::from("/workspace_1")));
        assert!(!config
            .recent_workspaces
            .contains(&PathBuf::from("/workspace_2")));
    }

    #[test]
    fn test_app_config_json_round_trip() {
        let mut config = AppConfig::default();
        config.set_last_opened_workspace(PathBuf::from("/test/path"));

        // Serialize
        let json = serde_json::to_string(&config).expect("Failed to serialize AppConfig");

        // Deserialize
        let deserialized: AppConfig =
            serde_json::from_str(&json).expect("Failed to deserialize AppConfig");

        assert_eq!(config, deserialized);
    }
}
