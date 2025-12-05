use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Station {
    pub station_id: i32,
    pub network_id: i32,
    pub name: String,
    pub address: String,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub total_ports: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StationReview {
    pub review_id: i32,
    pub station_id: i32,
    pub reviewer_name: String,
    #[schema(minimum = 1, maximum = 5)]
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StationWithReviews {
    #[serde(flatten)]
    pub station: Station,
    pub reviews: Vec<StationReview>,
    pub average_rating: Option<f64>,
}