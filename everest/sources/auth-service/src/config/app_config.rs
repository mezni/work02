// src/config/app_config.rs
use serde::Deserialize;
use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub keycloak: KeycloakConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeycloakConfig {
    pub base_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl AppConfig {
    /// Load configuration from environment variables with fallback to defaults
    pub fn from_env() -> Result<Self, ConfigError> {
        let environment = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());
        
        let config_builder = Config::builder()
            // Add default values
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .set_default("server.workers", std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1))?
            .set_default("keycloak.base_url", "http://localhost:8080")?
            .set_default("keycloak.realm", "master")?
            .set_default("keycloak.client_id", "admin-cli")?
            .set_default("keycloak.client_secret", "secret")?
            .set_default("keycloak.admin_username", "admin")?
            .set_default("keycloak.admin_password", "admin")?
            .set_default("logging.level", "info")?
            .set_default("logging.format", "json")?
            // Add environment-specific config file if exists
            .add_source(File::with_name(&format!("config/{}", environment)).required(false))
            // Add environment variables (APP_SERVER_HOST, APP_KEYCLOAK_BASE_URL, etc.)
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__")
                    .list_separator(",")
                    .try_parsing(true)
            );

        let config = config_builder.build()?;
        config.try_deserialize()
    }

    /// Get server bind address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        std::env::var("APP_ENV").unwrap_or_else(|_| "development".into()) == "development"
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        std::env::var("APP_ENV").unwrap_or_else(|_| "development".into()) == "production"
    }
}

impl ServerConfig {
    /// Get the number of workers, defaulting to available parallelism
    pub fn workers(&self) -> usize {
        self.workers.unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        })
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: Some(std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)),
            },
            keycloak: KeycloakConfig {
                base_url: "http://localhost:8080".to_string(),
                realm: "master".to_string(),
                client_id: "admin-cli".to_string(),
                client_secret: "secret".to_string(),
                admin_username: "admin".to_string(),
                admin_password: "admin".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.keycloak.realm, "master");
        assert_eq!(config.keycloak.client_id, "admin-cli");
        assert_eq!(config.logging.level, "info");
    }

    #[test]
    fn test_server_address() {
        let config = AppConfig::default();
        assert_eq!(config.server_address(), "127.0.0.1:8080");
    }

    #[test]
    fn test_workers_default() {
        let server_config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: None,
        };
        assert!(server_config.workers() >= 1);
    }

    #[test]
    fn test_environment_detection() {
        std::env::set_var("APP_ENV", "production");
        let config = AppConfig::default();
        assert!(config.is_production());
        assert!(!config.is_development());
        
        std::env::set_var("APP_ENV", "development");
        let config = AppConfig::default();
        assert!(config.is_development());
        assert!(!config.is_production());
        
        std::env::remove_var("APP_ENV");
        let config = AppConfig::default();
        assert!(config.is_development()); // Default to development
    }
}