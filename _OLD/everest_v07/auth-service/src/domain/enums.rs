use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum UserRole {
    Admin,
    Partner,
    Operator,
    User,
    Guest,
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "partner" => Ok(UserRole::Partner),
            "operator" => Ok(UserRole::Operator),
            "user" => Ok(UserRole::User),
            "guest" => Ok(UserRole::Guest),
            _ => Err(format!("Invalid user role: {}", s)),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::Partner => write!(f, "Partner"),
            UserRole::Operator => write!(f, "Operator"),
            UserRole::User => write!(f, "User"),
            UserRole::Guest => write!(f, "Guest"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum AuditAction {
    UserCreated,
    UserUpdated,
    UserDeleted,
    CompanyCreated,
    CompanyUpdated,
    CompanyDeleted,
    Login,
    Logout,
    PasswordReset,
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::UserCreated => write!(f, "UserCreated"),
            AuditAction::UserUpdated => write!(f, "UserUpdated"),
            AuditAction::UserDeleted => write!(f, "UserDeleted"),
            AuditAction::CompanyCreated => write!(f, "CompanyCreated"),
            AuditAction::CompanyUpdated => write!(f, "CompanyUpdated"),
            AuditAction::CompanyDeleted => write!(f, "CompanyDeleted"),
            AuditAction::Login => write!(f, "Login"),
            AuditAction::Logout => write!(f, "Logout"),
            AuditAction::PasswordReset => write!(f, "PasswordReset"),
        }
    }
}
