use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Invalid email: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, EmailError> {
        let email = email.trim();
        if email.is_empty() || email.len() > 254 {
            return Err(EmailError::Invalid(email.into()));
        }

        let re = regex::Regex::new(r"^[^@]+@[^@]+\.[^@]+$").unwrap();
        if !re.is_match(email) || email.contains("..") {
            return Err(EmailError::Invalid(email.into()));
        }

        Ok(Email(email.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Email {
    type Err = EmailError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Email::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert!(Email::new("test@example.com").is_ok());
        assert!(Email::new("invalid").is_err());
    }
}
