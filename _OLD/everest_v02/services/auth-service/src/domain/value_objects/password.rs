use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password must be at least 8 characters long")]
    TooShort,
    #[error("Password must contain at least one uppercase letter")]
    NoUppercase,
    #[error("Password must contain at least one lowercase letter")]
    NoLowercase,
    #[error("Password must contain at least one digit")]
    NoDigit,
    #[error("Password must contain at least one special character")]
    NoSpecialChar,
}

/// Password value object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password(String);

impl Password {
    /// Create a new validated password
    pub fn new(password: &str) -> Result<Self, PasswordError> {
        Self::validate(password)?;
        Ok(Self(password.to_string()))
    }

    /// Validate password strength
    fn validate(password: &str) -> Result<(), PasswordError> {
        if password.len() < 8 {
            return Err(PasswordError::TooShort);
        }
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(PasswordError::NoUppercase);
        }
        if !password.chars().any(|c| c.is_lowercase()) {
            return Err(PasswordError::NoLowercase);
        }
        if !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(PasswordError::NoDigit);
        }
        if !password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(PasswordError::NoSpecialChar);
        }
        Ok(())
    }

    /// Get password as string reference
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Verify password against a hash (for login)
    pub fn verify(&self, hash: &str) -> bool {
        bcrypt::verify(self.as_str(), hash).unwrap_or(false)
    }

    /// Hash password for storage
    pub fn hash(&self) -> String {
        bcrypt::hash(self.as_str(), 12).unwrap_or_default()
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Show asterisks matching the actual password length
        write!(f, "{}", "*".repeat(self.0.len()))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
