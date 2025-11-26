// src/application/error.rs
use thiserror::Error;
use crate::domain::DomainError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Repository error: {0}")]
    Repository(String),
    
    #[error("External service error: {0}")]
    ExternalService(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl ApplicationError {
    pub fn validation(msg: &str) -> Self {
        ApplicationError::Validation(msg.to_string())
    }
    
    pub fn repository(msg: &str) -> Self {
        ApplicationError::Repository(msg.to_string())
    }
    
    pub fn external_service(msg: &str) -> Self {
        ApplicationError::ExternalService(msg.to_string())
    }
    
    pub fn authentication(msg: &str) -> Self {
        ApplicationError::Authentication(msg.to_string())
    }
    
    pub fn authorization(msg: &str) -> Self {
        ApplicationError::Authorization(msg.to_string())
    }
    
    pub fn internal(msg: &str) -> Self {
        ApplicationError::Internal(msg.to_string())
    }
}