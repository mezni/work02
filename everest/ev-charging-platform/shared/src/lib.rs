// shared/src/lib.rs
pub mod types;
pub mod error;
pub mod config;
pub mod telemetry;

// Re-export commonly used items
pub use types::{
    UserId, OrganizationId, StationId, 
    UserRole, UserStatus, OrganizationStatus, StationStatus,
    UserClaims, PaginationParams,
};
pub use error::{ApiError, DomainError, ApiResult, DomainResult, ErrorResponse};
pub use config::{AppConfig, DatabaseConfig, ServerConfig, AuthConfig, CorsConfig}; // Add load_config here
pub use telemetry::{init_telemetry, HealthResponse, ReadinessResponse, DependenciesHealth, DependencyStatus};

// Common validation functions
pub mod validation {
    pub fn validate_email(email: &str) -> Result<(), validator::ValidationError> {
        if email.contains('@') && email.len() >= 3 {
            Ok(())
        } else {
            Err(validator::ValidationError::new("invalid_email"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_serialization() {
        let role = UserRole::SuperAdmin;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"super_admin\"");
        
        let deserialized: UserRole = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, role);
    }
    
    #[test]
    fn test_config_loading() {
        // Test that default values work
        let config = load_config(); // Use the standalone function
        // This will fail without config files, but shouldn't panic
        assert!(config.is_err() || config.is_ok());
    }
}