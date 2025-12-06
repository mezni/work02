#[utoipa::path(
    post,
    path = "/api/v1/networks",
    request_body = CreateNetworkRequest,
    responses(
        (status = 201, description = "Network created", body = Network),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin only")
    ),
    tag = "Networks",
    security(("bearer_auth" = ["ADMIN"]))
)]
pub async fn create_network(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    request: web::Json<CreateNetworkRequest>,
    claims: Claims, // Extracted by middleware
) -> Result<impl Responder, DomainError> {
    let network_repo = Arc::new(PostgresNetworkRepository::new(pool.get_ref().clone()));
    let service = NetworkService::new(network_repo);

    let network = service.create_network(request.into_inner(), &claims).await?;

    Ok(HttpResponse::Created().json(network))
}