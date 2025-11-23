use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use crate::domain::services::auth_service::AuthService;
use crate::domain::models::user::User;

#[derive(Clone)]
pub struct KeycloakAuthClient {
    client: Client,
    base_url: String,
    realm: String,
    admin_username: String,
    admin_password: String,
}

impl KeycloakAuthClient {
    pub fn new(base_url: String, realm: String, admin_username: String, admin_password: String) -> Self {
        Self { 
            client: Client::new(), 
            base_url, 
            realm, 
            admin_username, 
            admin_password 
        }
    }

    async fn get_admin_token(&self) -> anyhow::Result<String> {
        let url = format!("{}/realms/master/protocol/openid-connect/token", self.base_url);
        
        let params = [
            ("grant_type", "password"),
            ("client_id", "admin-cli"),
            ("username", &self.admin_username),
            ("password", &self.admin_password),
        ];

        let response = self.client.post(&url).form(&params).send().await?;
        let json: serde_json::Value = response.json().await?;
        
        json["access_token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("No access token in response"))
    }
}

#[async_trait]
impl AuthService for KeycloakAuthClient {
    async fn create_user(&self, username: &str, email: Option<&str>, first_name: Option<&str>, last_name: Option<&str>, password: &str) -> anyhow::Result<User> {
        let admin_token = self.get_admin_token().await?;
        let url = format!("{}/admin/realms/{}/users", self.base_url, self.realm);

        let user_data = json!({
            "username": username,
            "email": email,
            "firstName": first_name,
            "lastName": last_name,
            "enabled": true,
            "credentials": [{
                "type": "password",
                "value": password,
                "temporary": false
            }]
        });

        let response = self.client
            .post(&url)
            .bearer_auth(&admin_token)
            .json(&user_data)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to create user: {}", response.status()));
        }

        // Extract user ID from Location header
        let location = response.headers()
            .get("location")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| anyhow::anyhow!("No location header in response"))?;

        let user_id = location.split('/').last()
            .ok_or_else(|| anyhow::anyhow!("Invalid location header"))?;

        Ok(User {
            id: user_id.to_string(),
            username: username.to_string(),
            email: email.map(|s| s.to_string()),
            first_name: first_name.map(|s| s.to_string()),
            last_name: last_name.map(|s| s.to_string()),
            roles: Vec::new(),
            enabled: true,
        })
    }
}