use actix_web::{web, HttpResponse};
use crate::service::station_service::StationService;
use crate::domain::station::Station;
use crate::errors::AppError;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreateStationRequest {
    pub name: String,
    pub org_id: Option<Uuid>,
}

#[utoipa::path(
    post,
    path = "/station/create",
    request_body = CreateStationRequest,
    responses(
        (status = 200, description = "Station created", body = Station),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_station(
    svc: web::Data<StationService>,
    req: web::Json<CreateStationRequest>,
) -> Result<HttpResponse, AppError> {
    let station = svc.create_station(req.name.clone(), req.org_id).await?;
    Ok(HttpResponse::Ok().json(station))
}
