// src/lib.rs
pub mod domain;
pub mod error;
pub mod infrastructure;

use actix_web::{App, HttpServer, Responder, web};
use sqlx::Row;

#[derive(Clone)]
pub struct AppState {
    pub config: infrastructure::config::Config,
    pub db: infrastructure::db::Database,
}

impl AppState {
    pub async fn new(config: infrastructure::config::Config) -> Result<Self, error::Error> {
        let db = infrastructure::db::Database::new(&config).await?;
        Ok(Self { config, db })
    }

    pub async fn start(self) -> Result<(), error::Error> {
        let host = self.config.server_host.clone();
        let port = self.config.server_port.clone();

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(self.clone()))
                .route("/", web::get().to(root))
        })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await?;

        Ok(())
    }
}

async fn root(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.pool();
    let row = sqlx::query("SELECT 1").fetch_one(db).await.unwrap();
    let result: i32 = row.try_get(0).unwrap();
    format!("Hello, World! {}", result)
}
