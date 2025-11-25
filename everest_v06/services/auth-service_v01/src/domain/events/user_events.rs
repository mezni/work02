use super::{DomainEvent, Event, EventMetadata};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserRegisteredEvent {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub registered_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserVerifiedEvent {
    pub user_id: String,
    pub verified_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserRoleChangedEvent {
    pub user_id: String,
    pub old_role: String,
    pub new_role: String,
    pub changed_by: String,
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserAssignedToCompanyEvent {
    pub user_id: String,
    pub company_id: String,
    pub assigned_by: String,
    pub assigned_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserLoggedInEvent {
    pub user_id: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub logged_in_at: chrono::DateTime<chrono::Utc>,
}

// Implement Event trait for each event type
impl Event for UserRegisteredEvent {
    fn event_type(&self) -> String {
        "user.registered".to_string()
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn metadata(&self) -> EventMetadata {
        EventMetadata {
            correlation_id: None,
            causation_id: None,
            user_id: Some(self.user_id.clone()),
            source: "auth-service".to_string(),
        }
    }
}

impl Event for UserVerifiedEvent {
    fn event_type(&self) -> String {
        "user.verified".to_string()
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn metadata(&self) -> EventMetadata {
        EventMetadata {
            correlation_id: None,
            causation_id: None,
            user_id: Some(self.user_id.clone()),
            source: "auth-service".to_string(),
        }
    }
}

// Similar implementations for other event types...
impl Event for UserRoleChangedEvent {
    fn event_type(&self) -> String {
        "user.role_changed".to_string()
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn metadata(&self) -> EventMetadata {
        EventMetadata {
            correlation_id: None,
            causation_id: None,
            user_id: Some(self.user_id.clone()),
            source: "auth-service".to_string(),
        }
    }
}

impl Event for UserAssignedToCompanyEvent {
    fn event_type(&self) -> String {
        "user.assigned_to_company".to_string()
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn metadata(&self) -> EventMetadata {
        EventMetadata {
            correlation_id: None,
            causation_id: None,
            user_id: Some(self.user_id.clone()),
            source: "auth-service".to_string(),
        }
    }
}

impl Event for UserLoggedInEvent {
    fn event_type(&self) -> String {
        "user.logged_in".to_string()
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn metadata(&self) -> EventMetadata {
        EventMetadata {
            correlation_id: None,
            causation_id: None,
            user_id: Some(self.user_id.clone()),
            source: "auth-service".to_string(),
        }
    }
}
