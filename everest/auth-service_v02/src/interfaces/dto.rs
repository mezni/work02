use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "john_doe")]
    pub username: String,
    
    #[schema(example = "john@example.com")]
    pub email: String,
}