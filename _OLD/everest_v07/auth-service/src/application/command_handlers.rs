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
