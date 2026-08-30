use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Project name is empty")]
    ProjectNameIsEmpty,

    #[error("Project name is unsafe")]
    ProjectNameIsUnsafe,

    #[error("Project already exists")]
    ProjectAlreadyExists,

    #[error("Workspace already exists")]
    WorkspaceAlreadyExists,

    #[error("Workspace not found")]
    WorkspaceNotFound,

    #[error("Workspace cannot be opened")]
    WorkspaceCannotBeOpened,

    #[error("Project metadata cannot be read")]
    ProjectMetadataCannotBeRead,

    #[error("Project metadata cannot be written")]
    ProjectMetadataCannotBeWritten,

    #[error("Project metadata is invalid")]
    ProjectMetadataIsInvalid,

    #[error("Unsupported project schema")]
    UnsupportedProjectSchema,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
