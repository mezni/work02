// src/infrastructure/keycloak/keycloak_client.rs
use async_trait::async_trait;
use std::sync::{Arc, RwLock};
use crate::infrastructure::keycloak::models::{KeycloakUser, CreateKeycloakUser, UpdateKeycloakUser, TokenResponse, KeycloakToken};
use crate::infrastructure::keycloak::error::KeycloakError;

#[async_trait]
pub trait KeycloakClient: Send + Sync {
    async fn get_admin_token(&self) -> Result<String, KeycloakError>;
    async fn create_user(&self, user: &CreateKeycloakUser) -> Result<String, KeycloakError>;
    async fn get_user_by_id(&self, user_id: &str) -> Result<Option<KeycloakUser>, KeycloakError>;
    async fn get_user_by_username(&self, username: &str) -> Result<Option<KeycloakUser>, KeycloakError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<KeycloakUser>, KeycloakError>;
    async fn update_user(&self, user_id: &str, user: &UpdateKeycloakUser) -> Result<(), KeycloakError>;
    async fn delete_user(&self, user_id: &str) -> Result<(), KeycloakError>;
    async fn list_users(&self, first: Option<u32>, max: Option<u32>) -> Result<Vec<KeycloakUser>, KeycloakError>;
}

pub struct KeycloakClientImpl {
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: String,
    admin_username: String,
    admin_password: String,
    http_client: reqwest::Client,
    admin_token: RwLock<Option<KeycloakToken>>,
}

impl KeycloakClientImpl {
    pub fn new(
        base_url: String,
        realm: String,
        client_id: String,
        client_secret: String,
        admin_username: String,
        admin_password: String,
    ) -> Self {
        Self {
            base_url,
            realm,
            client_id,
            client_secret,
            admin_username,
            admin_password,
            http_client: reqwest::Client::new(),
            admin_token: RwLock::new(None),
        }
    }

    fn get_token_url(&self) -> String {
        format!("{}/realms/{}/protocol/openid-connect/token", self.base_url, self.realm)
    }

    fn get_admin_users_url(&self) -> String {
        format!("{}/admin/realms/{}/users", self.base_url, self.realm)
    }

    fn get_admin_user_url(&self, user_id: &str) -> String {
        format!("{}/admin/realms/{}/users/{}", self.base_url, self.realm, user_id)
    }

    async fn refresh_admin_token_if_needed(&self) -> Result<String, KeycloakError> {
        {
            let token_guard = self.admin_token.read().unwrap();
            if let Some(token) = token_guard.as_ref() {
                if !token.is_expired() {
                    return Ok(token.access_token.clone());
                }
            }
        }

        self.fetch_new_admin_token().await
    }

    async fn fetch_new_admin_token(&self) -> Result<String, KeycloakError> {
        let params = [
            ("grant_type", "password"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("username", &self.admin_username),
            ("password", &self.admin_password),
        ];

        let response = self.http_client
            .post(&self.get_token_url())
            .form(&params)
            .send()
            .await
            .map_err(KeycloakError::Request)?;

        if !response.status().is_success() {
            return Err(KeycloakError::authentication("Failed to get admin token"));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(KeycloakError::Request)?;

        let token = KeycloakToken::new(
            token_response.access_token.clone(),
            token_response.refresh_token,
            token_response.expires_in,
        );

        {
            let mut token_guard = self.admin_token.write().unwrap();
            *token_guard = Some(token);
        }

        Ok(token_response.access_token)
    }
}

#[async_trait]
impl KeycloakClient for KeycloakClientImpl {
    async fn get_admin_token(&self) -> Result<String, KeycloakError> {
        self.refresh_admin_token_if_needed().await
    }

    async fn create_user(&self, user: &CreateKeycloakUser) -> Result<String, KeycloakError> {
        let token = self.get_admin_token().await?;

        let response = self.http_client
            .post(&self.get_admin_users_url())
            .bearer_auth(&token)
            .json(user)
            .send()
            .await
            .map_err(KeycloakError::Request)?;

        if response.status().is_success() {
            // Extract user ID from Location header
            if let Some(location) = response.headers().get("Location") {
                if let Ok(location_str) = location.to_str() {
                    if let Some(user_id) = location_str.split('/').last() {
                        return Ok(user_id.to_string());
                    }
                }
            }
            Err(KeycloakError::Unknown("Failed to extract user ID from response".to_string()))
        } else if response.status() == StatusCode::CONFLICT {
            Err(KeycloakError::UserAlreadyExists)
        } else {
            let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(KeycloakError::api(response.status(), error_message))
        }
    }

    async fn get_user_by_id(&self, user_id: &str) -> Result<Option<KeycloakUser>, KeycloakError> {
        let token = self.get_admin_token().await?;

        let response = self.http_client
            .get(&self.get_admin_user_url(user_id))
            .bearer_auth(&token)
            .send()
            .await
            .map_err(KeycloakError::Request)?;

        match response.status() {
            StatusCode::OK => {
                let user: KeycloakUser = response.json().await.map_err(KeycloakError::Request)?;
                Ok(Some(user))
            }
            StatusCode::NOT_FOUND => Ok(None),
            _ => {
                let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(KeycloakError::api(response.status(), error_message))
            }
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<KeycloakUser>, KeycloakError> {
        let users = self.list_users(Some(0), Some(1)).await?;
        Ok(users.into_iter().find(|u| u.username == username))
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<KeycloakUser>, KeycloakError> {
        let users = self.list_users(Some(0), Some(1)).await?;
        Ok(users.into_iter().find(|u| u.email == email))
    }

    async fn update_user(&self, user_id: &str, user: &UpdateKeycloakUser) -> Result<(), KeycloakError> {
        let token = self.get_admin_token().await?;

        let response = self.http_client
            .put(&self.get_admin_user_url(user_id))
            .bearer_auth(&token)
            .json(user)
            .send()
            .await
            .map_err(KeycloakError::Request)?;

        if response.status().is_success() {
            Ok(())
        } else if response.status() == StatusCode::NOT_FOUND {
            Err(KeycloakError::UserNotFound)
        } else {
            let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(KeycloakError::api(response.status(), error_message))
        }
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), KeycloakError> {
        let token = self.get_admin_token().await?;

        let response = self.http_client
            .delete(&self.get_admin_user_url(user_id))
            .bearer_auth(&token)
            .send()
            .await
            .map_err(KeycloakError::Request)?;

        if response.status().is_success() {
            Ok(())
        } else if response.status() == StatusCode::NOT_FOUND {
            Err(KeycloakError::UserNotFound)
        } else {
            let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(KeycloakError::api(response.status(), error_message))
        }
    }

    async fn list_users(&self, first: Option<u32>, max: Option<u32>) -> Result<Vec<KeycloakUser>, KeycloakError> {
        let token = self.get_admin_token().await?;

        let mut request = self.http_client
            .get(&self.get_admin_users_url())
            .bearer_auth(&token);

        if let Some(first) = first {
            request = request.query(&[("first", first)]);
        }
        if let Some(max) = max {
            request = request.query(&[("max", max)]);
        }

        let response = request
            .send()
            .await
            .map_err(KeycloakError::Request)?;

        if response.status().is_success() {
            let users: Vec<KeycloakUser> = response.json().await.map_err(KeycloakError::Request)?;
            Ok(users)
        } else {
            let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(KeycloakError::api(response.status(), error_message))
        }
    }
}