// src/api/mod.rs
pub mod handlers;
pub mod middleware;
pub mod responses;
pub mod routes;

use actix_web::{web, App, HttpServer};

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
