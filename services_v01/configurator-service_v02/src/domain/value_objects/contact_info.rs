use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::email::Email;
use super::phone::Phone;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ContactInfoError {
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Invalid phone number")]
    InvalidPhone,
}

/// ContactInfo Value Object combining validated email and phone.
/// Fields are optional because a user might provide only email or only phone.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Option<Email>,
    pub phone: Option<Phone>,
}

impl ContactInfo {
    /// Creates a new ContactInfo after validating both fields.
    pub fn new(email: Option<String>, phone: Option<String>) -> Result<Self, ContactInfoError> {
        let email_vo = match email {
            Some(v) => Some(Email::new(v).map_err(|_| ContactInfoError::InvalidEmail)?),
            None => None,
        };

        let phone_vo = match phone {
            Some(v) => Some(Phone::new(v).map_err(|_| ContactInfoError::InvalidPhone)?),
            None => None,
        };

        Ok(Self {
            email: email_vo,
            phone: phone_vo,
        })
    }

    pub fn email(&self) -> Option<&Email> {
        self.email.as_ref()
    }

    pub fn phone(&self) -> Option<&Phone> {
        self.phone.as_ref()
    }

    /// Update email immutably
    pub fn with_email(&self, email: String) -> Result<Self, ContactInfoError> {
        let new_email = Email::new(email).map_err(|_| ContactInfoError::InvalidEmail)?;
        Ok(Self {
            email: Some(new_email),
            phone: self.phone.clone(),
        })
    }

    /// Update phone immutably
    pub fn with_phone(&self, phone: String) -> Result<Self, ContactInfoError> {
        let new_phone = Phone::new(phone).map_err(|_| ContactInfoError::InvalidPhone)?;
        Ok(Self {
            email: self.email.clone(),
            phone: Some(new_phone),
        })
    }
}
