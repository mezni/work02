use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub environment: String,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub keycloak: KeycloakConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeycloakConfig {
    pub server_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: u64,
}

/// Load configuration from environment variables
pub fn load_config() -> Result<AppConfig, config::ConfigError> {
    let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    
    let config = config::Config::builder()
        .add_source(
            config::Environment::with_prefix("AUTH_SERVICE")
                .separator("_")
                .list_separator(",")
        )
        .set_default("environment", environment)?
        .set_default("server.host", "0.0.0.0")?
        .set_default("server.port", 8080)?
        .set_default("database.max_connections", 10)?
        .set_default("jwt.expiration_hours", 24)?
        .build()?;
    
    config.try_deserialize()
}