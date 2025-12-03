use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    
    #[error("Invalid user role: {0}")]
    InvalidRole(String),
    
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Invalid registration data: {0}")]
    InvalidRegistration(String),
    
    #[error("Domain validation failed: {0}")]
    Validation(String),
    
    #[error("Invalid company name: {0}")]
    InvalidCompanyName(String),
    
    #[error("Invalid station name: {0}")]
    InvalidStationName(String),
}

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DomainError::InvalidEmail(msg) |
            DomainError::InvalidPassword(msg) |
            DomainError::InvalidRegistration(msg) |
            DomainError::Validation(msg) |
            DomainError::InvalidCompanyName(msg) |
            DomainError::InvalidStationName(msg) => HttpResponse::BadRequest().json(json!({
                "error": "Bad Request",
                "message": msg
            })),
            DomainError::UserNotFound => HttpResponse::NotFound().json(json!({
                "error": "Not Found",
                "message": "User not found"
            })),
            DomainError::UserAlreadyExists => HttpResponse::Conflict().json(json!({
                "error": "Conflict",
                "message": "User already exists"
            })),
            _ => HttpResponse::BadRequest().json(json!({
                "error": "Bad Request",
                "message": self.to_string()
            })),
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub type DomainResult<T> = std::result::Result<T, DomainError>;