use dotenvy::dotenv;
use serde::Deserialize;
use config::{Config as ConfigBuilder, Environment, File};

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
    pub expiration_hours: i64,
    pub issuer: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub enable_cors: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub default_ttl: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String, // "json" or "text"
    pub enable_structured: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub password_min_length: usize,
    pub require_special_char: bool,
    pub require_numbers: bool,
    pub max_login_attempts: u32,
    pub lockout_minutes: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub environment: String,
    pub log_level: String,
    pub database: DatabaseConfig,
    pub keycloak: KeycloakConfig,
    pub jwt: JwtConfig,
    pub server: ServerConfig,
    pub redis: Option<RedisConfig>,
    pub logging: LoggingConfig,
    pub rate_limit: RateLimitConfig,
    pub security: SecurityConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        dotenv().ok();
        
        let environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".into());
        
        let config_builder = ConfigBuilder::builder()
            // Load default config
            .add_source(File::with_name("config/default").required(false))
            // Load environment-specific config
            .add_source(File::with_name(&format!("config/{}", environment)).required(false))
            // Load local config (for overrides)
            .add_source(File::with_name("config/local").required(false))
            // Load from environment variables with APP_ prefix
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?;
        
        let config: AppConfig = config_builder.try_deserialize()?;
        
        // Validate configuration
        config.validate()?;
        
        Ok(config)
    }
    
    fn validate(&self) -> Result<(), config::ConfigError> {
        // Validate server config
        if self.server.port == 0 {
            return Err(config::ConfigError::Message("Server port cannot be 0".into()));
        }
        
        // Validate JWT secret
        if self.jwt.secret.is_empty() {
            return Err(config::ConfigError::Message("JWT secret cannot be empty".into()));
        }
        
        // Validate Keycloak config
        if self.keycloak.server_url.is_empty() {
            return Err(config::ConfigError::Message("Keycloak server URL cannot be empty".into()));
        }
        
        // Validate security config
        if self.security.password_min_length < 8 {
            return Err(config::ConfigError::Message("Password minimum length must be at least 8".into()));
        }
        
        Ok(())
    }
    
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
    
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }
    
    pub fn is_test(&self) -> bool {
        self.environment == "test"
    }
    
    pub fn get_log_level(&self) -> tracing::Level {
        match self.log_level.as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        }
    }
    
    pub fn get_cors_origins(&self) -> Vec<String> {
        if self.is_development() {
            vec![
                "http://localhost:3000".to_string(),
                "http://localhost:8080".to_string(),
                "http://127.0.0.1:3000".to_string(),
                "http://127.0.0.1:8080".to_string(),
            ]
        } else {
            // In production, you would load from config
            vec![]
        }
    }
}