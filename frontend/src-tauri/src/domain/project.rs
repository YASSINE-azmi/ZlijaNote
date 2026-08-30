//! This module contains the domain entities related to projects.
//! The main entities are [`Project`] and [`ProjectMetadata`].
//! This module also provides utility functions for validating project names.
use crate::error::AppError;

use std::path::PathBuf;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(name: impl Into<String>) -> Result<Self, AppError> {
        let name = name.into();
        let trimmed = name.trim();

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectMetadata {
    pub schema_version: u32,
    pub project_id: Uuid,
    pub name: ProjectName,
    pub description: String,
    pub banner_path: Option<PathBuf>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl ProjectMetadata {
    pub fn new(name: ProjectName, now: OffsetDateTime) -> Self {
        Self {
            schema_version: 1,
            project_id: Uuid::new_v4(),
            name,
            description: String::new(),
            banner_path: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub metadata: ProjectMetadata,
    pub path: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_project_names() {
        assert!(ProjectName::new("Rust Learning").is_ok());
        assert!(ProjectName::new("دروس Rust").is_ok());
        assert!(ProjectName::new("ⵜⵉⵎⵙⴰⵔ Rust").is_ok());

        let trimmed_test = ProjectName::new("  Rust Learning  ").unwrap();
        assert_eq!(trimmed_test.as_str(), "Rust Learning");
    }

    #[test]
    fn test_invalid_empty_project_names() {
        assert!(matches!(
            ProjectName::new(""),
            Err(AppError::ProjectNameIsEmpty)
        ));
        assert!(matches!(
            ProjectName::new("     "),
            Err(AppError::ProjectNameIsEmpty)
        ));
    }

    #[test]
    fn test_project_metadata_generates_unique_id() {
        let name = ProjectName::new("Rust Learning").unwrap();
        let now = OffsetDateTime::now_utc();

        let metadata1 = ProjectMetadata::new(name.clone(), now);
        let metadata2 = ProjectMetadata::new(name.clone(), now);

        assert_ne!(metadata1.project_id, metadata2.project_id);
    }

    #[test]
    fn test_invalid_unsafe_project_names() {
        assert!(matches!(
            ProjectName::new("."),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new(".."),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new(" . "),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("../outside"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("project/name"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("project\\name"),
            Err(AppError::ProjectNameIsUnsafe)
        ));

        assert!(matches!(
            ProjectName::new("project:name"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("project*name"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("project?name"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("project\"name"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("project<name"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("project>name"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("project|name"),
            Err(AppError::ProjectNameIsUnsafe)
        ));

        assert!(matches!(
            ProjectName::new("Rust\nLearning"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
        assert!(matches!(
            ProjectName::new("Rust\tLearning"),
            Err(AppError::ProjectNameIsUnsafe)
        ));
    }

    #[test]
    fn test_project_metadata_constructor() {
        let name = ProjectName::new("Zlija Project").unwrap();
        let now = OffsetDateTime::now_utc();
        let metadata = ProjectMetadata::new(name.clone(), now);

        assert_eq!(metadata.schema_version, 1);
        assert_eq!(metadata.name, name);
        assert_eq!(metadata.description, "");
        assert_eq!(metadata.banner_path, None);
        assert_eq!(metadata.created_at, now);
        assert_eq!(metadata.updated_at, now);
    }
}
