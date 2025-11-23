use crate::domain::enums::network_type::NetworkType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", content = "data")]
pub enum NetworkEvent {
    NetworkCreated(NetworkCreated),
    NetworkVerified(NetworkVerified),
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkCreated {
    #[serde(with = "uuid::serde::compact")]
    pub network_id: Uuid,
    pub name: Option<String>,
    pub network_type: NetworkType,
    #[serde(with = "uuid::serde::compact")]
    pub created_by: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkVerified {
    #[serde(with = "uuid::serde::compact")]
    pub network_id: Uuid,
    #[serde(with = "uuid::serde::compact")]
    pub verified_by: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub verified_at: DateTime<Utc>,
}
