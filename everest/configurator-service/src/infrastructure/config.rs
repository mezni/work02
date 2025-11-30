use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        info!("Loading configuration...");

        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid PORT: {}", e))?;

        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        Ok(Config {
            host,
            port,
            log_level,
        })
    }
}
