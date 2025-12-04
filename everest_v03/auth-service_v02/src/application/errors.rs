use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("User already exists: {0}")]
    UserAlreadyExists(String),

    #[error("Unexpected application error: {0}")]
    Unexpected(String),
}
