use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// Network DTOs
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateNetworkRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    #[validate(length(min = 1))]
    pub network_type: String,
    
    pub support_phone: Option<String>,
    
    #[validate(email)]
    pub support_email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateNetworkRequest {
    pub name: Option<String>,
    pub support_phone: Option<String>,
    pub support_email: Option<String>,
}

// Station DTOs
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateStationRequest {
    #[validate(length(min = 1))]
    pub network_id: String,
    
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub operational_status: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateStationRequest {
    pub name: Option<String>,
    pub address: Option<String>,
    pub operational_status: Option<String>,
}

// Charger DTOs
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateChargerRequest {
    #[validate(length(min = 1))]
    pub station_id: String,
    
    pub serial_number: Option<String>,
    pub max_power_kw: Option<f64>,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateChargerRequest {
    pub max_power_kw: Option<f64>,
    pub status: Option<String>,
}

// Connector DTOs
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateConnectorRequest {
    #[validate(length(min = 1))]
    pub charger_id: String,
    
    #[validate(length(min = 1))]
    pub station_id: String,
    
    pub connector_type_id: i32,
    pub connector_index: i32,
    pub capacity_kw: Option<f64>,
    pub operational_status: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateConnectorRequest {
    pub capacity_kw: Option<f64>,
    pub operational_status: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
}