use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(length(min = 3))]
    pub username: String,
    
    #[validate(email)]
    pub email: Option<String>,
    
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    
    #[validate(length(min = 8))]
    pub password: String,
}