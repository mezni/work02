use serde::{Deserialize, Serialize};
use reqwest::Client;
use thiserror::Error;
use crate::config::ServiceConfig;

#[derive(Error, Debug)]
pub enum KeycloakError {
    #[error("Keycloak request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Keycloak authentication failed")]
    AuthenticationFailed,
    #[error("Keycloak user creation failed")]
    UserCreationFailed,
    #[error("Keycloak operation failed: {0}")]
    OperationFailed(String),
}

#[derive(Debug, Clone)]
pub struct KeycloakConfig {
    pub server_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

impl KeycloakConfig {
    pub fn from_service_config(config: &ServiceConfig) -> Self {
        Self {
            server_url: config.keycloak.url.clone(),
            realm: config.keycloak.realm.clone(),
            client_id: config.keycloak.client.clone(),
            client_secret: "".to_string(), // You might want to add this to your config
            admin_username: config.keycloak.admin_user.clone(),
            admin_password: config.keycloak.admin_password.clone(),
        }
    }
}

#[derive(Debug, Clone)]
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

    pub async fn authenticate_user(&self, username: &str, password: &str) -> Result<String, KeycloakError> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.server_url, self.config.realm
        );

        let params = [
            ("grant_type", "password"),
            ("client_id", &self.config.client_id),
            ("username", username),
            ("password", password),
        ];

        let response = self.client
            .post(&token_url)
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: KeycloakTokenResponse = response.json().await?;
            Ok(token_response.access_token)
        } else {
            Err(KeycloakError::AuthenticationFailed)
        }
    }

    pub async fn create_user(&self, email: &str, password: &str, role: &str) -> Result<String, KeycloakError> {
        // Get admin token first using admin credentials
        let admin_token = self.get_admin_token().await?;

        // Create user in Keycloak
        let create_user_url = format!(
            "{}/admin/realms/{}/users",
            self.config.server_url, self.config.realm
        );

        let user_representation = KeycloakUserRepresentation {
            username: email,
            email,
            enabled: true,
            credentials: vec![KeycloakCredential {
                value: password,
                temporary: false,
            }],
            realm_roles: vec![role.to_string()],
        };

        let response = self.client
            .post(&create_user_url)
            .header("Authorization", format!("Bearer {}", admin_token))
            .json(&user_representation)
            .send()
            .await?;

        if response.status().is_success() {
            // Extract user ID from location header
            if let Some(location) = response.headers().get("Location") {
                let location_str = location.to_str().unwrap_or("");
                let user_id = location_str.split('/').last().unwrap_or("");
                Ok(user_id.to_string())
            } else {
                Err(KeycloakError::UserCreationFailed)
            }
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(KeycloakError::OperationFailed(format!("Failed to create user: {}", error_text)))
        }
    }

    async fn get_admin_token(&self) -> Result<String, KeycloakError> {
        let token_url = format!(
            "{}/realms/master/protocol/openid-connect/token",
            self.config.server_url
        );

        let params = [
            ("grant_type", "password"),
            ("client_id", "admin-cli"),
            ("username", &self.config.admin_username),
            ("password", &self.config.admin_password),
        ];

        let response = self.client
            .post(&token_url)
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: KeycloakTokenResponse = response.json().await?;
            Ok(token_response.access_token)
        } else {
            Err(KeycloakError::AuthenticationFailed)
        }
    }

    pub async fn verify_token(&self, token: &str) -> Result<KeycloakUserInfo, KeycloakError> {
        let introspect_url = format!(
            "{}/realms/{}/protocol/openid-connect/token/introspect",
            self.config.server_url, self.config.realm
        );

        let params = [
            ("token", token),
            ("client_id", &self.config.client_id),
        ];

        let response = self.client
            .post(&introspect_url)
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let introspect_response: KeycloakIntrospectResponse = response.json().await?;
            if introspect_response.active {
                Ok(KeycloakUserInfo {
                    sub: introspect_response.sub.unwrap_or_default(),
                    username: introspect_response.username.unwrap_or_default(),
                    email: introspect_response.email.unwrap_or_default(),
                    roles: introspect_response.realm_roles.unwrap_or_default(),
                })
            } else {
                Err(KeycloakError::AuthenticationFailed)
            }
        } else {
            Err(KeycloakError::AuthenticationFailed)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct KeycloakTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: Option<String>,
}

#[derive(Debug, Serialize)]
struct KeycloakUserRepresentation<'a> {
    username: &'a str,
    email: &'a str,
    enabled: bool,
    credentials: Vec<KeycloakCredential<'a>>,
    realm_roles: Vec<String>,
}

#[derive(Debug, Serialize)]
struct KeycloakCredential<'a> {
    value: &'a str,
    temporary: bool,
}

#[derive(Debug, Deserialize)]
struct KeycloakIntrospectResponse {
    active: bool,
    sub: Option<String>,
    username: Option<String>,
    email: Option<String>,
    realm_roles: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct KeycloakUserInfo {
    pub sub: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}