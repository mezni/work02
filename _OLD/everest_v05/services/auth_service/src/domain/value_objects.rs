use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if validator::validate_email(&email) {
            Ok(Email(email))
        } else {
            Err("Invalid email format".to_string())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Result<Self, String> {
        if password.len() >= 8 {
            Ok(Password(password))
        } else {
            Err("Password must be at least 8 characters long".to_string())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}