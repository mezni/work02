// main.rs // configurator-service/src/main.rs
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;

mod api;
mod config;
mod health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    init_telemetry();

    // Load configuration
    let config = match config::AppConfig::load() {
        Ok(config) => {
            println!("✅ Configuration loaded from file");
            config
        }
        Err(e) => {
            println!("⚠️ Using default configuration: {}", e);
            config::AppConfig::default()
        }
    };

    // Create database connection pool
    let pool = match PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.connection_string())
        .await
    {
        Ok(pool) => {
            println!("✅ Database connected successfully");
            pool
        }
        Err(e) => {
            println!("❌ Failed to connect to database: {}", e);
            println!("💡 Make sure PostgreSQL is running: docker-compose up -d postgres");
            std::process::exit(1);
        }
    };

    println!("🚀 Starting Configurator Service");
    println!("📍 Server: {}:{}", config.server.host, config.server.port);
    println!(
        "📊 Health: http://{}:{}/api/v1/health",
        config.server.host, config.server.port
    );
    println!(
        "📚 API Docs: http://{}:{}/docs",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            // ✅ API Scope: Contains health AND the JSON file handler
            .service(
                web::scope("/api/v1")
                    .configure(health::configure) // /api/v1/health, /api/v1/ready
                    .configure(api::docs::configure_json), // /api/v1/api-docs/openapi.json
            )
            // ✅ DOCS UI Scope: Contains only the UI interface at the root
            .configure(api::configure) // /docs
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}

fn init_telemetry() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("info".parse().unwrap()),
        )
        .init();

    tracing::info!("Telemetry initialized");
}
