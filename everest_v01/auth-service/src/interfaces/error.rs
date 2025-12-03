use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use validator::ValidationErrors;
use serde_json::json;
use std::fmt;

#[derive(Error, Debug)]
pub enum InterfaceError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationErrors),
    
    #[error("Missing authorization header")]
    MissingAuthHeader,
    
    #[error("Invalid authorization header")]
    InvalidAuthHeader,
    
    #[error("Invalid API version")]
    InvalidApiVersion,
    
    #[error("Rate limited")]
    RateLimited,
    
    #[error("Request timeout")]
    RequestTimeout,
    
    #[error("Payload too large")]
    PayloadTooLarge,
    
    #[error("Unsupported media type")]
    UnsupportedMediaType,
    
    #[error("Method not allowed")]
    MethodNotAllowed,
    
    #[error("Endpoint not found")]
    EndpointNotFound,
}

impl ResponseError for InterfaceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            InterfaceError::InvalidRequest(msg) => HttpResponse::BadRequest().json(json!({
                "error": "Bad Request",
                "message": msg,
                "code": "INVALID_REQUEST"
            })),
            InterfaceError::Validation(err) => {
                let errors: Vec<String> = err.field_errors()
                    .iter()
                    .map(|(field, errors)| {
                        format!("{}: {}", field, errors.iter()
                            .map(|e| e.to_string())
                            .collect::<Vec<String>>()
                            .join(", "))
                    })
                    .collect();
                
                HttpResponse::BadRequest().json(json!({
                    "error": "Validation Failed",
                    "message": "Request validation failed",
                    "code": "VALIDATION_ERROR",
                    "details": errors
                }))
            },
            InterfaceError::MissingAuthHeader |
            InterfaceError::InvalidAuthHeader => HttpResponse::Unauthorized().json(json!({
                "error": "Unauthorized",
                "message": self.to_string(),
                "code": "AUTH_REQUIRED"
            })),
            InterfaceError::RateLimited => HttpResponse::TooManyRequests().json(json!({
                "error": "Too Many Requests",
                "message": "Rate limit exceeded",
                "code": "RATE_LIMIT",
                "retry_after": 60
            })),
            InterfaceError::RequestTimeout => HttpResponse::RequestTimeout().json(json!({
                "error": "Request Timeout",
                "message": "Request timed out",
                "code": "TIMEOUT"
            })),
            InterfaceError::PayloadTooLarge => HttpResponse::PayloadTooLarge().json(json!({
                "error": "Payload Too Large",
                "message": "Request payload too large",
                "code": "PAYLOAD_TOO_LARGE"
            })),
            InterfaceError::UnsupportedMediaType => HttpResponse::UnsupportedMediaType().json(json!({
                "error": "Unsupported Media Type",
                "message": "Unsupported media type",
                "code": "UNSUPPORTED_MEDIA_TYPE"
            })),
            InterfaceError::MethodNotAllowed => HttpResponse::MethodNotAllowed().json(json!({
                "error": "Method Not Allowed",
                "message": "Method not allowed",
                "code": "METHOD_NOT_ALLOWED"
            })),
            InterfaceError::EndpointNotFound => HttpResponse::NotFound().json(json!({
                "error": "Not Found",
                "message": "Endpoint not found",
                "code": "NOT_FOUND"
            })),
            InterfaceError::InvalidApiVersion => HttpResponse::BadRequest().json(json!({
                "error": "Bad Request",
                "message": "Invalid API version",
                "code": "INVALID_API_VERSION"
            })),
        }
    }
}

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub type InterfaceResult<T> = std::result::Result<T, InterfaceError>;