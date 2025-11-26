use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use crate::infrastructure::config::Settings;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Debug, Serialize)]
pub struct KeycloakLoginRequest {
    pub username: String,
    pub password: String,
    pub grant_type: String,
    pub client_id: String,
}

#[derive(Debug, Deserialize)]
pub struct KeycloakTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Deserialize)]
pub struct KeycloakUserInfo {
    pub sub: String,
    pub email: String,
    pub preferred_username: String,
    pub email_verified: bool,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

pub struct KeycloakClient {
    client: Client,
    base_url: String,
    realm: String,
    client_id: String,
    admin_username: String,
    admin_password: String,
}

impl KeycloakClient {
    pub fn new(settings: &Settings) -> Self {
        Self {
            client: Client::new(),
            base_url: settings.keycloak.url.clone(),
            realm: settings.keycloak.realm_name.clone(), // Use correct field name
            client_id: settings.keycloak.client_name.clone(), // Use correct field name
            admin_username: settings.keycloak.admin.clone(), // Use correct field name
            admin_password: settings.keycloak.admin_password.clone(),
        }
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<KeycloakTokenResponse, InfrastructureError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let params = KeycloakLoginRequest {
            username: username.to_string(),
            password: password.to_string(),
            grant_type: "password".to_string(),
            client_id: self.client_id.clone(),
        };

        let response = self
            .client
            .post(&url)
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                error!("Keycloak login request failed: {}", e);
                InfrastructureError::Keycloak(format!("Login request failed: {}", e))
            })?;

        self.handle_response::<KeycloakTokenResponse>(response)
            .await
    }

    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<KeycloakTokenResponse, InfrastructureError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let mut params = std::collections::HashMap::new();
        params.insert("grant_type", "refresh_token");
        params.insert("client_id", &self.client_id);
        params.insert("refresh_token", refresh_token);

        let response = self
            .client
            .post(&url)
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                error!("Keycloak token refresh failed: {}", e);
                InfrastructureError::Keycloak(format!("Token refresh failed: {}", e))
            })?;

        self.handle_response::<KeycloakTokenResponse>(response)
            .await
    }

    pub async fn user_info(
        &self,
        access_token: &str,
    ) -> Result<KeycloakUserInfo, InfrastructureError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/userinfo",
            self.base_url, self.realm
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| {
                error!("Keycloak user info request failed: {}", e);
                InfrastructureError::Keycloak(format!("User info request failed: {}", e))
            })?;

        self.handle_response::<KeycloakUserInfo>(response).await
    }

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<String, InfrastructureError> {
        // First, get admin token
        let admin_token = self.get_admin_token().await?;

        let url = format!("{}/admin/realms/{}/users", self.base_url, self.realm);

        let user_data = serde_json::json!({
            "username": username,
            "email": email,
            "firstName": first_name.unwrap_or(""),
            "lastName": last_name.unwrap_or(""),
            "enabled": true,
            "emailVerified": false,
            "credentials": [{
                "type": "password",
                "value": password,
                "temporary": false
            }]
        });

        let response = self
            .client
            .post(&url)
            .bearer_auth(&admin_token)
            .json(&user_data)
            .send()
            .await
            .map_err(|e| {
                error!("Keycloak create user failed: {}", e);
                InfrastructureError::Keycloak(format!("Create user failed: {}", e))
            })?;

        let status = response.status();

        if status.is_success() {
            // Extract user ID from location header
            if let Some(location) = response.headers().get("location") {
                if let Ok(location_str) = location.to_str() {
                    if let Some(user_id) = location_str.split('/').last() {
                        info!("User created successfully in Keycloak: {}", user_id);
                        return Ok(user_id.to_string());
                    }
                }
            }
            Err(InfrastructureError::Keycloak(
                "Failed to extract user ID from response".to_string(),
            ))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            error!(
                "Keycloak create user failed with status {}: {}",
                status, error_text
            );
            Err(InfrastructureError::Keycloak(format!(
                "Create user failed: {}",
                error_text
            )))
        }
    }

    async fn get_admin_token(&self) -> Result<String, InfrastructureError> {
        let url = format!(
            "{}/realms/master/protocol/openid-connect/token",
            self.base_url
        );

        let mut params = std::collections::HashMap::new();
        params.insert("grant_type", "password");
        params.insert("client_id", "admin-cli");
        params.insert("username", &self.admin_username);
        params.insert("password", &self.admin_password);

        let response = self
            .client
            .post(&url)
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to get admin token: {}", e);
                InfrastructureError::Keycloak(format!("Admin token request failed: {}", e))
            })?;

        let token_response: KeycloakTokenResponse = self.handle_response(response).await?;
        Ok(token_response.access_token)
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T, InfrastructureError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let status = response.status();

        if status.is_success() {
            response.json::<T>().await.map_err(|e| {
                error!("Failed to parse Keycloak response: {}", e);
                InfrastructureError::Keycloak(format!("Response parsing failed: {}", e))
            })
        } else {
            let error_text = response.text().await.unwrap_or_default();
            error!("Keycloak API error {}: {}", status, error_text);
            Err(InfrastructureError::Keycloak(format!(
                "API error {}: {}",
                status, error_text
            )))
        }
    }
}
