//! ProjectService coordinates Project domain rules and ProjectRepository operations.
//! It does not access filesystem or Tauri APIs directly.

use time::OffsetDateTime;

use crate::domain::project::{Project, ProjectMetadata, ProjectName};
use crate::domain::workspace::Workspace;
use crate::error::AppError;
use crate::infrastructure::project_repository::ProjectRepository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateProjectInput {
    pub name: String,
    pub description: String,
}

pub struct ProjectService;

impl ProjectService {
    pub fn create_project(
        workspace: &Workspace,
        input: CreateProjectInput,
        now: OffsetDateTime,
    ) -> Result<Project, AppError> {
        let project_name = ProjectName::new(input.name)?;
        let cleaned_description = input.description.trim().to_string();

        let metadata = ProjectMetadata::new(project_name, cleaned_description, now);

        ProjectRepository::create_project(workspace, metadata)
    }

    pub fn list_projects(workspace: &Workspace) -> Result<Vec<Project>, AppError> {
        ProjectRepository::list_projects(workspace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::workspace::WorkspaceName;
    use crate::infrastructure::project_repository::PROJECT_METADATA_FILE;
    use crate::infrastructure::workspace_repository::WorkspaceRepository;
    use tempfile::tempdir;
    use time::macros::datetime;

    #[test]
    fn test_create_project_success_with_fixed_time_and_trimmed_description() {
        let temp = tempdir().unwrap();
        let ws_name = WorkspaceName::new("My Workspace").unwrap();
        let workspace = WorkspaceRepository::create_workspace(temp.path(), ws_name).unwrap();

        let fixed_now = datetime!(2026-08-31 01:00:00 UTC);
        let input = CreateProjectInput {
            name: "Rust Learning".to_string(),
            description: "  Study notes and exercises  ".to_string(),
        };

        let project = ProjectService::create_project(&workspace, input, fixed_now).unwrap();

        assert_eq!(project.metadata().name.as_str(), "Rust Learning");
        assert_eq!(project.metadata().description, "Study notes and exercises");
        assert_eq!(project.metadata().created_at, fixed_now);
        assert_eq!(project.metadata().updated_at, fixed_now);

        assert!(project.path().join(PROJECT_METADATA_FILE).exists());
    }

    #[test]
    fn test_create_project_empty_description_is_valid() {
        let temp = tempdir().unwrap();
        let ws_name = WorkspaceName::new("My Workspace").unwrap();
        let workspace = WorkspaceRepository::create_workspace(temp.path(), ws_name).unwrap();

        let fixed_now = datetime!(2026-08-31 01:00:00 UTC);
        let input = CreateProjectInput {
            name: "Empty Desc Project".to_string(),
            description: "     ".to_string(), // وصف يحتوي على مسافات فقط
        };

        let project = ProjectService::create_project(&workspace, input, fixed_now).unwrap();

        assert_eq!(project.metadata().description, "");
    }

    #[test]
    fn test_create_project_empty_name_returns_error() {
        let temp = tempdir().unwrap();
        let ws_name = WorkspaceName::new("My Workspace").unwrap();
        let workspace = WorkspaceRepository::create_workspace(temp.path(), ws_name).unwrap();

        let fixed_now = datetime!(2026-08-31 01:00:00 UTC);
        let input = CreateProjectInput {
            name: "   ".to_string(),
            description: "Some desc".to_string(),
        };

        let res = ProjectService::create_project(&workspace, input, fixed_now);
        assert!(matches!(res, Err(AppError::ProjectNameIsEmpty)));

        let entries = std::fs::read_dir(workspace.path().as_path()).unwrap();
        assert_eq!(entries.count(), 0);
    }

    #[test]
    fn test_create_project_unsafe_name_returns_error() {
        let temp = tempdir().unwrap();
        let ws_name = WorkspaceName::new("My Workspace").unwrap();
        let workspace = WorkspaceRepository::create_workspace(temp.path(), ws_name).unwrap();

        let fixed_now = datetime!(2026-08-31 01:00:00 UTC);
        let input = CreateProjectInput {
            name: "proj/name".to_string(),
            description: "Some desc".to_string(),
        };

        let res = ProjectService::create_project(&workspace, input, fixed_now);
        assert!(matches!(res, Err(AppError::ProjectNameIsUnsafe)));

        let entries = std::fs::read_dir(workspace.path().as_path()).unwrap();
        assert_eq!(entries.count(), 0);
    }

    #[test]
    fn test_create_project_duplicate_returns_error() {
        let temp = tempdir().unwrap();
        let ws_name = WorkspaceName::new("My Workspace").unwrap();
        let workspace = WorkspaceRepository::create_workspace(temp.path(), ws_name).unwrap();

        let fixed_now = datetime!(2026-08-31 01:00:00 UTC);
        let input1 = CreateProjectInput {
            name: "Rust Learning".to_string(),
            description: "First".to_string(),
        };
        let input2 = CreateProjectInput {
            name: "Rust Learning".to_string(),
            description: "Second".to_string(),
        };

        ProjectService::create_project(&workspace, input1, fixed_now).unwrap();

        let res = ProjectService::create_project(&workspace, input2, fixed_now);
        assert!(matches!(res, Err(AppError::ProjectAlreadyExists)));
    }

    #[test]
    fn test_list_projects_success() {
        let temp = tempdir().unwrap();
        let ws_name = WorkspaceName::new("My Workspace").unwrap();
        let workspace = WorkspaceRepository::create_workspace(temp.path(), ws_name).unwrap();

        let fixed_now = datetime!(2026-08-31 01:00:00 UTC);

        let input1 = CreateProjectInput {
            name: "Project One".to_string(),
            description: "".to_string(),
        };
        ProjectService::create_project(&workspace, input1, fixed_now).unwrap();

        let input2 = CreateProjectInput {
            name: "Project Two".to_string(),
            description: "".to_string(),
        };
        ProjectService::create_project(&workspace, input2, fixed_now).unwrap();

        let projects = ProjectService::list_projects(&workspace).unwrap();

        assert_eq!(projects.len(), 2);

        let has_one = projects
            .iter()
            .any(|p| p.metadata().name.as_str() == "Project One");
        let has_two = projects
            .iter()
            .any(|p| p.metadata().name.as_str() == "Project Two");

        assert!(has_one);
        assert!(has_two);
    }
}
