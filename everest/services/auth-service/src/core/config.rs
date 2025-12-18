use crate::core::constants::{DEFAULT_HOST, DEFAULT_LOG_LEVEL, DEFAULT_PORT};
use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_addr: String,
    pub database_url: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            server_addr: format!(
                "{}:{}",
                env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string()),
                env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string())
            ),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string()),
        }
    }
}
