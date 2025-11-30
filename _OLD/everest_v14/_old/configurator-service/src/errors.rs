use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalError,
    #[error("Not Found")]
    NotFound,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalError => HttpResponse::InternalServerError().body("Internal Error"),
            AppError::NotFound => HttpResponse::NotFound().body("Not Found"),
        }
    }
}
