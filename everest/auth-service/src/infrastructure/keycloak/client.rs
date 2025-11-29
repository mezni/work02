use crate::infrastructure::config::keycloak_config::KeycloakConfig;
use keycloak::{KeycloakAdmin, KeycloakAdminToken, KeycloakError};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct KeycloakClient {
    admin: Arc<RwLock<KeycloakAdmin<KeycloakAdminToken>>>,
    config: KeycloakConfig,
    http_client: reqwest::Client,
}

impl KeycloakClient {
    pub async fn new(config: KeycloakConfig) -> Result<Self, KeycloakError> {
        info!("Initializing Keycloak client for URL: {}", config.url);

        let http_client = reqwest::Client::new();

        let token = KeycloakAdminToken::acquire(
            &config.url,
            &config.admin_username,
            &config.admin_password,
            &http_client,
        )
        .await
        .map_err(|e| {
            error!("Failed to acquire Keycloak admin token: {}", e);
            e
        })?;

        let admin = KeycloakAdmin::new(&config.url, token, http_client.clone());

        info!("Keycloak client initialized successfully");

        Ok(Self {
            admin: Arc::new(RwLock::new(admin)),
            config,
            http_client,
        })
    }

    pub async fn get_admin(
        &self,
    ) -> tokio::sync::RwLockReadGuard<'_, KeycloakAdmin<KeycloakAdminToken>> {
        self.admin.read().await
    }

    /// Refresh the token and get a new admin instance
    pub async fn refresh_token(&self) -> Result<(), KeycloakError> {
        warn!("Refreshing Keycloak admin token");

        let token = KeycloakAdminToken::acquire(
            &self.config.url,
            &self.config.admin_username,
            &self.config.admin_password,
            &self.http_client,
        )
        .await
        .map_err(|e| {
            error!("Failed to refresh Keycloak admin token: {}", e);
            e
        })?;

        let new_admin = KeycloakAdmin::new(&self.config.url, token, self.http_client.clone());

        let mut admin_guard = self.admin.write().await;
        *admin_guard = new_admin;

        info!("Keycloak admin token refreshed successfully");
        Ok(())
    }

    /// Execute a request with automatic token refresh on 401
    pub async fn with_retry<F, T, Fut>(&self, operation: F) -> Result<T, KeycloakError>
    where
        F: Fn(tokio::sync::RwLockReadGuard<'_, KeycloakAdmin<KeycloakAdminToken>>) -> Fut,
        Fut: std::future::Future<Output = Result<T, KeycloakError>>,
    {
        // First attempt
        {
            let admin = self.get_admin().await;
            match operation(admin).await {
                Ok(result) => return Ok(result),
                Err(KeycloakError::HttpFailure { status, .. }) if status == 401 => {
                    // Token expired, refresh and retry
                    warn!("Received 401, refreshing token and retrying");
                    // admin is automatically dropped when it goes out of scope
                }
                Err(e) => return Err(e),
            }
        } // admin is dropped here when the scope ends

        // Refresh token and retry
        self.refresh_token().await?;

        let admin = self.get_admin().await;
        operation(admin).await
    }

    pub fn realm(&self) -> &str {
        &self.config.realm
    }
}
