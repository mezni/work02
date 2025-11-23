// src/domain/value_objects/user_id.rs
use std::fmt;
use crate::domain::DomainError;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.trim().is_empty() {
            return Err(DomainError::Validation("User ID cannot be empty".to_string()));
        }
        Ok(UserId(id))
    }
    
    pub fn generate() -> Self {
        UserId(uuid::Uuid::new_v4().to_string())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<UserId> for String {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}