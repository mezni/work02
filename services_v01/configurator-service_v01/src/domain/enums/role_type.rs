use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoleTypeError {
    #[error("Invalid role type: {0}")]
    InvalidType(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoleType {
    Admin,
    Billing,
    Technical,
    Operations,
    General,
}

impl RoleType {
    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin)
    }

    pub fn can_manage_users(&self) -> bool {
        matches!(self, Self::Admin)
    }

    pub fn can_manage_billing(&self) -> bool {
        matches!(self, Self::Admin | Self::Billing)
    }

    pub fn can_manage_operations(&self) -> bool {
        matches!(self, Self::Admin | Self::Operations | Self::Technical)
    }

    pub fn can_manage_technical(&self) -> bool {
        matches!(self, Self::Admin | Self::Technical)
    }
}

impl fmt::Display for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Admin => "ADMIN",
            Self::Billing => "BILLING",
            Self::Technical => "TECHNICAL",
            Self::Operations => "OPERATIONS",
            Self::General => "GENERAL",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for RoleType {
    type Err = RoleTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ADMIN" => Ok(Self::Admin),
            "BILLING" => Ok(Self::Billing),
            "TECHNICAL" => Ok(Self::Technical),
            "OPERATIONS" => Ok(Self::Operations),
            "GENERAL" => Ok(Self::General),
            _ => Err(RoleTypeError::InvalidType(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_type_permissions() {
        assert!(RoleType::Admin.can_manage_users());
        assert!(RoleType::Admin.can_manage_billing());
        assert!(RoleType::Billing.can_manage_billing());
        assert!(!RoleType::Billing.can_manage_users());
    }
}
