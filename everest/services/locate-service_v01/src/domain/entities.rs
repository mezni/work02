use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use bigdecimal::BigDecimal;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct NearbyStation {
    pub station_id: String,
    pub name: String,
    pub address: Option<String>,
    pub distance_meters: f64,
    pub has_available_connectors: bool,
    pub total_available_connectors: i64,

    #[schema(value_type = String)]
    pub max_power_kw: Option<BigDecimal>,

    pub power_tier: Option<String>,
    pub operator: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub operational_status: String,
    pub avg_rating: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct UserReview {
    pub review_id: String,
    pub user_id: String,
    pub station_id: String,
    pub rating: i32,
    pub review_text: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_by: Option<String>,
}

