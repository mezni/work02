use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Invalid email format: {0}")]
    InvalidFormat(String),
    #[error("Email is too long")]
    TooLong,
    #[error("Email is empty")]
    Empty,
}

/// Email value object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// Create a new validated email
    pub fn new(email: &str) -> Result<Self, EmailError> {
        Self::validate(email)?;
        Ok(Self(email.to_lowercase()))
    }

    /// Validate email format
    fn validate(email: &str) -> Result<(), EmailError> {
        if email.is_empty() {
            return Err(EmailError::Empty);
        }

        if email.len() > 254 {
            return Err(EmailError::TooLong);
        }

        let email_regex = regex::Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$"
        ).unwrap();

        if !email_regex.is_match(email) {
            return Err(EmailError::InvalidFormat(email.to_string()));
        }

        // TLD should be at least 2 characters
        if let Some(domain) = email.split('@').nth(1) {
            if let Some(tld) = domain.split('.').last() {
                if tld.len() < 2 {
                    return Err(EmailError::InvalidFormat(email.to_string()));
                }
            }
        }

        Ok(())
    }

    /// Get email as string reference
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get domain part of email
    pub fn domain(&self) -> Option<&str> {
        self.0.split('@').nth(1)
    }

    /// Get local part of email (before @)
    pub fn local_part(&self) -> Option<&str> {
        self.0.split('@').next()
    }

    /// Check if email is from a disposable email provider
    pub fn is_disposable(&self) -> bool {
        let disposable_domains = [
            "tempmail.com",
            "guerrillamail.com",
            "mailinator.com",
            "10minutemail.com",
            "yopmail.com",
            "throwaway.com",
            "fakeinbox.com",
            "trashmail.com",
        ];

        if let Some(domain) = self.domain() {
            disposable_domains.iter().any(|d| domain.ends_with(d))
        } else {
            false
        }
    }

    /// Compare with string (case-insensitive)
    pub fn eq_str(&self, other: &str) -> bool {
        self.0 == other.to_lowercase()
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<String> for Email {
    fn eq(&self, other: &String) -> bool {
        self.0 == other.to_lowercase()
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl TryFrom<&str> for Email {
    type Error = EmailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}
