use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq, Eq, Hash)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl Email {
    pub fn new(value: String) -> Result<Self, DomainError> {
        let email = Self {
            value: value.trim().to_lowercase(),
        };

        email
            .validate()
            .map_err(|e| DomainError::InvalidEmail(e.to_string()))?;

        Ok(email)
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> String {
        email.value
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
