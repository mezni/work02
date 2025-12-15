use crate::core::id_generator;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Domain events for eventual consistency and event sourcing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum DomainEvent {
    UserCreated(UserCreatedEvent),
    UserUpdated(UserUpdatedEvent),
    UserDeleted(UserDeletedEvent),
    UserVerified(UserVerifiedEvent),
    UserDeactivated(UserDeactivatedEvent),
    UserReactivated(UserReactivatedEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub source: String,
    pub network_id: String,
    pub station_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdatedEvent {
    pub user_id: String,
    pub updated_fields: Vec<String>,
    pub updated_by: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeletedEvent {
    pub user_id: String,
    pub deleted_by: String,
    pub deleted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVerifiedEvent {
    pub user_id: String,
    pub verified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeactivatedEvent {
    pub user_id: String,
    pub deactivated_by: String,
    pub deactivated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReactivatedEvent {
    pub user_id: String,
    pub reactivated_by: String,
    pub reactivated_at: DateTime<Utc>,
}

/// Outbox pattern for reliable event publishing
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OutboxEvent {
    pub event_id: String,
    pub event_type: String,
    pub aggregate_id: String,
    pub payload: serde_json::Value,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

impl OutboxEvent {
    pub fn new(event: DomainEvent, aggregate_id: String) -> Self {
        let event_type = match &event {
            DomainEvent::UserCreated(_) => "UserCreated",
            DomainEvent::UserUpdated(_) => "UserUpdated",
            DomainEvent::UserDeleted(_) => "UserDeleted",
            DomainEvent::UserVerified(_) => "UserVerified",
            DomainEvent::UserDeactivated(_) => "UserDeactivated",
            DomainEvent::UserReactivated(_) => "UserReactivated",
        };

        Self {
            event_id: id_generator::generate_event_id(),
            event_type: event_type.to_string(),
            aggregate_id,
            payload: serde_json::to_value(&event).unwrap(),
            published: false,
            created_at: Utc::now(),
            published_at: None,
        }
    }

    pub fn mark_published(&mut self) {
        self.published = true;
        self.published_at = Some(Utc::now());
    }

    pub fn to_domain_event(&self) -> Result<DomainEvent, serde_json::Error> {
        serde_json::from_value(self.payload.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outbox_event_creation() {
        let event = DomainEvent::UserCreated(UserCreatedEvent {
            user_id: "USR123".to_string(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            role: "user".to_string(),
            source: "web".to_string(),
            network_id: "X".to_string(),
            station_id: "X".to_string(),
            created_at: Utc::now(),
        });

        let outbox = OutboxEvent::new(event, "USR123".to_string());

        assert!(outbox.event_id.starts_with("EVT"));
        assert_eq!(outbox.event_type, "UserCreated");
        assert!(!outbox.published);
    }

    #[test]
    fn test_outbox_mark_published() {
        let event = DomainEvent::UserVerified(UserVerifiedEvent {
            user_id: "USR123".to_string(),
            verified_at: Utc::now(),
        });

        let mut outbox = OutboxEvent::new(event, "USR123".to_string());
        assert!(!outbox.published);

        outbox.mark_published();
        assert!(outbox.published);
        assert!(outbox.published_at.is_some());
    }

    #[test]
    fn test_event_serialization() {
        let event = DomainEvent::UserUpdated(UserUpdatedEvent {
            user_id: "USR123".to_string(),
            updated_fields: vec!["email".to_string(), "phone".to_string()],
            updated_by: Some("ADM456".to_string()),
            updated_at: Utc::now(),
        });

        let outbox = OutboxEvent::new(event.clone(), "USR123".to_string());
        let deserialized = outbox.to_domain_event().unwrap();

        match deserialized {
            DomainEvent::UserUpdated(e) => {
                assert_eq!(e.user_id, "USR123");
                assert_eq!(e.updated_fields.len(), 2);
            }
            _ => panic!("Wrong event type"),
        }
    }
}