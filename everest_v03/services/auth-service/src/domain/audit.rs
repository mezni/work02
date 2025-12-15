use crate::core::id_generator;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditAction {
    UserCreated,
    UserUpdated,
    UserDeleted,
    UserLoggedIn,
    UserLoggedOut,
    UserPasswordChanged,
    UserRoleChanged,
    UserVerified,
    UserDeactivated,
    UserReactivated,
    UserProfileViewed,
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AuditAction::UserCreated => "USER_CREATED",
            AuditAction::UserUpdated => "USER_UPDATED",
            AuditAction::UserDeleted => "USER_DELETED",
            AuditAction::UserLoggedIn => "USER_LOGGED_IN",
            AuditAction::UserLoggedOut => "USER_LOGGED_OUT",
            AuditAction::UserPasswordChanged => "USER_PASSWORD_CHANGED",
            AuditAction::UserRoleChanged => "USER_ROLE_CHANGED",
            AuditAction::UserVerified => "USER_VERIFIED",
            AuditAction::UserDeactivated => "USER_DEACTIVATED",
            AuditAction::UserReactivated => "USER_REACTIVATED",
            AuditAction::UserProfileViewed => "USER_PROFILE_VIEWED",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Audit {
    pub audit_id: String,
    pub user_id: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub ip_address: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub user_agent: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

impl Audit {
    pub fn new(
        user_id: String,
        action: AuditAction,
        resource_type: String,
        resource_id: Option<String>,
    ) -> Self {
        Self {
            audit_id: id_generator::generate_audit_id(),
            user_id,
            action: action.to_string(),
            resource_type,
            resource_id,
            ip_address: None,
            country: None,
            city: None,
            latitude: None,
            longitude: None,
            user_agent: None,
            metadata: None,
            created_at: Utc::now(),
        }
    }

    pub fn with_location(
        mut self,
        ip: String,
        country: Option<String>,
        city: Option<String>,
        lat: Option<f64>,
        lon: Option<f64>,
    ) -> Self {
        self.ip_address = Some(ip);
        self.country = country;
        self.city = city;
        self.latitude = lat;
        self.longitude = lon;
        self
    }

    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn location_string(&self) -> Option<String> {
        match (&self.city, &self.country) {
            (Some(city), Some(country)) => Some(format!("{}, {}", city, country)),
            (None, Some(country)) => Some(country.clone()),
            (Some(city), None) => Some(city.clone()),
            (None, None) => None,
        }
    }

    pub fn action_enum(&self) -> Option<AuditAction> {
        match self.action.as_str() {
            "USER_CREATED" => Some(AuditAction::UserCreated),
            "USER_UPDATED" => Some(AuditAction::UserUpdated),
            "USER_DELETED" => Some(AuditAction::UserDeleted),
            "USER_LOGGED_IN" => Some(AuditAction::UserLoggedIn),
            "USER_LOGGED_OUT" => Some(AuditAction::UserLoggedOut),
            "USER_PASSWORD_CHANGED" => Some(AuditAction::UserPasswordChanged),
            "USER_ROLE_CHANGED" => Some(AuditAction::UserRoleChanged),
            "USER_VERIFIED" => Some(AuditAction::UserVerified),
            "USER_DEACTIVATED" => Some(AuditAction::UserDeactivated),
            "USER_REACTIVATED" => Some(AuditAction::UserReactivated),
            "USER_PROFILE_VIEWED" => Some(AuditAction::UserProfileViewed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GeoLocation {
    pub ip: String,
    pub country: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl GeoLocation {
    pub fn new(ip: String) -> Self {
        Self {
            ip,
            country: None,
            city: None,
            latitude: None,
            longitude: None,
        }
    }

    pub fn with_location(mut self, country: String, city: String) -> Self {
        self.country = Some(country);
        self.city = Some(city);
        self
    }

    pub fn with_coordinates(mut self, lat: f64, lon: f64) -> Self {
        self.latitude = Some(lat);
        self.longitude = Some(lon);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_creation() {
        let audit = Audit::new(
            "USR123".to_string(),
            AuditAction::UserCreated,
            "user".to_string(),
            Some("USR456".to_string()),
        );

        assert!(audit.audit_id.starts_with("AUD"));
        assert_eq!(audit.action, "USER_CREATED");
        assert_eq!(audit.user_id, "USR123");
    }

    #[test]
    fn test_location_string() {
        let audit = Audit::new(
            "USR123".to_string(),
            AuditAction::UserLoggedIn,
            "user".to_string(),
            None,
        )
        .with_location(
            "192.168.1.1".to_string(),
            Some("Canada".to_string()),
            Some("Toronto".to_string()),
            Some(43.65),
            Some(-79.38),
        );

        assert_eq!(audit.location_string(), Some("Toronto, Canada".to_string()));
    }

    #[test]
    fn test_action_enum() {
        let audit = Audit::new(
            "USR123".to_string(),
            AuditAction::UserCreated,
            "user".to_string(),
            None,
        );

        assert_eq!(audit.action_enum(), Some(AuditAction::UserCreated));
    }
}