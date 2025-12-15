use crate::core::config::KeycloakConfig;
use crate::core::errors::{AppError, AppResult};
use crate::domain::value_objects::UserRole;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub token_type: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakUser {
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub enabled: bool,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakCredential {
    #[serde(rename = "type")]
    pub credential_type: String,
    pub value: String,
    pub temporary: bool,
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

    async fn get_admin_token(&self) -> AppResult<String> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.url, self.config.realm
        );

        let params = [
            ("client_id", self.config.backend_client_id.as_str()),
            ("client_secret", self.config.backend_client_secret.as_str()),
            ("grant_type", "client_credentials"),
        ];

        let response = self
            .client
            .post(&token_url)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::KeycloakError(format!(
                "Failed to get admin token: {}",
                error_text
            )));
        }

        let token_response: TokenResponse = response.json().await?;
        Ok(token_response.access_token)
    }

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        role: UserRole,
        network_id: &str,
        station_id: &str,
    ) -> AppResult<String> {
        let token = self.get_admin_token().await?;
        
        let mut attributes = HashMap::new();
        attributes.insert("network_id".to_string(), vec![network_id.to_string()]);
        attributes.insert("station_id".to_string(), vec![station_id.to_string()]);

        let keycloak_user = KeycloakUser {
            id: None,
            username: username.to_string(),
            email: email.to_string(),
            first_name: first_name.map(|s| s.to_string()),
            last_name: last_name.map(|s| s.to_string()),
            enabled: true,
            email_verified: false,
            attributes: Some(attributes),
        };

        let users_url = format!(
            "{}/admin/realms/{}/users",
            self.config.url, self.config.realm
        );

        let response = self
            .client
            .post(&users_url)
            .bearer_auth(&token)
            .json(&keycloak_user)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::KeycloakError(format!(
                "Failed to create user in Keycloak: {}",
                error_text
            )));
        }

        // Get the created user ID from Location header
        let location = response
            .headers()
            .get("Location")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::KeycloakError("No Location header in response".to_string()))?;

        let user_id = location
            .split('/')
            .last()
            .ok_or_else(|| AppError::KeycloakError("Invalid Location header".to_string()))?
            .to_string();

        // Set password
        self.set_user_password(&user_id, password, false).await?;

        // Assign role
        self.assign_role_to_user(&user_id, &role).await?;

        Ok(user_id)
    }

    async fn set_user_password(&self, user_id: &str, password: &str, temporary: bool) -> AppResult<()> {
        let token = self.get_admin_token().await?;

        let credential = KeycloakCredential {
            credential_type: "password".to_string(),
            value: password.to_string(),
            temporary,
        };

        let password_url = format!(
            "{}/admin/realms/{}/users/{}/reset-password",
            self.config.url, self.config.realm, user_id
        );

        let response = self
            .client
            .put(&password_url)
            .bearer_auth(&token)
            .json(&credential)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::KeycloakError(format!(
                "Failed to set password: {}",
                error_text
            )));
        }

        Ok(())
    }

    async fn assign_role_to_user(&self, user_id: &str, role: &UserRole) -> AppResult<()> {
        let token = self.get_admin_token().await?;

        // Get realm role
        let role_url = format!(
            "{}/admin/realms/{}/roles/{}",
            self.config.url, self.config.realm, role.to_string()
        );

        let role_response = self
            .client
            .get(&role_url)
            .bearer_auth(&token)
            .send()
            .await?;

        if !role_response.status().is_success() {
            return Err(AppError::KeycloakError(format!(
                "Role {} not found in Keycloak",
                role
            )));
        }

        let role_data: serde_json::Value = role_response.json().await?;

        // Assign role to user
        let assign_url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/realm",
            self.config.url, self.config.realm, user_id
        );

        let response = self
            .client
            .post(&assign_url)
            .bearer_auth(&token)
            .json(&vec![role_data])
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::KeycloakError(format!(
                "Failed to assign role: {}",
                error_text
            )));
        }

        Ok(())
    }

    pub async fn update_user(
        &self,
        keycloak_id: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        network_id: Option<&str>,
        station_id: Option<&str>,
    ) -> AppResult<()> {
        let token = self.get_admin_token().await?;

        // Get current user
        let user_url = format!(
            "{}/admin/realms/{}/users/{}",
            self.config.url, self.config.realm, keycloak_id
        );

        let user_response = self
            .client
            .get(&user_url)
            .bearer_auth(&token)
            .send()
            .await?;

        if !user_response.status().is_success() {
            return Err(AppError::KeycloakError("User not found in Keycloak".to_string()));
        }

        let mut user: KeycloakUser = user_response.json().await?;

        // Update fields
        if let Some(fname) = first_name {
            user.first_name = Some(fname.to_string());
        }
        if let Some(lname) = last_name {
            user.last_name = Some(lname.to_string());
        }

        // Update attributes
        if network_id.is_some() || station_id.is_some() {
            let mut attributes = user.attributes.unwrap_or_default();
            if let Some(nid) = network_id {
                attributes.insert("network_id".to_string(), vec![nid.to_string()]);
            }
            if let Some(sid) = station_id {
                attributes.insert("station_id".to_string(), vec![sid.to_string()]);
            }
            user.attributes = Some(attributes);
        }

        // Update user
        let response = self
            .client
            .put(&user_url)
            .bearer_auth(&token)
            .json(&user)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::KeycloakError(format!(
                "Failed to update user: {}",
                error_text
            )));
        }

        Ok(())
    }

    pub async fn delete_user(&self, keycloak_id: &str) -> AppResult<()> {
        let token = self.get_admin_token().await?;

        let user_url = format!(
            "{}/admin/realms/{}/users/{}",
            self.config.url, self.config.realm, keycloak_id
        );

        let response = self
            .client
            .delete(&user_url)
            .bearer_auth(&token)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::KeycloakError(format!(
                "Failed to delete user: {}",
                error_text
            )));
        }

        Ok(())
    }

    pub async fn authenticate(&self, email: &str, password: &str) -> AppResult<TokenResponse> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.url, self.config.realm
        );

        let params = [
            ("client_id", self.config.auth_client_id.as_str()),
            ("username", email),
            ("password", password),
            ("grant_type", "password"),
        ];

        let response = self
            .client
            .post(&token_url)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        let token_response: TokenResponse = response.json().await?;
        Ok(token_response)
    }

    pub async fn change_password(&self, keycloak_id: &str, new_password: &str) -> AppResult<()> {
        self.set_user_password(keycloak_id, new_password, false).await
    }

    pub async fn send_verification_email(&self, keycloak_id: &str) -> AppResult<()> {
        let token = self.get_admin_token().await?;

        let email_url = format!(
            "{}/admin/realms/{}/users/{}/send-verify-email",
            self.config.url, self.config.realm, keycloak_id
        );

        let response = self
            .client
            .put(&email_url)
            .bearer_auth(&token)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::KeycloakError(format!(
                "Failed to send verification email: {}",
                error_text
            )));
        }

        Ok(())
    }

    pub async fn send_password_reset_email(&self, keycloak_id: &str) -> AppResult<()> {
        let token = self.get_admin_token().await?;

        let reset_url = format!(
            "{}/admin/realms/{}/users/{}/execute-actions-email",
            self.config.url, self.config.realm, keycloak_id
        );

        let actions = vec!["UPDATE_PASSWORD"];

        let response = self
            .client
            .put(&reset_url)
            .bearer_auth(&token)
            .json(&actions)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::KeycloakError(format!(
                "Failed to send password reset email: {}",
                error_text
            )));
        }

        Ok(())
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> AppResult<TokenResponse> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.url, self.config.realm
        );

        let params = [
            ("client_id", self.config.auth_client_id.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ];

        let response = self
            .client
            .post(&token_url)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::Unauthorized("Invalid refresh token".to_string()));
        }

        let token_response: TokenResponse = response.json().await?;
        Ok(token_response)
    }

    pub async fn logout(&self, refresh_token: &str) -> AppResult<()> {
        let logout_url = format!(
            "{}/realms/{}/protocol/openid-connect/logout",
            self.config.url, self.config.realm
        );

        let params = [
            ("client_id", self.config.auth_client_id.as_str()),
            ("refresh_token", refresh_token),
        ];

        let response = self
            .client
            .post(&logout_url)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            tracing::warn!("Keycloak logout failed: {}", error_text);
        }

        Ok(())
    }
}