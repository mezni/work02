use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Registration failed: {0}")]
    RegistrationFailed(String),
    
    #[error("Token generation failed: {0}")]
    TokenGenerationFailed(String),
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Account disabled")]
    AccountDisabled,
    
    #[error("Email not verified")]
    EmailNotVerified,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Password mismatch")]
    PasswordMismatch,
    
    #[error("Weak password")]
    WeakPassword,
    
    #[error("Email already exists")]
    EmailAlreadyExists,
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApplicationError::AuthenticationFailed(msg) |
            ApplicationError::InvalidCredentials => HttpResponse::Unauthorized().json(json!({
                "error": "Unauthorized",
                "message": self.to_string(),
                "code": "AUTH_FAILED"
            })),
            ApplicationError::RegistrationFailed(msg) |
            ApplicationError::Validation(msg) |
            ApplicationError::WeakPassword |
            ApplicationError::PasswordMismatch => HttpResponse::BadRequest().json(json!({
                "error": "Bad Request",
                "message": self.to_string(),
                "code": "VALIDATION_ERROR"
            })),
            ApplicationError::AccountDisabled => HttpResponse::Forbidden().json(json!({
                "error": "Forbidden",
                "message": "Account is disabled",
                "code": "ACCOUNT_DISABLED"
            })),
            ApplicationError::EmailNotVerified => HttpResponse::Forbidden().json(json!({
                "error": "Forbidden",
                "message": "Email not verified",
                "code": "EMAIL_NOT_VERIFIED"
            })),
            ApplicationError::RateLimitExceeded => HttpResponse::TooManyRequests().json(json!({
                "error": "Too Many Requests",
                "message": "Rate limit exceeded",
                "code": "RATE_LIMIT"
            })),
            ApplicationError::ServiceUnavailable(msg) => HttpResponse::ServiceUnavailable().json(json!({
                "error": "Service Unavailable",
                "message": msg,
                "code": "SERVICE_UNAVAILABLE"
            })),
            ApplicationError::InvalidToken(msg) |
            ApplicationError::TokenExpired => HttpResponse::Unauthorized().json(json!({
                "error": "Unauthorized",
                "message": self.to_string(),
                "code": "INVALID_TOKEN"
            })),
            ApplicationError::UserNotFound => HttpResponse::NotFound().json(json!({
                "error": "Not Found",
                "message": "User not found",
                "code": "USER_NOT_FOUND"
            })),
            ApplicationError::PermissionDenied => HttpResponse::Forbidden().json(json!({
                "error": "Forbidden",
                "message": "Permission denied",
                "code": "PERMISSION_DENIED"
            })),
            ApplicationError::EmailAlreadyExists => HttpResponse::Conflict().json(json!({
                "error": "Conflict",
                "message": "Email already exists",
                "code": "EMAIL_EXISTS"
            })),
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub type ApplicationResult<T> = std::result::Result<T, ApplicationError>;