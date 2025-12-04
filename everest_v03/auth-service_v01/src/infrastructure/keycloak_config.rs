// src/infrastructure/keycloak_config.rs
use serde::Deserialize;

#[derive(Deserialize)]
pub struct KeycloakConfig {
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub auth_server_url: String,
}

impl KeycloakConfig {
    pub fn new(realm: String, client_id: String, client_secret: String, auth_server_url: String) -> Self {
        KeycloakConfig {
            realm,
            client_id,
            client_secret,
            auth_server_url,
        }
    }
}