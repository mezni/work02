use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub keycloak: KeycloakConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakConfig {
    pub url: String,
    pub realm: String,
    pub auth_client_id: String,
    pub backend_client_id: String,
    pub backend_client_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let database = DatabaseConfig {
            url: std::env::var("DATABASE_URL")?,
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            min_connections: std::env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()?,
        };

        let server = ServerConfig {
            host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
        };

        let keycloak = KeycloakConfig {
            url: std::env::var("KEYCLOAK_URL")?,
            realm: std::env::var("KEYCLOAK_REALM")?,
            auth_client_id: std::env::var("KEYCLOAK_AUTH_CLIENT_ID")?,
            backend_client_id: std::env::var("KEYCLOAK_BACKEND_CLIENT_ID")?,
            backend_client_secret: std::env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")?,
        };

        let logging = LoggingConfig {
            level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        };

        Ok(Config {
            database,
            server,
            keycloak,
            logging,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        std::env::set_var("DATABASE_URL", "postgresql://localhost/test");
        std::env::set_var("KEYCLOAK_URL", "http://localhost:8080");
        std::env::set_var("KEYCLOAK_REALM", "test");
        std::env::set_var("KEYCLOAK_AUTH_CLIENT_ID", "test-client");
        std::env::set_var("KEYCLOAK_BACKEND_CLIENT_ID", "test-backend");
        std::env::set_var("KEYCLOAK_BACKEND_CLIENT_SECRET", "secret");

        let config = Config::from_env().unwrap();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 3000);
    }
}