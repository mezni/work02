use configurator_service::{init_telemetry, Application, AppConfig};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    init_telemetry();

    // Load configuration
    let config = match AppConfig::load() {
        Ok(config) => {
            println!("✅ Configuration loaded from file");
            config
        }
        Err(e) => {
            println!("⚠️ Using default configuration: {}", e);
            AppConfig::default()
        }
    };

    // Build and run the application
    let application = Application::build(config).await?;
    application.run().await?;

    Ok(())
}