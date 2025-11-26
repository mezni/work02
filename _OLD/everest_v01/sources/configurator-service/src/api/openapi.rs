use crate::api::dtos::requests::{CreateNetworkRequest, VerifyNetworkRequest}; // FIXED IMPORT
use crate::api::dtos::responses::NetworkResponse; // FIXED IMPORT
use crate::domain::enums::network_type::NetworkType;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::controllers::create_network,
        crate::api::controllers::verify_network,
        crate::api::controllers::get_network,
        crate::api::controllers::list_networks,
        crate::api::controllers::delete_network,
    ),
    components(
        schemas(
            CreateNetworkRequest,
            VerifyNetworkRequest,
            NetworkResponse,
            NetworkType,
        )
    ),
    tags(
        (name = "networks", description = "Network management API")
    )
)]
pub struct ApiDoc;
