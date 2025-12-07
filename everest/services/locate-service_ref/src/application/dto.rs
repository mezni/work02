use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct NearbyStationsQuery {
    pub latitude: f64,
    pub longitude: f64,
    pub radius_meters: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateReviewRequest {
    #[validate(length(min = 1))]
    pub station_id: String,
    
    #[validate(range(min = 1, max = 5))]
    pub rating: i32,
    
    pub review_text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateReviewRequest {
    #[validate(range(min = 1, max = 5))]
    pub rating: Option<i32>,
    
    pub review_text: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
}