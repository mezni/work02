use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UserId = Uuid;
pub type OrganizationId = Uuid;
pub type StationId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    #[serde(rename = "super_admin")]
    SuperAdmin,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "operator")]
    Operator,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::SuperAdmin => write!(f, "super_admin"),
            UserRole::Partner => write!(f, "partner"),
            UserRole::Operator => write!(f, "operator"),
        }
    }
}

impl TryFrom<&str> for UserRole {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "super_admin" => Ok(UserRole::SuperAdmin),
            "partner" => Ok(UserRole::Partner),
            "operator" => Ok(UserRole::Operator),
            _ => Err(format!("Invalid user role: {}", value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "deleted")]
    Deleted,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Pending => write!(f, "pending"),
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Deleted => write!(f, "deleted"),
        }
    }
}

impl TryFrom<&str> for UserStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "pending" => Ok(UserStatus::Pending),
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "deleted" => Ok(UserStatus::Deleted),
            _ => Err(format!("Invalid user status: {}", value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

impl std::fmt::Display for OrganizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrganizationStatus::Active => write!(f, "active"),
            OrganizationStatus::Inactive => write!(f, "inactive"),
        }
    }
}

impl TryFrom<&str> for OrganizationStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "active" => Ok(OrganizationStatus::Active),
            "inactive" => Ok(OrganizationStatus::Inactive),
            _ => Err(format!("Invalid organization status: {}", value)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuditInfo {
    pub created_by: UserId,
    pub updated_by: UserId,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AuditInfo {
    pub fn new(created_by: UserId) -> Self {
        let now = chrono::Utc::now();
        Self {
            created_by,
            updated_by: created_by,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, updated_by: UserId) {
        self.updated_by = updated_by;
        self.updated_at = chrono::Utc::now();
    }
}

// Common validation functions
pub fn validate_email(email: &str) -> Result<(), String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }

    if !email.contains('@') {
        return Err("Email must contain @ symbol".to_string());
    }

    if email.len() > 255 {
        return Err("Email cannot exceed 255 characters".to_string());
    }

    Ok(())
}

pub fn validate_display_name(display_name: &str) -> Result<(), String> {
    if display_name.trim().is_empty() {
        return Err("Display name cannot be empty".to_string());
    }

    if display_name.len() > 255 {
        return Err("Display name cannot exceed 255 characters".to_string());
    }

    Ok(())
}
