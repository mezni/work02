use crate::user_dto::{CreateUserDto, KeycloakUser, RoleMapping};
use reqwest::Client;
use std::error::Error;

pub struct KeycloakClient {
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: String,
    client: Client,
}

impl KeycloakClient {
    pub fn new(base_url: String, realm: String, client_id: String, client_secret: String) -> Self {
        Self {
            base_url,
            realm,
            client_id,
            client_secret,
            client: Client::new(),
        }
    }

    pub async fn get_admin_token(&self) -> Result<String, Box<dyn Error>> {
        let resp = self
            .client
            .post(format!(
                "{}/realms/{}/protocol/openid-connect/token",
                self.base_url, self.realm
            ))
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
            ])
            .send()
            .await?;

        let text = resp.text().await?;
        if !resp.status().is_success() {
            return Err(format!("Failed to get admin token: {}", text).into());
        }

        let token_json: serde_json::Value = serde_json::from_str(&text)?;
        Ok(token_json["access_token"].as_str().unwrap_or("").to_string())
    }

    pub async fn create_user(&self, token: &str, user: &CreateUserDto) -> Result<(), Box<dyn Error>> {
        let resp = self
            .client
            .post(format!("{}/admin/realms/{}/users", self.base_url, self.realm))
            .bearer_auth(token)
            .json(user)
            .send()
            .await?;

        let text = resp.text().await?;
        if !resp.status().is_success() {
            return Err(format!("Failed to create user: {}", text).into());
        }

        Ok(())
    }

    pub async fn get_user_id(&self, token: &str, username: &str) -> Result<String, Box<dyn Error>> {
        let resp = self
            .client
            .get(format!("{}/admin/realms/{}/users", self.base_url, self.realm))
            .bearer_auth(token)
            .query(&[("username", username)])
            .send()
            .await?;

        let text = resp.text().await?;
        if !resp.status().is_success() {
            return Err(format!("Failed to fetch user ID: {}", text).into());
        }

        let users: Vec<KeycloakUser> = serde_json::from_str(&text)?;
        users.get(0)
            .map(|u| u.id.clone())
            .ok_or_else(|| "User not found".into())
    }

    pub async fn get_role(&self, token: &str, role_name: &str) -> Result<RoleMapping, Box<dyn Error>> {
        let resp = self
            .client
            .get(format!("{}/admin/realms/{}/roles/{}", self.base_url, self.realm, role_name))
            .bearer_auth(token)
            .send()
            .await?;

        let text = resp.text().await?;
        if !resp.status().is_success() {
            return Err(format!("Failed to fetch role '{}': {}", role_name, text).into());
        }

        let role: RoleMapping = serde_json::from_str(&text)?;
        Ok(role)
    }

    pub async fn assign_role(&self, token: &str, user_id: &str, role: &RoleMapping) -> Result<(), Box<dyn Error>> {
        let resp = self
            .client
            .post(format!(
                "{}/admin/realms/{}/users/{}/role-mappings/realm",
                self.base_url, self.realm, user_id
            ))
            .bearer_auth(token)
            .json(&vec![role])
            .send()
            .await?;

        let text = resp.text().await?;
        if !resp.status().is_success() {
            return Err(format!("Failed to assign role: {}", text).into());
        }

        Ok(())
    }
}
