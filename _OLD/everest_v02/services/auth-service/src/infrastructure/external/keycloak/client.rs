use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::time::Duration;

use crate::infrastructure::config::KeycloakConfig;

#[derive(Error, Debug)]
pub enum KeycloakError {
    #[error("Keycloak request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Keycloak authentication failed: {0}")]
    AuthFailed(String),
    #[error("Keycloak user operation failed: {0}")]
    UserOperationFailed(String),
    #[error("Keycloak configuration error: {0}")]
    ConfigError(String),
    #[error("Keycloak response parsing failed: {0}")]
    ParseError(String),
}

#[derive(Debug, Clone)]
pub struct KeycloakClient {
    http_client: Client,
    config: KeycloakConfig,
    admin_token: Option<String>,
    token_expiry: Option<std::time::SystemTime>,
}

#[derive(Debug, Serialize)]
struct AdminLoginRequest {
    username: String,
    password: String,
    grant_type: String,
    client_id: String,
}

#[derive(Debug, Serialize)]
struct UserLoginRequest {
    username: String,
    password: String,
    grant_type: String,
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Serialize)]
struct RefreshTokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: Option<String>,
    refresh_expires_in: Option<i64>,
}

#[derive(Debug, Serialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    firstName: String,
    lastName: String,
    enabled: bool,
    emailVerified: bool,
    credentials: Vec<Credential>,
}

#[derive(Debug, Serialize)]
struct Credential {
    #[serde(rename = "type")]
    cred_type: String,
    value: String,
    temporary: bool,
}

impl KeycloakClient {
    pub fn new(config: KeycloakConfig) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            config,
            admin_token: None,
            token_expiry: None,
        }
    }

    /// Ensure we have a valid admin token
    async fn ensure_admin_token(&mut self) -> Result<(), KeycloakError> {
        if let Some(expiry) = self.token_expiry {
            if expiry > std::time::SystemTime::now() {
                return Ok(());
            }
        }

        self.authenticate_admin().await
    }

    /// Authenticate as admin to get token
    pub async fn authenticate_admin(&mut self) -> Result<(), KeycloakError> {
        let login_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.server_url, "master"
        );

        let request = AdminLoginRequest {
            username: self.config.admin_username.clone(),
            password: self.config.admin_password.clone(),
            grant_type: "password".to_string(),
            client_id: "admin-cli".to_string(),
        };

        let response: TokenResponse = self.http_client
            .post(&login_url)
            .form(&request)
            .send()
            .await?
            .json()
            .await?;

        self.admin_token = Some(response.access_token.clone());
        self.token_expiry = Some(
            std::time::SystemTime::now() + Duration::from_secs(response.expires_in as u64 - 60)
        );

        Ok(())
    }

    /// Create a new user in Keycloak
    pub async fn create_user(
        &mut self,
        email: &str,
        username: &str,
        first_name: &str,
        last_name: &str,
        password: &str,
    ) -> Result<String, KeycloakError> {
        self.ensure_admin_token().await?;

        let url = format!(
            "{}/admin/realms/{}/users",
            self.config.server_url, self.config.realm
        );

        let user_request = CreateUserRequest {
            username: username.to_string(),
            email: email.to_string(),
            firstName: first_name.to_string(),
            lastName: last_name.to_string(),
            enabled: true,
            emailVerified: false,
            credentials: vec![Credential {
                cred_type: "password".to_string(),
                value: password.to_string(),
                temporary: false,
            }],
        };

        let response = self.http_client
            .post(&url)
            .bearer_auth(self.admin_token.as_ref().unwrap())
            .json(&user_request)
            .send()
            .await?;

        if response.status().is_success() {
            // Extract user ID from location header
            if let Some(location) = response.headers().get("location") {
                let location_str = location.to_str().map_err(|e| KeycloakError::ParseError(e.to_string()))?;
                let user_id = location_str.split('/').last().unwrap_or("").to_string();
                Ok(user_id)
            } else {
                Err(KeycloakError::UserOperationFailed("No location header in response".to_string()))
            }
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(KeycloakError::UserOperationFailed(format!("HTTP {}: {}", response.status(), error_text)))
        }
    }

    /// Authenticate user and get tokens
    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<UserTokens, KeycloakError> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.server_url, self.config.realm
        );

        let request = UserLoginRequest {
            username: username.to_string(),
            password: password.to_string(),
            grant_type: "password".to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
        };

        let response: TokenResponse = self.http_client
            .post(&token_url)
            .form(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(UserTokens {
            access_token: response.access_token,
            refresh_token: response.refresh_token.unwrap_or_default(),
            expires_in: response.expires_in,
            token_type: response.token_type,
            refresh_expires_in: response.refresh_expires_in,
        })
    }

    /// Refresh access token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<UserTokens, KeycloakError> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.server_url, self.config.realm
        );

        let request = RefreshTokenRequest {
            grant_type: "refresh_token".to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            refresh_token: refresh_token.to_string(),
        };

        let response: TokenResponse = self.http_client
            .post(&token_url)
            .form(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(UserTokens {
            access_token: response.access_token,
            refresh_token: response.refresh_token.unwrap_or_default(),
            expires_in: response.expires_in,
            token_type: response.token_type,
            refresh_expires_in: response.refresh_expires_in,
        })
    }

    /// Logout user
    pub async fn logout(&self, refresh_token: &str) -> Result<(), KeycloakError> {
        let logout_url = format!(
            "{}/realms/{}/protocol/openid-connect/logout",
            self.config.server_url, self.config.realm
        );

        let request = RefreshTokenRequest {
            grant_type: "refresh_token".to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            refresh_token: refresh_token.to_string(),
        };

        let response = self.http_client
            .post(&logout_url)
            .form(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(KeycloakError::UserOperationFailed(format!("Logout failed: {}", error_text)))
        }
    }

    /// Get user info
    pub async fn get_user_info(&self, access_token: &str) -> Result<UserInfo, KeycloakError> {
        let userinfo_url = format!(
            "{}/realms/{}/protocol/openid-connect/userinfo",
            self.config.server_url, self.config.realm
        );

        let response = self.http_client
            .get(&userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await?;

        if response.status().is_success() {
            let user_info: UserInfo = response.json().await?;
            Ok(user_info)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(KeycloakError::UserOperationFailed(format!("Failed to get user info: {}", error_text)))
        }
    }

    /// Verify token introspection
    pub async fn introspect_token(&self, token: &str) -> Result<TokenIntrospection, KeycloakError> {
        let introspect_url = format!(
            "{}/realms/{}/protocol/openid-connect/token/introspect",
            self.config.server_url, self.config.realm
        );

        let params = [
            ("token", token),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
        ];

        let response: TokenIntrospection = self.http_client
            .post(&introspect_url)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Get user by ID
    pub async fn get_user_by_id(&mut self, user_id: &str) -> Result<KeycloakUser, KeycloakError> {
        self.ensure_admin_token().await?;

        let url = format!(
            "{}/admin/realms/{}/users/{}",
            self.config.server_url, self.config.realm, user_id
        );

        let user: KeycloakUser = self.http_client
            .get(&url)
            .bearer_auth(self.admin_token.as_ref().unwrap())
            .send()
            .await?
            .json()
            .await?;

        Ok(user)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub refresh_expires_in: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    pub email: String,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
    pub email_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenIntrospection {
    pub active: bool,
    pub username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub exp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub firstName: String,
    pub lastName: String,
    pub enabled: bool,
    pub emailVerified: bool,
    pub createdTimestamp: Option<i64>,
}