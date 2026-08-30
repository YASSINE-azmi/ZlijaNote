//! Implementation of filesystem operations for Project entities inside a Workspace.

use std::fs;
use std::path::Path;

use crate::domain::project::{Project, ProjectMetadata};
use crate::domain::workspace::Workspace;
use crate::error::AppError;

pub const PROJECT_METADATA_FILE: &str = "project.zlija.json";
pub const NOTES_DIRECTORY: &str = "notes";
pub const ASSETS_DIRECTORY: &str = "assets";
pub const HISTORY_DIRECTORY: &str = "history";
pub const TRASH_DIRECTORY: &str = "trash";

pub struct ProjectRepository;

impl ProjectRepository {
    pub fn open_project(project_path: &Path) -> Result<Project, AppError> {
        if !project_path.exists() {
            return Err(AppError::ProjectNotFound);
        }

        if !project_path.is_dir() {
            return Err(AppError::ProjectCannotBeOpened);
        }

        let metadata_path = project_path.join(PROJECT_METADATA_FILE);
        if !metadata_path.exists() {
            return Err(AppError::ProjectMetadataCannotBeRead);
        }

        let metadata_content = fs::read_to_string(&metadata_path)
            .map_err(|_| AppError::ProjectMetadataCannotBeRead)?;

        let metadata: ProjectMetadata = serde_json::from_str(&metadata_content)
            .map_err(|_| AppError::ProjectMetadataIsInvalid)?;

        if metadata.schema_version != 1 {
            return Err(AppError::UnsupportedProjectSchema);
        }

        Ok(Project::new(metadata, project_path.to_path_buf()))
    }

    pub fn create_project(
        workspace: &Workspace,
        metadata: ProjectMetadata,
    ) -> Result<Project, AppError> {
        let project_path = workspace.path().as_path().join(metadata.name.as_str());

        if project_path.exists() {
            return Err(AppError::ProjectAlreadyExists);
        }

        let create_process = (|| -> Result<Project, AppError> {
            fs::create_dir(&project_path)?;
            fs::create_dir(project_path.join(NOTES_DIRECTORY))?;
            fs::create_dir(project_path.join(ASSETS_DIRECTORY))?;
            fs::create_dir(project_path.join(HISTORY_DIRECTORY))?;
            fs::create_dir(project_path.join(TRASH_DIRECTORY))?;

            let metadata_path = project_path.join(PROJECT_METADATA_FILE);
            let metadata_json = serde_json::to_string_pretty(&metadata)
                .map_err(|_| AppError::ProjectMetadataCannotBeWritten)?;

            fs::write(&metadata_path, metadata_json)
                .map_err(|_| AppError::ProjectMetadataCannotBeWritten)?;

            Self::open_project(&project_path)
        })();

        if create_process.is_err() && project_path.exists() {
            let _ = fs::remove_dir_all(&project_path);
        }

        create_process
    }

    pub fn list_projects(workspace: &Workspace) -> Result<Vec<Project>, AppError> {
        // TODO: Distinguish non-project directories from unreadable or corrupted Project directories.
        let ws_path = workspace.path().as_path();

        if !ws_path.exists() {
            return Err(AppError::WorkspaceNotFound);
        }

        if !ws_path.is_dir() {
            return Err(AppError::WorkspaceCannotBeOpened);
        }

        let mut projects = Vec::new();
        let entries = fs::read_dir(ws_path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                match Self::open_project(&path) {
                    Ok(project) => projects.push(project),
                    Err(_) => {
                        // TODO: Surface invalid projects in a recovery UI.
                    }
                }
            }
        }

        // TODO: Sort projects by metadata.updated_at descending before returning.
        Ok(projects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::project::ProjectName;
    use crate::domain::workspace::WorkspaceName;
    use crate::infrastructure::workspace_repository::WorkspaceRepository;
    use tempfile::tempdir;

    #[test]
    fn test_create_project_success_and_verify_disk_content() {
        let temp = tempdir().unwrap();
        let ws_name = WorkspaceName::new("My Workspace").unwrap();
        let workspace = WorkspaceRepository::create_workspace(temp.path(), ws_name).unwrap();

        let proj_name = ProjectName::new("Rust Learning").unwrap();
        let metadata = ProjectMetadata::new(proj_name.clone(), "Detailed Description".to_string());

        let project = ProjectRepository::create_project(&workspace, metadata.clone()).unwrap();

        assert_eq!(project.metadata().name, proj_name);
        assert!(project.path().join(PROJECT_METADATA_FILE).exists());
        assert!(project.path().join(NOTES_DIRECTORY).is_dir());
        assert!(project.path().join(ASSETS_DIRECTORY).is_dir());
        assert!(project.path().join(HISTORY_DIRECTORY).is_dir());
        assert!(project.path().join(TRASH_DIRECTORY).is_dir());

        let raw_json = fs::read_to_string(project.path().join(PROJECT_METADATA_FILE)).unwrap();
        let read_metadata: ProjectMetadata = serde_json::from_str(&raw_json).unwrap();

        assert_eq!(read_metadata.project_id, metadata.project_id);
        assert_eq!(read_metadata.name, metadata.name);
        assert_eq!(read_metadata.description, metadata.description);
    }

    #[test]
    fn test_list_projects_skips_invalid_projects_and_files() {
        let temp = tempdir().unwrap();
        let ws_name = WorkspaceName::new("My Workspace").unwrap();
        let workspace = WorkspaceRepository::create_workspace(temp.path(), ws_name).unwrap();

        let p1_name = ProjectName::new("Valid Project").unwrap();
        ProjectRepository::create_project(
            &workspace,
            ProjectMetadata::new(p1_name, "D1".to_string()),
        )
        .unwrap();

        let corrupted_dir = workspace.path().as_path().join("corrupted_proj");
        fs::create_dir(&corrupted_dir).unwrap();
        fs::write(corrupted_dir.join(PROJECT_METADATA_FILE), "{ bad json ").unwrap();

        let random_dir = workspace.path().as_path().join("random_folder");
        fs::create_dir(&random_dir).unwrap();

        fs::write(workspace.path().as_path().join("notes.txt"), "hello").unwrap();

        let projects = ProjectRepository::list_projects(&workspace).unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].metadata().name.as_str(), "Valid Project");
    }

    #[test]
    fn test_list_projects_workspace_validation() {
        let temp = tempdir().unwrap();

        let non_existent = temp.path().join("missing_ws");
        let ws_missing = Workspace::new(
            WorkspaceName::new("Ghost").unwrap(),
            crate::domain::workspace::WorkspacePath::new(non_existent),
        );
        assert!(matches!(
            ProjectRepository::list_projects(&ws_missing),
            Err(AppError::WorkspaceNotFound)
        ));

        let file_path = temp.path().join("ws_file.txt");
        fs::write(&file_path, "not a dir").unwrap();
        let ws_file = Workspace::new(
            WorkspaceName::new("FileWS").unwrap(),
            crate::domain::workspace::WorkspacePath::new(file_path),
        );
        assert!(matches!(
            ProjectRepository::list_projects(&ws_file),
            Err(AppError::WorkspaceCannotBeOpened)
        ));
    }

    #[test]
    fn test_create_project_already_exists() {
        let temp = tempdir().unwrap();
        let ws_name = WorkspaceName::new("My Workspace").unwrap();
        let workspace = WorkspaceRepository::create_workspace(temp.path(), ws_name).unwrap();

        let proj_name = ProjectName::new("Rust Learning").unwrap();
        let metadata = ProjectMetadata::new(proj_name, "Description".to_string());

        ProjectRepository::create_project(&workspace, metadata.clone()).unwrap();

        let duplicate_res = ProjectRepository::create_project(&workspace, metadata);
        assert!(matches!(duplicate_res, Err(AppError::ProjectAlreadyExists)));
    }

    #[test]
    fn test_open_project_not_found() {
        let temp = tempdir().unwrap();
        let non_existent = temp.path().join("ghost_project");

        let res = ProjectRepository::open_project(&non_existent);
        assert!(matches!(res, Err(AppError::ProjectNotFound)));
    }

    #[test]
    fn test_open_project_when_path_is_file() {
        let temp = tempdir().unwrap();
        let file_path = temp.path().join("file.txt");
        fs::write(&file_path, "content").unwrap();

        let res = ProjectRepository::open_project(&file_path);
        assert!(matches!(res, Err(AppError::ProjectCannotBeOpened)));
    }

    #[test]
    fn test_open_project_missing_metadata() {
        let temp = tempdir().unwrap();
        let proj_dir = temp.path().join("empty_project");
        fs::create_dir(&proj_dir).unwrap();

        let res = ProjectRepository::open_project(&proj_dir);
        assert!(matches!(res, Err(AppError::ProjectMetadataCannotBeRead)));
    }

    #[test]
    fn test_open_project_invalid_json() {
        let temp = tempdir().unwrap();
        let proj_dir = temp.path().join("corrupted_project");
        fs::create_dir(&proj_dir).unwrap();
        fs::write(proj_dir.join(PROJECT_METADATA_FILE), "{ invalid_json: ").unwrap();

        let res = ProjectRepository::open_project(&proj_dir);
        assert!(matches!(res, Err(AppError::ProjectMetadataIsInvalid)));
    }

    #[test]
    fn test_open_project_unsupported_schema() {
        let temp = tempdir().unwrap();
        let proj_dir = temp.path().join("future_project");
        fs::create_dir(&proj_dir).unwrap();

        let proj_name = ProjectName::new("Future").unwrap();
        let mut metadata = ProjectMetadata::new(proj_name, "Desc".to_string());
        metadata.schema_version = 999;

        let json = serde_json::to_string(&metadata).unwrap();
        fs::write(proj_dir.join(PROJECT_METADATA_FILE), json).unwrap();

        let res = ProjectRepository::open_project(&proj_dir);
        assert!(matches!(res, Err(AppError::UnsupportedProjectSchema)));
    }
}
