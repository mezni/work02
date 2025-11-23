use crate::application::services::network_application_service::ApplicationError;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            ApplicationError::NotFound => {
                (actix_web::http::StatusCode::NOT_FOUND, "Resource not found")
            }
            ApplicationError::AlreadyVerified => (
                actix_web::http::StatusCode::CONFLICT,
                "Network already verified",
            ),
            ApplicationError::Repository(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
            ),
            ApplicationError::Domain(msg) => {
                (actix_web::http::StatusCode::BAD_REQUEST, msg.as_str())
            }
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: status.to_string(),
            message: message.to_string(),
        })
    }
}
