// src/domain/value_objects/username.rs
use crate::domain::DomainError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, DomainError> {
        let username = username.trim();

        if username.len() < 3 {
            return Err(DomainError::Validation(
                "Username must be at least 3 characters".to_string(),
            ));
        }

        if username.len() > 50 {
            return Err(DomainError::Validation(
                "Username must be less than 50 characters".to_string(),
            ));
        }

        if !username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
        {
            return Err(DomainError::Validation(
                "Username can only contain alphanumeric characters, hyphens, underscores, and dots"
                    .to_string(),
            ));
        }

        Ok(Username(username.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Username> for String {
    fn from(username: Username) -> Self {
        username.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
