use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("User not found")]
    UserNotFound,

    #[error("Invalid email format: {0}")]
    InvalidEmail(String),

    #[error("Invalid organisation name: {0}")]
    InvalidOrganisationName(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Operator must belong to an organisation")]
    OperatorRequiresOrganisation,

    #[error("Repository error: {0}")]
    RepositoryError(String),
}