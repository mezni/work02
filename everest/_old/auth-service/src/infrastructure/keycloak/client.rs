use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::infrastructure::config::KeycloakConfig;
use super::errors::KeycloakError;

#[derive(Debug, Serialize)]
struct KeycloakUserCreate {
    username: String,
    email: String,
    enabled: bool,
    credentials: Vec<KeycloakCredential>,
    attributes: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize)]
struct KeycloakCredential {
    #[serde(rename = "type")]
    credential_type: String,
    value: String,
    temporary: bool,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_expires_in: i64,
    refresh_token: String,
    token_type: String,
}

pub struct KeycloakClient {
    config: KeycloakConfig,
    client: Client,
}

impl KeycloakClient {
    pub fn new(config: KeycloakConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    async fn get_admin_token(&self) -> Result<String, KeycloakError> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.url, self.config.realm
        );

        let params = [
            ("client_id", self.config.client_id.as_str()),
            ("client_secret", self.config.client_secret.as_str()),
            ("grant_type", "password"),
            ("username", self.config.admin_username.as_str()),
            ("password", self.config.admin_password.as_str()),
        ];

        let response = self.client
            .post(&token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| KeycloakError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(KeycloakError::AuthenticationFailed(
                response.text().await.unwrap_or_default(),
            ));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|e| KeycloakError::ParseError(e.to_string()))?;

        Ok(token_response.access_token)
    }

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        role: &str,
        organisation_name: Option<&str>,
    ) -> Result<String, KeycloakError> {
        let admin_token = self.get_admin_token().await?;

        let mut attributes = std::collections::HashMap::new();
        if let Some(org) = organisation_name {
            attributes.insert("organisation_name".to_string(), vec![org.to_string()]);
        }

        let user = KeycloakUserCreate {
            username: username.to_string(),
            email: email.to_string(),
            enabled: true,
            credentials: vec![KeycloakCredential {
                credential_type: "password".to_string(),
                value: password.to_string(),
                temporary: false,
            }],
            attributes: if attributes.is_empty() { None } else { Some(attributes) },
        };

        let create_url = format!(
            "{}/admin/realms/{}/users",
            self.config.url, self.config.realm
        );

        let response = self.client
            .post(&create_url)
            .header("Authorization", format!("Bearer {}", admin_token))
            .json(&user)
            .send()
            .await
            .map_err(|e| KeycloakError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(KeycloakError::UserCreationFailed(
                response.text().await.unwrap_or_default(),
            ));
        }

        // Extract user ID from Location header
        let location = response
            .headers()
            .get("Location")
            .and_then(|h| h.to_str().ok())
            .ok_or(KeycloakError::UserCreationFailed(
                "No Location header".to_string(),
            ))?;

        let user_id = location.split('/').last().unwrap_or("").to_string();

        // Assign role
        self.assign_role_to_user(&user_id, role, &admin_token).await?;

        Ok(user_id)
    }

    async fn assign_role_to_user(
        &self,
        user_id: &str,
        role_name: &str,
        admin_token: &str,
    ) -> Result<(), KeycloakError> {
        // Get role representation
        let role_url = format!(
            "{}/admin/realms/{}/roles/{}",
            self.config.url, self.config.realm, role_name
        );

        let role_response = self.client
            .get(&role_url)
            .header("Authorization", format!("Bearer {}", admin_token))
            .send()
            .await
            .map_err(|e| KeycloakError::RequestFailed(e.to_string()))?;

        let role: serde_json::Value = role_response
            .json()
            .await
            .map_err(|e| KeycloakError::ParseError(e.to_string()))?;

        // Assign role to user
        let assign_url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/realm",
            self.config.url, self.config.realm, user_id
        );

        let response = self.client
            .post(&assign_url)
            .header("Authorization", format!("Bearer {}", admin_token))
            .json(&vec![role])
            .send()
            .await
            .map_err(|e| KeycloakError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(KeycloakError::RoleAssignmentFailed(
                response.text().await.unwrap_or_default(),
            ));
        }

        Ok(())
    }

    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<TokenResponse, KeycloakError> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.url, self.config.realm
        );

        let params = [
            ("client_id", self.config.client_id.as_str()),
            ("client_secret", self.config.client_secret.as_str()),
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
        ];

        let response = self.client
            .post(&token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| KeycloakError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(KeycloakError::AuthenticationFailed(
                response.text().await.unwrap_or_default(),
            ));
        }

        response
            .json()
            .await
            .map_err(|e| KeycloakError::ParseError(e.to_string()))
    }
}
