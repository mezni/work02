use crate::application::dtos::NetworkDto;
use crate::domain::enums::network_type::NetworkType;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct NetworkResponse {
    pub network_id: Uuid, // Default serialization as string
    pub name: Option<String>,
    pub network_type: NetworkType,
    pub is_verified: bool,
    pub is_active: bool,
    pub is_live: bool,

    pub created_by: Uuid, // Default serialization as string

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<Uuid>, // Default serialization handles Option fine

    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<NetworkDto> for NetworkResponse {
    fn from(dto: NetworkDto) -> Self {
        Self {
            network_id: dto.network_id,
            name: dto.name,
            network_type: dto.network_type,
            is_verified: dto.is_verified,
            is_active: dto.is_active,
            is_live: dto.is_live,
            created_by: dto.created_by,
            updated_by: dto.updated_by,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}
