use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use crate::domain::models::User;

#[derive(Clone)]
pub struct KeycloakClient {
    pub base_url: String,
    pub realm: String,
    pub admin_user: String,
    pub admin_password: String,
    pub client: Client,
}

impl KeycloakClient {
    pub fn new(base_url: &str, realm: &str, admin_user: &str, admin_password: &str) -> Self {
        Self {
            base_url: base_url.into(),
            realm: realm.into(),
            admin_user: admin_user.into(),
            admin_password: admin_password.into(),
            client: Client::new(),
        }
    }

    pub async fn get_admin_token(&self) -> Result<String> {
        let url = format!("{}/realms/master/protocol/openid-connect/token", self.base_url);
        let params = [
            ("grant_type", "password"),
            ("client_id", "admin-cli"),
            ("username", &self.admin_user),
            ("password", &self.admin_password),
        ];

        let res: Value = self.client.post(&url).form(&params).send().await?.json().await?;
        Ok(res["access_token"].as_str().unwrap().to_string())
    }

    pub async fn create_user(&self, user: &User, password: &str) -> Result<String> {
        let token = self.get_admin_token().await?;
        let url = format!("{}/admin/realms/{}/users", self.base_url, self.realm);

        let body = serde_json::json!({
            "username": user.username,
            "email": user.email.0,
            "enabled": true,
            "attributes": { "company_name": user.company.0 },
            "credentials": [{ "type": "password", "value": password, "temporary": false }]
        });

        let resp = self.client.post(&url).bearer_auth(token).json(&body).send().await?;

        if resp.status().as_u16() == 201 {
            let loc = resp.headers().get("Location").unwrap().to_str()?;
            let id = loc.split('/').last().unwrap();
            Ok(id.to_string())
        } else {
            Err(anyhow::anyhow!("Failed to create user: {:?}", resp.text().await?))
        }
    }

    pub async fn assign_role(&self, user_id: &str, role: &str) -> Result<()> {
        // implement Keycloak assign role API
        Ok(())
    }
}
