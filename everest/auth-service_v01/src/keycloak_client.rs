use keycloak::{KeycloakAdmin, KeycloakAdminToken};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct KeycloakConfig {
    pub url: String,
    pub realm: String,
}

impl KeycloakConfig {
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("KEYCLOAK_URL").unwrap_or_else(|_| "http://localhost:5080/auth".to_string()),
            realm: std::env::var("KEYCLOAK_REALM").unwrap_or_else(|_| "master".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub token_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

pub struct KeycloakClient {
    config: KeycloakConfig,
}

impl KeycloakClient {
    pub fn new(config: KeycloakConfig) -> Self {
        Self { config }
    }

    pub async fn get_admin_client(&self, client: &Client) -> Result<(KeycloakAdmin, KeycloakAdminToken), String> {
        let admin_username = std::env::var("KEYCLOAK_ADMIN_USER").unwrap_or_else(|_| "admin".to_string());
        let admin_password = std::env::var("KEYCLOAK_ADMIN_PASSWORD").unwrap_or_else(|_| "admin".to_string());

        let admin_token = KeycloakAdminToken::acquire(&self.config.url, &admin_username, &admin_password, client)
            .await
            .map_err(|e| format!("Failed to acquire admin token: {:?}", e))?;

        let admin = KeycloakAdmin::new(&self.config.url, admin_token.clone(), client.clone());

        Ok((admin, admin_token))
    }

    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
        client: &Client,
    ) -> Result<TokenResponse, String> {
        let client_id = std::env::var("KEYCLOAK_CLIENT_ID").unwrap_or_else(|_| "admin-cli".to_string());
        
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.url, self.config.realm
        );

        let response = client
            .post(&token_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!(
                "grant_type=password&username={}&password={}&client_id={}",
                username, password, client_id
            ))
            .send()
            .await
            .map_err(|e| format!("Failed to connect to auth server: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Authentication failed: {}", error_text));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse token response: {}", e))?;

        Ok(token_response)
    }

    pub fn get_realm(&self) -> &str {
        &self.config.realm
    }
}