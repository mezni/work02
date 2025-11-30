use crate::error::{AppError, Result};

pub struct UpdateAppNameCommand {
    pub new_name: String,
}

impl UpdateAppNameCommand {
    pub fn execute(&self) -> Result<String> {
        if self.new_name.is_empty() {
            return Err(AppError::Validation("App name cannot be empty".to_string()));
        }

        if self.new_name.len() > 50 {
            return Err(AppError::Validation("App name too long".to_string()));
        }

        Ok(format!("App name updated to: {}", self.new_name))
    }
}

pub struct CreateUserCommand {
    pub username: String,
    pub email: String,
}

impl CreateUserCommand {
    pub fn execute(&self) -> Result<super::dtos::UserResponse> {
        if self.username.is_empty() || self.email.is_empty() {
            return Err(AppError::Validation(
                "Username and email are required".to_string(),
            ));
        }

        if !self.email.contains('@') {
            return Err(AppError::Validation("Invalid email format".to_string()));
        }

        Ok(super::dtos::UserResponse {
            id: 1,
            username: self.username.clone(),
            email: self.email.clone(),
        })
    }
}
