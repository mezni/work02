use config::{Config as ConfigBuilder, Environment, File};
use dotenvy::dotenv;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::AppConfig;
use super::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone)]
pub struct ConfigManager {
    config: Arc<RwLock<AppConfig>>,
    config_path: String,
}

impl ConfigManager {
    pub fn new(config_path: &str) -> InfrastructureResult<Self> {
        dotenv().ok();
        
        let config = Self::load_config(config_path)?;
        
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path: config_path.to_string(),
        })
    }
    
    pub async fn get_config(&self) -> AppConfig {
        let config = self.config.read().await;
        config.clone()
    }
    
    pub async fn reload(&self) -> InfrastructureResult<()> {
        let new_config = Self::load_config(&self.config_path)?;
        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }
    
    fn load_config(config_path: &str) -> InfrastructureResult<AppConfig> {
        let environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".into());
        
        let config_builder = ConfigBuilder::builder()
            .add_source(File::with_name(&format!("{}/default", config_path)).required(false))
            .add_source(File::with_name(&format!("{}/{}", config_path, environment)).required(false))
            .add_source(File::with_name(&format!("{}/local", config_path)).required(false))
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()
            .map_err(|e| InfrastructureError::Configuration(e.to_string()))?;
        
        config_builder.try_deserialize()
            .map_err(|e| InfrastructureError::Configuration(e.to_string()))
    }
    
    pub async fn get_keycloak_config(&self) -> super::keycloak_client::KeycloakConfig {
        let config = self.config.read().await;
        super::keycloak_client::KeycloakConfig::from(&*config)
    }
    
    pub async fn get_jwt_secret(&self) -> String {
        let config = self.config.read().await;
        config.jwt.secret.clone()
    }
    
    pub async fn is_production(&self) -> bool {
        let config = self.config.read().await;
        config.environment == "production"
    }
}