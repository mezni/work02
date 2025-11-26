use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub keycloak: KeycloakConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakConfig {
    pub server_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

impl KeycloakConfig {
    pub fn admin_token_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.server_url, self.realm
        )
    }
    
    pub fn admin_users_url(&self) -> String {
        format!(
            "{}/admin/realms/{}/users",
            self.server_url, self.realm
        )
    }
    
    pub fn token_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.server_url, self.realm
        )
    }
    
    pub fn user_info_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/userinfo",
            self.server_url, self.realm
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub expiration_days: i64,
}

impl Config {
    pub fn load() -> Result<Self, crate::infrastructure::errors::InfrastructureError> {
        let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        
        let config = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::File::with_name(&format!("config/{}", environment)).required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        
        config.try_deserialize()
            .map_err(|e| crate::infrastructure::errors::InfrastructureError::ConfigError(e.to_string()))
    }
}
