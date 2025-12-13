use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use crate::domain::value_objects::{UserRole, UserSource};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub user_id: String,
    pub keycloak_id: String,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub photo: Option<String>,
    pub is_verified: bool,
    pub role: String,
    pub network_id: String,
    pub station_id: String,
    pub source: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl User {
    pub fn new(
        keycloak_id: String,
        email: String,
        username: String,
        role: UserRole,
        source: UserSource,
        network_id: String,
        station_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id: format!("USR{}", nanoid::nanoid!(29)),
            keycloak_id,
            email,
            username,
            first_name: None,
            last_name: None,
            phone: None,
            photo: None,
            is_verified: false,
            role: role.to_string(),
            network_id,
            station_id,
            source: source.to_string(),
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }

    pub fn role(&self) -> Result<UserRole, String> {
        self.role.parse()
    }

    pub fn source(&self) -> Result<UserSource, String> {
        self.source.parse()
    }

    pub fn validate_role_constraints(&self) -> Result<(), String> {
        let role = self.role()?;
        
        match role {
            UserRole::Partner => {
                if self.network_id.is_empty() || self.network_id == "X" {
                    return Err("Partner role requires a valid network_id".to_string());
                }
            }
            UserRole::Operator => {
                if self.network_id.is_empty() || self.network_id == "X" {
                    return Err("Operator role requires a valid network_id".to_string());
                }
                if self.station_id.is_empty() || self.station_id == "X" {
                    return Err("Operator role requires a valid station_id".to_string());
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn full_name(&self) -> String {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            (Some(first), None) => first.clone(),
            (None, Some(last)) => last.clone(),
            (None, None) => self.username.clone(),
        }
    }

    pub fn can_be_modified_by(&self, modifier_role: &UserRole) -> bool {
        match modifier_role {
            UserRole::Admin => true,
            _ => false,
        }
    }

    pub fn deactivate(&mut self, by_user_id: &str) {
        self.is_active = false;
        self.updated_by = Some(by_user_id.to_string());
    }

    pub fn activate(&mut self, by_user_id: &str) {
        self.is_active = true;
        self.updated_by = Some(by_user_id.to_string());
    }

    pub fn verify(&mut self) {
        self.is_verified = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "kc123".to_string(),
            "test@example.com".to_string(),
            "testuser".to_string(),
            UserRole::User,
            UserSource::Web,
            "X".to_string(),
            "X".to_string(),
        );

        assert!(user.user_id.starts_with("USR"));
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.role, "user");
    }

    #[test]
    fn test_role_constraints_partner() {
        let mut user = User::new(
            "kc123".to_string(),
            "partner@example.com".to_string(),
            "partner1".to_string(),
            UserRole::Partner,
            UserSource::Internal,
            "X".to_string(),
            "X".to_string(),
        );

        assert!(user.validate_role_constraints().is_err());

        user.network_id = "NET123".to_string();
        assert!(user.validate_role_constraints().is_ok());
    }

    #[test]
    fn test_role_constraints_operator() {
        let mut user = User::new(
            "kc123".to_string(),
            "operator@example.com".to_string(),
            "operator1".to_string(),
            UserRole::Operator,
            UserSource::Internal,
            "X".to_string(),
            "X".to_string(),
        );

        assert!(user.validate_role_constraints().is_err());

        user.network_id = "NET123".to_string();
        assert!(user.validate_role_constraints().is_err());

        user.station_id = "STA456".to_string();
        assert!(user.validate_role_constraints().is_ok());
    }
}