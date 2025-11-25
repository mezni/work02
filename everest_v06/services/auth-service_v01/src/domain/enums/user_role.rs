use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, ToSchema, PartialEq, Eq)]
pub enum UserRole {
    #[strum(serialize = "admin")]
    Admin,

    #[strum(serialize = "partner")]
    Partner,

    #[strum(serialize = "operator")]
    Operator,

    #[strum(serialize = "user")]
    User,

    #[strum(serialize = "guest")]
    Guest,
}

impl UserRole {
    pub fn is_company_scoped(&self) -> bool {
        matches!(
            self,
            UserRole::Admin | UserRole::Partner | UserRole::Operator
        )
    }

    pub fn can_manage_users(&self) -> bool {
        matches!(
            self,
            UserRole::Admin | UserRole::Partner | UserRole::Operator
        )
    }

    pub fn can_manage_companies(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn from_str(role: &str) -> Option<Self> {
        match role.to_lowercase().as_str() {
            "admin" => Some(UserRole::Admin),
            "partner" => Some(UserRole::Partner),
            "operator" => Some(UserRole::Operator),
            "user" => Some(UserRole::User),
            "guest" => Some(UserRole::Guest),
            _ => None,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            UserRole::Admin,
            UserRole::Partner,
            UserRole::Operator,
            UserRole::User,
            UserRole::Guest,
        ]
    }
}
