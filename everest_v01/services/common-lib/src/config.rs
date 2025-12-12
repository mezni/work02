use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub service_name: String,
    pub environment: String,
    pub server: ServerConfig,
    #[cfg(feature = "database")]
    pub database: DatabaseConfig,
    #[cfg(feature = "redis")]
    pub redis: RedisConfig,
    #[cfg(feature = "messaging")]
    pub messaging: MessagingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub keep_alive: u64,
    pub client_timeout: u64,
}

#[cfg(feature = "database")]
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[cfg(feature = "redis")]
#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

#[cfg(feature = "messaging")]
#[derive(Debug, Deserialize, Clone)]
pub struct MessagingConfig {
    pub rabbitmq_url: String,
    pub exchange: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        let service_name = std::env::var("SERVICE_NAME")
            .unwrap_or_else(|_| "microservice".to_string());
        let environment = std::env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string());
        
        let server = ServerConfig {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            workers: std::env::var("WORKERS")
                .unwrap_or_else(|_| num_cpus::get().to_string())
                .parse()?,
            keep_alive: std::env::var("KEEP_ALIVE")
                .unwrap_or_else(|_| "75".to_string())
                .parse()?,
            client_timeout: std::env::var("CLIENT_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
        };
        
        #[cfg(feature = "database")]
        let database = DatabaseConfig {
            url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            min_connections: std::env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()?,
        };
        
        #[cfg(feature = "redis")]
        let redis = RedisConfig {
            url: std::env::var("REDIS_URL")
                .expect("REDIS_URL must be set"),
            pool_size: std::env::var("REDIS_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
        };
        
        #[cfg(feature = "messaging")]
        let messaging = MessagingConfig {
            rabbitmq_url: std::env::var("RABBITMQ_URL")
                .expect("RABBITMQ_URL must be set"),
            exchange: std::env::var("RABBITMQ_EXCHANGE")
                .unwrap_or_else(|_| "events".to_string()),
        };
        
        Ok(AppConfig {
            service_name,
            environment,
            server,
            #[cfg(feature = "database")]
            database,
            #[cfg(feature = "redis")]
            redis,
            #[cfg(feature = "messaging")]
            messaging,
        })
    }
}
