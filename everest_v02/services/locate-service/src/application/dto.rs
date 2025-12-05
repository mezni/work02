use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateReviewRequest {
    #[validate(length(min = 1, max = 100))]
    pub reviewer_name: String,
    
    #[validate(range(min = 1, max = 5))]
    #[schema(minimum = 1, maximum = 5)]
    pub rating: i32,
    
    #[validate(length(max = 1000))]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
}