use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyNetworkCommand {
    pub network_id: Uuid,
    pub verified_by: Uuid,
}
