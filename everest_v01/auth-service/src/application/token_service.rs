use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    User,
    Token,
    repository::{UserRepository, TokenRepository},
};
use crate::application::error::{ApplicationError, ApplicationResult};
use super::service_traits::{TokenServiceTrait, UserServiceTrait};

pub struct TokenService {
    user_repository: Arc<dyn UserRepository>,
    token_repository: Arc<dyn TokenRepository>,
}

impl TokenService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        token_repository: Arc<dyn TokenRepository>,
    ) -> Self {
        Self {
            user_repository,
            token_repository,
        }
    }
}

#[async_trait]
impl TokenServiceTrait for TokenService {
    async fn refresh_token(&self, refresh_token: &str) -> ApplicationResult<Token> {
        if refresh_token.is_empty() {
            return Err(ApplicationError::InvalidToken("Refresh token cannot be empty".to_string()));
        }
        
        // Validate refresh token
        let refresh_token_data = self.token_repository.validate_refresh_token(refresh_token).await
            .map_err(|e| ApplicationError::InvalidToken(e.to_string()))?;
        
        // Check if refresh token is expired
        if refresh_token_data.is_expired() {
            return Err(ApplicationError::TokenExpired);
        }
        
        // Get user
        let user = self.user_repository.find_by_id(refresh_token_data.user_id).await
            .map_err(|e| match e {
                crate::domain::error::DomainError::UserNotFound => ApplicationError::UserNotFound,
                _ => ApplicationError::Validation(e.to_string()),
            })?;
        
        // Check if user is active
        if !user.is_active {
            return Err(ApplicationError::AccountDisabled);
        }
        
        // Generate new access token
        let claims = crate::infrastructure::token_generator::TokenClaims::new(
            user.id,
            user.email.clone(),
            user.role,
            user.company_name.clone(),
            user.station_name.clone(),
            user.is_active,
            user.email_verified,
            "auth-service".to_string(),
            "auth-service".to_string(),
            24, // 24 hours expiration
        );
        
        let token = self.token_repository.generate_access_token(&claims).await
            .map_err(|e| ApplicationError::TokenGenerationFailed(e.to_string()))?;
        
        // Generate new refresh token (optional - implement refresh token rotation)
        // let new_refresh_token = self.token_repository.generate_refresh_token(user.id).await?;
        
        // Revoke old refresh token (if implementing rotation)
        // self.token_repository.revoke_refresh_token(refresh_token).await?;
        
        Ok(token)
    }
    
    async fn validate_token(&self, token: &str) -> ApplicationResult<User> {
        if token.is_empty() {
            return Err(ApplicationError::InvalidToken("Token cannot be empty".to_string()));
        }
        
        // Validate token
        let claims = self.token_repository.validate_access_token(token).await
            .map_err(|e| ApplicationError::InvalidToken(e.to_string()))?;
        
        // Check if token is expired
        let now = chrono::Utc::now().timestamp();
        if now >= claims.exp {
            return Err(ApplicationError::TokenExpired);
        }
        
        // Get user ID from claims
        let user_id = claims.user_id()
            .map_err(|_| ApplicationError::InvalidToken("Invalid user ID in token".to_string()))?;
        
        // Get user from repository
        let user = self.user_repository.find_by_id(user_id).await
            .map_err(|e| match e {
                crate::domain::error::DomainError::UserNotFound => ApplicationError::UserNotFound,
                _ => ApplicationError::Validation(e.to_string()),
            })?;
        
        // Verify user is still active
        if !user.is_active {
            return Err(ApplicationError::AccountDisabled);
        }
        
        // Verify email is still verified (if required)
        if !user.email_verified {
            // You might want to allow access without verification in some cases
            // return Err(ApplicationError::EmailNotVerified);
        }
        
        // Verify that token claims match current user state
        // This prevents using old tokens after role changes
        if user.email != claims.email ||
           user.role.as_str() != claims.role ||
           user.company_name != claims.company_name ||
           user.station_name != claims.station_name ||
           user.is_active != claims.is_active ||
           user.email_verified != claims.email_verified {
            
            // Token claims don't match current user state
            // You might want to force re-authentication
            return Err(ApplicationError::InvalidToken("Token claims do not match current user state".to_string()));
        }
        
        Ok(user)
    }
    
    async fn revoke_token(&self, token: &str) -> ApplicationResult<()> {
        // This would typically revoke a refresh token
        self.token_repository.revoke_refresh_token(token).await
            .map_err(|e| ApplicationError::InvalidToken(e.to_string()))?;
        
        Ok(())
    }
    
    async fn revoke_all_tokens(&self, user_id: Uuid) -> ApplicationResult<()> {
        self.token_repository.revoke_all_user_tokens(user_id).await
            .map_err(|e| ApplicationError::InvalidToken(e.to_string()))?;
        
        Ok(())
    }
}

pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get_user(&self, user_id: Uuid) -> ApplicationResult<User> {
        self.user_repository.find_by_id(user_id).await
            .map_err(|e| match e {
                crate::domain::error::DomainError::UserNotFound => ApplicationError::UserNotFound,
                _ => ApplicationError::Validation(e.to_string()),
            })
    }
    
    async fn update_user_profile(&self, user_id: Uuid, company_name: Option<String>, station_name: Option<String>) -> ApplicationResult<User> {
        let mut user = self.user_repository.find_by_id(user_id).await
            .map_err(|e| match e {
                crate::domain::error::DomainError::UserNotFound => ApplicationError::UserNotFound,
                _ => ApplicationError::Validation(e.to_string()),
            })?;
        
        // Update profile
        user.update_profile(company_name, station_name);
        
        // Save updated user
        self.user_repository.update(&user).await
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        Ok(user)
    }
    
    async fn update_user_role(&self, user_id: Uuid, role: crate::domain::value_objects::UserRole) -> ApplicationResult<User> {
        let mut user = self.user_repository.find_by_id(user_id).await
            .map_err(|e| match e {
                crate::domain::error::DomainError::UserNotFound => ApplicationError::UserNotFound,
                _ => ApplicationError::Validation(e.to_string()),
            })?;
        
        // Check permissions (in real app, you'd have authorization logic)
        // For now, just update the role
        
        user.update_role(role);
        
        // Save updated user
        self.user_repository.update(&user).await
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        Ok(user)
    }
}