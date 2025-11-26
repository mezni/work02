use crate::domain::enums::network_type::NetworkType;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNetworkRequest {
    pub name: Option<String>,
    pub network_type: NetworkType,
}
