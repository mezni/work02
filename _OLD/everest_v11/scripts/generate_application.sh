#!/bin/bash

set -e

echo "Generating complete application layer with fixes..."

cd auth-service

# Create application layer directories
mkdir -p src/application/{services,commands,queries,dtos}

# First, let's fix the shared error.rs to include DomainError conversion
cat > src/shared/error.rs << 'EOF'
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

use crate::domain::errors::DomainError;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Internal server error")]
    Internal,
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Domain error: {0}")]
    DomainError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Internal => HttpResponse::InternalServerError().json("Internal server error"),
            AppError::ConfigError(msg) => HttpResponse::InternalServerError().json(format!("Configuration error: {}", msg)),
            AppError::DatabaseError(msg) => HttpResponse::InternalServerError().json(format!("Database error: {}", msg)),
            AppError::AuthError(msg) => HttpResponse::Unauthorized().json(format!("Authentication error: {}", msg)),
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(format!("Validation error: {}", msg)),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(format!("Not found: {}", msg)),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(format!("Unauthorized: {}", msg)),
            AppError::DomainError(msg) => HttpResponse::BadRequest().json(format!("Domain error: {}", msg)),
        }
    }
}

impl From<config::ConfigError> for AppError {
    fn from(e: config::ConfigError) -> Self {
        AppError::ConfigError(e.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::DatabaseError(e.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::AuthError(e.to_string())
    }
}

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        AppError::DomainError(e.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        AppError::AuthError(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Internal
    }
}

impl From<uuid::Error> for AppError {
    fn from(e: uuid::Error) -> Self {
        AppError::ValidationError("Invalid UUID format".to_string())
    }
}
EOF

# Application Services - User Service (Fixed error handling)
cat > src/application/services/user_service.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::enums::UserRole;
use crate::domain::repositories::{UserRepository, CompanyRepository, AuditLogRepository};
use crate::application::dtos::{UserDto, CreateUserDto, UpdateUserDto};
use crate::shared::error::AppError;
use crate::infrastructure::auth::KeycloakClient;

#[async_trait]
pub trait UserApplicationService: Send + Sync {
    async fn create_user(&self, create_user_dto: CreateUserDto) -> Result<UserDto, AppError>;
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<UserDto, AppError>;
    async fn get_user_by_email(&self, email: &str) -> Result<UserDto, AppError>;
    async fn update_user(&self, user_id: Uuid, update_user_dto: UpdateUserDto, actor_id: Uuid) -> Result<UserDto, AppError>;
    async fn delete_user(&self, user_id: Uuid, actor_id: Uuid) -> Result<(), AppError>;
    async fn list_users_by_company(&self, company_id: Uuid) -> Result<Vec<UserDto>, AppError>;
    async fn assign_user_to_company(&self, user_id: Uuid, company_id: Uuid, actor_id: Uuid) -> Result<UserDto, AppError>;
    async fn change_user_role(&self, user_id: Uuid, new_role: UserRole, actor_id: Uuid) -> Result<UserDto, AppError>;
}

pub struct UserApplicationServiceImpl {
    user_repository: Box<dyn UserRepository>,
    company_repository: Box<dyn CompanyRepository>,
    audit_log_repository: Box<dyn AuditLogRepository>,
    keycloak_client: KeycloakClient,
}

impl UserApplicationServiceImpl {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        company_repository: Box<dyn CompanyRepository>,
        audit_log_repository: Box<dyn AuditLogRepository>,
        keycloak_client: KeycloakClient,
    ) -> Self {
        Self {
            user_repository,
            company_repository,
            audit_log_repository,
            keycloak_client,
        }
    }
}

#[async_trait]
impl UserApplicationService for UserApplicationServiceImpl {
    async fn create_user(&self, create_user_dto: CreateUserDto) -> Result<UserDto, AppError> {
        // Check if user already exists
        if self.user_repository.find_by_email(&create_user_dto.email).await.map_err(AppError::from)?.is_some() {
            return Err(AppError::ValidationError("User with this email already exists".to_string()));
        }

        if self.user_repository.find_by_username(&create_user_dto.username).await.map_err(AppError::from)?.is_some() {
            return Err(AppError::ValidationError("User with this username already exists".to_string()));
        }

        // Create user in Keycloak
        let keycloak_user_id = self.keycloak_client.create_user(
            &create_user_dto.username,
            &create_user_dto.email,
            &create_user_dto.password,
        ).await
        .map_err(|e| AppError::AuthError(e.to_string()))?;

        // Create user in database
        let user = User::new(
            keycloak_user_id,
            create_user_dto.username,
            create_user_dto.email,
            UserRole::User,
            create_user_dto.company_id,
        ).map_err(AppError::from)?;

        let saved_user = self.user_repository.create(&user).await.map_err(AppError::from)?;
        
        Ok(UserDto::from_entity(&saved_user))
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<UserDto, AppError> {
        let user = self.user_repository.find_by_id(user_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))?;
        
        Ok(UserDto::from_entity(&user))
    }

    async fn get_user_by_email(&self, email: &str) -> Result<UserDto, AppError> {
        let user = self.user_repository.find_by_email(email).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("User with email {} not found", email)))?;
        
        Ok(UserDto::from_entity(&user))
    }

    async fn update_user(&self, user_id: Uuid, update_user_dto: UpdateUserDto, actor_id: Uuid) -> Result<UserDto, AppError> {
        let mut user = self.user_repository.find_by_id(user_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))?;

        // Check permissions
        let actor = self.user_repository.find_by_id(actor_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Actor user not found".to_string()))?;

        if actor.id != user.id && !actor.is_admin() {
            return Err(AppError::Unauthorized("Insufficient permissions to update this user".to_string()));
        }

        // Update fields
        if let Some(email) = update_user_dto.email {
            user.email = email;
        }
        if let Some(username) = update_user_dto.username {
            user.username = username;
        }

        let updated_user = self.user_repository.update(&user).await.map_err(AppError::from)?;
        
        Ok(UserDto::from_entity(&updated_user))
    }

    async fn delete_user(&self, user_id: Uuid, actor_id: Uuid) -> Result<(), AppError> {
        let user = self.user_repository.find_by_id(user_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))?;

        let actor = self.user_repository.find_by_id(actor_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Actor user not found".to_string()))?;

        if actor.id != user.id && !actor.is_admin() {
            return Err(AppError::Unauthorized("Insufficient permissions to delete this user".to_string()));
        }

        self.user_repository.delete(user_id).await.map_err(AppError::from)?;
        
        Ok(())
    }

    async fn list_users_by_company(&self, company_id: Uuid) -> Result<Vec<UserDto>, AppError> {
        let users = self.user_repository.list_by_company(company_id).await.map_err(AppError::from)?;
        
        Ok(users.iter().map(UserDto::from_entity).collect())
    }

    async fn assign_user_to_company(&self, user_id: Uuid, company_id: Uuid, actor_id: Uuid) -> Result<UserDto, AppError> {
        let mut user = self.user_repository.find_by_id(user_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))?;

        let company = self.company_repository.find_by_id(company_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("Company with ID {} not found", company_id)))?;

        let actor = self.user_repository.find_by_id(actor_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Actor user not found".to_string()))?;

        if !actor.can_manage_company(company.id) {
            return Err(AppError::Unauthorized("Insufficient permissions to assign users to this company".to_string()));
        }

        user.company_id = Some(company.id);
        let updated_user = self.user_repository.update(&user).await.map_err(AppError::from)?;
        
        Ok(UserDto::from_entity(&updated_user))
    }

    async fn change_user_role(&self, user_id: Uuid, new_role: UserRole, actor_id: Uuid) -> Result<UserDto, AppError> {
        let mut user = self.user_repository.find_by_id(user_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))?;

        let actor = self.user_repository.find_by_id(actor_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Actor user not found".to_string()))?;

        if !actor.is_admin() && !actor.can_manage_user(&user) {
            return Err(AppError::Unauthorized("Insufficient permissions to change user role".to_string()));
        }

        user.role = new_role;
        let updated_user = self.user_repository.update(&user).await.map_err(AppError::from)?;
        
        Ok(UserDto::from_entity(&updated_user))
    }
}
EOF

# Application Services - Auth Service (Fixed error handling)
cat > src/application/services/auth_service.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::repositories::UserRepository;
use crate::application::dtos::{LoginDto, AuthResponseDto, RefreshTokenDto};
use crate::shared::error::AppError;
use crate::infrastructure::auth::{KeycloakClient, JwtService};

#[async_trait]
pub trait AuthApplicationService: Send + Sync {
    async fn login(&self, login_dto: LoginDto) -> Result<AuthResponseDto, AppError>;
    async fn refresh_token(&self, refresh_dto: RefreshTokenDto) -> Result<AuthResponseDto, AppError>;
    async fn logout(&self, _user_id: Uuid) -> Result<(), AppError>;
    async fn validate_token(&self, token: &str) -> Result<bool, AppError>;
}

pub struct AuthApplicationServiceImpl {
    user_repository: Box<dyn UserRepository>,
    keycloak_client: KeycloakClient,
    jwt_service: JwtService,
}

impl AuthApplicationServiceImpl {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        keycloak_client: KeycloakClient,
        jwt_service: JwtService,
    ) -> Self {
        Self {
            user_repository,
            keycloak_client,
            jwt_service,
        }
    }
}

#[async_trait]
impl AuthApplicationService for AuthApplicationServiceImpl {
    async fn login(&self, login_dto: LoginDto) -> Result<AuthResponseDto, AppError> {
        // Authenticate with Keycloak
        let keycloak_tokens = self.keycloak_client.login(&login_dto.username, &login_dto.password).await
            .map_err(|e| AppError::AuthError(e.to_string()))?;

        // Get user info from Keycloak
        let user_info = self.keycloak_client.user_info(&keycloak_tokens.access_token).await
            .map_err(|e| AppError::AuthError(e.to_string()))?;

        // Find user in our database
        let user = self.user_repository.find_by_keycloak_id(&user_info.sub).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("User not found in database".to_string()))?;

        // Generate JWT token
        let jwt_token = self.jwt_service.generate_token(
            user.id,
            &user.username,
            &user.email,
            &user.role.to_string(),
            user.company_id,
            None, // company_name would come from company service
        ).map_err(|e| AppError::AuthError(e.to_string()))?;

        Ok(AuthResponseDto {
            access_token: jwt_token,
            refresh_token: keycloak_tokens.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: keycloak_tokens.expires_in,
            user: crate::application::dtos::UserDto::from_entity(&user),
        })
    }

    async fn refresh_token(&self, refresh_dto: RefreshTokenDto) -> Result<AuthResponseDto, AppError> {
        let keycloak_tokens = self.keycloak_client.refresh_token(&refresh_dto.refresh_token).await
            .map_err(|e| AppError::AuthError(e.to_string()))?;

        // Get user info from new access token
        let user_info = self.keycloak_client.user_info(&keycloak_tokens.access_token).await
            .map_err(|e| AppError::AuthError(e.to_string()))?;

        // Find user in our database
        let user = self.user_repository.find_by_keycloak_id(&user_info.sub).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("User not found in database".to_string()))?;

        // Generate new JWT token
        let jwt_token = self.jwt_service.generate_token(
            user.id,
            &user.username,
            &user.email,
            &user.role.to_string(),
            user.company_id,
            None,
        ).map_err(|e| AppError::AuthError(e.to_string()))?;

        Ok(AuthResponseDto {
            access_token: jwt_token,
            refresh_token: keycloak_tokens.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: keycloak_tokens.expires_in,
            user: crate::application::dtos::UserDto::from_entity(&user),
        })
    }

    async fn logout(&self, _user_id: Uuid) -> Result<(), AppError> {
        // In a real implementation, you might want to blacklist the token
        // or call Keycloak's logout endpoint
        Ok(())
    }

    async fn validate_token(&self, token: &str) -> Result<bool, AppError> {
        match self.jwt_service.validate_token(token) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
EOF

# Application Services - Company Service (Fixed error handling)
cat > src/application/services/company_service.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Company;
use crate::domain::repositories::{CompanyRepository, UserRepository};
use crate::application::dtos::{CompanyDto, CreateCompanyDto, UpdateCompanyDto};
use crate::shared::error::AppError;

#[async_trait]
pub trait CompanyApplicationService: Send + Sync {
    async fn create_company(&self, create_company_dto: CreateCompanyDto, creator_id: Uuid) -> Result<CompanyDto, AppError>;
    async fn get_company_by_id(&self, company_id: Uuid) -> Result<CompanyDto, AppError>;
    async fn update_company(&self, company_id: Uuid, update_company_dto: UpdateCompanyDto, actor_id: Uuid) -> Result<CompanyDto, AppError>;
    async fn delete_company(&self, company_id: Uuid, actor_id: Uuid) -> Result<(), AppError>;
    async fn list_companies(&self) -> Result<Vec<CompanyDto>, AppError>;
    async fn list_user_companies(&self, user_id: Uuid) -> Result<Vec<CompanyDto>, AppError>;
}

pub struct CompanyApplicationServiceImpl {
    company_repository: Box<dyn CompanyRepository>,
    user_repository: Box<dyn UserRepository>,
}

impl CompanyApplicationServiceImpl {
    pub fn new(
        company_repository: Box<dyn CompanyRepository>,
        user_repository: Box<dyn UserRepository>,
    ) -> Self {
        Self {
            company_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl CompanyApplicationService for CompanyApplicationServiceImpl {
    async fn create_company(&self, create_company_dto: CreateCompanyDto, creator_id: Uuid) -> Result<CompanyDto, AppError> {
        // Check if company name already exists
        if self.company_repository.find_by_name(&create_company_dto.name).await.map_err(AppError::from)?.is_some() {
            return Err(AppError::ValidationError("Company with this name already exists".to_string()));
        }

        // Verify creator exists
        let creator = self.user_repository.find_by_id(creator_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Creator user not found".to_string()))?;

        let company = Company::new(
            create_company_dto.name,
            create_company_dto.description,
            creator.id,
        );

        let saved_company = self.company_repository.create(&company).await.map_err(AppError::from)?;
        
        Ok(CompanyDto::from_entity(&saved_company))
    }

    async fn get_company_by_id(&self, company_id: Uuid) -> Result<CompanyDto, AppError> {
        let company = self.company_repository.find_by_id(company_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("Company with ID {} not found", company_id)))?;
        
        Ok(CompanyDto::from_entity(&company))
    }

    async fn update_company(&self, company_id: Uuid, update_company_dto: UpdateCompanyDto, actor_id: Uuid) -> Result<CompanyDto, AppError> {
        let mut company = self.company_repository.find_by_id(company_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("Company with ID {} not found", company_id)))?;

        let actor = self.user_repository.find_by_id(actor_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Actor user not found".to_string()))?;

        if !actor.can_manage_company(company.id) {
            return Err(AppError::Unauthorized("Insufficient permissions to update this company".to_string()));
        }

        if let Some(name) = update_company_dto.name {
            company.name = name;
        }
        if let Some(description) = update_company_dto.description {
            company.description = Some(description);
        }

        let updated_company = self.company_repository.update(&company).await.map_err(AppError::from)?;
        
        Ok(CompanyDto::from_entity(&updated_company))
    }

    async fn delete_company(&self, company_id: Uuid, actor_id: Uuid) -> Result<(), AppError> {
        let company = self.company_repository.find_by_id(company_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("Company with ID {} not found", company_id)))?;

        let actor = self.user_repository.find_by_id(actor_id).await.map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Actor user not found".to_string()))?;

        if !actor.can_manage_company(company.id) {
            return Err(AppError::Unauthorized("Insufficient permissions to delete this company".to_string()));
        }

        self.company_repository.delete(company_id).await.map_err(AppError::from)?;
        
        Ok(())
    }

    async fn list_companies(&self) -> Result<Vec<CompanyDto>, AppError> {
        let companies = self.company_repository.list_all().await.map_err(AppError::from)?;
        
        Ok(companies.iter().map(CompanyDto::from_entity).collect())
    }

    async fn list_user_companies(&self, user_id: Uuid) -> Result<Vec<CompanyDto>, AppError> {
        let companies = self.company_repository.list_by_user(user_id).await.map_err(AppError::from)?;
        
        Ok(companies.iter().map(CompanyDto::from_entity).collect())
    }
}
EOF

# Application Services - Audit Service (Fixed error handling)
cat > src/application/services/audit_service.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::AuditLog;
use crate::domain::enums::AuditAction;
use crate::domain::repositories::AuditLogRepository;
use crate::application::dtos::AuditLogDto;
use crate::shared::error::AppError;

#[async_trait]
pub trait AuditApplicationService: Send + Sync {
    async fn log_action(
        &self,
        user_id: Option<Uuid>,
        action: AuditAction,
        resource_type: String,
        resource_id: Option<String>,
        details: Option<serde_json::Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), AppError>;
    
    async fn get_audit_logs_by_user(&self, user_id: Uuid) -> Result<Vec<AuditLogDto>, AppError>;
    async fn get_audit_logs_by_company(&self, company_id: Uuid) -> Result<Vec<AuditLogDto>, AppError>;
    async fn get_recent_audit_logs(&self, limit: u32) -> Result<Vec<AuditLogDto>, AppError>;
}

pub struct AuditApplicationServiceImpl {
    audit_log_repository: Box<dyn AuditLogRepository>,
}

impl AuditApplicationServiceImpl {
    pub fn new(audit_log_repository: Box<dyn AuditLogRepository>) -> Self {
        Self {
            audit_log_repository,
        }
    }
}

#[async_trait]
impl AuditApplicationService for AuditApplicationServiceImpl {
    async fn log_action(
        &self,
        user_id: Option<Uuid>,
        action: AuditAction,
        resource_type: String,
        resource_id: Option<String>,
        details: Option<serde_json::Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), AppError> {
        let audit_log = AuditLog::new(
            user_id,
            action,
            resource_type,
            resource_id,
            details,
            ip_address,
            user_agent,
        );

        self.audit_log_repository.create(&audit_log).await.map_err(AppError::from)?;
        
        Ok(())
    }

    async fn get_audit_logs_by_user(&self, user_id: Uuid) -> Result<Vec<AuditLogDto>, AppError> {
        let logs = self.audit_log_repository.find_by_user(user_id).await.map_err(AppError::from)?;
        
        Ok(logs.iter().map(AuditLogDto::from_entity).collect())
    }

    async fn get_audit_logs_by_company(&self, company_id: Uuid) -> Result<Vec<AuditLogDto>, AppError> {
        let logs = self.audit_log_repository.find_by_company(company_id).await.map_err(AppError::from)?;
        
        Ok(logs.iter().map(AuditLogDto::from_entity).collect())
    }

    async fn get_recent_audit_logs(&self, limit: u32) -> Result<Vec<AuditLogDto>, AppError> {
        let logs = self.audit_log_repository.list_recent(limit).await.map_err(AppError::from)?;
        
        Ok(logs.iter().map(AuditLogDto::from_entity).collect())
    }
}
EOF

# Services mod.rs
cat > src/application/services/mod.rs << 'EOF'
pub mod user_service;
pub mod auth_service;
pub mod company_service;
pub mod audit_service;

pub use user_service::{UserApplicationService, UserApplicationServiceImpl};
pub use auth_service::{AuthApplicationService, AuthApplicationServiceImpl};
pub use company_service::{CompanyApplicationService, CompanyApplicationServiceImpl};
pub use audit_service::{AuditApplicationService, AuditApplicationServiceImpl};
EOF

# Application DTOs - User DTOs (Removed utoipa dependency)
cat > src/application/dtos/user_dto.rs << 'EOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::enums::UserRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: Uuid,
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub company_id: Option<Uuid>,
    pub email_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl UserDto {
    pub fn from_entity(user: &User) -> Self {
        Self {
            id: user.id,
            keycloak_id: user.keycloak_id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserListDto {
    pub users: Vec<UserDto>,
    pub total: usize,
}
EOF

# Application DTOs - Auth DTOs (Removed utoipa dependency)
cat > src/application/dtos/auth_dto.rs << 'EOF'
use serde::{Deserialize, Serialize};

use crate::application::dtos::UserDto;

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub company_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenDto {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserDto,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordDto {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordDto {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordDto {
    pub token: String,
    pub new_password: String,
}
EOF

# Application DTOs - Company DTOs (Removed utoipa dependency)
cat > src/application/dtos/company_dto.rs << 'EOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::Company;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CompanyDto {
    pub fn from_entity(company: &Company) -> Self {
        Self {
            id: company.id,
            name: company.name.clone(),
            description: company.description.clone(),
            created_by: company.created_by,
            created_at: company.created_at,
            updated_at: company.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCompanyDto {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCompanyDto {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CompanyListDto {
    pub companies: Vec<CompanyDto>,
    pub total: usize,
}
EOF

# Application DTOs - Audit DTOs (Removed utoipa dependency)
cat > src/application/dtos/audit_dto.rs << 'EOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::AuditLog;
use crate::domain::enums::AuditAction;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogDto {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: AuditAction,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl AuditLogDto {
    pub fn from_entity(audit_log: &AuditLog) -> Self {
        Self {
            id: audit_log.id,
            user_id: audit_log.user_id,
            action: audit_log.action,
            resource_type: audit_log.resource_type.clone(),
            resource_id: audit_log.resource_id.clone(),
            details: audit_log.details.clone(),
            ip_address: audit_log.ip_address.clone(),
            user_agent: audit_log.user_agent.clone(),
            created_at: audit_log.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuditLogListDto {
    pub logs: Vec<AuditLogDto>,
    pub total: usize,
}
EOF

# Application DTOs - Common DTOs
cat > src/application/dtos/common_dto.rs << 'EOF'
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl Default for PaginationRequest {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
        }
    }
}

impl PaginationRequest {
    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn get_per_page(&self) -> u32 {
        self.per_page.unwrap_or(20).clamp(1, 100)
    }

    pub fn get_offset(&self) -> u32 {
        (self.get_page() - 1) * self.get_per_page()
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub pagination: PaginationRequest,
}

#[derive(Debug, Deserialize)]
pub struct FilterRequest {
    pub company_id: Option<uuid::Uuid>,
    pub role: Option<String>,
    pub date_from: Option<chrono::DateTime<chrono::Utc>>,
    pub date_to: Option<chrono::DateTime<chrono::Utc>>,
}
EOF

# Application DTOs - Response DTOs
cat > src/application/dtos/response_dto.rs << 'EOF'
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn with_message(data: T, message: &str) -> Self {
        Self {
            success: true,
            data,
            message: Some(message.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ApiError,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ErrorResponse {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            success: false,
            error: ApiError {
                code: code.to_string(),
                message: message.to_string(),
                details: None,
            },
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn with_details(code: &str, message: &str, details: serde_json::Value) -> Self {
        Self {
            success: false,
            error: ApiError {
                code: code.to_string(),
                message: message.to_string(),
                details: Some(details),
            },
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginationMetadata {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: PaginationMetadata,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, page: u32, per_page: u32, total: u64) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
        
        Self {
            items,
            pagination: PaginationMetadata {
                page,
                per_page,
                total,
                total_pages,
            },
        }
    }
}
EOF

# DTOs mod.rs
cat > src/application/dtos/mod.rs << 'EOF'
pub mod user_dto;
pub mod auth_dto;
pub mod company_dto;
pub mod audit_dto;
pub mod common_dto;
pub mod response_dto;

pub use user_dto::*;
pub use auth_dto::*;
pub use company_dto::*;
pub use audit_dto::*;
pub use common_dto::*;
pub use response_dto::*;
EOF

# Application Commands - User Commands
cat > src/application/commands/user_commands.rs << 'EOF'
use uuid::Uuid;

use crate::application::dtos::CreateUserDto;

pub struct CreateUserCommand {
    pub user_data: CreateUserDto,
}

impl CreateUserCommand {
    pub fn new(user_data: CreateUserDto) -> Self {
        Self { user_data }
    }
}

pub struct CreateUserCommandResult {
    pub user_id: Uuid,
}

pub struct UpdateUserCommand {
    pub user_id: Uuid,
    pub user_data: crate::application::dtos::UpdateUserDto,
    pub actor_id: Uuid,
}

impl UpdateUserCommand {
    pub fn new(user_id: Uuid, user_data: crate::application::dtos::UpdateUserDto, actor_id: Uuid) -> Self {
        Self { user_id, user_data, actor_id }
    }
}

pub struct DeleteUserCommand {
    pub user_id: Uuid,
    pub actor_id: Uuid,
}

impl DeleteUserCommand {
    pub fn new(user_id: Uuid, actor_id: Uuid) -> Self {
        Self { user_id, actor_id }
    }
}

pub struct AssignUserToCompanyCommand {
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub actor_id: Uuid,
}

impl AssignUserToCompanyCommand {
    pub fn new(user_id: Uuid, company_id: Uuid, actor_id: Uuid) -> Self {
        Self { user_id, company_id, actor_id }
    }
}

pub struct ChangeUserRoleCommand {
    pub user_id: Uuid,
    pub new_role: crate::domain::enums::UserRole,
    pub actor_id: Uuid,
}

impl ChangeUserRoleCommand {
    pub fn new(user_id: Uuid, new_role: crate::domain::enums::UserRole, actor_id: Uuid) -> Self {
        Self { user_id, new_role, actor_id }
    }
}
EOF

# Application Commands - Auth Commands
cat > src/application/commands/auth_commands.rs << 'EOF'
use crate::application::dtos::LoginDto;

pub struct LoginCommand {
    pub login_data: LoginDto,
}

impl LoginCommand {
    pub fn new(login_data: LoginDto) -> Self {
        Self { login_data }
    }
}

pub struct LoginCommandResult {
    pub auth_response: crate::application::dtos::AuthResponseDto,
}

pub struct RefreshTokenCommand {
    pub refresh_data: crate::application::dtos::RefreshTokenDto,
}

impl RefreshTokenCommand {
    pub fn new(refresh_data: crate::application::dtos::RefreshTokenDto) -> Self {
        Self { refresh_data }
    }
}

pub struct LogoutCommand {
    pub user_id: uuid::Uuid,
}

impl LogoutCommand {
    pub fn new(user_id: uuid::Uuid) -> Self {
        Self { user_id }
    }
}
EOF

# Application Commands - Company Commands
cat > src/application/commands/company_commands.rs << 'EOF'
use uuid::Uuid;

use crate::application::dtos::CreateCompanyDto;

pub struct CreateCompanyCommand {
    pub company_data: CreateCompanyDto,
    pub creator_id: Uuid,
}

impl CreateCompanyCommand {
    pub fn new(company_data: CreateCompanyDto, creator_id: Uuid) -> Self {
        Self { company_data, creator_id }
    }
}

pub struct CreateCompanyCommandResult {
    pub company_id: Uuid,
}

pub struct UpdateCompanyCommand {
    pub company_id: Uuid,
    pub company_data: crate::application::dtos::UpdateCompanyDto,
    pub actor_id: Uuid,
}

impl UpdateCompanyCommand {
    pub fn new(company_id: Uuid, company_data: crate::application::dtos::UpdateCompanyDto, actor_id: Uuid) -> Self {
        Self { company_id, company_data, actor_id }
    }
}

pub struct DeleteCompanyCommand {
    pub company_id: Uuid,
    pub actor_id: Uuid,
}

impl DeleteCompanyCommand {
    pub fn new(company_id: Uuid, actor_id: Uuid) -> Self {
        Self { company_id, actor_id }
    }
}
EOF

# Application Commands - Audit Commands
cat > src/application/commands/audit_commands.rs << 'EOF'
use uuid::Uuid;

use crate::domain::enums::AuditAction;

pub struct LogActionCommand {
    pub user_id: Option<Uuid>,
    pub action: AuditAction,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl LogActionCommand {
    pub fn new(
        user_id: Option<Uuid>,
        action: AuditAction,
        resource_type: String,
        resource_id: Option<String>,
        details: Option<serde_json::Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        Self {
            user_id,
            action,
            resource_type,
            resource_id,
            details,
            ip_address,
            user_agent,
        }
    }
}

pub struct GetUserAuditLogsCommand {
    pub user_id: Uuid,
}

impl GetUserAuditLogsCommand {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

pub struct GetCompanyAuditLogsCommand {
    pub company_id: Uuid,
}

impl GetCompanyAuditLogsCommand {
    pub fn new(company_id: Uuid) -> Self {
        Self { company_id }
    }
}

pub struct GetRecentAuditLogsCommand {
    pub limit: u32,
}

impl GetRecentAuditLogsCommand {
    pub fn new(limit: u32) -> Self {
        Self { limit }
    }
}
EOF

# Commands mod.rs
cat > src/application/commands/mod.rs << 'EOF'
pub mod user_commands;
pub mod auth_commands;
pub mod company_commands;
pub mod audit_commands;

pub use user_commands::*;
pub use auth_commands::*;
pub use company_commands::*;
pub use audit_commands::*;
EOF

# Application Queries - User Queries
cat > src/application/queries/user_queries.rs << 'EOF'
use uuid::Uuid;

pub struct GetUserByIdQuery {
    pub user_id: Uuid,
}

impl GetUserByIdQuery {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

pub struct GetUserByEmailQuery {
    pub email: String,
}

impl GetUserByEmailQuery {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

pub struct ListUsersByCompanyQuery {
    pub company_id: Uuid,
}

impl ListUsersByCompanyQuery {
    pub fn new(company_id: Uuid) -> Self {
        Self { company_id }
    }
}

pub struct SearchUsersQuery {
    pub query: Option<String>,
    pub company_id: Option<Uuid>,
    pub role: Option<String>,
    pub page: u32,
    pub per_page: u32,
}

impl SearchUsersQuery {
    pub fn new(
        query: Option<String>,
        company_id: Option<Uuid>,
        role: Option<String>,
        page: u32,
        per_page: u32,
    ) -> Self {
        Self {
            query,
            company_id,
            role,
            page,
            per_page,
        }
    }
}
EOF

# Application Queries - Company Queries
cat > src/application/queries/company_queries.rs << 'EOF'
use uuid::Uuid;

pub struct GetCompanyByIdQuery {
    pub company_id: Uuid,
}

impl GetCompanyByIdQuery {
    pub fn new(company_id: Uuid) -> Self {
        Self { company_id }
    }
}

pub struct ListCompaniesQuery {
    pub page: u32,
    pub per_page: u32,
}

impl ListCompaniesQuery {
    pub fn new(page: u32, per_page: u32) -> Self {
        Self { page, per_page }
    }
}

pub struct ListUserCompaniesQuery {
    pub user_id: Uuid,
}

impl ListUserCompaniesQuery {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

pub struct SearchCompaniesQuery {
    pub query: Option<String>,
    pub page: u32,
    pub per_page: u32,
}

impl SearchCompaniesQuery {
    pub fn new(query: Option<String>, page: u32, per_page: u32) -> Self {
        Self {
            query,
            page,
            per_page,
        }
    }
}
EOF

# Application Queries - Audit Queries
cat > src/application/queries/audit_queries.rs << 'EOF'
use uuid::Uuid;

pub struct GetAuditLogsByUserQuery {
    pub user_id: Uuid,
}

impl GetAuditLogsByUserQuery {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

pub struct GetAuditLogsByCompanyQuery {
    pub company_id: Uuid,
}

impl GetAuditLogsByCompanyQuery {
    pub fn new(company_id: Uuid) -> Self {
        Self { company_id }
    }
}

pub struct GetRecentAuditLogsQuery {
    pub limit: u32,
}

impl GetRecentAuditLogsQuery {
    pub fn new(limit: u32) -> Self {
        Self { limit }
    }
}

pub struct SearchAuditLogsQuery {
    pub user_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub action: Option<String>,
    pub date_from: Option<chrono::DateTime<chrono::Utc>>,
    pub date_to: Option<chrono::DateTime<chrono::Utc>>,
    pub page: u32,
    pub per_page: u32,
}

impl SearchAuditLogsQuery {
    pub fn new(
        user_id: Option<Uuid>,
        company_id: Option<Uuid>,
        action: Option<String>,
        date_from: Option<chrono::DateTime<chrono::Utc>>,
        date_to: Option<chrono::DateTime<chrono::Utc>>,
        page: u32,
        per_page: u32,
    ) -> Self {
        Self {
            user_id,
            company_id,
            action,
            date_from,
            date_to,
            page,
            per_page,
        }
    }
}
EOF

# Queries mod.rs
cat > src/application/queries/mod.rs << 'EOF'
pub mod user_queries;
pub mod company_queries;
pub mod audit_queries;

pub use user_queries::*;
pub use company_queries::*;
pub use audit_queries::*;
EOF

# Application Layer Mod File
cat > src/application/mod.rs << 'EOF'
pub mod services;
pub mod commands;
pub mod queries;
pub mod dtos;

pub use services::*;
pub use commands::*;
pub use queries::*;
pub use dtos::*;
EOF

# Create the Cargo.toml with all required dependencies
cat > Cargo.toml << 'EOF'
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"
description = "Authentication and Authorization Microservice with Keycloak"
authors = ["M.MEZNI"]
license = "MIT"

[dependencies]
actix-web = "4.12.0"
serde = { version = "1.0.228", features = ["derive"] }
thiserror = "2.0.17"
tracing = "0.1.41"
tracing-subscriber = "0.3.20"
tokio = { version = "1.48.0", features = ["full"] }
config = "0.15.19"
serde_json = "1.0.145"
chrono = { version = "0.4.42", features = ["serde"] }
validator = { version = "0.20.0", features = ["derive"] }
uuid = { version = "1.18.1", features = ["v4", "serde"] }
async-trait = "0.1.68"
sqlx = { version = "0.8.6", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls"] }
reqwest = { version = "0.12.24", features = ["json"] }
jsonwebtoken = { version = "10.2.0", features = ["rust_crypto"] }

[dev-dependencies]
actix-web = "4.12.0"
tokio = { version = "1.48.0", features = ["full"] }

[[bin]]
name = "auth-service"
path = "src/main.rs"
EOF

# Update Cargo.toml to add utoipa dependency if needed later
echo "Application layer generated successfully with all fixes!"