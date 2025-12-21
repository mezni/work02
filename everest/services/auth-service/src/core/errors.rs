use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Keycloak error: {0}")]
    KeycloakError(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Internal(_) | AppError::DatabaseError(_) | AppError::KeycloakError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::BadRequest(_) | AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        // Log the actual error for the developer before hiding details in the response
        tracing::error!("{}", self.to_string());

        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.to_string(),
        })
    }
}

// Map the Keycloak specific errors to our Global AppError
impl From<crate::infrastructure::keycloak_client::AppError> for AppError {
    fn from(err: crate::infrastructure::keycloak_client::AppError) -> Self {
        use crate::infrastructure::keycloak_client::AppError as K;
        match err {
            K::NotFound(m) => AppError::NotFound(m),
            K::Unauthorized(m) => AppError::Unauthorized(m),
            K::AuthenticationError(m) => AppError::Unauthorized(m),
            K::NetworkError(m) => AppError::Internal(format!("Network failure: {}", m)),
            K::KeycloakError(m) => AppError::KeycloakError(m),
        }
    }
}
