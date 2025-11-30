use actix_web::{error::ResponseError, HttpResponse};
use thiserror::Error;
use crate::application::errors::ApplicationError;
use crate::infrastructure::keycloak::errors::KeycloakError;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("Application error: {0}")]
    ApplicationError(#[from] ApplicationError),
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HttpError::BadRequest(msg) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": "bad_request",
                "message": msg
            })),
            HttpError::Unauthorized(msg) => HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "unauthorized",
                "message": msg
            })),
            HttpError::NotFound(msg) => HttpResponse::NotFound().json(serde_json::json!({
                "error": "not_found",
                "message": msg
            })),
            HttpError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "internal_server_error",
                    "message": msg
                }))
            }
            HttpError::ExternalServiceError(msg) => {
                HttpResponse::ServiceUnavailable().json(serde_json::json!({
                    "error": "external_service_error",
                    "message": msg
                }))
            }
            HttpError::ApplicationError(err) => match err {
                ApplicationError::ValidationError(msg) => {
                    HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "validation_error",
                        "message": msg
                    }))
                }
                ApplicationError::Unauthorized(msg) => {
                    HttpResponse::Unauthorized().json(serde_json::json!({
                        "error": "unauthorized",
                        "message": msg
                    }))
                }
                ApplicationError::UserNotFound => HttpResponse::NotFound().json(serde_json::json!({
                    "error": "user_not_found",
                    "message": "User not found"
                })),
                _ => HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "internal_server_error",
                    "message": err.to_string()
                })),
            },
        }
    }
}

impl From<validator::ValidationErrors> for HttpError {
    fn from(err: validator::ValidationErrors) -> Self {
        HttpError::BadRequest(err.to_string())
    }
}

impl From<KeycloakError> for HttpError {
    fn from(err: KeycloakError) -> Self {
        HttpError::ExternalServiceError(err.to_string())
    }
}
