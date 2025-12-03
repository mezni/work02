use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    User,
    Token,
    repository::{UserRepository, AuthRepository, TokenRepository},
    value_objects::UserRole,
};
use crate::application::error::{ApplicationError, ApplicationResult};
use super::service_traits::AuthServiceTrait;

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    auth_repository: Arc<dyn AuthRepository>,
    token_repository: Arc<dyn TokenRepository>,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        auth_repository: Arc<dyn AuthRepository>,
        token_repository: Arc<dyn TokenRepository>,
    ) -> Self {
        Self {
            user_repository,
            auth_repository,
            token_repository,
        }
    }
}

#[async_trait]
impl AuthServiceTrait for AuthService {
    async fn login(&self, email: &str, password: &str) -> ApplicationResult<Token> {
        // Validate input
        if email.is_empty() || password.is_empty() {
            return Err(ApplicationError::InvalidCredentials);
        }
        
        // Rate limiting check (simplified)
        // In production, you would use a proper rate limiter
        
        // Authenticate user
        let user = self.auth_repository.authenticate(email, password).await
            .map_err(|e| match e {
                crate::domain::error::DomainError::UserNotFound => ApplicationError::InvalidCredentials,
                _ => ApplicationError::AuthenticationFailed(e.to_string()),
            })?;
        
        // Check if user is active
        if !user.is_active {
            return Err(ApplicationError::AccountDisabled);
        }
        
        // Check if email is verified (if required)
        if !user.email_verified {
            // You might want to allow login without verification in some cases
            // return Err(ApplicationError::EmailNotVerified);
        }
        
        // Generate token claims
        let claims = crate::infrastructure::token_generator::TokenClaims::new(
            user.id,
            user.email.clone(),
            user.role,
            user.company_name.clone(),
            user.station_name.clone(),
            user.is_active,
            user.email_verified,
            "auth-service".to_string(), // issuer
            "auth-service".to_string(), // audience
            24, // 24 hours expiration
        );
        
        // Generate tokens
        let token = self.token_repository.generate_access_token(&claims).await
            .map_err(|e| ApplicationError::TokenGenerationFailed(e.to_string()))?;
        
        // Generate refresh token
        let _refresh_token = self.token_repository.generate_refresh_token(user.id).await
            .map_err(|e| ApplicationError::TokenGenerationFailed(e.to_string()))?;
        
        // In real implementation, store refresh token
        
        // Emit login event if you have event system
        
        Ok(token)
    }
    
    async fn logout(&self, refresh_token: &str) -> ApplicationResult<()> {
        // Revoke refresh token
        self.token_repository.revoke_refresh_token(refresh_token).await
            .map_err(|e| ApplicationError::InvalidToken(e.to_string()))?;
        
        // Invalidate any cached sessions
        
        Ok(())
    }
    
    async fn change_password(&self, user_id: Uuid, current_password: &str, new_password: &str) -> ApplicationResult<()> {
        // Get user to verify they exist
        let user = self.user_repository.find_by_id(user_id).await
            .map_err(|e| match e {
                crate::domain::error::DomainError::UserNotFound => ApplicationError::UserNotFound,
                _ => ApplicationError::Validation(e.to_string()),
            })?;
        
        // Validate new password
        if new_password.len() < 8 {
            return Err(ApplicationError::WeakPassword);
        }
        
        // Change password through auth repository
        self.auth_repository.change_password(user_id, current_password, new_password).await
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        // Revoke all tokens for security
        self.token_repository.revoke_all_user_tokens(user_id).await
            .map_err(|e| ApplicationError::InvalidToken(e.to_string()))?;
        
        // Emit password changed event
        
        Ok(())
    }
    
    async fn reset_password(&self, email: &str) -> ApplicationResult<()> {
        // Check if user exists
        let user_exists = self.user_repository.exists_by_email(email).await
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        if !user_exists {
            // Don't reveal that user doesn't exist for security
            return Ok(());
        }
        
        // Initiate password reset
        self.auth_repository.reset_password(email).await
            .map_err(|e| ApplicationError::ServiceUnavailable(e.to_string()))?;
        
        Ok(())
    }
    
    async fn confirm_password_reset(&self, token: &str, new_password: &str) -> ApplicationResult<()> {
        // Validate new password
        if new_password.len() < 8 {
            return Err(ApplicationError::WeakPassword);
        }
        
        // Confirm password reset
        self.auth_repository.confirm_password_reset(token, new_password).await
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
}