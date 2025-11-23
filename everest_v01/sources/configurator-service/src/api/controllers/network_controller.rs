use actix_web::{HttpResponse, Result, web};
use uuid::Uuid;
// Remove unused import: use utoipa::ToSchema;

use crate::api::dtos::requests::{CreateNetworkRequest, VerifyNetworkRequest};
use crate::api::dtos::responses::NetworkResponse;
use crate::application::{
    CreateNetworkCommand, GetNetworkQuery, ListNetworksQuery, NetworkApplicationService,
    VerifyNetworkCommand,
};
use crate::infrastructure::repositories::PostgresNetworkRepository;

// Type alias for service
type AppService = NetworkApplicationService<PostgresNetworkRepository>;

#[utoipa::path(
    post,
    path = "/api/networks",
    request_body = CreateNetworkRequest,
    responses(
        (status = 201, description = "Network created successfully", body = NetworkResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn create_network(
    service: web::Data<AppService>,
    request: web::Json<CreateNetworkRequest>,
) -> Result<HttpResponse> {
    // In a real app, you'd get this from authentication
    let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from auth

    let command = CreateNetworkCommand {
        name: request.name.clone(),                 // Clone the name
        network_type: request.network_type.clone(), // Clone the network_type
        created_by,
    };

    let network_dto = service.create_network(command).await?;
    let response = NetworkResponse::from(network_dto);

    Ok(HttpResponse::Created().json(response))
}

#[utoipa::path(
    post,
    path = "/api/networks/{network_id}/verify",
    params(
        ("network_id" = Uuid, Path, description = "Network ID")
    ),
    request_body = VerifyNetworkRequest,
    responses(
        (status = 200, description = "Network verified successfully", body = NetworkResponse),
        (status = 404, description = "Network not found"),
        (status = 409, description = "Network already verified"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn verify_network(
    service: web::Data<AppService>,
    network_id: web::Path<Uuid>,
    request: web::Json<VerifyNetworkRequest>,
) -> Result<HttpResponse> {
    let command = VerifyNetworkCommand {
        network_id: *network_id,
        verified_by: request.verified_by, // Uuid is Copy, so no clone needed
    };

    let network_dto = service.verify_network(command).await?;
    let response = NetworkResponse::from(network_dto);

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/networks/{network_id}",
    params(
        ("network_id" = Uuid, Path, description = "Network ID")
    ),
    responses(
        (status = 200, description = "Network found", body = NetworkResponse),
        (status = 404, description = "Network not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_network(
    service: web::Data<AppService>,
    network_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let query = GetNetworkQuery {
        network_id: *network_id,
    };

    match service.get_network(query).await? {
        Some(network_dto) => {
            let response = NetworkResponse::from(network_dto);
            Ok(HttpResponse::Ok().json(response))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Not Found",
            "message": "Network not found"
        }))),
    }
}

#[utoipa::path(
    get,
    path = "/api/networks",
    responses(
        (status = 200, description = "List of networks", body = [NetworkResponse]),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn list_networks(service: web::Data<AppService>) -> Result<HttpResponse> {
    let query = ListNetworksQuery;

    let networks_dto = service.list_networks(query).await?;
    let responses: Vec<NetworkResponse> = networks_dto
        .into_iter()
        .map(NetworkResponse::from)
        .collect();

    Ok(HttpResponse::Ok().json(responses))
}

#[utoipa::path(
    delete,
    path = "/api/networks/{network_id}",
    params(
        ("network_id" = Uuid, Path, description = "Network ID")
    ),
    responses(
        (status = 204, description = "Network deleted successfully"),
        (status = 404, description = "Network not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn delete_network(
    service: web::Data<AppService>,
    network_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    service.delete_network(*network_id).await?;

    Ok(HttpResponse::NoContent().finish())
}
