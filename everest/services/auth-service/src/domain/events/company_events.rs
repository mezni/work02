use super::{DomainEvent, Event, EventMetadata};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyCreatedEvent {
    pub company_id: String,
    pub name: String,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyUpdatedEvent {
    pub company_id: String,
    pub name: String,
    pub updated_by: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyDeletedEvent {
    pub company_id: String,
    pub deleted_by: String,
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

// Implement Event trait for each event type
impl Event for CompanyCreatedEvent {
    fn event_type(&self) -> String {
        "company.created".to_string()
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn metadata(&self) -> EventMetadata {
        EventMetadata {
            correlation_id: None,
            causation_id: None,
            user_id: Some(self.created_by.clone()),
            source: "auth-service".to_string(),
        }
    }
}

impl Event for CompanyUpdatedEvent {
    fn event_type(&self) -> String {
        "company.updated".to_string()
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn metadata(&self) -> EventMetadata {
        EventMetadata {
            correlation_id: None,
            causation_id: None,
            user_id: Some(self.updated_by.clone()),
            source: "auth-service".to_string(),
        }
    }
}

impl Event for CompanyDeletedEvent {
    fn event_type(&self) -> String {
        "company.deleted".to_string()
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn metadata(&self) -> EventMetadata {
        EventMetadata {
            correlation_id: None,
            causation_id: None,
            user_id: Some(self.deleted_by.clone()),
            source: "auth-service".to_string(),
        }
    }
}
