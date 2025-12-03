use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use crate::application::error::ApplicationResult;

// Get User Query
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserQuery {
    pub user_id: Uuid,
}

impl GetUserQuery {
    pub fn validate_query(&self) -> ApplicationResult<()> {
        // Simple validation for UUID
        if self.user_id.is_nil() {
            return Err(crate::application::error::ApplicationError::Validation(
                "Invalid user ID".to_string()
            ));
        }
        
        Ok(())
    }
}

// Validate Token Query
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ValidateTokenQuery {
    #[validate(length(min = 1))]
    pub token: String,
}

impl ValidateTokenQuery {
    pub fn validate_query(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
}

// List Users Query (for admin)
#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub role: Option<String>,
    pub active: Option<bool>,
}

impl ListUsersQuery {
    pub fn validate_query(&self) -> ApplicationResult<()> {
        // Validate pagination parameters
        if let Some(page) = self.page {
            if page == 0 {
                return Err(crate::application::error::ApplicationError::Validation(
                    "Page must be greater than 0".to_string()
                ));
            }
        }
        
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 100 {
                return Err(crate::application::error::ApplicationError::Validation(
                    "Limit must be between 1 and 100".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or(1)
    }
    
    pub fn get_limit(&self) -> u32 {
        self.limit.unwrap_or(20)
    }
    
    pub fn get_offset(&self) -> u32 {
        (self.get_page() - 1) * self.get_limit()
    }
}

// Search Users Query
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SearchUsersQuery {
    #[validate(length(min = 1, max = 100))]
    pub query: Option<String>,
    
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl SearchUsersQuery {
    pub fn validate_query(&self) -> ApplicationResult<()> {
        if let Some(query) = &self.query {
            if query.len() < 1 || query.len() > 100 {
                return Err(crate::application::error::ApplicationError::Validation(
                    "Search query must be between 1 and 100 characters".to_string()
                ));
            }
        }
        
        if let Some(page) = self.page {
            if page == 0 {
                return Err(crate::application::error::ApplicationError::Validation(
                    "Page must be greater than 0".to_string()
                ));
            }
        }
        
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 50 {
                return Err(crate::application::error::ApplicationError::Validation(
                    "Limit must be between 1 and 50".to_string()
                ));
            }
        }
        
        Ok(())
    }
}

// User Statistics Query
#[derive(Debug, Serialize, Deserialize)]
pub struct UserStatisticsQuery {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

impl UserStatisticsQuery {
    pub fn validate_query(&self) -> ApplicationResult<()> {
        // Validate date range if both dates are provided
        if let (Some(start), Some(end)) = (self.start_date, self.end_date) {
            if start > end {
                return Err(crate::application::error::ApplicationError::Validation(
                    "Start date must be before end date".to_string()
                ));
            }
            
            // Check if date range is too large (e.g., more than 1 year)
            let max_days = 365;
            let days_diff = (end - start).num_days();
            if days_diff > max_days {
                return Err(crate::application::error::ApplicationError::Validation(
                    format!("Date range cannot exceed {} days", max_days)
                ));
            }
        }
        
        Ok(())
    }
}

// Token Information Query
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TokenInfoQuery {
    #[validate(length(min = 1))]
    pub token: String,
}

impl TokenInfoQuery {
    pub fn validate_query(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
}