use crate::domain::errors::DomainError;
use std::ops::Deref;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn parse(username: String) -> Result<Self, DomainError> {
        if username.len() >= 3 && username.len() <= 50 && username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            Ok(Self(username))
        } else {
            Err(DomainError::InvalidUsername(username))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for Username {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
