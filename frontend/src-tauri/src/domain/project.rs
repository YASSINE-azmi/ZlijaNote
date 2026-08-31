//! This module contains the domain model for a project inside a workspace.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(name: impl Into<String>) -> Result<Self, AppError> {
        let raw_name = name.into();
        let trimmed = raw_name.trim();

        if trimmed.is_empty() {
            return Err(AppError::ProjectNameIsEmpty);
        }

        if trimmed == "." || trimmed == ".." {
            return Err(AppError::ProjectNameIsUnsafe);
        }

        if trimmed.chars().any(|c| {
            c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
        }) {
            return Err(AppError::ProjectNameIsUnsafe);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ProjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub schema_version: u32,
    pub project_id: Uuid,
    pub name: ProjectName,
    pub description: String,

    pub banner_path: Option<PathBuf>,

    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

impl ProjectMetadata {
    pub fn new(name: ProjectName, description: String, now: OffsetDateTime) -> Self {
        Self {
            schema_version: 1,
            project_id: Uuid::new_v4(),
            name,
            description,
            banner_path: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    metadata: ProjectMetadata,
    path: PathBuf,
}

impl Project {
    pub fn new(metadata: ProjectMetadata, path: PathBuf) -> Self {
        Self { metadata, path }
    }

    pub fn metadata(&self) -> &ProjectMetadata {
        &self.metadata
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_project_names() {
        assert!(ProjectName::new("Rust Learning").is_ok());
        let name = ProjectName::new("  My Project  ").unwrap();
        assert_eq!(name.as_str(), "My Project");
    }

    #[test]
    fn test_invalid_project_names() {
        assert!(matches!(
            ProjectName::new(""),
            Err(AppError::ProjectNameIsEmpty)
        ));
        assert!(matches!(
            ProjectName::new("."),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("proj/name"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
    }

    #[test]
    fn test_project_metadata_serde_roundtrip() {
        let name = ProjectName::new("Rust Learning").unwrap();
        let now = time::OffsetDateTime::now_utc();
        let metadata = ProjectMetadata::new(name, "Learning Rust step by step".to_string(), now);

        let json_output =
            serde_json::to_string_pretty(&metadata).expect("Failed to serialize metadata");

        assert!(json_output.contains("\"schema_version\": 1"));
        assert!(json_output.contains("\"name\": \"Rust Learning\""));

        let deserialized: ProjectMetadata =
            serde_json::from_str(&json_output).expect("Failed to deserialize metadata");

        assert_eq!(metadata.schema_version, deserialized.schema_version);
        assert_eq!(metadata.project_id, deserialized.project_id);
        assert_eq!(metadata.name, deserialized.name);
        assert_eq!(metadata.description, deserialized.description);
        assert_eq!(
            metadata.created_at.unix_timestamp(),
            deserialized.created_at.unix_timestamp()
        );
    }
}
