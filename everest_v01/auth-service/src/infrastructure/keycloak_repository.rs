use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use crate::domain::{
    user::User,
    token::{Token, TokenClaims, RefreshToken},
    registration::Registration,
    repository::{UserRepository, TokenRepository, RegistrationRepository, AuthRepository},
    error::{DomainError, DomainResult},
    value_objects::UserRole,
};
use super::{
    keycloak_client::{KeycloakClient, KeycloakUserRepresentation, KeycloakTokenResponse},
    error::{InfrastructureError, InfrastructureResult},
    token_generator::JwtTokenGenerator,
};

pub struct KeycloakUserRepository {
    client: Arc<KeycloakClient>,
    token_generator: Arc<JwtTokenGenerator>,
}

impl KeycloakUserRepository {
    pub fn new(client: Arc<KeycloakClient>, token_generator: Arc<JwtTokenGenerator>) -> Self {
        Self {
            client,
            token_generator,
        }
    }
    
    fn to_domain_user(&self, kc_user: &KeycloakUserRepresentation) -> DomainResult<User> {
        let id = kc_user.id.as_ref()
            .ok_or_else(|| DomainError::UserNotFound)?
            .parse::<Uuid>()
            .map_err(|_| DomainError::InvalidToken("Invalid user ID".to_string()))?;
        
        let role = kc_user.realm_roles.first()
            .map(|r| UserRole::from_str(r))
            .unwrap_or(Ok(UserRole::User))?;
        
        let attributes = kc_user.attributes.as_ref();
        let company_name = attributes
            .and_then(|attr| attr.get("company_name"))
            .and_then(|v| v.first())
            .cloned()
            .unwrap_or_default();
        
        let station_name = attributes
            .and_then(|attr| attr.get("station_name"))
            .and_then(|v| v.first())
            .cloned()
            .unwrap_or_default();
        
        Ok(User {
            id,
            email: kc_user.email.clone(),
            role,
            company_name,
            station_name,
            is_active: kc_user.enabled,
            email_verified: kc_user.email_verified,
            created_at: chrono::Utc::now(), // Keycloak doesn't provide this
            updated_at: chrono::Utc::now(),
        })
    }
    
    fn to_keycloak_user(&self, user: &User) -> KeycloakUserRepresentation {
        let mut attributes = std::collections::HashMap::new();
        if !user.company_name.is_empty() {
            attributes.insert("company_name".to_string(), vec![user.company_name.clone()]);
        }
        if !user.station_name.is_empty() {
            attributes.insert("station_name".to_string(), vec![user.station_name.clone()]);
        }
        
        KeycloakUserRepresentation {
            id: Some(user.id.to_string()),
            username: user.email.clone(),
            email: user.email.clone(),
            enabled: user.is_active,
            email_verified: user.email_verified,
            attributes: Some(attributes),
            realm_roles: vec![user.role.as_str().to_string()],
        }
    }
}

#[async_trait]
impl UserRepository for KeycloakUserRepository {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<User> {
        let kc_user = self.client.get_user_by_id(&id.to_string()).await
            .map_err(|e| match e {
                InfrastructureError::KeycloakApi(msg) if msg.contains("not found") => {
                    DomainError::UserNotFound
                }
                _ => DomainError::Validation(e.to_string()),
            })?;
        
        self.to_domain_user(&kc_user)
    }
    
    async fn find_by_email(&self, email: &str) -> DomainResult<User> {
        let kc_user = self.client.get_user_by_email(email).await
            .map_err(|e| match e {
                InfrastructureError::KeycloakApi(msg) if msg.contains("not found") => {
                    DomainError::UserNotFound
                }
                _ => DomainError::Validation(e.to_string()),
            })?;
        
        self.to_domain_user(&kc_user)
    }
    
    async fn create(&self, user: &User, password: &str) -> DomainResult<Uuid> {
        let kc_user = self.to_keycloak_user(user);
        let user_id = self.client.create_user(&kc_user, password).await
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        
        Uuid::parse_str(&user_id)
            .map_err(|_| DomainError::Validation("Invalid user ID returned".to_string()))
    }
    
    async fn update(&self, user: &User) -> DomainResult<()> {
        let kc_user = self.to_keycloak_user(user);
        self.client.update_user(&user.id.to_string(), &kc_user).await
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        
        Ok(())
    }
    
    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        // Note: Keycloak client doesn't have delete method in our example
        // You would need to implement it
        Err(DomainError::Validation("Delete not implemented".to_string()))
    }
    
    async fn exists_by_email(&self, email: &str) -> DomainResult<bool> {
        match self.client.get_user_by_email(email).await {
            Ok(_) => Ok(true),
            Err(InfrastructureError::KeycloakApi(msg)) if msg.contains("not found") => Ok(false),
            Err(e) => Err(DomainError::Validation(e.to_string())),
        }
    }
}

#[async_trait]
impl TokenRepository for KeycloakUserRepository {
    async fn generate_access_token(&self, claims: &TokenClaims) -> DomainResult<Token> {
        let token_string = self.token_generator.generate(claims)
            .map_err(|e| DomainError::InvalidToken(e.to_string()))?;
        
        // For refresh token, we'll use Keycloak's refresh token
        // In a real implementation, you might want to store refresh tokens
        let refresh_token = Uuid::new_v4().to_string();
        
        Ok(Token::new(
            token_string,
            refresh_token,
            (claims.exp - claims.iat) as i64,
        ))
    }
    
    async fn validate_access_token(&self, token: &str) -> DomainResult<TokenClaims> {
        self.token_generator.validate(token)
            .map_err(|e| DomainError::InvalidToken(e.to_string()))
    }
    
    async fn generate_refresh_token(&self, user_id: Uuid) -> DomainResult<RefreshToken> {
        // In Keycloak, refresh tokens are issued by Keycloak itself
        // We'll create a simple refresh token for demonstration
        let token = Uuid::new_v4().to_string();
        Ok(RefreshToken::new(
            user_id,
            token,
            30, // 30 days
        ))
    }
    
    async fn validate_refresh_token(&self, token: &str) -> DomainResult<RefreshToken> {
        // Validate refresh token format
        Uuid::parse_str(token)
            .map_err(|_| DomainError::InvalidToken("Invalid refresh token".to_string()))?;
        
        // In real implementation, you would check against stored refresh tokens
        // For now, return a dummy token
        Ok(RefreshToken::new(
            Uuid::new_v4(),
            token.to_string(),
            30,
        ))
    }
    
    async fn revoke_refresh_token(&self, token: &str) -> DomainResult<()> {
        // In Keycloak, you would call logout endpoint
        self.client.logout_user(token).await
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        
        Ok(())
    }
    
    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> DomainResult<()> {
        // Keycloak doesn't have a direct endpoint for this
        // You would need to implement token revocation logic
        Ok(())
    }
}

#[async_trait]
impl RegistrationRepository for KeycloakUserRepository {
    async fn register_user(&self, registration: &Registration, password: &str) -> DomainResult<Uuid> {
        let kc_user = registration.to_keycloak_user();
        let user_id = self.client.create_user(&kc_user, password).await
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        
        Uuid::parse_str(&user_id)
            .map_err(|_| DomainError::Validation("Invalid user ID returned".to_string()))
    }
    
    async fn verify_email(&self, token: &str) -> DomainResult<()> {
        // Email verification would be handled by Keycloak
        // This is a placeholder implementation
        Ok(())
    }
    
    async fn resend_verification(&self, email: &str) -> DomainResult<()> {
        // This would trigger Keycloak's email verification
        Ok(())
    }
}

#[async_trait]
impl AuthRepository for KeycloakUserRepository {
    async fn authenticate(&self, email: &str, password: &str) -> DomainResult<User> {
        // First authenticate with Keycloak
        let token_response = self.client.authenticate_user(email, password).await
            .map_err(|e| match e {
                InfrastructureError::KeycloakApi(_) => DomainError::Validation("Invalid credentials".to_string()),
                _ => DomainError::Validation(e.to_string()),
            })?;
        
        // Then get user details
        let user = self.find_by_email(email).await?;
        
        // Store refresh token if needed
        // In real implementation, you might want to store the refresh token
        
        Ok(user)
    }
    
    async fn change_password(&self, user_id: Uuid, old_password: &str, new_password: &str) -> DomainResult<()> {
        // Keycloak has password change endpoints
        // This is a placeholder implementation
        Ok(())
    }
    
    async fn reset_password(&self, email: &str) -> DomainResult<()> {
        // Trigger Keycloak's password reset
        Ok(())
    }
    
    async fn confirm_password_reset(&self, token: &str, new_password: &str) -> DomainResult<()> {
        // Complete password reset in Keycloak
        Ok(())
    }
}