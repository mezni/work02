// src/infrastructure/keycloak_auth.rs
use crate::domain::user::User;
use crate::domain::errors::DomainError;
use crate::infrastructure::keycloak_config::KeycloakConfig;
use openid_client::{Client, Provider, Token};
use reqwest::Client as HttpClient;

pub struct KeycloakAuth {
    config: KeycloakConfig,
    client: Client,
}

impl KeycloakAuth {
    pub fn new(config: KeycloakConfig) -> Self {
        let provider = Provider::new(config.auth_server_url.clone(), config.realm.clone());
        let client = Client::new(provider, config.client_id.clone(), config.client_secret.clone());
        KeycloakAuth { config, client }
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Result<User, DomainError> {
        let token_url = self.client.token_endpoint();
        let token_response = HttpClient::new()
            .post(token_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("grant_type=password&username={}&password={}&client_id={}&client_secret={}", username, password, self.config.client_id, self.config.client_secret))
            .send()
            .await
            .map_err(|_| DomainError::InternalError)?;

        let token: Token = token_response.json().await.map_err(|_| DomainError::InternalError)?;
        let user = self.get_user_from_token(token).await?;
        Ok(user)
    }

    async fn get_user_from_token(&self, token: Token) -> Result<User, DomainError> {
        let user_url = format!("{}/realms/{}/protocol/openid-connect/userinfo", self.config.auth_server_url, self.config.realm);
        let response = HttpClient::new()
            .get(user_url)
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await
            .map_err(|_| DomainError::InternalError)?;

        let user: User = response.json().await.map_err(|_| DomainError::InternalError)?;
        Ok(user)
    }
}