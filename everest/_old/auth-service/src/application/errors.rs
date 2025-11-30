use thiserror::Error;
use crate::domain::errors::DomainError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("User not found")]
    UserNotFound,

    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),

    #[error("External service error: {0}")]
    ExternalServiceError(String),
}

impl From<validator::ValidationErrors> for ApplicationError {
    fn from(err: validator::ValidationErrors) -> Self {
        ApplicationError::ValidationError(err.to_string())
    }
}