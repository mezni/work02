use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterfaceError {
    #[error("Application error: {0}")]
    ApplicationError(#[from] crate::application::errors::ApplicationError),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Authentication required")]
    AuthenticationRequired,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Resource not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for InterfaceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            InterfaceError::ApplicationError(app_error) => {
                let status_code = match app_error {
                    crate::application::errors::ApplicationError::AuthenticationFailed => {
                        actix_web::http::StatusCode::UNAUTHORIZED
                    }
                    crate::application::errors::ApplicationError::AuthorizationFailed(_) => {
                        actix_web::http::StatusCode::FORBIDDEN
                    }
                    crate::application::errors::ApplicationError::UserNotFound
                    | crate::application::errors::ApplicationError::CompanyNotFound => {
                        actix_web::http::StatusCode::NOT_FOUND
                    }
                    crate::application::errors::ApplicationError::InvalidToken
                    | crate::application::errors::ApplicationError::TokenExpired => {
                        actix_web::http::StatusCode::UNAUTHORIZED
                    }
                    _ => actix_web::http::StatusCode::BAD_REQUEST,
                };

                HttpResponse::build(status_code).json(json!({
                    "error": app_error.code(),
                    "message": app_error.to_string(),
                }))
            }
            InterfaceError::ValidationError(msg) => HttpResponse::BadRequest().json(json!({
                "error": "VALIDATION_ERROR",
                "message": msg,
            })),
            InterfaceError::AuthenticationRequired => HttpResponse::Unauthorized().json(json!({
                "error": "AUTHENTICATION_REQUIRED",
                "message": "Authentication is required to access this resource",
            })),
            InterfaceError::InsufficientPermissions => HttpResponse::Forbidden().json(json!({
                "error": "INSUFFICIENT_PERMISSIONS",
                "message": "You don't have sufficient permissions to access this resource",
            })),
            InterfaceError::NotFound => HttpResponse::NotFound().json(json!({
                "error": "NOT_FOUND",
                "message": "The requested resource was not found",
            })),
            InterfaceError::BadRequest(msg) => HttpResponse::BadRequest().json(json!({
                "error": "BAD_REQUEST",
                "message": msg,
            })),
            InterfaceError::InternalServerError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "INTERNAL_SERVER_ERROR",
                    "message": "An internal server error occurred",
                }))
            }
        }
    }
}

pub type WebResult<T> = Result<T, InterfaceError>;
