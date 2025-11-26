use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub event_id: Uuid,
    pub aggregate_id: String,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub version: i32,
    pub metadata: EventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
    pub user_id: Option<String>,
    pub source: String,
}

pub trait Event: Send + Sync {
    fn event_type(&self) -> String;
    fn event_data(&self) -> serde_json::Value;
    fn metadata(&self) -> EventMetadata;
}

impl DomainEvent {
    pub fn new(
        aggregate_id: String,
        event_type: String,
        event_data: serde_json::Value,
        metadata: EventMetadata,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            aggregate_id,
            event_type,
            event_data,
            occurred_at: Utc::now(),
            version: 1,
            metadata,
        }
    }
}
