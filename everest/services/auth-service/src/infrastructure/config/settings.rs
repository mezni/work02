use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeycloakSettings {
    pub url: String,
    pub admin: String,
    pub admin_password: String,
    pub realm_name: String,
    pub client_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthSettings {
    pub jwt_secret: String,
    pub jwt_expiration_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CacheSettings {
    pub redis_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuditSettings {
    pub retention_days: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub keycloak: KeycloakSettings,
    pub auth: AuthSettings,
    pub server: ServerSettings,
    pub cache: CacheSettings,
    pub audit: AuditSettings,
    pub log_level: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // Start with default values
            .add_source(File::with_name("config/default").required(false))
            // Add environment-specific values
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add local overrides
            .add_source(File::with_name("config/local").required(false))
            // Add environment variables
            .add_source(Environment::with_prefix("APP").separator("_"))
            // Add .env file
            .add_source(
                Environment::with_prefix("APP")
                    .separator("_")
                    .ignore_empty(true),
            )
            .build()?;

        config.try_deserialize()
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.name
        )
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database: DatabaseSettings {
                url: "postgres://auth_user:password@localhost:5433/auth_db".to_string(),
                host: "localhost".to_string(),
                port: 5433,
                name: "auth_db".to_string(),
                username: "auth_user".to_string(),
                password: "password".to_string(),
            },
            keycloak: KeycloakSettings {
                url: "http://localhost:5080".to_string(),
                admin: "admin".to_string(),
                admin_password: "password".to_string(),
                realm_name: "ev-realm".to_string(),
                client_name: "auth-service".to_string(),
            },
            auth: AuthSettings {
                jwt_secret: "your_jwt_secret_key_here".to_string(),
                jwt_expiration_seconds: 3600,
            },
            server: ServerSettings {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            cache: CacheSettings {
                redis_url: "redis://localhost:6379".to_string(),
            },
            audit: AuditSettings {
                retention_days: 365,
            },
            log_level: "info".to_string(),
        }
    }
}
