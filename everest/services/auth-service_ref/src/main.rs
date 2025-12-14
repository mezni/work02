use auth_service::{core::config::Config, start_server};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::from_env()?;

    // Start server
    start_server(config).await?;

    Ok(())
}
