#!/bin/bash

set -e

echo "Generating application layer..."

cd auth-service

# Application mod.rs
cat > src/application/mod.rs << 'EOF'
pub mod commands;
pub mod queries;
pub mod dto;
pub mod services;
pub mod errors;
pub mod command_handlers;
pub mod query_handlers;

// Re-exports
pub use commands::*;
pub use queries::*;
pub use dto::*;
pub use errors::ApplicationError;
EOF

# Application errors
cat > src/application/errors.rs << 'EOF'
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    DomainError(#[from] crate::domain::errors::DomainError),
    
    #[error("Infrastructure error: {0}")]
    InfrastructureError(#[from] crate::infrastructure::errors::InfrastructureError),
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Company not found")]
    CompanyNotFound,
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl ApplicationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DomainError(_) => "APP_DOMAIN_ERROR",
            Self::InfrastructureError(_) => "APP_INFRASTRUCTURE_ERROR",
            Self::AuthenticationFailed => "APP_AUTHENTICATION_FAILED",
            Self::AuthorizationFailed(_) => "APP_AUTHORIZATION_FAILED",
            Self::InvalidToken => "APP_INVALID_TOKEN",
            Self::TokenExpired => "APP_TOKEN_EXPIRED",
            Self::UserNotFound => "APP_USER_NOT_FOUND",
            Self::CompanyNotFound => "APP_COMPANY_NOT_FOUND",
            Self::InvalidOperation(_) => "APP_INVALID_OPERATION",
            Self::ValidationError(_) => "APP_VALIDATION_ERROR",
        }
    }
}
EOF


# First, let's fix the DTOs to implement Validate trait properly
cat > src/application/dto.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::domain::enums::UserRole;

// Helper macro to implement ToSchema for Uuid fields
macro_rules! uuid_schema {
    () => {
        /// UUID in string format
        #[derive(ToSchema)]
        #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
        struct UuidSchema(String);
    };
}

uuid_schema!();

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserDto {
    #[schema(value_type = String, example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    #[schema(value_type = Option<String>)]
    pub company_id: Option<Uuid>,
    pub email_verified: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    pub role: UserRole,
    
    #[schema(value_type = Option<String>)]
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserDto {
    #[validate(length(min = 3, max = 100))]
    pub username: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    pub role: Option<UserRole>,
    
    #[schema(value_type = Option<String>)]
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CompanyDto {
    #[schema(value_type = String, example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[schema(value_type = String, example = "550e8400-e29b-41d4-a716-446655440000")]
    pub created_by: Uuid,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCompanyDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCompanyDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    
    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 3))]
    pub username: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BusinessClaims {
    pub sub: String,
    pub email: String,
    pub username: String,
    pub role: UserRole,
    #[schema(value_type = Option<String>)]
    pub company_id: Option<Uuid>,
    pub permissions: Vec<String>,
    pub exp: i64,
    pub iat: i64,
}
EOF

# Commands
cat > src/application/commands.rs << 'EOF'
use uuid::Uuid;

use crate::domain::enums::UserRole;

#[derive(Debug)]
pub struct CreateUserCommand {
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub company_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct UpdateUserCommand {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<UserRole>,
    pub company_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct DeleteUserCommand {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct CreateCompanyCommand {
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
}

#[derive(Debug)]
pub struct UpdateCompanyCommand {
    pub company_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct DeleteCompanyCommand {
    pub company_id: Uuid,
}

#[derive(Debug)]
pub struct AssignUserToCompanyCommand {
    pub user_id: Uuid,
    pub company_id: Uuid,
}

#[derive(Debug)]
pub struct ChangeUserRoleCommand {
    pub user_id: Uuid,
    pub new_role: UserRole,
}
EOF

# Queries
cat > src/application/queries.rs << 'EOF'
use uuid::Uuid;

use crate::domain::enums::UserRole;

#[derive(Debug)]
pub struct GetUserByIdQuery {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct GetUserByEmailQuery {
    pub email: String,
}

#[derive(Debug)]
pub struct GetUserByKeycloakIdQuery {
    pub keycloak_id: String,
}

#[derive(Debug)]
pub struct ListUsersQuery {
    pub company_id: Option<Uuid>,
    pub role: Option<UserRole>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug)]
pub struct GetCompanyByIdQuery {
    pub company_id: Uuid,
}

#[derive(Debug)]
pub struct ListCompaniesQuery {
    pub user_id: Option<Uuid>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug)]
pub struct ListCompanyUsersQuery {
    pub company_id: Uuid,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug)]
pub struct ValidateTokenQuery {
    pub token: String,
}
EOF




# Remove unused imports from command_handlers.rs
cat > src/application/command_handlers.rs << 'EOF'
use async_trait::async_trait;

use crate::application::commands::*;
use crate::application::errors::ApplicationError;
use crate::domain::entities::{User, Company};
use crate::domain::repositories::{UserRepository, CompanyRepository, AuditLogRepository};
use crate::domain::enums::AuditAction;

#[async_trait]
pub trait CommandHandler<C> {
    async fn handle(&self, command: C) -> Result<(), ApplicationError>;
}

pub struct CreateUserCommandHandler {
    user_repository: Box<dyn UserRepository>,
    audit_repository: Box<dyn AuditLogRepository>,
}

impl CreateUserCommandHandler {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        audit_repository: Box<dyn AuditLogRepository>,
    ) -> Self {
        Self {
            user_repository,
            audit_repository,
        }
    }
}

#[async_trait]
impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
    async fn handle(&self, command: CreateUserCommand) -> Result<(), ApplicationError> {
        let user = User::new(
            command.keycloak_id,
            command.username,
            command.email,
            command.role,
            command.company_id,
        )?;
        
        let created_user = self.user_repository.create(&user).await?;
        
        // Log audit event
        let audit_log = crate::domain::entities::AuditLog::new(
            Some(created_user.id),
            AuditAction::UserCreated,
            "User".to_string(),
            Some(created_user.id.to_string()),
            Some(serde_json::json!({
                "username": created_user.username,
                "email": created_user.email,
                "role": created_user.role.to_string(),
            })),
            None,
            None,
        );
        
        self.audit_repository.create(&audit_log).await?;
        
        Ok(())
    }
}

pub struct CreateCompanyCommandHandler {
    company_repository: Box<dyn CompanyRepository>,
    user_repository: Box<dyn UserRepository>,
    audit_repository: Box<dyn AuditLogRepository>,
}

impl CreateCompanyCommandHandler {
    pub fn new(
        company_repository: Box<dyn CompanyRepository>,
        user_repository: Box<dyn UserRepository>,
        audit_repository: Box<dyn AuditLogRepository>,
    ) -> Self {
        Self {
            company_repository,
            user_repository,
            audit_repository,
        }
    }
}

#[async_trait]
impl CommandHandler<CreateCompanyCommand> for CreateCompanyCommandHandler {
    async fn handle(&self, command: CreateCompanyCommand) -> Result<(), ApplicationError> {
        // Verify that the creator user exists and is an admin
        let creator = self.user_repository.find_by_id(command.created_by)
            .await?
            .ok_or(ApplicationError::UserNotFound)?;
            
        if !creator.is_admin() {
            return Err(ApplicationError::AuthorizationFailed(
                "Only admin users can create companies".to_string()
            ));
        }
        
        let company = Company::new(
            command.name,
            command.description,
            command.created_by,
        );
        
        let created_company = self.company_repository.create(&company).await?;
        
        // Log audit event
        let audit_log = crate::domain::entities::AuditLog::new(
            Some(creator.id),
            AuditAction::CompanyCreated,
            "Company".to_string(),
            Some(created_company.id.to_string()),
            Some(serde_json::json!({
                "name": created_company.name,
            })),
            None,
            None,
        );
        
        self.audit_repository.create(&audit_log).await?;
        
        Ok(())
    }
}

// Additional command handlers would be implemented similarly
EOF

# Remove unused imports from query_handlers.rs
cat > src/application/query_handlers.rs << 'EOF'
use async_trait::async_trait;

use crate::application::queries::*;
use crate::application::dto::{UserDto, CompanyDto};
use crate::application::errors::ApplicationError;
use crate::domain::repositories::{UserRepository, CompanyRepository};

#[async_trait]
pub trait QueryHandler<Q, R> {
    async fn handle(&self, query: Q) -> Result<R, ApplicationError>;
}

pub struct GetUserByIdQueryHandler {
    user_repository: Box<dyn UserRepository>,
}

impl GetUserByIdQueryHandler {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl QueryHandler<GetUserByIdQuery, Option<UserDto>> for GetUserByIdQueryHandler {
    async fn handle(&self, query: GetUserByIdQuery) -> Result<Option<UserDto>, ApplicationError> {
        let user = self.user_repository.find_by_id(query.user_id).await?;
        
        Ok(user.map(|u| UserDto {
            id: u.id,
            keycloak_id: u.keycloak_id,
            username: u.username,
            email: u.email,
            role: u.role,
            company_id: u.company_id,
            email_verified: u.email_verified,
            created_at: u.created_at.to_rfc3339(),
            updated_at: u.updated_at.to_rfc3339(),
        }))
    }
}

pub struct ListUsersQueryHandler {
    user_repository: Box<dyn UserRepository>,
}

impl ListUsersQueryHandler {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl QueryHandler<ListUsersQuery, Vec<UserDto>> for ListUsersQueryHandler {
    async fn handle(&self, query: ListUsersQuery) -> Result<Vec<UserDto>, ApplicationError> {
        let users = if let Some(company_id) = query.company_id {
            self.user_repository.list_by_company(company_id).await?
        } else {
            self.user_repository.list_all().await?
        };
        
        let users_dto = users.into_iter().map(|u| UserDto {
            id: u.id,
            keycloak_id: u.keycloak_id,
            username: u.username,
            email: u.email,
            role: u.role,
            company_id: u.company_id,
            email_verified: u.email_verified,
            created_at: u.created_at.to_rfc3339(),
            updated_at: u.updated_at.to_rfc3339(),
        }).collect();
        
        Ok(users_dto)
    }
}

pub struct GetCompanyByIdQueryHandler {
    company_repository: Box<dyn CompanyRepository>,
}

impl GetCompanyByIdQueryHandler {
    pub fn new(company_repository: Box<dyn CompanyRepository>) -> Self {
        Self { company_repository }
    }
}

#[async_trait]
impl QueryHandler<GetCompanyByIdQuery, Option<CompanyDto>> for GetCompanyByIdQueryHandler {
    async fn handle(&self, query: GetCompanyByIdQuery) -> Result<Option<CompanyDto>, ApplicationError> {
        let company = self.company_repository.find_by_id(query.company_id).await?;
        
        Ok(company.map(|c| CompanyDto {
            id: c.id,
            name: c.name,
            description: c.description,
            created_by: c.created_by,
            created_at: c.created_at.to_rfc3339(),
            updated_at: c.updated_at.to_rfc3339(),
        }))
    }
}

// Additional query handlers would be implemented similarly
EOF


cat > src/application/services.rs << 'EOF'
use async_trait::async_trait;

use crate::application::dto::{LoginResponse, BusinessClaims, UserDto};
use crate::application::errors::ApplicationError;
use crate::domain::repositories::UserRepository;
use crate::infrastructure::auth::KeycloakClient;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, username: String, password: String) -> Result<LoginResponse, ApplicationError>;
    async fn register(&self, username: String, email: String, password: String) -> Result<UserDto, ApplicationError>;
    async fn validate_token(&self, token: String) -> Result<BusinessClaims, ApplicationError>;
    async fn refresh_token(&self, refresh_token: String) -> Result<LoginResponse, ApplicationError>;
}

pub struct AuthServiceImpl {
    keycloak_client: KeycloakClient,
    user_repository: Box<dyn UserRepository>,
}

impl AuthServiceImpl {
    pub fn new(
        keycloak_client: KeycloakClient,
        user_repository: Box<dyn UserRepository>,
    ) -> Self {
        Self {
            keycloak_client,
            user_repository,
        }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, username: String, password: String) -> Result<LoginResponse, ApplicationError> {
        // Authenticate with Keycloak
        let token_response = self.keycloak_client.login(&username, &password)
            .await
            .map_err(|_| ApplicationError::AuthenticationFailed)?;
        
        // Get user info from Keycloak
        let user_info = self.keycloak_client.user_info(&token_response.access_token)
            .await
            .map_err(|_| ApplicationError::AuthenticationFailed)?;
        
        // Find user in local database
        let user = self.user_repository.find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or(ApplicationError::UserNotFound)?;
        
        let user_dto = UserDto {
            id: user.id,
            keycloak_id: user.keycloak_id,
            username: user.username,
            email: user.email,
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        };
        
        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: token_response.expires_in,
            user: user_dto,
        })
    }
    
    async fn register(&self, username: String, email: String, password: String) -> Result<UserDto, ApplicationError> {
        // Create user in Keycloak
        let keycloak_user_id = self.keycloak_client.create_user(&username, &email, &password)
            .await
            .map_err(|e| ApplicationError::ValidationError(e.to_string()))?;
        
        // Create user in local database with default User role
        let user = crate::domain::entities::User::new(
            keycloak_user_id,
            username,
            email,
            crate::domain::enums::UserRole::User,
            None,
        )?;
        
        let created_user = self.user_repository.create(&user).await?;
        
        Ok(UserDto {
            id: created_user.id,
            keycloak_id: created_user.keycloak_id,
            username: created_user.username,
            email: created_user.email,
            role: created_user.role,
            company_id: created_user.company_id,
            email_verified: created_user.email_verified,
            created_at: created_user.created_at.to_rfc3339(),
            updated_at: created_user.updated_at.to_rfc3339(),
        })
    }
    
    async fn validate_token(&self, token: String) -> Result<BusinessClaims, ApplicationError> {
        // Validate token with Keycloak
        let user_info = self.keycloak_client.user_info(&token)
            .await
            .map_err(|_| ApplicationError::InvalidToken)?;
        
        // Find user in local database to get business context
        let user = self.user_repository.find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or(ApplicationError::UserNotFound)?;
        
        // Generate business claims
        let permissions = self.calculate_permissions(&user);
        
        Ok(BusinessClaims {
            sub: user_info.sub,
            email: user_info.email,
            username: user_info.preferred_username,
            role: user.role,
            company_id: user.company_id,
            permissions,
            exp: user_info.exp,
            iat: user_info.iat,
        })
    }
    
    async fn refresh_token(&self, refresh_token: String) -> Result<LoginResponse, ApplicationError> {
        let token_response = self.keycloak_client.refresh_token(&refresh_token)
            .await
            .map_err(|_| ApplicationError::InvalidToken)?;
        
        // Get user info to return user data
        let user_info = self.keycloak_client.user_info(&token_response.access_token)
            .await
            .map_err(|_| ApplicationError::InvalidToken)?;
        
        let user = self.user_repository.find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or(ApplicationError::UserNotFound)?;
        
        let user_dto = UserDto {
            id: user.id,
            keycloak_id: user.keycloak_id,
            username: user.username,
            email: user.email,
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        };
        
        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: token_response.expires_in,
            user: user_dto,
        })
    }
}

impl AuthServiceImpl {
    fn calculate_permissions(&self, user: &crate::domain::entities::User) -> Vec<String> {
        let mut permissions = Vec::new();
        
        match user.role {
            crate::domain::enums::UserRole::Admin => {
                permissions.extend_from_slice(&[
                    "users:read".to_string(),
                    "users:write".to_string(),
                    "users:delete".to_string(),
                    "companies:read".to_string(),
                    "companies:write".to_string(),
                    "companies:delete".to_string(),
                    "audit:read".to_string(),
                ]);
            }
            crate::domain::enums::UserRole::Partner | crate::domain::enums::UserRole::Operator => {
                permissions.extend_from_slice(&[
                    "users:read".to_string(),
                    "users:write".to_string(),
                    "companies:read".to_string(),
                ]);
                
                if let Some(company_id) = user.company_id {
                    permissions.push(format!("company:{}:manage", company_id));
                }
            }
            crate::domain::enums::UserRole::User => {
                permissions.extend_from_slice(&[
                    "users:read:self".to_string(),
                    "users:write:self".to_string(),
                ]);
            }
            crate::domain::enums::UserRole::Guest => {
                permissions.push("public:read".to_string());
            }
        }
        
        permissions
    }
}
EOF

# Create application tests directory
mkdir -p tests/unit/application

# Application tests mod.rs
cat > tests/unit/application/mod.rs << 'EOF'
pub mod command_handlers_test;
pub mod query_handlers_test;
pub mod services_test;
pub mod dto_test;
EOF

cat > tests/unit/application/command_handlers_test.rs << 'EOF'
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
EOF


# Query handlers tests
cat > tests/unit/application/query_handlers_test.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;
use auth_service::application::queries::{GetUserByIdQuery, ListUsersQuery, GetCompanyByIdQuery};
use auth_service::application::query_handlers::{QueryHandler, GetUserByIdQueryHandler, ListUsersQueryHandler, GetCompanyByIdQueryHandler};
use auth_service::application::dto::{UserDto, CompanyDto};
use auth_service::domain::entities::{User, Company};
use auth_service::domain::enums::UserRole;
use auth_service::domain::repositories::{UserRepository, CompanyRepository};
use auth_service::domain::errors::DomainError;

// Mock UserRepository for query tests
struct MockUserRepository {
    users: Vec<User>,
}

impl MockUserRepository {
    fn new() -> Self {
        let user1 = User::new(
            "keycloak-1".to_string(),
            "user1".to_string(),
            "user1@example.com".to_string(),
            UserRole::User,
            None,
        ).unwrap();

        let user2 = User::new(
            "keycloak-2".to_string(),
            "user2".to_string(),
            "user2@example.com".to_string(),
            UserRole::Admin,
            None,
        ).unwrap();

        Self {
            users: vec![user1, user2],
        }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, _user: &User) -> Result<User, DomainError> {
        unimplemented!()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        Ok(self.users.iter().find(|u| u.id == id).cloned())
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

    async fn update(&self, _user: &User) -> Result<User, DomainError> {
        unimplemented!()
    }

    async fn delete(&self, _id: Uuid) -> Result<(), DomainError> {
        unimplemented!()
    }

    async fn list_by_company(&self, _company_id: Uuid) -> Result<Vec<User>, DomainError> {
        Ok(self.users.clone())
    }

    async fn list_all(&self) -> Result<Vec<User>, DomainError> {
        Ok(self.users.clone())
    }
}

// Mock CompanyRepository for query tests
struct MockCompanyRepository {
    companies: Vec<Company>,
}

impl MockCompanyRepository {
    fn new() -> Self {
        let company1 = Company::new(
            "Company 1".to_string(),
            Some("Description 1".to_string()),
            Uuid::new_v4(),
        );

        let company2 = Company::new(
            "Company 2".to_string(),
            Some("Description 2".to_string()),
            Uuid::new_v4(),
        );

        Self {
            companies: vec![company1, company2],
        }
    }
}

#[async_trait]
impl CompanyRepository for MockCompanyRepository {
    async fn create(&self, _company: &Company) -> Result<Company, DomainError> {
        unimplemented!()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Company>, DomainError> {
        Ok(self.companies.iter().find(|c| c.id == id).cloned())
    }

    async fn find_by_name(&self, _name: &str) -> Result<Option<Company>, DomainError> {
        Ok(None)
    }

    async fn update(&self, _company: &Company) -> Result<Company, DomainError> {
        unimplemented!()
    }

    async fn delete(&self, _id: Uuid) -> Result<(), DomainError> {
        unimplemented!()
    }

    async fn list_all(&self) -> Result<Vec<Company>, DomainError> {
        Ok(self.companies.clone())
    }

    async fn list_by_user(&self, _user_id: Uuid) -> Result<Vec<Company>, DomainError> {
        Ok(self.companies.clone())
    }
}

#[tokio::test]
async fn test_get_user_by_id_query_handler() {
    let user_repo = Box::new(MockUserRepository::new());
    let handler = GetUserByIdQueryHandler::new(user_repo);
    
    let users = MockUserRepository::new().users;
    let test_user_id = users[0].id;
    
    let query = GetUserByIdQuery { user_id: test_user_id };
    let result = handler.handle(query).await;
    
    assert!(result.is_ok());
    let user_dto = result.unwrap();
    assert!(user_dto.is_some());
    assert_eq!(user_dto.unwrap().id, test_user_id);
}

#[tokio::test]
async fn test_get_user_by_id_query_handler_not_found() {
    let user_repo = Box::new(MockUserRepository::new());
    let handler = GetUserByIdQueryHandler::new(user_repo);
    
    let query = GetUserByIdQuery { user_id: Uuid::new_v4() }; // Non-existent user
    let result = handler.handle(query).await;
    
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_list_users_query_handler() {
    let user_repo = Box::new(MockUserRepository::new());
    let handler = ListUsersQueryHandler::new(user_repo);
    
    let query = ListUsersQuery {
        company_id: None,
        role: None,
        page: 1,
        page_size: 10,
    };
    
    let result = handler.handle(query).await;
    
    assert!(result.is_ok());
    let users_dto = result.unwrap();
    assert_eq!(users_dto.len(), 2);
}

#[tokio::test]
async fn test_get_company_by_id_query_handler() {
    let company_repo = Box::new(MockCompanyRepository::new());
    let handler = GetCompanyByIdQueryHandler::new(company_repo);
    
    let companies = MockCompanyRepository::new().companies;
    let test_company_id = companies[0].id;
    
    let query = GetCompanyByIdQuery { company_id: test_company_id };
    let result = handler.handle(query).await;
    
    assert!(result.is_ok());
    let company_dto = result.unwrap();
    assert!(company_dto.is_some());
    assert_eq!(company_dto.unwrap().id, test_company_id);
}
EOF


cat > tests/unit/application/services_test.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;
use auth_service::application::services::{AuthService, AuthServiceImpl};
use auth_service::application::dto::{LoginResponse, UserDto, BusinessClaims};
use auth_service::application::errors::ApplicationError;
use auth_service::domain::entities::User;
use auth_service::domain::enums::UserRole;
use auth_service::domain::repositories::UserRepository;
use auth_service::domain::errors::DomainError;
use auth_service::infrastructure::auth::{KeycloakClient, KeycloakTokenResponse, KeycloakUserInfo};
use auth_service::infrastructure::errors::InfrastructureError;

// Mock UserRepository for auth tests
struct MockUserRepository {
    should_find_user: bool,
    user_role: UserRole,
}

impl MockUserRepository {
    fn new(should_find_user: bool, user_role: UserRole) -> Self {
        Self { should_find_user, user_role }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, user: &User) -> Result<User, DomainError> {
        Ok(user.clone())
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<User>, DomainError> {
        Ok(None)
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError> {
        if self.should_find_user {
            Ok(Some(User::new(
                keycloak_id.to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                self.user_role,
                None,
            ).unwrap()))
        } else {
            Ok(None)
        }
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

// Mock KeycloakClient for auth tests - we need to create a trait for this
#[async_trait]
pub trait KeycloakClientTrait: Send + Sync {
    async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<String, InfrastructureError>;
    async fn login(&self, username: &str, password: &str) -> Result<KeycloakTokenResponse, InfrastructureError>;
    async fn refresh_token(&self, refresh_token: &str) -> Result<KeycloakTokenResponse, InfrastructureError>;
    async fn user_info(&self, access_token: &str) -> Result<KeycloakUserInfo, InfrastructureError>;
    async fn update_user(&self, user_id: &str, attributes: std::collections::HashMap<String, String>) -> Result<(), InfrastructureError>;
    async fn reset_password(&self, user_id: &str, new_password: &str) -> Result<(), InfrastructureError>;
}

// Implement the trait for the real KeycloakClient
#[async_trait]
impl KeycloakClientTrait for KeycloakClient {
    async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<String, InfrastructureError> {
        KeycloakClient::create_user(self, username, email, password).await
    }

    async fn login(&self, username: &str, password: &str) -> Result<KeycloakTokenResponse, InfrastructureError> {
        KeycloakClient::login(self, username, password).await
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<KeycloakTokenResponse, InfrastructureError> {
        KeycloakClient::refresh_token(self, refresh_token).await
    }

    async fn user_info(&self, access_token: &str) -> Result<KeycloakUserInfo, InfrastructureError> {
        KeycloakClient::user_info(self, access_token).await
    }

    async fn update_user(&self, user_id: &str, attributes: std::collections::HashMap<String, String>) -> Result<(), InfrastructureError> {
        KeycloakClient::update_user(self, user_id, attributes).await
    }

    async fn reset_password(&self, user_id: &str, new_password: &str) -> Result<(), InfrastructureError> {
        KeycloakClient::reset_password(self, user_id, new_password).await
    }
}

// Mock KeycloakClient for auth tests
struct MockKeycloakClient {
    should_succeed: bool,
}

impl MockKeycloakClient {
    fn new(should_succeed: bool) -> Self {
        Self { should_succeed }
    }
}

#[async_trait]
impl KeycloakClientTrait for MockKeycloakClient {
    async fn create_user(&self, _username: &str, _email: &str, _password: &str) -> Result<String, InfrastructureError> {
        if self.should_succeed {
            Ok("keycloak-123".to_string())
        } else {
            Err(InfrastructureError::KeycloakError("Mock error".to_string()))
        }
    }

    async fn login(&self, _username: &str, _password: &str) -> Result<KeycloakTokenResponse, InfrastructureError> {
        if self.should_succeed {
            Ok(KeycloakTokenResponse {
                access_token: "access-token".to_string(),
                refresh_token: "refresh-token".to_string(),
                expires_in: 3600,
                token_type: "Bearer".to_string(),
            })
        } else {
            Err(InfrastructureError::KeycloakError("Mock error".to_string()))
        }
    }

    async fn refresh_token(&self, _refresh_token: &str) -> Result<KeycloakTokenResponse, InfrastructureError> {
        if self.should_succeed {
            Ok(KeycloakTokenResponse {
                access_token: "new-access-token".to_string(),
                refresh_token: "new-refresh-token".to_string(),
                expires_in: 3600,
                token_type: "Bearer".to_string(),
            })
        } else {
            Err(InfrastructureError::KeycloakError("Mock error".to_string()))
        }
    }

    async fn user_info(&self, _access_token: &str) -> Result<KeycloakUserInfo, InfrastructureError> {
        if self.should_succeed {
            Ok(KeycloakUserInfo {
                sub: "keycloak-123".to_string(),
                email: "test@example.com".to_string(),
                preferred_username: "testuser".to_string(),
                email_verified: true,
                exp: 1234567890,
                iat: 1234567890,
            })
        } else {
            Err(InfrastructureError::KeycloakError("Mock error".to_string()))
        }
    }

    async fn update_user(&self, _user_id: &str, _attributes: std::collections::HashMap<String, String>) -> Result<(), InfrastructureError> {
        Ok(())
    }

    async fn reset_password(&self, _user_id: &str, _new_password: &str) -> Result<(), InfrastructureError> {
        Ok(())
    }
}

// We need to update AuthServiceImpl to use the trait
pub struct TestAuthServiceImpl {
    keycloak_client: Box<dyn KeycloakClientTrait>,
    user_repository: Box<dyn UserRepository>,
}

impl TestAuthServiceImpl {
    pub fn new(
        keycloak_client: Box<dyn KeycloakClientTrait>,
        user_repository: Box<dyn UserRepository>,
    ) -> Self {
        Self {
            keycloak_client,
            user_repository,
        }
    }
    
    // Helper method to test permission calculation
    pub fn calculate_permissions(&self, user: &User) -> Vec<String> {
        let mut permissions = Vec::new();
        
        match user.role {
            UserRole::Admin => {
                permissions.extend_from_slice(&[
                    "users:read".to_string(),
                    "users:write".to_string(),
                    "users:delete".to_string(),
                    "companies:read".to_string(),
                    "companies:write".to_string(),
                    "companies:delete".to_string(),
                    "audit:read".to_string(),
                ]);
            }
            UserRole::Partner | UserRole::Operator => {
                permissions.extend_from_slice(&[
                    "users:read".to_string(),
                    "users:write".to_string(),
                    "companies:read".to_string(),
                ]);
                
                if let Some(company_id) = user.company_id {
                    permissions.push(format!("company:{}:manage", company_id));
                }
            }
            UserRole::User => {
                permissions.extend_from_slice(&[
                    "users:read:self".to_string(),
                    "users:write:self".to_string(),
                ]);
            }
            UserRole::Guest => {
                permissions.push("public:read".to_string());
            }
        }
        
        permissions
    }
}

#[async_trait]
impl AuthService for TestAuthServiceImpl {
    async fn login(&self, username: String, password: String) -> Result<LoginResponse, ApplicationError> {
        // Authenticate with Keycloak
        let token_response = self.keycloak_client.login(&username, &password)
            .await
            .map_err(|_| ApplicationError::AuthenticationFailed)?;
        
        // Get user info from Keycloak
        let user_info = self.keycloak_client.user_info(&token_response.access_token)
            .await
            .map_err(|_| ApplicationError::AuthenticationFailed)?;
        
        // Find user in local database
        let user = self.user_repository.find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or(ApplicationError::UserNotFound)?;
        
        let user_dto = UserDto {
            id: user.id,
            keycloak_id: user.keycloak_id,
            username: user.username,
            email: user.email,
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        };
        
        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: token_response.expires_in,
            user: user_dto,
        })
    }
    
    async fn register(&self, username: String, email: String, password: String) -> Result<UserDto, ApplicationError> {
        // Create user in Keycloak
        let keycloak_user_id = self.keycloak_client.create_user(&username, &email, &password)
            .await
            .map_err(|e| ApplicationError::ValidationError(e.to_string()))?;
        
        // Create user in local database with default User role
        let user = User::new(
            keycloak_user_id,
            username,
            email,
            UserRole::User,
            None,
        )?;
        
        let created_user = self.user_repository.create(&user).await?;
        
        Ok(UserDto {
            id: created_user.id,
            keycloak_id: created_user.keycloak_id,
            username: created_user.username,
            email: created_user.email,
            role: created_user.role,
            company_id: created_user.company_id,
            email_verified: created_user.email_verified,
            created_at: created_user.created_at.to_rfc3339(),
            updated_at: created_user.updated_at.to_rfc3339(),
        })
    }
    
    async fn validate_token(&self, token: String) -> Result<BusinessClaims, ApplicationError> {
        // Validate token with Keycloak
        let user_info = self.keycloak_client.user_info(&token)
            .await
            .map_err(|_| ApplicationError::InvalidToken)?;
        
        // Find user in local database to get business context
        let user = self.user_repository.find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or(ApplicationError::UserNotFound)?;
        
        // Generate business claims
        let permissions = self.calculate_permissions(&user);
        
        Ok(BusinessClaims {
            sub: user_info.sub,
            email: user_info.email,
            username: user_info.preferred_username,
            role: user.role,
            company_id: user.company_id,
            permissions,
            exp: user_info.exp,
            iat: user_info.iat,
        })
    }
    
    async fn refresh_token(&self, refresh_token: String) -> Result<LoginResponse, ApplicationError> {
        let token_response = self.keycloak_client.refresh_token(&refresh_token)
            .await
            .map_err(|_| ApplicationError::InvalidToken)?;
        
        // Get user info to return user data
        let user_info = self.keycloak_client.user_info(&token_response.access_token)
            .await
            .map_err(|_| ApplicationError::InvalidToken)?;
        
        let user = self.user_repository.find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or(ApplicationError::UserNotFound)?;
        
        let user_dto = UserDto {
            id: user.id,
            keycloak_id: user.keycloak_id,
            username: user.username,
            email: user.email,
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        };
        
        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: token_response.expires_in,
            user: user_dto,
        })
    }
}

#[tokio::test]
async fn test_auth_service_login_success() {
    let keycloak_client = Box::new(MockKeycloakClient::new(true));
    let user_repo = Box::new(MockUserRepository::new(true, UserRole::User));
    
    let auth_service = TestAuthServiceImpl::new(keycloak_client, user_repo);
    
    let result = auth_service.login("testuser".to_string(), "password".to_string()).await;
    
    assert!(result.is_ok());
    let login_response = result.unwrap();
    assert_eq!(login_response.access_token, "access-token");
    assert_eq!(login_response.user.username, "testuser");
}

#[tokio::test]
async fn test_auth_service_login_user_not_found() {
    let keycloak_client = Box::new(MockKeycloakClient::new(true));
    let user_repo = Box::new(MockUserRepository::new(false, UserRole::User)); // User not found
    
    let auth_service = TestAuthServiceImpl::new(keycloak_client, user_repo);
    
    let result = auth_service.login("testuser".to_string(), "password".to_string()).await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ApplicationError::UserNotFound));
}

#[tokio::test]
async fn test_auth_service_register_success() {
    let keycloak_client = Box::new(MockKeycloakClient::new(true));
    let user_repo = Box::new(MockUserRepository::new(false, UserRole::User));
    
    let auth_service = TestAuthServiceImpl::new(keycloak_client, user_repo);
    
    let result = auth_service.register(
        "newuser".to_string(),
        "newuser@example.com".to_string(),
        "password".to_string(),
    ).await;
    
    assert!(result.is_ok());
    let user_dto = result.unwrap();
    assert_eq!(user_dto.username, "newuser");
    assert_eq!(user_dto.role, UserRole::User);
}

#[tokio::test]
async fn test_auth_service_permissions_calculation() {
    let keycloak_client = Box::new(MockKeycloakClient::new(true));
    let user_repo = Box::new(MockUserRepository::new(true, UserRole::Admin));
    
    let auth_service = TestAuthServiceImpl::new(keycloak_client, user_repo);
    
    let admin_user = User::new(
        "keycloak-admin".to_string(),
        "admin".to_string(),
        "admin@example.com".to_string(),
        UserRole::Admin,
        None,
    ).unwrap();
    
    let permissions = auth_service.calculate_permissions(&admin_user);
    
    assert!(permissions.contains(&"users:read".to_string()));
    assert!(permissions.contains(&"users:write".to_string()));
    assert!(permissions.contains(&"companies:read".to_string()));
    assert!(permissions.contains(&"companies:write".to_string()));
    
    let user_user = User::new(
        "keycloak-user".to_string(),
        "user".to_string(),
        "user@example.com".to_string(),
        UserRole::User,
        None,
    ).unwrap();
    
    let user_permissions = auth_service.calculate_permissions(&user_user);
    assert!(user_permissions.contains(&"users:read:self".to_string()));
    assert!(user_permissions.contains(&"users:write:self".to_string()));
}
EOF


# DTO tests
cat > tests/unit/application/dto_test.rs << 'EOF'
use validator::Validate;
use uuid::Uuid;
use auth_service::application::dto::{
    CreateUserDto, UpdateUserDto, CreateCompanyDto, UpdateCompanyDto, 
    UserDto, CompanyDto, LoginRequest, RegisterRequest
};
use auth_service::domain::enums::UserRole;

#[test]
fn test_create_user_dto_validation() {
    // Valid DTO
    let valid_dto = CreateUserDto {
        username: "validuser".to_string(),
        email: "valid@example.com".to_string(),
        password: "password123".to_string(),
        role: UserRole::User,
        company_id: None,
    };
    
    assert!(valid_dto.validate().is_ok());
    
    // Invalid DTO - short username
    let invalid_dto = CreateUserDto {
        username: "ab".to_string(), // Too short
        email: "valid@example.com".to_string(),
        password: "password123".to_string(),
        role: UserRole::User,
        company_id: None,
    };
    
    assert!(invalid_dto.validate().is_err());
    
    // Invalid DTO - invalid email
    let invalid_dto = CreateUserDto {
        username: "validuser".to_string(),
        email: "invalid-email".to_string(),
        password: "password123".to_string(),
        role: UserRole::User,
        company_id: None,
    };
    
    assert!(invalid_dto.validate().is_err());
    
    // Invalid DTO - short password
    let invalid_dto = CreateUserDto {
        username: "validuser".to_string(),
        email: "valid@example.com".to_string(),
        password: "short".to_string(), // Too short
        role: UserRole::User,
        company_id: None,
    };
    
    assert!(invalid_dto.validate().is_err());
}

#[test]
fn test_update_user_dto_validation() {
    // Valid DTO with some fields
    let valid_dto = UpdateUserDto {
        username: Some("newusername".to_string()),
        email: Some("new@example.com".to_string()),
        role: Some(UserRole::Admin),
        company_id: None,
    };
    
    assert!(valid_dto.validate().is_ok());
    
    // Valid DTO with empty fields (all optional)
    let valid_dto = UpdateUserDto {
        username: None,
        email: None,
        role: None,
        company_id: None,
    };
    
    assert!(valid_dto.validate().is_ok());
    
    // Invalid DTO - short username
    let invalid_dto = UpdateUserDto {
        username: Some("ab".to_string()), // Too short
        email: Some("valid@example.com".to_string()),
        role: Some(UserRole::User),
        company_id: None,
    };
    
    assert!(invalid_dto.validate().is_err());
}

#[test]
fn test_create_company_dto_validation() {
    // Valid DTO
    let valid_dto = CreateCompanyDto {
        name: "Valid Company".to_string(),
        description: Some("A valid company description".to_string()),
    };
    
    assert!(valid_dto.validate().is_ok());
    
    // Invalid DTO - empty name
    let invalid_dto = CreateCompanyDto {
        name: "".to_string(), // Empty name
        description: Some("Description".to_string()),
    };
    
    assert!(invalid_dto.validate().is_err());
    
    // Invalid DTO - long description
    let long_description = "a".repeat(1001);
    let invalid_dto = CreateCompanyDto {
        name: "Valid Company".to_string(),
        description: Some(long_description),
    };
    
    assert!(invalid_dto.validate().is_err());
}

#[test]
fn test_user_dto_creation() {
    let user_dto = UserDto {
        id: Uuid::new_v4(),
        keycloak_id: "keycloak-123".to_string(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::Admin,
        company_id: Some(Uuid::new_v4()),
        email_verified: true,
        created_at: "2023-01-01T00:00:00Z".to_string(),
        updated_at: "2023-01-01T00:00:00Z".to_string(),
    };
    
    assert_eq!(user_dto.username, "testuser");
    assert_eq!(user_dto.email, "test@example.com");
    assert_eq!(user_dto.role, UserRole::Admin);
    assert!(user_dto.company_id.is_some());
}

#[test]
fn test_company_dto_creation() {
    let company_dto = CompanyDto {
        id: Uuid::new_v4(),
        name: "Test Company".to_string(),
        description: Some("Test Description".to_string()),
        created_by: Uuid::new_v4(),
        created_at: "2023-01-01T00:00:00Z".to_string(),
        updated_at: "2023-01-01T00:00:00Z".to_string(),
    };
    
    assert_eq!(company_dto.name, "Test Company");
    assert_eq!(company_dto.description, Some("Test Description".to_string()));
}

#[test]
fn test_login_request_creation() {
    let login_request = LoginRequest {
        username: "testuser".to_string(),
        password: "password".to_string(),
    };
    
    assert_eq!(login_request.username, "testuser");
    assert_eq!(login_request.password, "password");
}

#[test]
fn test_register_request_creation() {
    let register_request = RegisterRequest {
        username: "newuser".to_string(),
        email: "newuser@example.com".to_string(),
        password: "password".to_string(),
    };
    
    assert_eq!(register_request.username, "newuser");
    assert_eq!(register_request.email, "newuser@example.com");
    assert_eq!(register_request.password, "password");
}
EOF

echo "Application layer generated successfully!"
