use crate::domain::user::User;
use crate::infrastructure::errors::InfrastructureError;
use reqwest::StatusCode;

#[derive(Clone)]
pub struct KeycloakClient {
    pub url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
}

impl KeycloakClient {
    pub fn new(url: String, realm: String, client_id: String, client_secret: String) -> Self {
        Self { url, realm, client_id, client_secret }
    }

    pub async fn get_admin_token(&self) -> Result<String, InfrastructureError> {
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];

        let resp = client
            .post(format!("{}/realms/{}/protocol/openid-connect/token", self.url, self.realm))
            .form(&params)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakError(e.to_string()))?;

        let json: serde_json::Value = resp.json().await.map_err(|e| InfrastructureError::KeycloakError(e.to_string()))?;
        let token = json["access_token"]
            .as_str()
            .ok_or_else(|| InfrastructureError::KeycloakError("access_token missing".to_string()))?
            .to_string();
        Ok(token)
    }

    pub async fn create_user(&self, user: &User) -> Result<(), InfrastructureError> {
        let token = self.get_admin_token().await?;

        let payload = serde_json::json!({
            "username": user.username,
            "email": user.email,
            "enabled": true,
            "attributes": {
                "company_name": user.company_name,
                "station_name": user.station_name
            },
            "credentials": [{
                "type": "password",
                "value": user.password,
                "temporary": false
            }],
            "realmRoles": [user.role.to_lowercase()]
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(format!("{}/admin/realms/{}/users", self.url, self.realm))
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakError(e.to_string()))?;

        match resp.status() {
            StatusCode::CREATED => Ok(()),
            StatusCode::CONFLICT => Err(InfrastructureError::KeycloakError("User already exists".to_string())),
            s => {
                let text = resp.text().await.unwrap_or_default();
                Err(InfrastructureError::KeycloakError(format!("Unexpected response {}: {}", s, text)))
            }
        }
    }
}
