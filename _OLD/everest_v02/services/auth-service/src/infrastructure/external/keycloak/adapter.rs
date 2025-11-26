use crate::domain::value_objects::{Email, Password};
use crate::domain::entities::User;
use super::client::{KeycloakClient, KeycloakError, UserTokens, UserInfo};

#[derive(Debug, Clone)]
pub struct KeycloakAdapter {
    client: KeycloakClient,
}

impl KeycloakAdapter {
    pub fn new(client: KeycloakClient) -> Self {
        Self { client }
    }

    /// Register a new user in Keycloak
    pub async fn register_user(
        &mut self,
        email: &Email,
        password: &Password,
        first_name: &str,
        last_name: &str,
        username: Option<&str>,
    ) -> Result<String, KeycloakError> {
        let username = username.unwrap_or_else(|| {
            email.as_str().split('@').next().unwrap_or("user")
        });

        let keycloak_user_id = self.client.create_user(
            email.as_str(),
            username,
            first_name,
            last_name,
            password.as_str(),
        ).await?;

        Ok(keycloak_user_id)
    }

    /// Authenticate user with Keycloak
    pub async fn authenticate(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<UserTokens, KeycloakError> {
        self.client.authenticate_user(email.as_str(), password.as_str()).await
    }

    /// Refresh tokens
    pub async fn refresh_tokens(&self, refresh_token: &str) -> Result<UserTokens, KeycloakError> {
        self.client.refresh_token(refresh_token).await
    }

    /// Logout user
    pub async fn logout(&self, refresh_token: &str) -> Result<(), KeycloakError> {
        self.client.logout(refresh_token).await
    }

    /// Get user info from access token
    pub async fn get_user_info(&self, access_token: &str) -> Result<UserInfo, KeycloakError> {
        self.client.get_user_info(access_token).await
    }

    /// Verify token is active
    pub async fn verify_token(&self, token: &str) -> Result<bool, KeycloakError> {
        let introspection = self.client.introspect_token(token).await?;
        Ok(introspection.active)
    }

    /// Sync user from Keycloak to our domain
    pub async fn sync_user_from_keycloak(
        &self,
        access_token: &str,
    ) -> Result<User, KeycloakError> {
        let user_info = self.get_user_info(access_token).await?;

        // Create domain user from Keycloak user info
        let email = Email::new(&user_info.email)
            .map_err(|e| KeycloakError::UserOperationFailed(e.to_string()))?;

        // Create a temporary password - in real implementation, you'd get this from your DB
        let temp_password = Password::new("temporary").unwrap();

        let mut user = User::new(
            email,
            temp_password,
            user_info.given_name,
            user_info.family_name,
            Some(user_info.preferred_username),
        );

        if user_info.email_verified {
            user.verify_email();
        }

        Ok(user)
    }

    /// Get user by Keycloak user ID
    pub async fn get_user_by_id(
        &mut self,
        keycloak_user_id: &str,
    ) -> Result<User, KeycloakError> {
        let keycloak_user = self.client.get_user_by_id(keycloak_user_id).await?;

        let email = Email::new(&keycloak_user.email)
            .map_err(|e| KeycloakError::UserOperationFailed(e.to_string()))?;

        let temp_password = Password::new("temporary").unwrap();

        let mut user = User::new(
            email,
            temp_password,
            keycloak_user.firstName,
            keycloak_user.lastName,
            Some(keycloak_user.username),
        );

        if keycloak_user.emailVerified {
            user.verify_email();
        }

        if !keycloak_user.enabled {
            user.deactivate();
        }

        Ok(user)
    }
}