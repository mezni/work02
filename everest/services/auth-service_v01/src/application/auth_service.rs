use crate::application::dto::{LoginRequest, LoginResponse, RegisterRequest, UserResponse};
use crate::core::errors::{AppError, AppResult};
use crate::domain::audit_entity::{AuditAction, AuditLog, GeoLocation};
use crate::domain::audit_repository::AuditRepository;
use crate::domain::user_entity::User;
use crate::domain::user_repository::UserRepository;
use crate::domain::value_objects::{UserRole, UserSource};
use crate::infrastructure::keycloak::service::KeycloakService;
use std::sync::Arc;
use validator::Validate;

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    audit_repo: Arc<dyn AuditRepository>,
    keycloak: Arc<KeycloakService>,
}

impl AuthService {
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

    pub async fn register(
        &self,
        req: RegisterRequest,
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

        // Create user in Keycloak
        let keycloak_id = self
            .keycloak
            .create_user(
                &req.username,
                &req.email,
                &req.password,
                req.first_name.as_deref(),
                req.last_name.as_deref(),
                UserRole::User,
                "X",
                "X",
            )
            .await?;

        // Create user in database
        let mut user = User::new(
            keycloak_id.clone(),
            req.email,
            req.username,
            UserRole::User,
            UserSource::Web,
            "X".to_string(),
            "X".to_string(),
        );

        user.first_name = req.first_name;
        user.last_name = req.last_name;
        user.phone = req.phone;

        let created_user = self.user_repo.create(&user).await?;

        // Send verification email
        if let Err(e) = self.keycloak.send_verification_email(&keycloak_id).await {
            tracing::warn!("Failed to send verification email: {}", e);
        }

        // Create audit log
        self.log_audit(
            created_user.user_id.clone(),
            AuditAction::UserCreated,
            Some(created_user.user_id.clone()),
            geo,
            user_agent,
        )
        .await;

        Ok(created_user.into())
    }

    pub async fn login(
        &self,
        req: LoginRequest,
        geo: Option<GeoLocation>,
        user_agent: Option<String>,
    ) -> AppResult<LoginResponse> {
        // Validate input
        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Authenticate with Keycloak
        let token_response = self
            .keycloak
            .authenticate(&req.email, &req.password)
            .await?;

        // Get user from database
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        if !user.is_active {
            return Err(AppError::Unauthorized("Account is deactivated".to_string()));
        }

        // Create audit log
        self.log_audit(
            user.user_id.clone(),
            AuditAction::UserLoggedIn,
            None,
            geo,
            user_agent,
        )
        .await;

        Ok(LoginResponse {
            access_token: token_response.access_token,
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            user: user.into(),
        })
    }

    pub async fn logout(
        &self,
        user_id: &str,
        geo: Option<GeoLocation>,
        user_agent: Option<String>,
    ) -> AppResult<()> {
        // Create audit log
        self.log_audit(
            user_id.to_string(),
            AuditAction::UserLoggedOut,
            None,
            geo,
            user_agent,
        )
        .await;

        Ok(())
    }

    pub async fn verify_email(&self, keycloak_id: &str) -> AppResult<UserResponse> {
        let mut user = self
            .user_repo
            .find_by_keycloak_id(keycloak_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        user.verify();
        let updated_user = self.user_repo.update(&user).await?;

        // Create audit log
        self.log_audit(
            user.user_id.clone(),
            AuditAction::UserVerified,
            Some(user.user_id),
            None,
            None,
        )
        .await;

        Ok(updated_user.into())
    }

    pub async fn change_password(
        &self,
        user_id: &str,
        current_password: &str,
        new_password: &str,
        geo: Option<GeoLocation>,
        user_agent: Option<String>,
    ) -> AppResult<()> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Verify current password with Keycloak
        self.keycloak
            .authenticate(&user.email, current_password)
            .await?;

        // Change password in Keycloak
        self.keycloak
            .change_password(&user.keycloak_id, new_password)
            .await?;

        // Create audit log
        self.log_audit(
            user_id.to_string(),
            AuditAction::UserPasswordChanged,
            None,
            geo,
            user_agent,
        )
        .await;

        Ok(())
    }

    pub async fn request_password_reset(&self, email: &str) -> AppResult<()> {
        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Send password reset email via Keycloak
        self.keycloak
            .send_password_reset_email(&user.keycloak_id)
            .await?;

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

#[cfg(test)]
mod tests {
    use super::*;

    // Tests would go here with mocked dependencies
    // using mockall for the repository traits
}