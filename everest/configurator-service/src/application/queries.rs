use crate::error::{AppError, Result};

pub struct GetAppInfoQuery;

impl GetAppInfoQuery {
    pub fn execute(&self) -> Result<super::dtos::AppInfoResponse> {
        if std::env::var("SIMULATE_ERROR").is_ok() {
            return Err(AppError::Internal);
        }

        Ok(super::dtos::AppInfoResponse {
            name: "My Actix Web App".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            status: "healthy".to_string(),
        })
    }
}

pub struct GetUserQuery {
    pub user_id: u64,
}

impl GetUserQuery {
    pub fn execute(&self) -> Result<super::dtos::UserResponse> {
        if self.user_id == 0 {
            return Err(AppError::Validation("Invalid user ID".to_string()));
        }

        if self.user_id > 100 {
            return Err(AppError::NotFound(format!(
                "User {} not found",
                self.user_id
            )));
        }

        Ok(super::dtos::UserResponse {
            id: self.user_id,
            username: format!("user_{}", self.user_id),
            email: format!("user{}@example.com", self.user_id),
        })
    }
}
