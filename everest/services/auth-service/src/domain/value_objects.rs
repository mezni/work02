use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// User Role
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
    Partner,
    Operator,
}

impl UserRole {
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn can_create_users(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn can_delete_users(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn can_manage_network(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Partner)
    }

    pub fn can_manage_station(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Partner | UserRole::Operator)
    }

    pub fn requires_network_id(&self) -> bool {
        matches!(self, UserRole::Partner | UserRole::Operator)
    }

    pub fn requires_station_id(&self) -> bool {
        matches!(self, UserRole::Operator)
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::Partner => write!(f, "partner"),
            UserRole::Operator => write!(f, "operator"),
        }
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(UserRole::User),
            "admin" => Ok(UserRole::Admin),
            "partner" => Ok(UserRole::Partner),
            "operator" => Ok(UserRole::Operator),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

// User Source
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserSource {
    Web,
    Internal,
}

impl std::fmt::Display for UserSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSource::Web => write!(f, "web"),
            UserSource::Internal => write!(f, "internal"),
        }
    }
}

impl std::str::FromStr for UserSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "web" => Ok(UserSource::Web),
            "internal" => Ok(UserSource::Internal),
            _ => Err(format!("Invalid source: {}", s)),
        }
    }
}

// Email Value Object
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, String> {
        let email = email.trim().to_lowercase();
        
        if email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        
        if !email.contains('@') {
            return Err("Email must contain @".to_string());
        }
        
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err("Invalid email format".to_string());
        }
        
        if !parts[1].contains('.') {
            return Err("Email domain must contain a dot".to_string());
        }
        
        Ok(Email(email))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Phone Value Object
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct Phone(String);

impl Phone {
    pub fn new(phone: &str) -> Result<Self, String> {
        let phone = phone.trim().to_string();
        
        if phone.is_empty() {
            return Err("Phone cannot be empty".to_string());
        }
        
        // Basic validation - can be enhanced with regex
        if phone.len() < 10 || phone.len() > 20 {
            return Err("Phone must be between 10 and 20 characters".to_string());
        }
        
        Ok(Phone(phone))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Phone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Name Value Object
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self, String> {
        let name = name.trim().to_string();
        
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        if name.len() > 100 {
            return Err("Name cannot exceed 100 characters".to_string());
        }
        
        Ok(Name(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(Email::new("test@example.com").is_ok());
        assert!(Email::new("user.name+tag@example.co.uk").is_ok());
        assert!(Email::new("invalid").is_err());
        assert!(Email::new("@example.com").is_err());
        assert!(Email::new("test@").is_err());
        assert!(Email::new("test@domain").is_err());
    }

    #[test]
    fn test_phone_validation() {
        assert!(Phone::new("+1234567890").is_ok());
        assert!(Phone::new("123-456-7890").is_ok());
        assert!(Phone::new("123").is_err());
        assert!(Phone::new("12345678901234567890123").is_err());
    }

    #[test]
    fn test_name_validation() {
        assert!(Name::new("John Doe").is_ok());
        assert!(Name::new("").is_err());
        assert!(Name::new(&"a".repeat(101)).is_err());
    }

    #[test]
    fn test_user_role_permissions() {
        let admin = UserRole::Admin;
        assert!(admin.is_admin());
        assert!(admin.can_create_users());
        assert!(admin.can_delete_users());

        let user = UserRole::User;
        assert!(!user.is_admin());
        assert!(!user.requires_network_id());

        let partner = UserRole::Partner;
        assert!(partner.requires_network_id());
        assert!(!partner.requires_station_id());

        let operator = UserRole::Operator;
        assert!(operator.requires_network_id());
        assert!(operator.requires_station_id());
    }
}