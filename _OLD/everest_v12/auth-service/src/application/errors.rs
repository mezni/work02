use thiserror::Error;
use crate::domain::errors::DomainError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Authentication failed: Invalid credentials.")]
    AuthenticationFailed,
    #[error("User not found.")]
    UserNotFound,
    #[error("Organisation not found.")]
    OrganisationNotFound,
    #[error("Station not found.")]
    StationNotFound,
    #[error("Unauthorized access.")]
    Unauthorized,
    #[error("Password hashing error: {0}")]
    PasswordHashing(String),
}

impl actix_web::ResponseError for ApplicationError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApplicationError::Domain(DomainError::InvalidCredentials) 
            | ApplicationError::AuthenticationFailed => actix_web::http::StatusCode::UNAUTHORIZED,
            ApplicationError::Domain(DomainError::UserNotFound(_)) 
            | ApplicationError::UserNotFound => actix_web::http::StatusCode::NOT_FOUND,
            ApplicationError::Domain(DomainError::UsernameAlreadyExists(_))
            | ApplicationError::Domain(DomainError::EmailAlreadyExists(_))
            | ApplicationError::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApplicationError::Unauthorized => actix_web::http::StatusCode::FORBIDDEN,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
