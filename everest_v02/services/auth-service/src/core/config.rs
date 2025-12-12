use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub rust_log: String,
    pub keycloak: KeycloakConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakConfig {
    pub url: String,
    pub realm: String,
    pub auth_client_id: String,
    pub backend_client_id: String,
    pub backend_client_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .context("Failed to parse SERVER_PORT")?,
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            keycloak: KeycloakConfig {
                url: env::var("KEYCLOAK_URL").context("KEYCLOAK_URL must be set")?,
                realm: env::var("KEYCLOAK_REALM").context("KEYCLOAK_REALM must be set")?,
                auth_client_id: env::var("KEYCLOAK_AUTH_CLIENT_ID")
                    .context("KEYCLOAK_AUTH_CLIENT_ID must be set")?,
                backend_client_id: env::var("KEYCLOAK_BACKEND_CLIENT_ID")
                    .context("KEYCLOAK_BACKEND_CLIENT_ID must be set")?,
                backend_client_secret: env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")
                    .context("KEYCLOAK_BACKEND_CLIENT_SECRET must be set")?,
            },
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
