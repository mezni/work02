// src/domain/value_objects/email.rs
use std::fmt;
use crate::domain::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, DomainError> {
        let email = email.trim().to_lowercase();
        
        if email.is_empty() {
            return Err(DomainError::Validation("Email cannot be empty".to_string()));
        }
        
        if !email.contains('@') {
            return Err(DomainError::Validation("Invalid email format".to_string()));
        }
        
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(DomainError::Validation("Invalid email format".to_string()));
        }
        
        if !parts[1].contains('.') {
            return Err(DomainError::Validation("Invalid email domain".to_string()));
        }
        
        Ok(Email(email))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}