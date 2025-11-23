// src/application/commands/update_user.rs
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserCommand {
    #[validate(length(max = 100, message = "First name too long"))]
    pub first_name: Option<String>,
    
    #[validate(length(max = 100, message = "Last name too long"))]
    pub last_name: Option<String>,
    
    #[validate(length(max = 20, message = "Phone number too long"))]
    pub phone_number: Option<String>,
}

impl UpdateUserCommand {
    pub fn new(
        first_name: Option<String>,
        last_name: Option<String>,
        phone_number: Option<String>,
    ) -> Self {
        Self {
            first_name,
            last_name,
            phone_number,
        }
    }
}