// src/domain/value_objects/phone_number.rs
use std::fmt;
use crate::domain::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone: &str) -> Result<Self, DomainError> {
        let phone = phone.trim().replace(' ', "").replace('-', "");
        
        if phone.is_empty() {
            return Ok(PhoneNumber(phone)); // Optional field
        }
        
        if !phone.chars().all(|c| c.is_numeric() || c == '+') {
            return Err(DomainError::Validation("Phone number can only contain numbers and '+'".to_string()));
        }
        
        if phone.len() < 10 || phone.len() > 15 {
            return Err(DomainError::Validation("Phone number must be between 10-15 digits".to_string()));
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