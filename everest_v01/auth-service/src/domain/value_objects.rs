use serde::{Serialize, Deserialize};
use std::fmt;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct Email {
    #[validate(email)]
    pub value: String,
}

impl Email {
    pub fn new(value: String) -> DomainResult<Self> {
        let email = Email { value };
        email.validate()
            .map_err(|e| domain::error::DomainError::InvalidEmail(e.to_string()))?;
        Ok(email)
    }
    
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    pub hash: String,
}

impl Password {
    pub fn new(plain_text: &str) -> DomainResult<Self> {
        if plain_text.len() < 8 {
            return Err(domain::error::DomainError::InvalidPassword(
                "Password must be at least 8 characters".to_string()
            ));
        }
        
        // In practice, you'd hash the password here
        // For now, we'll just store it (but in real app, use argon2 or bcrypt)
        let hash = plain_text.to_string(); // TODO: Replace with actual hashing
        
        Ok(Password { hash })
    }
    
    pub fn verify(&self, plain_text: &str) -> bool {
        // In practice, compare with hash
        self.hash == plain_text // TODO: Replace with hash verification
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "manager")]
    Manager,
}

impl UserRole {
    pub fn from_str(role: &str) -> DomainResult<Self> {
        match role.to_lowercase().as_str() {
            "user" => Ok(UserRole::User),
            "admin" => Ok(UserRole::Admin),
            "manager" => Ok(UserRole::Manager),
            _ => Err(domain::error::DomainError::InvalidRole(
                format!("Invalid role: {}", role)
            )),
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => "user",
            UserRole::Admin => "admin",
            UserRole::Manager => "manager",
        }
    }
    
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CompanyName {
    #[validate(length(min = 1, max = 100))]
    pub value: String,
}

impl CompanyName {
    pub fn new(value: String) -> DomainResult<Self> {
        let company_name = CompanyName { value };
        company_name.validate()
            .map_err(|e| domain::error::DomainError::InvalidCompanyName(e.to_string()))?;
        Ok(company_name)
    }
    
    pub fn empty() -> Self {
        CompanyName { value: String::new() }
    }
    
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct StationName {
    #[validate(length(min = 1, max = 50))]
    pub value: String,
}

impl StationName {
    pub fn new(value: String) -> DomainResult<Self> {
        let station_name = StationName { value };
        station_name.validate()
            .map_err(|e| domain::error::DomainError::InvalidStationName(e.to_string()))?;
        Ok(station_name)
    }
    
    pub fn empty() -> Self {
        StationName { value: String::new() }
    }
    
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}