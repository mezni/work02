use crate::domain::errors::DomainError;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    hash: String,
}

impl Password {
    pub fn new(plain_password: &str) -> Result<Self, DomainError> {
        if plain_password.len() < 8 {
            return Err(DomainError::InvalidPassword(
                "Password must be at least 8 characters long".to_string(),
            ));
        }

        if plain_password.len() > 100 {
            return Err(DomainError::InvalidPassword(
                "Password too long".to_string(),
            ));
        }

        // Check for whitespace
        if plain_password.contains(char::is_whitespace) {
            return Err(DomainError::InvalidPassword(
                "Password cannot contain whitespace".to_string(),
            ));
        }

        // Check for password complexity
        let has_upper = plain_password.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = plain_password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = plain_password.chars().any(|c| c.is_ascii_digit());

        if !(has_upper && has_lower && has_digit) {
            return Err(DomainError::InvalidPassword(
                "Password must contain uppercase, lowercase letters and digits".to_string(),
            ));
        }

        let hash = hash(plain_password, DEFAULT_COST)
            .map_err(|e| DomainError::InvalidPassword(e.to_string()))?;

        Ok(Self { hash })
    }

    pub fn from_hash(hash: String) -> Self {
        Self { hash }
    }

    pub fn verify(&self, plain_password: &str) -> bool {
        verify(plain_password, &self.hash).unwrap_or(false)
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }
}

impl TryFrom<String> for Password {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Password::new(&value)
    }
}
