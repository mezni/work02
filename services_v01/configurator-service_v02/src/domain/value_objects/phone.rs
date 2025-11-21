use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PhoneError {
    #[error("Invalid phone number format")]
    InvalidFormat,
}

/// Phone Value Object following E.164-like format.
/// Example valid numbers:
/// +12024561111
/// +212600001111
/// +447911123456
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phone(String);

impl Phone {
    /// Create a validated phone number.
    pub fn new(value: impl Into<String>) -> Result<Self, PhoneError> {
        let value = value.into();

        // E.164 pattern: "+" followed by 8–15 digits
        // Allows optional spaces, hyphens, or parentheses which we normalize.
        let cleaned = value
            .replace(|c: char| c == ' ' || c == '-' || c == '(', "")
            .replace(")", "");

        let regex = Regex::new(r"^\+[0-9]{8,15}$").unwrap();

        if !regex.is_match(&cleaned) {
            return Err(PhoneError::InvalidFormat);
        }

        Ok(Self(cleaned))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for Phone {
    type Err = PhoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Phone::new(s)
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
