use crate::application::dto::{CreateUserRequest, UpdateUserRequest, UserResponse, AuditLogResponse};
use crate::core::errors::{AppError, AppResult};
use crate::domain::audit_entity::{AuditAction, AuditLog, GeoLocation};
use crate::domain::audit_repository::AuditRepository;
use crate::domain::user_entity::User;
use crate::domain::user_repository::UserRepository;
use crate::domain::value_objects::{UserRole, UserSource};
use crate::infrastructure::keycloak::service::KeycloakService;
use std::sync::Arc;
use validator::Validate;

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
    audit_repo: Arc<dyn AuditRepository>,
    keycloak: Arc<KeycloakService>,
}

impl UserService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        audit_repo: Arc<dyn AuditRepository>,
        keycloak: Arc<KeycloakService>,
    ) -> Self {
        Self {
            user_repo,
            audit_repo,
            keycloak,
        }
    }

    pub async fn create_user_by_admin(
        &self,
        req: CreateUserRequest,
        admin_user_id: &str,
        geo: Option<GeoLocation>,
        user_agent: Option<String>,
    ) -> AppResult<UserResponse> {
        // Validate input
        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Check if email or username already exists
        if self.user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        if self.user_repo.find_by_username(&req.username).await?.is_some() {
            return Err(AppError::Conflict("Username already exists".to_string()));
        }

        // Validate role constraints
        let network_id = if req.role.requires_network_id() {
            req.network_id.clone().ok_or_else(|| {
                AppError::ValidationError("network_id is required for this role".to_string())
            })?
        } else {
            "X".to_string()
        };

        let station_id = if req.role.requires_station_id() {
            req.station_id.clone().ok_or_else(|| {
                AppError::ValidationError("station_id is required for operator role".to_string())
            })?
        } else {
            "X".to_string()
        };

        // Create user in Keycloak
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

        // Create user in database
        let mut user = User::new(
            keycloak_id.clone(),
            req.email,
            req.username,
            req.role,
            UserSource::Internal,
            network_id,
            station_id,
        );

        user.first_name = req.first_name;
        user.last_name = req.last_name;
        user.phone = req.phone;
        user.photo = req.photo;
        user.created_by = Some(admin_user_id.to_string());
        user.updated_by = Some(admin_user_id.to_string());

        user.validate_role_constraints()
            .map_err(|e| AppError::ValidationError(e))?;

        let created_user = self.user_repo.create(&user).await?;

        // Create audit log
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

    pub async fn get_user(&self, user_id: &str) -> AppResult<UserResponse> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user.into())
    }

    pub async fn update_user(
        &self,
        user_id: &str,
        req: UpdateUserRequest,
        updated_by: &str,
        geo: Option<GeoLocation>,
        user_agent: Option<String>,
    ) -> AppResult<UserResponse> {
        // Validate input
        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Update fields
        if let Some(first_name) = req.first_name {
            user.first_name = Some(first_name);
        }
        if let Some(last_name) = req.last_name {
            user.last_name = Some(last_name);
        }
        if let Some(phone) = req.phone {
            user.phone = Some(phone);
        }
        if let Some(photo) = req.photo {
            user.photo = Some(photo);
        }
        if let Some(is_active) = req.is_active {
            if is_active {
                user.activate(updated_by);
            } else {
                user.deactivate(updated_by);
            }
        }

        user.updated_by = Some(updated_by.to_string());

        // Update in database
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
            tracing::error!("Failed to sync user with Keycloak: {}", e);
        }

        // Create audit log
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
            tracing::error!("Failed to delete user from Keycloak: {}", e);
        }

        // Delete from database
        self.user_repo.delete(user_id).await?;

        // Create audit log
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

    pub async fn list_users(&self, limit: i64, offset: i64) -> AppResult<Vec<UserResponse>> {
        let users = self.user_repo.list(limit, offset).await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn count_users(&self) -> AppResult<i64> {
        self.user_repo.count().await
    }

    pub async fn search_users(&self, query: &str, limit: i64, offset: i64) -> AppResult<Vec<UserResponse>> {
        let users = self.user_repo.search(query, limit, offset).await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn get_users_by_network(&self, network_id: &str, limit: i64, offset: i64) -> AppResult<Vec<UserResponse>> {
        let users = self.user_repo.find_by_network(network_id, limit, offset).await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn get_users_by_station(&self, station_id: &str, limit: i64, offset: i64) -> AppResult<Vec<UserResponse>> {
        let users = self.user_repo.find_by_station(station_id, limit, offset).await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn get_users_by_role(&self, role: &str, limit: i64, offset: i64) -> AppResult<Vec<UserResponse>> {
        let users = self.user_repo.find_by_role(role, limit, offset).await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn get_user_audit_logs(&self, user_id: &str, limit: i64, offset: i64) -> AppResult<Vec<AuditLogResponse>> {
        let logs = self.audit_repo.find_by_user(user_id, limit, offset).await?;
        Ok(logs.into_iter().map(|l| AuditLogResponse {
            audit_id: l.audit_id,
            user_id: l.user_id,
            action: l.action,
            resource_type: l.resource_type,
            resource_id: l.resource_id,
            ip_address: l.ip_address,
            country: l.country,
            city: l.city,
            user_agent: l.user_agent,
            created_at: l.created_at,
        }).collect())
    }

    pub async fn count_user_audit_logs(&self, user_id: &str) -> AppResult<i64> {
        self.audit_repo.count_by_user(user_id).await
    }

    async fn log_audit(
        &self,
        user_id: String,
        action: AuditAction,
        resource_id: Option<String>,
        geo: Option<GeoLocation>,
        user_agent: Option<String>,
    ) {
        let mut audit = AuditLog::new(user_id, action, "user".to_string(), resource_id);

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