use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Domain error: {0}")]
    Domain(#[from] crate::domain::error::DomainError),
    
    #[error("Application error: {0}")]
    Application(#[from] crate::application::error::ApplicationError),
    
    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] crate::infrastructure::error::InfrastructureError),
    
    #[error("Interface error: {0}")]
    Interface(#[from] crate::interfaces::error::InterfaceError),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Service unavailable")]
    ServiceUnavailable,
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Domain(e) => e.error_response(),
            AppError::Application(e) => e.error_response(),
            AppError::Infrastructure(e) => e.error_response(),
            AppError::Interface(e) => e.error_response(),
            AppError::Auth(msg) => HttpResponse::Unauthorized().json(json!({
                "error": "Unauthorized",
                "message": msg,
                "code": "AUTH_ERROR"
            })),
            AppError::Validation(msg) => HttpResponse::BadRequest().json(json!({
                "error": "Validation Failed",
                "message": msg,
                "code": "VALIDATION_ERROR"
            })),
            AppError::RateLimitExceeded => HttpResponse::TooManyRequests().json(json!({
                "error": "Too Many Requests",
                "message": "Rate limit exceeded",
                "code": "RATE_LIMIT_EXCEEDED",
                "retry_after": 60
            })),
            AppError::ServiceUnavailable => HttpResponse::ServiceUnavailable().json(json!({
                "error": "Service Unavailable",
                "message": "Service temporarily unavailable",
                "code": "SERVICE_UNAVAILABLE"
            })),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(json!({
                "error": "Not Found",
                "message": msg,
                "code": "NOT_FOUND"
            })),
            AppError::Conflict(msg) => HttpResponse::Conflict().json(json!({
                "error": "Conflict",
                "message": msg,
                "code": "CONFLICT"
            })),
            AppError::Config(msg) => HttpResponse::InternalServerError().json(json!({
                "error": "Configuration Error",
                "message": msg,
                "code": "CONFIG_ERROR"
            })),
            AppError::Database(e) => {
                log::error!("Database error: {}", e);
                HttpResponse::InternalServerError().json(json!({
                    "error": "Database Error",
                    "message": "Internal database error",
                    "code": "DATABASE_ERROR"
                }))
            }
            AppError::Io(e) => {
                log::error!("IO error: {}", e);
                HttpResponse::InternalServerError().json(json!({
                    "error": "IO Error",
                    "message": "Internal IO error",
                    "code": "IO_ERROR"
                }))
            }
            AppError::Serialization(e) => {
                log::error!("Serialization error: {}", e);
                HttpResponse::InternalServerError().json(json!({
                    "error": "Serialization Error",
                    "message": "Internal serialization error",
                    "code": "SERIALIZATION_ERROR"
                }))
            }
            AppError::Internal(msg) => {
                log::error!("Internal error: {}", msg);
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal Server Error",
                    "message": "An internal server error occurred",
                    "code": "INTERNAL_ERROR"
                }))
            }
        }
    }
    
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Domain(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::Application(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::Infrastructure(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Interface(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::Auth(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            AppError::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::RateLimitExceeded => actix_web::http::StatusCode::TOO_MANY_REQUESTS,
            AppError::ServiceUnavailable => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            AppError::Conflict(_) => actix_web::http::StatusCode::CONFLICT,
            AppError::Config(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Io(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Serialization(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Domain(e) => write!(f, "Domain error: {}", e),
            AppError::Application(e) => write!(f, "Application error: {}", e),
            AppError::Infrastructure(e) => write!(f, "Infrastructure error: {}", e),
            AppError::Interface(e) => write!(f, "Interface error: {}", e),
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal server error: {}", msg),
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Serialization(e) => write!(f, "Serialization error: {}", e),
            AppError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            AppError::ServiceUnavailable => write!(f, "Service unavailable"),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}

// Conversion from anyhow::Error
impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::Internal(error.to_string())
    }
}

// Conversion from config::ConfigError
impl From<config::ConfigError> for AppError {
    fn from(error: config::ConfigError) -> Self {
        AppError::Config(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

// Helper trait for error conversions
pub trait ErrorExt<T> {
    fn app_error(self) -> Result<T>;
    fn with_context<F>(self, context: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ErrorExt<T> for std::result::Result<T, E>
where
    E: std::fmt::Display,
{
    fn app_error(self) -> Result<T> {
        self.map_err(|e| AppError::Internal(e.to_string()))
    }
    
    fn with_context<F>(self, context: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| AppError::Internal(format!("{}: {}", context(), e)))
    }
}

// Error reporting utilities
pub struct ErrorReporter;

impl ErrorReporter {
    pub fn report(error: &AppError, context: &str, user_id: Option<uuid::Uuid>) {
        // In production, you might want to send errors to Sentry, Datadog, etc.
        match error {
            AppError::Domain(e) => {
                log::warn!("Domain error in {}: {} (user: {:?})", context, e, user_id);
            }
            AppError::Application(e) => {
                log::warn!("Application error in {}: {} (user: {:?})", context, e, user_id);
            }
            AppError::Infrastructure(e) => {
                log::error!("Infrastructure error in {}: {} (user: {:?})", context, e, user_id);
            }
            AppError::Interface(e) => {
                log::warn!("Interface error in {}: {} (user: {:?})", context, e, user_id);
            }
            AppError::Internal(e) => {
                log::error!("Internal error in {}: {} (user: {:?})", context, e, user_id);
            }
            _ => {
                log::error!("Error in {}: {} (user: {:?})", context, error, user_id);
            }
        }
    }
}