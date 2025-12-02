use crate::infrastructure::errors::InfrastructureError as Error;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_url: String,
    pub jwt_secret_key: String,
    pub jwt_maxage: String,
    pub log_level: String,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        dotenv().ok();

        let server_host = env::var("AUTH_SERVER_HOST")
            .or_else(|_| env::var("auth_server_host"))
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let server_port = env::var("AUTH_SERVER_PORT")
            .or_else(|_| env::var("auth_server_port"))
            .unwrap_or_else(|_| "3000".to_string());

        let db_user = env::var("AUTH_DB_USER")
            .or_else(|_| env::var("auth_db_user"))
            .map_err(|e| Error::ConfigError(dotenvy::Error::EnvVar(e)))?;

        let db_password = env::var("AUTH_DB_PASSWORD")
            .or_else(|_| env::var("auth_db_password"))
            .map_err(|e| Error::ConfigError(dotenvy::Error::EnvVar(e)))?;

        let db_name = env::var("AUTH_DB_NAME")
            .or_else(|_| env::var("auth_db_name"))
            .map_err(|e| Error::ConfigError(dotenvy::Error::EnvVar(e)))?;

        let db_url = env::var("AUTH_DB_URL")
            .or_else(|_| env::var("auth_db_url"))
            .map_err(|e| Error::ConfigError(dotenvy::Error::EnvVar(e)))?;

        let jwt_secret_key = env::var("JWT_SECRET_KEY")
            .or_else(|_| env::var("jwt_secret_key"))
            .map_err(|e| Error::ConfigError(dotenvy::Error::EnvVar(e)))?;

        let jwt_maxage = env::var("JWT_MAXAGE")
            .or_else(|_| env::var("jwt_maxage"))
            .map_err(|e| Error::ConfigError(dotenvy::Error::EnvVar(e)))?;

        let log_level = env::var("LOG_LEVEL")
            .or_else(|_| env::var("log_level"))
            .map_err(|e| Error::ConfigError(dotenvy::Error::EnvVar(e)))?;

        Ok(Self {
            server_host,
            server_port,
            db_user,
            db_password,
            db_name,
            db_url,
            jwt_secret_key,
            jwt_maxage,
            log_level,
        })
    }
}
