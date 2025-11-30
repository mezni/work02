// configurator-service/src/lib.rs
pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used types
pub use application::{
    ApplicationError, ApplicationResult, CompositeApplicationService,
    OrganizationApplicationService, StationApplicationService, UserApplicationService,
};
pub use infrastructure::repositories::{
    OrganizationRepositoryImpl, RepositoryFactory, StationRepositoryImpl, UserRepositoryImpl,
};

pub mod api;
pub mod config;
pub mod health;

// Re-export commonly used items for easier access
pub use config::AppConfig;

// Application state and initialization
use actix_web::web;
use sqlx::PgPool;

pub struct Application {
    pool: PgPool,
    config: AppConfig,
}

impl Application {
    pub async fn build(config: AppConfig) -> Result<Self, anyhow::Error> {
        // Create database connection pool
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .connect(&config.database.connection_string())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

        println!("✅ Database connected successfully");

        // Removed migrations for now to simplify

        Ok(Self { pool, config })
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let pool = self.pool;
        let host = self.config.server.host.clone();
        let port = self.config.server.port;

        println!("🚀 Starting Configurator Service");
        println!("📍 Server: {}:{}", host, port);
        println!("📊 Health: http://{}:{}/api/v1/health", host, port);
        println!(
            "🏢 Organizations: http://{}:{}/api/v1/organizations",
            host, port
        );
        println!("📚 API Docs: http://{}:{}/docs", host, port);

        let server = actix_web::HttpServer::new(move || {
            actix_web::App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(
                    actix_cors::Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header(),
                )
                // Configure all API routes through the api module
                .configure(api::configure)
        })
        .bind((host, port))?
        .run();

        server.await
    }
}

// Telemetry initialization
pub fn init_telemetry() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("info".parse().unwrap()),
        )
        .init();

    tracing::info!("Telemetry initialized");
}
