use regex::Regex;
use crate::domain::errors::DomainError;
use std::ops::Deref;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Self, DomainError> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if email_regex.is_match(&email) {
            Ok(Self(email))
        } else {
            Err(DomainError::InvalidEmail(email))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for Email {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
