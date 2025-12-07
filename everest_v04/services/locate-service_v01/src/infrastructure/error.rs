use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type) = match self {
            DomainError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            DomainError::ValidationError(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            DomainError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            DomainError::Forbidden(_) => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            DomainError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            DomainError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
        })
    }
}