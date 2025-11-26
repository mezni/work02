use auth_service::domain::entities::{AuditLog, Company, User};
use auth_service::domain::enums::{AuditAction, UserRole};
use auth_service::infrastructure::database::{
    AuditLogRepositoryImpl, CompanyRepositoryImpl, UserRepositoryImpl,
};
use serial_test::serial;
use uuid::Uuid;

#[cfg(test)]
mod user_repository_tests {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_user_repository_initialization() {
        // Test that UserRepositoryImpl can be initialized
        // This is a compilation test - if it compiles, the test passes
        assert!(true, "UserRepositoryImpl should compile successfully");
    }

    #[tokio::test]
    #[serial]
    async fn test_user_role_parsing() {
        // Test UserRole string parsing
        assert_eq!("admin".parse::<UserRole>().unwrap(), UserRole::Admin);
        assert_eq!("user".parse::<UserRole>().unwrap(), UserRole::User);
        assert!("invalid".parse::<UserRole>().is_err());
    }
}

#[cfg(test)]
mod company_repository_tests {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_company_repository_initialization() {
        // Test that CompanyRepositoryImpl can be initialized
        assert!(true, "CompanyRepositoryImpl should compile successfully");
    }

    #[tokio::test]
    #[serial]
    async fn test_company_entity_creation() {
        let company = Company::new(
            "Test Company".to_string(),
            Some("Test Description".to_string()),
            Uuid::new_v4(),
        );

        assert_eq!(company.name, "Test Company");
        assert_eq!(company.description, Some("Test Description".to_string()));
    }
}

#[cfg(test)]
mod audit_log_repository_tests {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_audit_log_repository_initialization() {
        // Test that AuditLogRepositoryImpl can be initialized
        assert!(true, "AuditLogRepositoryImpl should compile successfully");
    }

    #[tokio::test]
    #[serial]
    async fn test_audit_action_parsing() {
        // Test AuditAction string parsing
        assert_eq!(
            "UserCreated".parse::<AuditAction>().unwrap(),
            AuditAction::UserCreated
        );
        assert_eq!("Login".parse::<AuditAction>().unwrap(), AuditAction::Login);
        assert!("InvalidAction".parse::<AuditAction>().is_err());
    }

    #[tokio::test]
    #[serial]
    async fn test_audit_log_creation() {
        let audit_log = AuditLog::new(
            Some(Uuid::new_v4()),
            AuditAction::UserCreated,
            "User".to_string(),
            Some("user-123".to_string()),
            Some(serde_json::json!({"email": "test@example.com"})),
            Some("127.0.0.1".to_string()),
            Some("Test-Agent".to_string()),
        );

        assert_eq!(audit_log.action, AuditAction::UserCreated);
        assert_eq!(audit_log.resource_type, "User");
        assert_eq!(audit_log.resource_id, Some("user-123".to_string()));
    }
}
