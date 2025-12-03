use serde::{Serialize, Deserialize};
use secrecy::{Secret, ExposeSecret};

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}

impl Credentials {
    pub fn new(email: String, password: String) -> Self {
        Self {
            email,
            password: Secret::new(password),
        }
    }
    
    pub fn validate(&self) -> Result<(), super::error::DomainError> {
        if self.email.is_empty() {
            return Err(super::error::DomainError::InvalidEmail(
                "Email cannot be empty".to_string()
            ));
        }
        
        if self.password.expose_secret().is_empty() {
            return Err(super::error::DomainError::InvalidPassword(
                "Password cannot be empty".to_string()
            ));
        }
        
        if self.password.expose_secret().len() < 8 {
            return Err(super::error::DomainError::InvalidPassword(
                "Password must be at least 8 characters".to_string()
            ));
        }
        
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

impl LoginRequest {
    pub fn to_credentials(&self) -> Credentials {
        Credentials::new(self.email.clone(), self.password.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

impl ChangePasswordRequest {
    pub fn validate(&self) -> Result<(), super::error::DomainError> {
        if self.new_password != self.confirm_password {
            return Err(super::error::DomainError::InvalidPassword(
                "New password and confirmation do not match".to_string()
            ));
        }
        
        if self.new_password.len() < 8 {
            return Err(super::error::DomainError::InvalidPassword(
                "New password must be at least 8 characters".to_string()
            ));
        }
        
        Ok(())
    }
}