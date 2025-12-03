use actix_web::{HttpResponse, Responder, get, post, web::Json};
use serde::Deserialize;
use utoipa::OpenApi;
use utoipa::{
    Modify, ToSchema,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(Deserialize, ToSchema)]
struct CreateUser {
    #[schema(example = "Eggs Benedict", required = false)]
    menu_item: Option<String>,
    #[schema(
        example = "The restaurant was clean and the staff were helpful!",
        required = true
    )]
    review_description: String,
}

pub struct AppState {}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct SimpleStatus {
    pub status: u16,
}

// POST endpoint for health check (if you want it to be POST)
#[utoipa::path(
    post,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Happy Path", body = SimpleStatus),
        (status = 400, description = "Missing information"),
        (status = 401, description = "Unauthorized user"),
        (status = 403, description = "Feature not turned on"),
        (status = 500, description = "Internal Server Error"),
    )
)]
#[post("/api/v1/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(SimpleStatus { status: 200 })
}

// POST endpoint for register with request_body (as requested)
#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = CreateUser,
    responses(
        (status = 200, description = "User registered successfully", body = SimpleStatus),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/api/v1/register")]
pub async fn register(user_data: Json<CreateUser>) -> impl Responder {
    // You can access the user data like this:
    // println!("Menu item: {:?}", user_data.menu_item);
    // println!("Review: {}", user_data.review_description);

    // Process the user registration here...

    HttpResponse::Ok().json(SimpleStatus { status: 200 })
}

pub struct SecurityModifier;

impl Modify for SecurityModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        components.add_security_scheme(
            "basic_auth",
            SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build()),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(health_check, register),
    components(schemas(SimpleStatus, CreateUser)),
    modifiers(&SecurityModifier)
)]
pub struct ApiDoc;
