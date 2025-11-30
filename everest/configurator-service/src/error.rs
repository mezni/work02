use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error")]
    Internal,

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),

    #[error("Invalid configuration: {0}")]
    Invalid(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        error!("Error occurred: {}", self);

        match self {
            AppError::Config(_) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Configuration error".to_string(),
                code: "CONFIG_ERROR".to_string(),
            }),
            AppError::Validation(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                error: msg.to_string(),
                code: "VALIDATION_ERROR".to_string(),
            }),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse {
                error: msg.to_string(),
                code: "NOT_FOUND".to_string(),
            }),
            AppError::Internal => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            }),
            AppError::Auth(msg) => HttpResponse::Unauthorized().json(ErrorResponse {
                error: msg.to_string(),
                code: "AUTH_ERROR".to_string(),
            }),
            AppError::Database(_) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database error".to_string(),
                code: "DATABASE_ERROR".to_string(),
            }),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        error!("Anyhow error converted to AppError: {}", err);
        AppError::Internal
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
