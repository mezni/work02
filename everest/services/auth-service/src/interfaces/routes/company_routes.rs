use actix_web::{web, HttpResponse};
use uuid::Uuid;
use utoipa::OpenApi;
use crate::interfaces::controllers::company_controller::*;
use crate::application::dto::{CreateCompanyRequest, UpdateCompanyRequest};

#[utoipa::path(
    post,
    path = "/companies",
    request_body = CreateCompanyRequest,
    responses(
        (status = 201, description = "Company created successfully", body = CompanyDto),
        (status = 400, description = "Invalid input"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_company(
    controller: web::Data<CompanyController>,
    request: web::Json<CreateCompanyRequest>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.create_company(request.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    get,
    path = "/companies/{id}",
    params(
        ("id" = Uuid, Path, description = "Company ID")
    ),
    responses(
        (status = 200, description = "Company found", body = CompanyDto),
        (status = 404, description = "Company not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_company(
    controller: web::Data<CompanyController>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.get_company(path.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    get,
    path = "/companies",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Companies retrieved", body = CompanyListResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_companies(
    controller: web::Data<CompanyController>,
    query: web::Query<ListCompaniesQuery>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.list_companies(query.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    put,
    path = "/companies/{id}",
    params(
        ("id" = Uuid, Path, description = "Company ID")
    ),
    request_body = UpdateCompanyRequest,
    responses(
        (status = 200, description = "Company updated successfully", body = CompanyDto),
        (status = 404, description = "Company not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_company(
    controller: web::Data<CompanyController>,
    path: web::Path<Uuid>,
    request: web::Json<UpdateCompanyRequest>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.update_company(path.into_inner(), request.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    delete,
    path = "/companies/{id}",
    params(
        ("id" = Uuid, Path, description = "Company ID")
    ),
    responses(
        (status = 204, description = "Company deleted successfully"),
        (status = 404, description = "Company not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_company(
    controller: web::Data<CompanyController>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.delete_company(path.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    get,
    path = "/companies/{id}/users",
    params(
        ("id" = Uuid, Path, description = "Company ID"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Company users retrieved", body = CompanyUsersResponse),
        (status = 404, description = "Company not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_company_users(
    controller: web::Data<CompanyController>,
    path: web::Path<Uuid>,
    query: web::Query<CompanyUsersQuery>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.get_company_users(path.into_inner(), query.into_inner(), user_id.into_inner()).await
}

#[derive(serde::Deserialize)]
pub struct ListCompaniesQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(serde::Deserialize)]
pub struct CompanyUsersQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

pub fn configure_company_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/companies")
            .service(
                web::resource("")
                    .route(web::post().to(create_company))
                    .route(web::get().to(list_companies))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_company))
                    .route(web::put().to(update_company))
                    .route(web::delete().to(delete_company))
            )
            .service(
                web::resource("/{id}/users")
                    .route(web::get().to(get_company_users))
            )
    );
}