// src/main.rs
use auth_service::AppState;
use auth_service::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let config = auth_service::infrastructure::config::Config::new()?;
    auth_service::infrastructure::logger::init_logger(&config);
    auth_service::infrastructure::logger::log_info("Application started");

    let app_state = AppState::new(config).await?;
    app_state.start().await?;

    Ok(())
}
