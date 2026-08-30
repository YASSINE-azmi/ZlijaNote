use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // Workspace Errors
    #[error("Workspace name cannot be empty")]
    WorkspaceNameIsEmpty,

    #[error("Workspace name contains unsafe characters")]
    WorkspaceNameIsUnsafe,

    #[error("Workspace not found")]
    WorkspaceNotFound,

    #[error("Workspace cannot be opened")]
    WorkspaceCannotBeOpened,

    #[error("Workspace parent location not found")]
    WorkspaceParentNotFound,

    #[error("Workspace parent location cannot be opened")]
    WorkspaceParentCannotBeOpened,

    #[error("Workspace already exists")]
    WorkspaceAlreadyExists,

    // Project Errors
    #[error("Project name cannot be empty")]
    ProjectNameIsEmpty,

    #[error("Project name contains unsafe characters")]
    ProjectNameIsUnsafe,

    #[error("Project already exists")]
    ProjectAlreadyExists,

    #[error("Project not found")]
    ProjectNotFound,

    #[error("Project cannot be opened")]
    ProjectCannotBeOpened,

    #[error("Project metadata cannot be read")]
    ProjectMetadataCannotBeRead,

    #[error("Project metadata cannot be written")]
    ProjectMetadataCannotBeWritten,

    #[error("Project metadata is invalid")]
    ProjectMetadataIsInvalid,

    #[error("Unsupported project schema")]
    UnsupportedProjectSchema,

    // System Errors
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
