use crate::infrastructure::config::keycloak_config::KeycloakConfig;
use keycloak::{KeycloakAdmin, KeycloakAdminToken, KeycloakError};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

pub struct KeycloakClient {
    admin: Arc<RwLock<KeycloakAdmin<KeycloakAdminToken>>>,
    config: KeycloakConfig,
}

impl KeycloakClient {
    pub async fn new(config: KeycloakConfig) -> Result<Self, KeycloakError> {
        info!("Initializing Keycloak client for URL: {}", config.url);
        
        let client = reqwest::Client::new();

        let token = KeycloakAdminToken::acquire(
            &config.url,
            &config.admin_username,
            &config.admin_password,
            &client,
        )
        .await
        .map_err(|e| {
            error!("Failed to acquire Keycloak admin token: {}", e);
            e
        })?;

        let admin = KeycloakAdmin::new(&config.url, token, client);

        info!("Keycloak client initialized successfully");

        Ok(Self {
            admin: Arc::new(RwLock::new(admin)),
            config,
        })
    }

    pub async fn get_admin(&self) -> tokio::sync::RwLockReadGuard<'_, KeycloakAdmin<KeycloakAdminToken>> {
        self.admin.read().await
    }

    pub fn realm(&self) -> &str {
        &self.config.realm
    }
}