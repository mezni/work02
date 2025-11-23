use crate::domain::enums::network_type::NetworkType;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNetworkCommand {
    pub name: Option<String>,
    pub network_type: NetworkType,
    pub created_by: Uuid,
}
