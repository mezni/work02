use crate::domain::enums::network_type::NetworkType;
use crate::domain::models::network::Network;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct NetworkDto {
    #[serde(with = "uuid::serde::compact")]
    pub network_id: Uuid,
    pub name: Option<String>,
    pub network_type: NetworkType,
    pub is_verified: bool,
    pub is_active: bool,
    pub is_live: bool,

    #[serde(with = "uuid::serde::compact")]
    pub created_by: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<Uuid>,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Network> for NetworkDto {
    fn from(network: Network) -> Self {
        Self {
            network_id: network.network_id,
            name: network.name,
            network_type: network.network_type,
            is_verified: network.is_verified,
            is_active: network.is_active,
            is_live: network.is_live,
            created_by: network.created_by,
            updated_by: network.updated_by,
            created_at: network.created_at,
            updated_at: network.updated_at,
        }
    }
}
