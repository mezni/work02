use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize)]
pub struct KeycloakToken {
    pub access_token: String,
}

#[derive(Serialize)]
pub struct Credential {
    #[serde(rename = "type")]
    pub cred_type: String,
    pub value: String,
    pub temporary: bool,
}

#[derive(Serialize)]
pub struct CreateUserDto {
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub enabled: bool,
    pub credentials: Vec<Credential>,
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Deserialize)]
pub struct KeycloakUser {
    pub id: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoleMapping {
    pub id: String,
    pub name: String,
}

pub struct KeycloakClient {
    pub base_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    http: Client,
}

impl KeycloakClient {
    pub fn new(base_url: &str, realm: &str, client_id: &str, client_secret: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            realm: realm.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            http: Client::new(),
        }
    }

    pub async fn get_token(&self) -> Result<String, Box<dyn Error>> {
        let resp = self
            .http
            .post(&format!(
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

        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            return Err(format!("Failed to get token: {}", text).into());
        }

        let token: KeycloakToken = serde_json::from_str(&text)?;
        Ok(token.access_token)
    }

    pub async fn create_user(&self, user: &CreateUserDto) -> Result<(), Box<dyn Error>> {
        let token = self.get_token().await?;
        let resp = self
            .http
            .post(&format!(
                "{}/admin/realms/{}/users",
                self.base_url, self.realm
            ))
            .bearer_auth(&token)
            .json(user)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await?;
            return Err(format!("Failed to create user: {}", text).into());
        }

        Ok(())
    }

    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<KeycloakUser, Box<dyn Error>> {
        let token = self.get_token().await?;
        let resp = self
            .http
            .get(&format!(
                "{}/admin/realms/{}/users",
                self.base_url, self.realm
            ))
            .bearer_auth(&token)
            .query(&[("username", username)])
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            return Err(format!("Failed to fetch user: {}", text).into());
        }

        let users: Vec<KeycloakUser> = serde_json::from_str(&text)?;
        users
            .into_iter()
            .next()
            .ok_or_else(|| "User not found".into())
    }

    pub async fn get_role(&self, role_name: &str) -> Result<RoleMapping, Box<dyn Error>> {
        let token = self.get_token().await?;
        let resp = self
            .http
            .get(&format!(
                "{}/admin/realms/{}/roles/{}",
                self.base_url, self.realm, role_name
            ))
            .bearer_auth(&token)
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            return Err(format!("Failed to fetch role: {}", text).into());
        }

        let role: RoleMapping = serde_json::from_str(&text)?;
        Ok(role)
    }

    pub async fn assign_realm_role(
        &self,
        user_id: &str,
        role: &RoleMapping,
    ) -> Result<(), Box<dyn Error>> {
        let token = self.get_token().await?;
        let resp = self
            .http
            .post(&format!(
                "{}/admin/realms/{}/users/{}/role-mappings/realm",
                self.base_url, self.realm, user_id
            ))
            .bearer_auth(&token)
            .json(&vec![role])
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await?;
            return Err(format!("Failed to assign role: {}", text).into());
        }

        Ok(())
    }
}
