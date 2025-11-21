use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EmailError {
    #[error("Invalid email format")]
    InvalidFormat,
}

/// A strongly typed email value object.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// Create a new validated Email.
    pub fn new(value: impl Into<String>) -> Result<Self, EmailError> {
        let value = value.into();

        let regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

        if !regex.is_match(&value) {
            return Err(EmailError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Get the email value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Allow parsing from string.
impl FromStr for Email {
    type Err = EmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Email::new(s)
    }
}

/// Display the underlying email.
impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
