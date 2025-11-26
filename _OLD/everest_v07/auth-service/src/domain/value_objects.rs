use validator::Validate;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl Email {
    pub fn new(value: String) -> Result<Self, crate::domain::errors::DomainError> {
        let email = Email { value };
        email.validate()
            .map_err(|e| crate::domain::errors::DomainError::InvalidEmail(e.to_string()))?;
        Ok(email)
    }
    
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn new(value: String) -> Result<Self, crate::domain::errors::DomainError> {
        if value.len() < 8 {
            return Err(crate::domain::errors::DomainError::InvalidPassword(
                "Password must be at least 8 characters long".to_string(),
            ));
        }
        
        Ok(Password { value })
    }
    
    pub fn value(&self) -> &str {
        &self.value
    }
    
    pub fn into_inner(self) -> String {
        self.value
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "***") // Don't expose password in logs
    }
}
