use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub keycloak_admin: String,
    pub keycloak_admin_password: String,
    pub keycloak_client: String,
}

impl Config {
    pub fn from_env() -> Self {
        // Load .env with dotenvy
        dotenvy::dotenv().ok();

        Self {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("SERVER_PORT must be a number"),
            keycloak_url: env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL missing"),
            keycloak_realm: env::var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM missing"),
            keycloak_admin: env::var("KEYCLOAK_ADMIN").expect("KEYCLOAK_ADMIN missing"),
            keycloak_admin_password: env::var("KEYCLOAK_ADMIN_PASSWORD")
                .expect("KEYCLOAK_ADMIN_PASSWORD missing"),
            keycloak_client: env::var("KEYCLOAK_CLIENT").expect("KEYCLOAK_CLIENT missing"),
        }
    }
}
