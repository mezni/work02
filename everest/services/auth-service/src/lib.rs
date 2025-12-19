pub mod core {
    pub mod config;
    pub mod constants;
    pub mod database;
    pub mod errors;
    pub mod id_generator;
    pub mod logging;
    pub mod state;
}

use crate::core::database::create_pool;
use crate::core::state::AppState;
use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, web};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub async fn run() -> anyhow::Result<()> {
    let config = core::config::Config::from_env();
    let pool = create_pool(&config.database_url).await?;

    // Wrap state in web::Data (Actix's way of sharing state)
    let app_state = web::Data::new(AppState::new(config.clone(), pool));
    let server_addr = config.server_addr.clone();

    tracing::info!("Starting Actix server on {}", server_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Logging middleware
            .app_data(app_state.clone()) // Inject state
            .route("/health", web::get().to(health_check))
        // .configure(routes::init) // You can add your routes here
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
