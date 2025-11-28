use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    #[error("Invalid username: {0}")]
    InvalidUsername(String),
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    #[error("Invalid role string.")]
    InvalidRole,
    #[error("User with ID {0} not found.")]
    UserNotFound(Uuid),
    #[error("User with username '{0}' already exists.")]
    UsernameAlreadyExists(String),
    #[error("User with email '{0}' already exists.")]
    EmailAlreadyExists(String),
    #[error("Organisation with ID {0} not found.")]
    OrganisationNotFound(Uuid),
    #[error("Station with ID {0} not found.")]
    StationNotFound(Uuid),
    #[error("Invalid credentials.")]
    InvalidCredentials,
    #[error("A critical internal domain consistency error occurred: {0}")]
    InternalError(String),
}
