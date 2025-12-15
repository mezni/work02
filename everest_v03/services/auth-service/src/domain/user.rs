use crate::core::id_generator;
use crate::domain::value_objects::{Email, Phone, UserRole, UserSource};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

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
        email: Email,
        username: String,
        role: UserRole,
        source: UserSource,
        network_id: String,
        station_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id: id_generator::generate_user_id(),
            keycloak_id,
            email: email.to_string(),
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

    pub fn email_obj(&self) -> Result<Email, String> {
        Email::new(&self.email)
    }

    pub fn phone_obj(&self) -> Result<Option<Phone>, String> {
        match &self.phone {
            Some(p) => Ok(Some(Phone::new(p)?)),
            None => Ok(None),
        }
    }

    pub fn validate_role_constraints(&self) -> Result<(), String> {
        let role = self.role()?;
        
        if role.requires_network_id() && (self.network_id.is_empty() || self.network_id == "X") {
            return Err(format!("{} role requires a valid network_id", role));
        }

        if role.requires_station_id() && (self.station_id.is_empty() || self.station_id == "X") {
            return Err(format!("{} role requires a valid station_id", role));
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

    pub fn update_profile(
        &mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        photo: Option<String>,
        updated_by: &str,
    ) {
        if let Some(fn_) = first_name {
            self.first_name = Some(fn_);
        }
        if let Some(ln) = last_name {
            self.last_name = Some(ln);
        }
        if let Some(p) = phone {
            self.phone = Some(p);
        }
        if let Some(ph) = photo {
            self.photo = Some(ph);
        }
        self.updated_by = Some(updated_by.to_string());
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self, by_user_id: &str) {
        self.is_active = false;
        self.updated_by = Some(by_user_id.to_string());
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self, by_user_id: &str) {
        self.is_active = true;
        self.updated_by = Some(by_user_id.to_string());
        self.updated_at = Utc::now();
    }

    pub fn verify_email(&mut self) {
        self.is_verified = true;
        self.updated_at = Utc::now();
    }

    pub fn can_be_modified_by(&self, modifier_role: &UserRole) -> bool {
        modifier_role.can_create_users()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "kc123".to_string(),
            Email::new("test@example.com").unwrap(),
            "testuser".to_string(),
            UserRole::User,
            UserSource::Web,
            "X".to_string(),
            "X".to_string(),
        );

        assert!(user.user_id.starts_with("USR"));
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.role, "user");
        assert!(user.is_active);
        assert!(!user.is_verified);
    }

    #[test]
    fn test_role_constraints() {
        let mut user = User::new(
            "kc123".to_string(),
            Email::new("partner@example.com").unwrap(),
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
    fn test_update_profile() {
        let mut user = User::new(
            "kc123".to_string(),
            Email::new("test@example.com").unwrap(),
            "testuser".to_string(),
            UserRole::User,
            UserSource::Web,
            "X".to_string(),
            "X".to_string(),
        );

        user.update_profile(
            Some("John".to_string()),
            Some("Doe".to_string()),
            None,
            None,
            "admin123",
        );

        assert_eq!(user.first_name, Some("John".to_string()));
        assert_eq!(user.last_name, Some("Doe".to_string()));
        assert_eq!(user.full_name(), "John Doe");
    }
}