// src/infrastructure/keycloak_user_repository.rs
use crate::domain::user::User;
use crate::domain::user_repository::{UserRepository, Error};
use crate::infrastructure::keycloak_config::KeycloakConfig;
use reqwest::Client;
use serde_json::json;

pub struct KeycloakUserRepository {
    config: KeycloakConfig,
    client: Client,
}

impl KeycloakUserRepository {
    pub fn new(config: KeycloakConfig) -> Self {
        KeycloakUserRepository {
            config,
            client: Client::new(),
        }
    }

    async fn get_token(&self) -> Result<String, Error> {
        let token_url = format!("{}/realms/{}/protocol/openid-connect/token", self.config.auth_server_url, self.config.realm);
        let response = self.client.post(&token_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("grant_type=client_credentials&client_id={}&client_secret={}", self.config.client_id, self.config.client_secret))
            .send()
            .await
            .map_err(|_| Error::InternalError)?;

        let json: serde_json::Value = response.json().await.map_err(|_| Error::InternalError)?;
        let token = json["access_token"].as_str().ok_or(Error::InternalError)?;

        Ok(token.to_string())
    }
}

impl UserRepository for KeycloakUserRepository {
    async fn find_by_id(&self, id: &str) -> Option<User> {
        let token = self.get_token().await.ok()?;
        let user_url = format!("{}/admin/realms/{}/users/{}", self.config.auth_server_url, self.config.realm, id);
        let response = self.client.get(&user_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .ok()?;

        if response.status().is_success() {
            let user: User = response.json().await.ok()?;
            Some(user)
        } else {
            None
        }
    }

    async fn save(&self, user: User) -> Result<(), Error> {
        let token = self.get_token().await?;
        let user_url = format!("{}/admin/realms/{}/users", self.config.auth_server_url, self.config.realm);
        let user_json = json!({
            "username": user.username,
            "email": user.email,
            "enabled": true,
            "attributes": {
                "role": [user.role],
                "company_name": [user.company_name],
                "station_name": [user.station_name],
            }
        });

        let response = self.client.post(&user_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .body(user_json.to_string())
            .send()
            .await
            .map_err(|_| Error::InternalError)?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::InternalError)
        }
    }
}