// shared/src/config.rs
use serde::Deserialize;
use config::{Config, File, Environment};

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub keycloak_url: String,
    pub realm: String,
    pub client_id: String,
    #[serde(default = "default_jwt_leeway")]
    pub jwt_leeway: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: Vec<String>,
    #[serde(default = "default_allowed_methods")]
    pub allowed_methods: Vec<String>,
    #[serde(default = "default_allowed_headers")]
    pub allowed_headers: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub cors: CorsConfig,
}

// Default values (keep these)
fn default_host() -> String { "0.0.0.0".to_string() }
fn default_port() -> u16 { 8080 }
fn default_log_level() -> String { "info".to_string() }
fn default_max_connections() -> u32 { 20 }
fn default_jwt_leeway() -> u64 { 60 }
fn default_allowed_origins() -> Vec<String> { vec!["*".to_string()] }
fn default_allowed_methods() -> Vec<String> {
    vec![
        "GET".to_string(),
        "POST".to_string(), 
        "PUT".to_string(),
        "DELETE".to_string(),
        "OPTIONS".to_string(),
    ]
}
fn default_allowed_headers() -> Vec<String> { vec!["*".to_string()] }

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let environment = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        
        println!("Loading configuration for environment: {}", environment);
        
        let config = Config::builder()
            .add_source(File::with_name("../config/default").required(false))
            .add_source(Environment::with_prefix("EV_CHARGING").separator("_"))
            .build()?;
            
        match config.try_deserialize() {
            Ok(app_config) => {
                println!("Configuration loaded successfully from file!");
                Ok(app_config)
            },
            Err(e) => {
                println!("Failed to load config file ({}), using defaults", e);
                Ok(Self::default())
            }
        }
    }
    
    pub fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                log_level: "info".to_string(),
            },
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                username: "ev_charging".to_string(),
                password: "password".to_string(),
                database_name: "ev_charging_configurator".to_string(),
                max_connections: 20,
            },
            auth: AuthConfig {
                keycloak_url: "http://localhost:8080".to_string(),
                realm: "ev-charging".to_string(),
                client_id: "configurator-service".to_string(),
                jwt_leeway: 60,
            },
            cors: CorsConfig {
                allowed_origins: vec!["*".to_string()],
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                    "OPTIONS".to_string(),
                ],
                allowed_headers: vec!["*".to_string()],
            },
        }
    }
}