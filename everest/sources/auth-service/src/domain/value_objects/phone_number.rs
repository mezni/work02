// src/domain/value_objects/phone_number.rs
use crate::domain::DomainError;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone: &str) -> Result<Self, DomainError> {
        let phone = phone
            .trim()
            .replace(' ', "")
            .replace('-', "")
            .replace('(', "")
            .replace(')', "");

        if phone.is_empty() {
            return Ok(PhoneNumber(phone)); // Optional field
        }

        // Allow + at start and digits only
        let mut chars = phone.chars();
        if let Some('+') = chars.next() {
            // If starts with +, the rest must be digits
            if !chars.all(|c| c.is_numeric()) {
                return Err(DomainError::Validation(
                    "Phone number can only contain numbers and '+' at start".to_string(),
                ));
            }
        } else {
            // If no +, all must be digits
            if !phone.chars().all(|c| c.is_numeric()) {
                return Err(DomainError::Validation(
                    "Phone number can only contain numbers and '+' at start".to_string(),
                ));
            }
        }

        // Count digits only for length validation
        let digit_count = phone.chars().filter(|c| c.is_numeric()).count();
        if digit_count < 10 || digit_count > 15 {
            return Err(DomainError::Validation(
                "Phone number must be between 10-15 digits".to_string(),
            ));
        }

        Ok(PhoneNumber(phone))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<PhoneNumber> for String {
    fn from(phone: PhoneNumber) -> Self {
        phone.0
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
