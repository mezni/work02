use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserSource {
    Web,
    Internal,
}

impl UserSource {
    pub fn is_web(&self) -> bool {
        matches!(self, UserSource::Web)
    }

    pub fn is_internal(&self) -> bool {
        matches!(self, UserSource::Internal)
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if email.contains('@') && email.len() > 3 {
            Ok(Email(email))
        } else {
            Err("Invalid email format".to_string())
        }
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Result<Self, String> {
        if username.len() >= 3 && username.len() <= 100 {
            Ok(Username(username))
        } else {
            Err("Username must be between 3 and 100 characters".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkId(String);

impl NetworkId {
    pub fn new(id: String) -> Self {
        NetworkId(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn is_default(&self) -> bool {
        self.0 == "X" || self.0.is_empty()
    }
}

impl std::fmt::Display for NetworkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StationId(String);

impl StationId {
    pub fn new(id: String) -> Self {
        StationId(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn is_default(&self) -> bool {
        self.0 == "X" || self.0.is_empty()
    }
}

impl std::fmt::Display for StationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_permissions() {
        let admin = UserRole::Admin;
        assert!(admin.is_admin());
        assert!(admin.can_create_users());
        assert!(admin.can_delete_users());

        let user = UserRole::User;
        assert!(!user.is_admin());
        assert!(!user.can_create_users());
        assert!(!user.requires_network_id());

        let partner = UserRole::Partner;
        assert!(partner.can_manage_network());
        assert!(partner.requires_network_id());
        assert!(!partner.requires_station_id());

        let operator = UserRole::Operator;
        assert!(operator.requires_network_id());
        assert!(operator.requires_station_id());
    }

    #[test]
    fn test_email_validation() {
        assert!(Email::new("test@example.com".to_string()).is_ok());
        assert!(Email::new("invalid".to_string()).is_err());
        assert!(Email::new("@".to_string()).is_err());
    }

    #[test]
    fn test_username_validation() {
        assert!(Username::new("validuser".to_string()).is_ok());
        assert!(Username::new("ab".to_string()).is_err());
        assert!(Username::new("a".repeat(101)).is_err());
    }

    #[test]
    fn test_network_station_ids() {
        let net = NetworkId::new("NET123".to_string());
        assert!(!net.is_default());

        let default_net = NetworkId::new("X".to_string());
        assert!(default_net.is_default());

        let sta = StationId::new("STA456".to_string());
        assert!(!sta.is_default());
    }
}