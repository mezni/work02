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

# DTOs
cat > src/application/dto.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::domain::enums::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserDto {
    pub id: Uuid,
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
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
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserDto {
    #[validate(length(min = 3, max = 100))]
    pub username: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    pub role: Option<UserRole>,
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CompanyDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
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

# Command handlers
cat > src/application/command_handlers.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

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

# Query handlers
cat > src/application/query_handlers.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

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

# Services
cat > src/application/services.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

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

# Application tests
cat > tests/unit/application_tests.rs << 'EOF'
#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::services::{AuthService, AuthServiceImpl};
    use crate::domain::entities::User;
    use crate::domain::enums::UserRole;
    use async_trait::async_trait;
    use uuid::Uuid;

    // Mock repositories for testing
    struct MockUserRepository;
    
    #[async_trait]
    impl crate::domain::repositories::UserRepository for MockUserRepository {
        async fn create(&self, _user: &User) -> Result<User, crate::domain::errors::DomainError> {
            Ok(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap())
        }
        
        async fn find_by_id(&self, _id: Uuid) -> Result<Option<User>, crate::domain::errors::DomainError> {
            Ok(Some(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap()))
        }
        
        async fn find_by_keycloak_id(&self, _keycloak_id: &str) -> Result<Option<User>, crate::domain::errors::DomainError> {
            Ok(Some(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap()))
        }
        
        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, crate::domain::errors::DomainError> {
            Ok(Some(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap()))
        }
        
        async fn find_by_username(&self, _username: &str) -> Result<Option<User>, crate::domain::errors::DomainError> {
            Ok(Some(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap()))
        }
        
        async fn update(&self, user: &User) -> Result<User, crate::domain::errors::DomainError> {
            Ok(user.clone())
        }
        
        async fn delete(&self, _id: Uuid) -> Result<(), crate::domain::errors::DomainError> {
            Ok(())
        }
        
        async fn list_by_company(&self, _company_id: Uuid) -> Result<Vec<User>, crate::domain::errors::DomainError> {
            Ok(vec![])
        }
        
        async fn list_all(&self) -> Result<Vec<User>, crate::domain::errors::DomainError> {
            Ok(vec![])
        }
    }

    // Mock Keycloak client for testing
    struct MockKeycloakClient;
    
    impl crate::infrastructure::auth::KeycloakClient for MockKeycloakClient {
        async fn create_user(&self, _username: &str, _email: &str, _password: &str) -> Result<String, crate::infrastructure::errors::InfrastructureError> {
            Ok("keycloak-123".to_string())
        }
        
        async fn login(&self, _username: &str, _password: &str) -> Result<crate::infrastructure::auth::KeycloakTokenResponse, crate::infrastructure::errors::InfrastructureError> {
            Ok(crate::infrastructure::auth::KeycloakTokenResponse {
                access_token: "access-token".to_string(),
                refresh_token: "refresh-token".to_string(),
                expires_in: 3600,
                token_type: "Bearer".to_string(),
            })
        }
        
        async fn refresh_token(&self, _refresh_token: &str) -> Result<crate::infrastructure::auth::KeycloakTokenResponse, crate::infrastructure::errors::InfrastructureError> {
            Ok(crate::infrastructure::auth::KeycloakTokenResponse {
                access_token: "new-access-token".to_string(),
                refresh_token: "new-refresh-token".to_string(),
                expires_in: 3600,
                token_type: "Bearer".to_string(),
            })
        }
        
        async fn user_info(&self, _access_token: &str) -> Result<crate::infrastructure::auth::KeycloakUserInfo, crate::infrastructure::errors::InfrastructureError> {
            Ok(crate::infrastructure::auth::KeycloakUserInfo {
                sub: "keycloak-123".to_string(),
                email: "test@example.com".to_string(),
                preferred_username: "testuser".to_string(),
                email_verified: true,
                exp: 1234567890,
                iat: 1234567890,
            })
        }
        
        async fn update_user(&self, _user_id: &str, _attributes: std::collections::HashMap<String, String>) -> Result<(), crate::infrastructure::errors::InfrastructureError> {
            Ok(())
        }
        
        async fn reset_password(&self, _user_id: &str, _new_password: &str) -> Result<(), crate::infrastructure::errors::InfrastructureError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_auth_service_permissions_calculation() {
        let auth_service = AuthServiceImpl::new(
            MockKeycloakClient,
            Box::new(MockUserRepository),
        );
        
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
    }
}
EOF

echo "Application layer generated successfully!"