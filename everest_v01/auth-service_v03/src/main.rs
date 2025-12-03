use actix_web::{App, HttpServer, web::Data};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use auth_service::*; // Replace with your actual crate name

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {}))
            //            .service(index)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("localhost", 3000))?
    .run()
    .await
}
