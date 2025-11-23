// src/api/error.rs
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;
use crate::application::ApplicationError;
use crate::domain::DomainError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Application error: {0}")]
    Application(#[from] ApplicationError),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Authentication required")]
    Unauthorized,
    
    #[error("Access denied")]
    Forbidden,
    
    #[error("Resource not found")]
    NotFound,
    
    #[error("Internal server error")]
    Internal,
    
    #[error("Bad request: {0}")]
    BadRequest(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::Application(app_error) => match app_error {
                ApplicationError::UserNotFound => {
                    HttpResponse::NotFound().json(ErrorResponse {
                        error: "user_not_found".to_string(),
                        message: self.to_string(),
                        details: None,
                    })
                }
                ApplicationError::UserAlreadyExists => {
                    HttpResponse::Conflict().json(ErrorResponse {
                        error: "user_already_exists".to_string(),
                        message: self.to_string(),
                        details: None,
                    })
                }
                ApplicationError::Validation(msg) => {
                    HttpResponse::BadRequest().json(ErrorResponse {
                        error: "validation_error".to_string(),
                        message: msg.clone(),
                        details: None,
                    })
                }
                ApplicationError::Authentication(msg) => {
                    HttpResponse::Unauthorized().json(ErrorResponse {
                        error: "authentication_error".to_string(),
                        message: msg.clone(),
                        details: None,
                    })
                }
                ApplicationError::Authorization(msg) => {
                    HttpResponse::Forbidden().json(ErrorResponse {
                        error: "authorization_error".to_string(),
                        message: msg.clone(),
                        details: None,
                    })
                }
                _ => {
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "internal_error".to_string(),
                        message: "An internal error occurred".to_string(),
                        details: None,
                    })
                }
            },
            ApiError::Validation(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "validation_error".to_string(),
                    message: msg.clone(),
                    details: None,
                })
            }
            ApiError::Unauthorized => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "unauthorized".to_string(),
                    message: self.to_string(),
                    details: None,
                })
            }
            ApiError::Forbidden => {
                HttpResponse::Forbidden().json(ErrorResponse {
                    error: "forbidden".to_string(),
                    message: self.to_string(),
                    details: None,
                })
            }
            ApiError::NotFound => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: "not_found".to_string(),
                    message: self.to_string(),
                    details: None,
                })
            }
            ApiError::Internal => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "internal_error".to_string(),
                    message: self.to_string(),
                    details: None,
                })
            }
            ApiError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "bad_request".to_string(),
                    message: msg.clone(),
                    details: None,
                })
            }
        }
    }
}

impl From<DomainError> for ApiError {
    fn from(error: DomainError) -> Self {
        ApiError::Application(ApplicationError::from(error))
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(errors: validator::ValidationErrors) -> Self {
        ApiError::Validation(errors.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::BadRequest(format!("Invalid JSON: {}", error))
    }
}