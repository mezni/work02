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
