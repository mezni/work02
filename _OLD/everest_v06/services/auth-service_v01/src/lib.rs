//! Auth Service
//! 
//! A microservice for handling authentication, authorization, and user management
//! with Keycloak integration and JWT token support.

#![warn(missing_docs)]
#![warn(unused_crate_dependencies)]

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;

// Re-export commonly used types for easier consumption
pub use application::errors::ApplicationError;
pub use infrastructure::errors::InfrastructureError;
pub use domain::errors::DomainError;

// Re-export configuration
pub use infrastructure::config::Settings;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::application::{
        commands::*,
        queries::*,
        dto::*,
        services::*,
        errors::ApplicationError,
    };
    
    pub use crate::domain::{
        entities::{User, Company, AuditLog},
        value_objects::{Email, Password},
        enums::{UserRole, AuditAction},
        repositories::{UserRepository, CompanyRepository, AuditRepository},
        errors::DomainError,
    };
    
    pub use crate::infrastructure::{
        auth::{JwtService, KeycloakClient},
        audit::Auditor,
        config::Settings,
        errors::InfrastructureError,
    };
    
    pub use crate::interfaces::{
        controllers::*,
        routes::*,
        openapi::ApiDoc,
    };
}

/// Application version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Application description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Health check response
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct HealthCheck {
    /// Service name
    pub service: String,
    /// Service version
    pub version: String,
    /// Service status
    pub status: String,
    /// Timestamp of the check
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HealthCheck {
    /// Create a new health check response
    pub fn new() -> Self {
        Self {
            service: NAME.to_string(),
            version: VERSION.to_string(),
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

/// API response wrapper
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ApiResponse<T> {
    /// Whether the request was successful
    pub success: bool,
    /// Response data
    pub data: Option<T>,
    /// Error message if any
    pub message: Option<String>,
    /// HTTP status code
    pub status_code: u16,
}

impl<T> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            status_code: 200,
        }
    }
    
    /// Create an error response
    pub fn error(message: String, status_code: u16) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            status_code,
        }
    }
}

/// Error response for API
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// HTTP status code
    pub status_code: u16,
    /// Optional details about the error
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    /// Create a new error response
    pub fn new(code: String, message: String, status_code: u16) -> Self {
        Self {
            code,
            message,
            status_code,
            details: None,
        }
    }
    
    /// Add details to the error response
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Pagination parameters
#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct PaginationParams {
    /// Page number (1-based)
    pub page: Option<u32>,
    /// Number of items per page
    pub page_size: Option<u32>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(20),
        }
    }
}

impl PaginationParams {
    /// Get the page number
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
    }
    
    /// Get the page size
    pub fn page_size(&self) -> u32 {
        self.page_size.unwrap_or(20)
    }
    
    /// Calculate the offset for database queries
    pub fn offset(&self) -> u32 {
        (self.page() - 1) * self.page_size()
    }
}

/// Paginated response
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct PaginatedResponse<T> {
    /// The data items
    pub items: Vec<T>,
    /// Total number of items
    pub total: u64,
    /// Current page number
    pub page: u32,
    /// Number of items per page
    pub page_size: u32,
    /// Total number of pages
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    pub fn new(items: Vec<T>, total: u64, page: u32, page_size: u32) -> Self {
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;
        
        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}

/// Utility functions
pub mod utils {
    use uuid::Uuid;
    
    /// Generate a new UUID
    pub fn generate_uuid() -> Uuid {
        Uuid::new_v4()
    }
    
    /// Validate if a string is a valid UUID
    pub fn is_valid_uuid(uuid_str: &str) -> bool {
        Uuid::parse_str(uuid_str).is_ok()
    }
    
    /// Hash a password (placeholder - use proper hashing in real implementation)
    pub fn hash_password(_password: &str) -> String {
        // In real implementation, use Argon2 or bcrypt
        // For now, return a placeholder
        "hashed_password".to_string()
    }
    
    /// Verify a password against a hash (placeholder)
    pub fn verify_password(_password: &str, _hash: &str) -> bool {
        // In real implementation, use proper verification
        // For now, return true for testing
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_health_check() {
        let health = HealthCheck::new();
        assert_eq!(health.service, NAME);
        assert_eq!(health.version, VERSION);
        assert_eq!(health.status, "healthy");
    }
    
    #[test]
    fn test_api_response() {
        let success_response = ApiResponse::success("test data");
        assert!(success_response.success);
        assert_eq!(success_response.status_code, 200);
        
        let error_response = ApiResponse::error("test error".to_string(), 400);
        assert!(!error_response.success);
        assert_eq!(error_response.status_code, 400);
    }
    
    #[test]
    fn test_pagination_params() {
        let params = PaginationParams::default();
        assert_eq!(params.page(), 1);
        assert_eq!(params.page_size(), 20);
        assert_eq!(params.offset(), 0);
        
        let custom_params = PaginationParams {
            page: Some(3),
            page_size: Some(10),
        };
        assert_eq!(custom_params.page(), 3);
        assert_eq!(custom_params.page_size(), 10);
        assert_eq!(custom_params.offset(), 20);
    }
    
    #[test]
    fn test_paginated_response() {
        let items = vec!["item1", "item2", "item3"];
        let response = PaginatedResponse::new(items, 25, 2, 10);
        
        assert_eq!(response.total, 25);
        assert_eq!(response.page, 2);
        assert_eq!(response.page_size, 10);
        assert_eq!(response.total_pages, 3);
    }
    
    #[test]
    fn test_utils() {
        let uuid = utils::generate_uuid();
        assert!(utils::is_valid_uuid(&uuid.to_string()));
        
        assert!(!utils::is_valid_uuid("invalid-uuid"));
    }
}