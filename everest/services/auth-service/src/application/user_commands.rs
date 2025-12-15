use crate::application::dtos::*;
use crate::core::errors::{AppError, AppResult};
use crate::domain::audit::{Audit, AuditAction, GeoLocation};
use crate::domain::events::{DomainEvent, OutboxEvent, UserCreatedEvent, UserDeletedEvent, UserUpdatedEvent};
use crate::domain::user::User;
use crate::domain::value_objects::{Email, UserRole, UserSource};
use crate::infrastructure::keycloak_client::KeycloakClient;
use crate::infrastructure::repositories::{
    audit_repository::AuditRepository, outbox_repository::OutboxRepository,
    user_repository::UserRepository,
};
use std::sync::Arc;
use validator::Validate;

pub struct UserCommandHandler {
    user_repo: Arc<dyn UserRepository>,
    audit_repo: Arc<dyn AuditRepository>,
    outbox_repo: Arc<dyn OutboxRepository>,
    keycloak: Arc<KeycloakClient>,
}

impl UserCommandHandler {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        audit_repo: Arc<dyn AuditRepository>,
        outbox_repo: Arc<dyn OutboxRepository>,
        keycloak: Arc<KeycloakClient>,
    ) -> Self {
        Self {
            user_repo,
            audit_repo,
            outbox_repo,
            keycloak,
        }
    }

    pub async fn create_user(
        &self,
        req: CreateUserRequest,
        admin_user_id: &str,
        geo: Option<GeoLocation>,
        user_agent: Option<String>,
    ) -> AppResult<UserResponse> {
        // Validate
        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Check duplicates
        if self.user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        if self.user_repo.find_by_username(&req.username).await?.is_some() {
            return Err(AppError::Conflict("Username already exists".to_string()));
        }

        // Validate role constraints
        let network_id = if req.role.requires_network_id() {
            req.network_id
                .clone()
                .ok_or_else(|| AppError::ValidationError("network_id required".to_string()))?
        } else {
            "X".to_string()
        };

        let station_id = if req.role.requires_station_id() {
            req.station_id
                .clone()
                .ok_or_else(|| AppError::ValidationError("station_id required".to_string()))?
        } else {
            "X".to_string()
        };

        // Create in Keycloak
        let keycloak_id = self
            .keycloak
            .create_user(
                &req.username,
                &req.email,
                &req.password,
                req.first_name.as_deref(),
                req.last_name.as_deref(),
                req.role.clone(),
                &network_id,
                &station_id,
            )
            .await?;

        // Create domain entity
        let mut user = User::new(
            keycloak_id,
            Email::new(&req.email).map_err(|e| AppError::ValidationError(e))?,
            req.username.clone(),
            req.role.clone(),
            UserSource::Internal,
            network_id.clone(),
            station_id.clone(),
        );

        user.first_name = req.first_name;
        user.last_name = req.last_name;
        user.phone = req.phone;
        user.photo = req.photo;
        user.created_by = Some(admin_user_id.to_string());
        user.updated_by = Some(admin_user_id.to_string());

        user.validate_role_constraints()
            .map_err(|e| AppError::ValidationError(e))?;

        // Persist
        let created_user = self.user_repo.create(&user).await?;

        // Create domain event
        let event = DomainEvent::UserCreated(UserCreatedEvent {
            user_id: created_user.user_id.clone(),
            email: created_user.email.clone(),
            username: created_user.username.clone(),
            role: created_user.role.clone(),
            source: created_user.source.clone(),
            network_id: network_id.clone(),
            station_id: station_id.clone(),
            created_at: created_user.created_at,
        });

        let outbox = OutboxEvent::new(event, created_user.user_id.clone());
        if let Err(e) = self.outbox_repo.create(&outbox).await {
            tracing::error!("Failed to store outbox event: {}", e);
        }

        // Audit
        self.log_audit(
            admin_user_id.to_string(),
            AuditAction::UserCreated,
            Some(created_user.user_id.clone()),
            geo,
            user_agent,
        )
        .await;

        Ok(created_user.into())
    }

    pub async fn update_user(
        &self,
        user_id: &str,
        req: UpdateUserRequest,
        updated_by: &str,
        geo: Option<GeoLocation>,
        user_agent: Option<String>,
    ) -> AppResult<UserResponse> {
        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Track updated fields
        let mut updated_fields = Vec::new();

        if let Some(ref first_name) = req.first_name {
            user.first_name = Some(first_name.clone());
            updated_fields.push("first_name".to_string());
        }
        if let Some(ref last_name) = req.last_name {
            user.last_name = Some(last_name.clone());
            updated_fields.push("last_name".to_string());
        }
        if let Some(ref phone) = req.phone {
            user.phone = Some(phone.clone());
            updated_fields.push("phone".to_string());
        }
        if let Some(ref photo) = req.photo {
            user.photo = Some(photo.clone());
            updated_fields.push("photo".to_string());
        }
        if let Some(is_active) = req.is_active {
            if is_active {
                user.activate(updated_by);
            } else {
                user.deactivate(updated_by);
            }
            updated_fields.push("is_active".to_string());
        }

        user.updated_by = Some(updated_by.to_string());

        let updated_user = self.user_repo.update(&user).await?;

        // Sync with Keycloak
        if let Err(e) = self
            .keycloak
            .update_user(
                &user.keycloak_id,
                user.first_name.as_deref(),
                user.last_name.as_deref(),
                None,
                None,
            )
            .await
        {
            tracing::error!("Failed to sync with Keycloak: {}", e);
        }

        // Domain event
        let event = DomainEvent::UserUpdated(UserUpdatedEvent {
            user_id: updated_user.user_id.clone(),
            updated_fields,
            updated_by: Some(updated_by.to_string()),
            updated_at: updated_user.updated_at,
        });

        let outbox = OutboxEvent::new(event, updated_user.user_id.clone());
        if let Err(e) = self.outbox_repo.create(&outbox).await {
            tracing::error!("Failed to store outbox event: {}", e);
        }

        // Audit
        self.log_audit(
            updated_by.to_string(),
            AuditAction::UserUpdated,
            Some(user_id.to_string()),
            geo,
            user_agent,
        )
        .await;

        Ok(updated_user.into())
    }

    pub async fn delete_user(
        &self,
        user_id: &str,
        deleted_by: &str,
        geo: Option<GeoLocation>,
        user_agent: Option<String>,
    ) -> AppResult<()> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Delete from Keycloak
        if let Err(e) = self.keycloak.delete_user(&user.keycloak_id).await {
            tracing::error!("Failed to delete from Keycloak: {}", e);
        }

        // Delete from database
        self.user_repo.delete(user_id).await?;

        // Domain event
        let event = DomainEvent::UserDeleted(UserDeletedEvent {
            user_id: user_id.to_string(),
            deleted_by: deleted_by.to_string(),
            deleted_at: chrono::Utc::now(),
        });

        let outbox = OutboxEvent::new(event, user_id.to_string());
        if let Err(e) = self.outbox_repo.create(&outbox).await {
            tracing::error!("Failed to store outbox event: {}", e);
        }

        // Audit
        self.log_audit(
            deleted_by.to_string(),
            AuditAction::UserDeleted,
            Some(user_id.to_string()),
            geo,
            user_agent,
        )
        .await;

        Ok(())
    }

    async fn log_audit(
        &self,
        user_id: String,
        action: AuditAction,
        resource_id: Option<String>,
        geo: Option<GeoLocation>,
        user_agent: Option<String>,
    ) {
        let mut audit = Audit::new(user_id, action, "user".to_string(), resource_id);

        if let Some(g) = geo {
            audit = audit.with_location(g.ip, g.country, g.city, g.latitude, g.longitude);
        }

        if let Some(ua) = user_agent {
            audit = audit.with_user_agent(ua);
        }

        if let Err(e) = self.audit_repo.create(&audit).await {
            tracing::error!("Failed to create audit log: {}", e);
        }
    }
}