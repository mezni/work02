use actix_web::{App, HttpServer, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod domain;
mod application;
mod infrastructure;
mod interfaces;

use application::user_service::UserService;
use infrastructure::user_repository::InMemoryUserRepository;
use interfaces::controllers::*;
use interfaces::openapi::ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = InMemoryUserRepository;
    let user_service = UserService::new(repo);

    let openapi = ApiDoc::openapi();

    println!("🚀 Server starting at: http://localhost:8080");
    println!("📚 Swagger UI: http://localhost:8080/swagger-ui/");
    println!("📄 OpenAPI spec: http://localhost:8080/api-docs/openapi.json");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .service(health)
            .service(register_user)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}