use actix_web::{App, HttpServer, Responder, web};
use tracing::info;

mod infrastructure;

use infrastructure::config::Config;
use infrastructure::logger;

async fn hello() -> impl Responder {
    info!("Hello endpoint called");
    "Hello, World!"
}

async fn health_check() -> impl Responder {
    "OK"
}

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration first
    let config = Config::load()?;

    // Initialize logger with config
    logger::init(&config);

    info!("Starting server on {}:{}", config.host, config.port);
    info!("Log level: {}", config.log_level);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/health", web::get().to(health_check))
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
    .map_err(|e| anyhow::anyhow!("Server error: {}", e))
}
