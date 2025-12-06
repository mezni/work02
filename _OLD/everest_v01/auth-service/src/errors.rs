use actix_web::{HttpResponse, ResponseError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Keycloak request error: {0}")]
    Keycloak(String),

    #[error("JSON parse error: {0}")]
    Json(String),

    #[error("Unexpected error: {0}")]
    Other(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}
