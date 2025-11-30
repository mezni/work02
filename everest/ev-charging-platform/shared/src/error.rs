// shared/src/error.rs
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User not found: {user_id}")]
    UserNotFound { user_id: String },
    
    #[error("Organization not found: {organization_id}")]
    OrganizationNotFound { organization_id: String },
    
    #[error("Station not found: {station_id}")]
    StationNotFound { station_id: String },
    
    #[error("Insufficient permissions")]
    InsufficientPermissions,
    
    #[error("Invalid user state transition from {from} to {to}")]
    InvalidStateTransition { from: String, to: String },
    
    #[error("Invalid user configuration: {message}")]
    InvalidUserConfiguration { message: String },
    
    #[error("Email already exists: {email}")]
    EmailAlreadyExists { email: String },
    
    #[error("Database error: {source}")]
    DatabaseError {
        #[from]
        source: sqlx::Error,
    },
    
    #[error("External service error: {service} - {message}")]
    ExternalServiceError { service: String, message: String },
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: &str, message: String) -> Self {
        Self {
            code: code.to_string(),
            message,
        }
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Authentication required")]
    Unauthorized,
    
    #[error("Access forbidden")]
    Forbidden,
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Invalid input: {0}")]
    BadRequest(String),
    
    #[error("Internal server error")]
    InternalServerError,
    
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::Unauthorized => HttpResponse::Unauthorized().json(
                ErrorResponse::new("UNAUTHORIZED", self.to_string())
            ),
            ApiError::Forbidden => HttpResponse::Forbidden().json(
                ErrorResponse::new("FORBIDDEN", self.to_string())
            ),
            ApiError::NotFound(_) => HttpResponse::NotFound().json(
                ErrorResponse::new("NOT_FOUND", self.to_string())
            ),
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(
                ErrorResponse::new("BAD_REQUEST", self.to_string())
            ),
            ApiError::InternalServerError => HttpResponse::InternalServerError().json(
                ErrorResponse::new("INTERNAL_ERROR", self.to_string())
            ),
            ApiError::DomainError(domain_error) => match domain_error {
                DomainError::UserNotFound { .. } | 
                DomainError::OrganizationNotFound { .. } | 
                DomainError::StationNotFound { .. } => HttpResponse::NotFound().json(
                    ErrorResponse::new("NOT_FOUND", domain_error.to_string())
                ),
                DomainError::InsufficientPermissions => HttpResponse::Forbidden().json(
                    ErrorResponse::new("FORBIDDEN", domain_error.to_string())
                ),
                DomainError::InvalidUserConfiguration { .. } | 
                DomainError::InvalidStateTransition { .. } => HttpResponse::BadRequest().json(
                    ErrorResponse::new("BAD_REQUEST", domain_error.to_string())
                ),
                DomainError::EmailAlreadyExists { .. } => HttpResponse::Conflict().json(
                    ErrorResponse::new("CONFLICT", domain_error.to_string())
                ),
                _ => HttpResponse::InternalServerError().json(
                    ErrorResponse::new("INTERNAL_ERROR", "An unexpected error occurred".to_string())
                ),
            },
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => ApiError::NotFound("Resource not found".to_string()),
            _ => ApiError::InternalServerError,
        }
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let message = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                format!("{}: {}", field, errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))
            })
            .collect::<Vec<_>>()
            .join("; ");
        
        ApiError::BadRequest(format!("Validation failed: {}", message))
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
pub type DomainResult<T> = Result<T, DomainError>;