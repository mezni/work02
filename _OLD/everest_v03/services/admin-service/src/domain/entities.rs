use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Network {
    pub network_id: String,
    pub name: String,
    pub network_type: String,
    pub support_phone: Option<String>,
    pub support_email: Option<String>,
    pub is_verified: bool,
    pub is_active: bool,
    pub is_live: bool,
    pub created_by: String,
    pub updated_by: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Station {
    pub station_id: String,
    pub network_id: String,
    pub name: String,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub tags: Option<serde_json::Value>,
    pub operational_status: String,
    pub verification_status: String,
    pub is_live: bool,
    pub created_by: String,
    pub updated_by: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Charger {
    pub charger_id: String,
    pub station_id: String,
    pub serial_number: Option<String>,
    pub model_id: Option<String>,
    pub max_power_kw: Option<f64>,
    pub status: String,
    pub last_seen_at: Option<NaiveDateTime>,
    pub tags: Option<serde_json::Value>,
    pub is_live: bool,
    pub is_active: bool,
    pub is_verified: bool,
    pub created_by: String,
    pub updated_by: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Connector {
    pub connector_id: String,
    pub charger_id: String,
    pub station_id: String,
    pub connector_type_id: i32,
    pub connector_index: i32,
    pub capacity_kw: Option<f64>,
    pub max_current_a: Option<i32>,
    pub operational_status: String,
    pub verification_status: String,
    pub tags: Option<serde_json::Value>,
    pub is_live: bool,
    pub is_active: bool,
    pub is_verified: bool,
    pub created_by: String,
    pub updated_by: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}