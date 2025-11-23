// src/domain/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Business rule violation: {0}")]
    BusinessRule(String),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Repository error: {0}")]
    Repository(String),
    
    #[error("Unknown domain error: {0}")]
    Unknown(String),
}

impl DomainError {
    pub fn validation(msg: &str) -> Self {
        DomainError::Validation(msg.to_string())
    }
    
    pub fn business_rule(msg: &str) -> Self {
        DomainError::BusinessRule(msg.to_string())
    }
}