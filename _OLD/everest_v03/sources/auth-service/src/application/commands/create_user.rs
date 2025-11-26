// src/application/commands/create_user.rs
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserCommand {
    #[validate(length(min = 1, message = "Keycloak ID is required"))]
    pub keycloak_id: String,
    
    #[validate(length(min = 3, max = 50, message = "Username must be between 3-50 characters"))]
    pub username: String,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(max = 100, message = "First name too long"))]
    pub first_name: Option<String>,
    
    #[validate(length(max = 100, message = "Last name too long"))]
    pub last_name: Option<String>,
}

impl CreateUserCommand {
    pub fn new(
        keycloak_id: String,
        username: String,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Self {
        Self {
            keycloak_id,
            username,
            email,
            first_name,
            last_name,
        }
    }
}