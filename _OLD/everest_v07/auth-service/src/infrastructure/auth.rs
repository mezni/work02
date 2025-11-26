use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::infrastructure::config::KeycloakConfig;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Debug, Clone)]
pub struct KeycloakClient {
    config: KeycloakConfig,
    http_client: Client,
}

impl KeycloakClient {
    pub fn new(config: KeycloakConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
        }
    }
    
    pub async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<String, InfrastructureError> {
        let admin_token = self.get_admin_token().await?;
        
        let user_data = serde_json::json!({
            "username": username,
            "email": email,
            "enabled": true,
            "emailVerified": false,
            "credentials": [{
                "type": "password",
                "value": password,
                "temporary": false
            }]
        });
        
        let response = self.http_client
            .post(&self.config.admin_users_url())
            .header("Authorization", format!("Bearer {}", admin_token))
            .header("Content-Type", "application/json")
            .json(&user_data)
            .send()
            .await?;
        
        if response.status().is_success() {
            // Extract user ID from location header
            if let Some(location) = response.headers().get("Location") {
                if let Ok(location_str) = location.to_str() {
                    if let Some(user_id) = location_str.split('/').last() {
                        return Ok(user_id.to_string());
                    }
                }
            }
            Err(InfrastructureError::KeycloakError("Failed to extract user ID from response".to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("Keycloak API error: {}", error_text)))
        }
    }
    
    pub async fn login(&self, username: &str, password: &str) -> Result<KeycloakTokenResponse, InfrastructureError> {
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "password");
        form_data.insert("client_id", &self.config.client_id);
        form_data.insert("client_secret", &self.config.client_secret);
        form_data.insert("username", username);
        form_data.insert("password", password);
        
        let response = self.http_client
            .post(&self.config.token_url())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await?;
        
        if response.status().is_success() {
            let token_response: KeycloakTokenResponse = response.json().await?;
            Ok(token_response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("Login failed: {}", error_text)))
        }
    }
    
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<KeycloakTokenResponse, InfrastructureError> {
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "refresh_token");
        form_data.insert("client_id", &self.config.client_id);
        form_data.insert("client_secret", &self.config.client_secret);
        form_data.insert("refresh_token", refresh_token);
        
        let response = self.http_client
            .post(&self.config.token_url())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await?;
        
        if response.status().is_success() {
            let token_response: KeycloakTokenResponse = response.json().await?;
            Ok(token_response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("Token refresh failed: {}", error_text)))
        }
    }
    
    pub async fn user_info(&self, access_token: &str) -> Result<KeycloakUserInfo, InfrastructureError> {
        let response = self.http_client
            .get(&self.config.user_info_url())
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;
        
        if response.status().is_success() {
            let user_info: KeycloakUserInfo = response.json().await?;
            Ok(user_info)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("User info request failed: {}", error_text)))
        }
    }
    
    pub async fn update_user(&self, user_id: &str, attributes: HashMap<String, String>) -> Result<(), InfrastructureError> {
        let admin_token = self.get_admin_token().await?;
        
        let user_data = serde_json::json!({
            "attributes": attributes
        });
        
        let response = self.http_client
            .put(&format!("{}/{}", self.config.admin_users_url(), user_id))
            .header("Authorization", format!("Bearer {}", admin_token))
            .header("Content-Type", "application/json")
            .json(&user_data)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("User update failed: {}", error_text)))
        }
    }
    
    pub async fn reset_password(&self, user_id: &str, new_password: &str) -> Result<(), InfrastructureError> {
        let admin_token = self.get_admin_token().await?;
        
        let password_data = serde_json::json!({
            "type": "password",
            "value": new_password,
            "temporary": false
        });
        
        let response = self.http_client
            .put(&format!("{}/{}/reset-password", self.config.admin_users_url(), user_id))
            .header("Authorization", format!("Bearer {}", admin_token))
            .header("Content-Type", "application/json")
            .json(&password_data)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("Password reset failed: {}", error_text)))
        }
    }
    
    async fn get_admin_token(&self) -> Result<String, InfrastructureError> {
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "password");
        form_data.insert("client_id", &self.config.client_id);
        form_data.insert("client_secret", &self.config.client_secret);
        form_data.insert("username", &self.config.admin_username);
        form_data.insert("password", &self.config.admin_password);
        
        let response = self.http_client
            .post(&self.config.admin_token_url())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await?;
        
        if response.status().is_success() {
            let token_response: KeycloakTokenResponse = response.json().await?;
            Ok(token_response.access_token)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("Admin token request failed: {}", error_text)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakUserInfo {
    pub sub: String,
    pub email: String,
    pub preferred_username: String,
    pub email_verified: bool,
    pub exp: i64,
    pub iat: i64,
}
