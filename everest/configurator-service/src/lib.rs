pub mod application;
pub mod error;
pub mod infrastructure;
pub mod interfaces;

use actix_web::{App, HttpServer, web};
use infrastructure::config::Config;
use tracing::info;

#[derive(Debug, Clone)]
pub struct AppState {
    pub app_name: String,
    pub version: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_name: "My Actix Web App".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

pub async fn run() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    let config = Config::load()?;
    infrastructure::logger::init(&config);
    let state = AppState::new();

    info!("Starting server on {}:{}", config.host, config.port);
    info!("App: {} v{}", state.app_name, state.version);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(interfaces::http::routes::configure)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
    .map_err(|e| anyhow::anyhow!("Server error: {}", e))
}

pub use error::AppError;
