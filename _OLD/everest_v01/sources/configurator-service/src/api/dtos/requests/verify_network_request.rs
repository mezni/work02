use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyNetworkRequest {
    pub verified_by: Uuid,
}
