use std::fs;
use std::path::Path;

use crate::application::app_config::{AppConfig, APP_CONFIG_SCHEMA_VERSION};
use crate::error::AppError;

pub const CONFIG_FILE_NAME: &str = "app_config.json";

pub struct AppConfigStore;

impl AppConfigStore {
    pub fn load(config_dir: &Path) -> Result<AppConfig, AppError> {
        let config_path = config_dir.join(CONFIG_FILE_NAME);

        if !config_path.exists() {
            return Ok(AppConfig::default());
        }

        let content =
            fs::read_to_string(&config_path).map_err(|_| AppError::AppConfigCannotBeRead)?;

        let config: AppConfig =
            serde_json::from_str(&content).map_err(|_| AppError::AppConfigIsInvalid)?;

        if config.schema_version != APP_CONFIG_SCHEMA_VERSION {
            return Err(AppError::UnsupportedAppConfigSchema);
        }

        Ok(config)
    }

    pub fn save(config_dir: &Path, config: &AppConfig) -> Result<(), AppError> {
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).map_err(|_| AppError::AppConfigCannotBeWritten)?;
        }

        let config_path = config_dir.join(CONFIG_FILE_NAME);

        let content =
            serde_json::to_string_pretty(config).map_err(|_| AppError::AppConfigCannotBeWritten)?;

        fs::write(&config_path, content).map_err(|_| AppError::AppConfigCannotBeWritten)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_load_missing_returns_default() {
        let temp = tempdir().unwrap();
        let config = AppConfigStore::load(temp.path()).unwrap();

        assert_eq!(config, AppConfig::default());
    }

    #[test]
    fn test_save_and_load_success() {
        let temp = tempdir().unwrap();

        let mut original_config = AppConfig::default();
        original_config.set_last_opened_workspace(PathBuf::from("/test/workspace"));

        AppConfigStore::save(temp.path(), &original_config).unwrap();

        let loaded_config = AppConfigStore::load(temp.path()).unwrap();

        assert_eq!(original_config, loaded_config);
    }

    #[test]
    fn test_save_creates_config_directory_automatically() {
        let temp = tempdir().unwrap();
        let nested_dir = temp.path().join("zlijanote").join("config");

        let config = AppConfig::default();
        AppConfigStore::save(&nested_dir, &config).unwrap();

        let expected_file = nested_dir.join(CONFIG_FILE_NAME);
        assert!(nested_dir.exists());
        assert!(expected_file.exists());
    }

    #[test]
    fn test_load_invalid_json_returns_error() {
        let temp = tempdir().unwrap();
        let config_path = temp.path().join(CONFIG_FILE_NAME);

        fs::write(&config_path, "{ invalid_json: ").unwrap();

        let res = AppConfigStore::load(temp.path());
        assert!(matches!(res, Err(AppError::AppConfigIsInvalid)));
    }

    #[test]
    fn test_load_unsupported_schema_version_returns_error() {
        let temp = tempdir().unwrap();
        let config_path = temp.path().join(CONFIG_FILE_NAME);

        let future_config = r#"{
            "schema_version": 999,
            "last_opened_workspace": null,
            "recent_workspaces": []
        }"#;
        fs::write(&config_path, future_config).unwrap();

        let res = AppConfigStore::load(temp.path());
        assert!(matches!(res, Err(AppError::UnsupportedAppConfigSchema)));
    }

    #[test]
    fn test_save_fails_when_config_dir_is_a_file() {
        let temp = tempdir().unwrap();
        let fake_dir = temp.path().join("fake_dir.txt");

        fs::write(&fake_dir, "I am a file").unwrap();

        let config = AppConfig::default();

        let res = AppConfigStore::save(&fake_dir, &config);
        assert!(matches!(res, Err(AppError::AppConfigCannotBeWritten)));
    }
}
