use async_trait::async_trait;
use uuid::Uuid;
use auth_service::application::commands::{CreateUserCommand, CreateCompanyCommand};
use auth_service::application::command_handlers::{CommandHandler, CreateUserCommandHandler, CreateCompanyCommandHandler};
use auth_service::application::errors::ApplicationError;
use auth_service::domain::entities::{User, Company, AuditLog};
use auth_service::domain::enums::UserRole;
use auth_service::domain::repositories::{UserRepository, CompanyRepository, AuditLogRepository};
use auth_service::domain::errors::DomainError;

// Mock UserRepository
struct MockUserRepository {
    should_fail: bool,
    admin_user: bool,
}

impl MockUserRepository {
    fn new(should_fail: bool, admin_user: bool) -> Self {
        Self { should_fail, admin_user }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, user: &User) -> Result<User, DomainError> {
        if self.should_fail {
            Err(DomainError::ValidationError("Mock error".to_string()))
        } else {
            Ok(user.clone())
        }
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<User>, DomainError> {
        if self.should_fail {
            Err(DomainError::ValidationError("Mock error".to_string()))
        } else {
            let role = if self.admin_user { UserRole::Admin } else { UserRole::User };
            Ok(Some(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                role,
                None,
            ).unwrap()))
        }
    }

    async fn find_by_keycloak_id(&self, _keycloak_id: &str) -> Result<Option<User>, DomainError> {
        Ok(None)
    }

    async fn find_by_email(&self, _email: &str) -> Result<Option<User>, DomainError> {
        Ok(None)
    }

    async fn find_by_username(&self, _username: &str) -> Result<Option<User>, DomainError> {
        Ok(None)
    }

    async fn update(&self, user: &User) -> Result<User, DomainError> {
        Ok(user.clone())
    }

    async fn delete(&self, _id: Uuid) -> Result<(), DomainError> {
        Ok(())
    }

    async fn list_by_company(&self, _company_id: Uuid) -> Result<Vec<User>, DomainError> {
        Ok(vec![])
    }

    async fn list_all(&self) -> Result<Vec<User>, DomainError> {
        Ok(vec![])
    }
}

// Mock CompanyRepository
struct MockCompanyRepository {
    should_fail: bool,
}

impl MockCompanyRepository {
    fn new(should_fail: bool) -> Self {
        Self { should_fail }
    }
}

#[async_trait]
impl CompanyRepository for MockCompanyRepository {
    async fn create(&self, company: &Company) -> Result<Company, DomainError> {
        if self.should_fail {
            Err(DomainError::ValidationError("Mock error".to_string()))
        } else {
            Ok(company.clone())
        }
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<Company>, DomainError> {
        Ok(None)
    }

    async fn find_by_name(&self, _name: &str) -> Result<Option<Company>, DomainError> {
        Ok(None)
    }

    async fn update(&self, company: &Company) -> Result<Company, DomainError> {
        Ok(company.clone())
    }

    async fn delete(&self, _id: Uuid) -> Result<(), DomainError> {
        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<Company>, DomainError> {
        Ok(vec![])
    }

    async fn list_by_user(&self, _user_id: Uuid) -> Result<Vec<Company>, DomainError> {
        Ok(vec![])
    }
}

// Mock AuditLogRepository
struct MockAuditLogRepository {
    should_fail: bool,
}

impl MockAuditLogRepository {
    fn new(should_fail: bool) -> Self {
        Self { should_fail }
    }
}

#[async_trait]
impl AuditLogRepository for MockAuditLogRepository {
    async fn create(&self, _audit_log: &AuditLog) -> Result<(), DomainError> {
        if self.should_fail {
            Err(DomainError::ValidationError("Mock error".to_string()))
        } else {
            Ok(())
        }
    }

    async fn find_by_user(&self, _user_id: Uuid) -> Result<Vec<AuditLog>, DomainError> {
        Ok(vec![])
    }

    async fn find_by_company(&self, _company_id: Uuid) -> Result<Vec<AuditLog>, DomainError> {
        Ok(vec![])
    }

    async fn list_recent(&self, _limit: u32) -> Result<Vec<AuditLog>, DomainError> {
        Ok(vec![])
    }
}

#[tokio::test]
async fn test_create_user_command_handler_success() {
    let user_repo = Box::new(MockUserRepository::new(false, false));
    let audit_repo = Box::new(MockAuditLogRepository::new(false));
    
    let handler = CreateUserCommandHandler::new(user_repo, audit_repo);
    
    let command = CreateUserCommand {
        keycloak_id: "keycloak-123".to_string(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::User,
        company_id: None,
    };
    
    let result = handler.handle(command).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_user_command_handler_failure() {
    let user_repo = Box::new(MockUserRepository::new(true, false));
    let audit_repo = Box::new(MockAuditLogRepository::new(false));
    
    let handler = CreateUserCommandHandler::new(user_repo, audit_repo);
    
    let command = CreateUserCommand {
        keycloak_id: "keycloak-123".to_string(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::User,
        company_id: None,
    };
    
    let result = handler.handle(command).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_company_command_handler_success() {
    let company_repo = Box::new(MockCompanyRepository::new(false));
    let user_repo = Box::new(MockUserRepository::new(false, true)); // Admin user
    let audit_repo = Box::new(MockAuditLogRepository::new(false));
    
    let handler = CreateCompanyCommandHandler::new(company_repo, user_repo, audit_repo);
    
    let command = CreateCompanyCommand {
        name: "Test Company".to_string(),
        description: Some("Test Description".to_string()),
        created_by: Uuid::new_v4(),
    };
    
    let result = handler.handle(command).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_company_command_handler_unauthorized() {
    let company_repo = Box::new(MockCompanyRepository::new(false));
    let user_repo = Box::new(MockUserRepository::new(false, false)); // Non-admin user
    let audit_repo = Box::new(MockAuditLogRepository::new(false));
    
    let handler = CreateCompanyCommandHandler::new(company_repo, user_repo, audit_repo);
    
    let command = CreateCompanyCommand {
        name: "Test Company".to_string(),
        description: Some("Test Description".to_string()),
        created_by: Uuid::new_v4(),
    };
    
    let result = handler.handle(command).await;
    assert!(result.is_err());
    
    if let Err(ApplicationError::AuthorizationFailed(msg)) = result {
        assert!(msg.contains("Only admin users can create companies"));
    } else {
        panic!("Expected AuthorizationFailed error");
    }
}
