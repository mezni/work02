use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Invalid email format: {0}")]
    InvalidFormat(String),
}

impl Email {
    pub fn new(email: String) -> Result<Self, EmailError> {
        if email.contains('@') && email.len() > 3 {
            Ok(Self(email))
        } else {
            Err(EmailError::InvalidFormat(email))
        }
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

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}