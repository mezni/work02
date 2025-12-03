use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::config::AppConfig;
use super::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone)]
pub struct KeycloakConfig {
    pub server_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

impl From<&AppConfig> for KeycloakConfig {
    fn from(config: &AppConfig) -> Self {
        Self {
            server_url: config.keycloak.server_url.clone(),
            realm: config.keycloak.realm.clone(),
            client_id: config.keycloak.client_id.clone(),
            client_secret: config.keycloak.client_secret.clone(),
            admin_username: config.keycloak.admin_username.clone(),
            admin_password: config.keycloak.admin_password.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakUserRepresentation {
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub enabled: bool,
    pub email_verified: bool,
    pub attributes: Option<std::collections::HashMap<String, Vec<String>>>,
    pub realm_roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakCredentialRepresentation {
    #[serde(rename = "type")]
    pub cred_type: String,
    pub value: String,
    pub temporary: bool,
}

#[derive(Debug, Serialize)]
struct KeycloakTokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    username: Option<String>,
    password: Option<String>,
    refresh_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KeycloakClient {
    config: KeycloakConfig,
    http_client: Client,
    admin_token: Arc<RwLock<Option<AdminToken>>>,
}

#[derive(Debug, Clone)]
struct AdminToken {
    token: String,
    expires_at: DateTime<Utc>,
}

impl KeycloakClient {
    pub fn new(config: KeycloakConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
            admin_token: Arc::new(RwLock::new(None)),
        }
    }
    
    pub async fn authenticate_user(&self, username: &str, password: &str) -> InfrastructureResult<KeycloakTokenResponse> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.server_url, self.config.realm
        );
        
        let request = KeycloakTokenRequest {
            grant_type: "password".to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            username: Some(username.to_string()),
            password: Some(password.to_string()),
            refresh_token: None,
        };
        
        let response = self.http_client
            .post(&token_url)
            .form(&request)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakConnection(e.to_string()))?;
        
        if response.status() != StatusCode::OK {
            let error_text = response.text().await.unwrap_or_default();
            return Err(InfrastructureError::KeycloakApi(format!(
                "Authentication failed: {}",
                error_text
            )));
        }
        
        let token_response: KeycloakTokenResponse = response.json().await?;
        Ok(token_response)
    }
    
    pub async fn refresh_token(&self, refresh_token: &str) -> InfrastructureResult<KeycloakTokenResponse> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.server_url, self.config.realm
        );
        
        let request = KeycloakTokenRequest {
            grant_type: "refresh_token".to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            username: None,
            password: None,
            refresh_token: Some(refresh_token.to_string()),
        };
        
        let response = self.http_client
            .post(&token_url)
            .form(&request)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakConnection(e.to_string()))?;
        
        if response.status() != StatusCode::OK {
            return Err(InfrastructureError::KeycloakApi(
                "Token refresh failed".to_string()
            ));
        }
        
        let token_response: KeycloakTokenResponse = response.json().await?;
        Ok(token_response)
    }
    
    pub async fn create_user(&self, user: &KeycloakUserRepresentation, password: &str) -> InfrastructureResult<String> {
        let admin_token = self.get_admin_token().await?;
        let users_url = format!(
            "{}/admin/realms/{}/users",
            self.config.server_url, self.config.realm
        );
        
        let mut user_data = serde_json::to_value(user)?;
        if let Some(obj) = user_data.as_object_mut() {
            obj.insert(
                "credentials".to_string(),
                serde_json::json!([{
                    "type": "password",
                    "value": password,
                    "temporary": false
                }])
            );
        }
        
        let response = self.http_client
            .post(&users_url)
            .bearer_auth(&admin_token.token)
            .json(&user_data)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakConnection(e.to_string()))?;
        
        if response.status() != StatusCode::CREATED {
            let error_text = response.text().await.unwrap_or_default();
            return Err(InfrastructureError::KeycloakApi(format!(
                "User creation failed: {}",
                error_text
            )));
        }
        
        // Extract user ID from Location header
        if let Some(location) = response.headers().get("Location") {
            let location_str = location.to_str().unwrap_or("");
            let user_id = location_str
                .split('/')
                .last()
                .unwrap_or("")
                .to_string();
            
            if !user_id.is_empty() {
                Ok(user_id)
            } else {
                Err(InfrastructureError::KeycloakApi(
                    "Failed to extract user ID".to_string()
                ))
            }
        } else {
            Err(InfrastructureError::KeycloakApi(
                "No Location header in response".to_string()
            ))
        }
    }
    
    pub async fn get_user_by_id(&self, user_id: &str) -> InfrastructureResult<KeycloakUserRepresentation> {
        let admin_token = self.get_admin_token().await?;
        let user_url = format!(
            "{}/admin/realms/{}/users/{}",
            self.config.server_url, self.config.realm, user_id
        );
        
        let response = self.http_client
            .get(&user_url)
            .bearer_auth(&admin_token.token)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakConnection(e.to_string()))?;
        
        if response.status() != StatusCode::OK {
            return Err(InfrastructureError::KeycloakApi(
                format!("User not found: {}", user_id)
            ));
        }
        
        let user: KeycloakUserRepresentation = response.json().await?;
        Ok(user)
    }
    
    pub async fn get_user_by_email(&self, email: &str) -> InfrastructureResult<KeycloakUserRepresentation> {
        let admin_token = self.get_admin_token().await?;
        let users_url = format!(
            "{}/admin/realms/{}/users?email={}&exact=true",
            self.config.server_url, self.config.realm, email
        );
        
        let response = self.http_client
            .get(&users_url)
            .bearer_auth(&admin_token.token)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakConnection(e.to_string()))?;
        
        if response.status() != StatusCode::OK {
            return Err(InfrastructureError::KeycloakApi(
                "Failed to fetch user".to_string()
            ));
        }
        
        let users: Vec<KeycloakUserRepresentation> = response.json().await?;
        
        users.into_iter().next()
            .ok_or_else(|| InfrastructureError::KeycloakApi(
                format!("User with email {} not found", email)
            ))
    }
    
    pub async fn update_user(&self, user_id: &str, user: &KeycloakUserRepresentation) -> InfrastructureResult<()> {
        let admin_token = self.get_admin_token().await?;
        let user_url = format!(
            "{}/admin/realms/{}/users/{}",
            self.config.server_url, self.config.realm, user_id
        );
        
        let response = self.http_client
            .put(&user_url)
            .bearer_auth(&admin_token.token)
            .json(user)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakConnection(e.to_string()))?;
        
        if response.status() != StatusCode::NO_CONTENT {
            let error_text = response.text().await.unwrap_or_default();
            return Err(InfrastructureError::KeycloakApi(format!(
                "User update failed: {}",
                error_text
            )));
        }
        
        Ok(())
    }
    
    pub async fn logout_user(&self, refresh_token: &str) -> InfrastructureResult<()> {
        let logout_url = format!(
            "{}/realms/{}/protocol/openid-connect/logout",
            self.config.server_url, self.config.realm
        );
        
        let request = KeycloakTokenRequest {
            grant_type: "refresh_token".to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            username: None,
            password: None,
            refresh_token: Some(refresh_token.to_string()),
        };
        
        let response = self.http_client
            .post(&logout_url)
            .form(&request)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakConnection(e.to_string()))?;
        
        if response.status() != StatusCode::NO_CONTENT && response.status() != StatusCode::OK {
            return Err(InfrastructureError::KeycloakApi(
                "Logout failed".to_string()
            ));
        }
        
        Ok(())
    }
    
    async fn get_admin_token(&self) -> InfrastructureResult<AdminToken> {
        let mut token_lock = self.admin_token.write().await;
        
        if let Some(token) = &*token_lock {
            if Utc::now() < token.expires_at {
                return Ok(token.clone());
            }
        }
        
        let new_token = self.authenticate_admin().await?;
        *token_lock = Some(new_token.clone());
        
        Ok(new_token)
    }
    
    async fn authenticate_admin(&self) -> InfrastructureResult<AdminToken> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.server_url, self.config.realm
        );
        
        let request = KeycloakTokenRequest {
            grant_type: "password".to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            username: Some(self.config.admin_username.clone()),
            password: Some(self.config.admin_password.clone()),
            refresh_token: None,
        };
        
        let response = self.http_client
            .post(&token_url)
            .form(&request)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakConnection(e.to_string()))?;
        
        if response.status() != StatusCode::OK {
            return Err(InfrastructureError::KeycloakApi(
                "Admin authentication failed".to_string()
            ));
        }
        
        let token_response: KeycloakTokenResponse = response.json().await?;
        let expires_at = Utc::now() + chrono::Duration::seconds(token_response.expires_in);
        
        Ok(AdminToken {
            token: token_response.access_token,
            expires_at,
        })
    }
    
    pub async fn health_check(&self) -> InfrastructureResult<()> {
        let realm_url = format!(
            "{}/realms/{}",
            self.config.server_url, self.config.realm
        );
        
        let response = self.http_client
            .get(&realm_url)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakConnection(e.to_string()))?;
        
        if response.status() != StatusCode::OK {
            return Err(InfrastructureError::HealthCheckFailed(
                "Keycloak realm not accessible".to_string()
            ));
        }
        
        Ok(())
    }
}