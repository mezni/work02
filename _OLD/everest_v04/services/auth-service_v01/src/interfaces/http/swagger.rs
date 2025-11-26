use utoipa::OpenApi;
use crate::application::dtos::{CreateUserDTO};

#[derive(OpenApi)]
#[openapi(
    paths(crate::interfaces::http::handlers::create_user),
    components(schemas(CreateUserDTO)),
    tags(
        (name = "Users", description = "Users management endpoints")
    )
)]
pub struct ApiDoc;
