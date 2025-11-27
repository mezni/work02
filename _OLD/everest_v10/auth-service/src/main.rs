use auth_service::infrastructure::{config, logger};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger first
    logger::init_logger();

    // Load configuration
    let config = config::Config::load()?;

    tracing::info!("Starting {} service...", env!("CARGO_PKG_NAME"));
    tracing::info!("Environment: {}", config.environment());
    tracing::info!("Server: {}:{}", config.server.host, config.server.port);

    // Start Actix-web server
    auth_service::start_server(config).await?;

    Ok(())
}
