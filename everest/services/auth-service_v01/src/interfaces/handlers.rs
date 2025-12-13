use crate::application::dto::*;
use crate::core::errors::AppError;
use crate::domain::audit_entity::GeoLocation;
use crate::domain::value_objects::UserRole;
use crate::interfaces::middleware::auth::AuthenticatedUser;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

// Helper functions
fn extract_geo_info(req: &HttpRequest) -> Option<GeoLocation> {
    let ip = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string())?;

    // In production, use a GeoIP database like MaxMind
    Some(GeoLocation::new(ip))
}

fn extract_user_agent(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}

// Health check
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse::healthy())
}

// Authentication handlers
pub async fn register(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    let user = state
        .auth_service
        .register(body.into_inner(), geo, user_agent)
        .await?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn login(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    let response = state
        .auth_service
        .login(body.into_inner(), geo, user_agent)
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

pub async fn logout(
    state: web::Data<AppState>,
    req: HttpRequest,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    state
        .auth_service
        .logout(&auth.user_id, geo, user_agent)
        .await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Logged out successfully")))
}

pub async fn change_password(
    state: web::Data<AppState>,
    req: HttpRequest,
    auth: AuthenticatedUser,
    body: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse, AppError> {
    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    state
        .auth_service
        .change_password(
            &auth.user_id,
            &body.current_password,
            &body.new_password,
            geo,
            user_agent,
        )
        .await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Password changed successfully")))
}

pub async fn request_password_reset(
    state: web::Data<AppState>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, AppError> {
    let email = body
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("Email is required".to_string()))?;

    state.auth_service.request_password_reset(email).await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Password reset email sent")))
}

// User management handlers
pub async fn create_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    auth: AuthenticatedUser,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user is admin
    let role = auth.role.parse::<UserRole>()
        .map_err(|_| AppError::Unauthorized("Invalid role".to_string()))?;
    
    if !role.can_create_users() {
        return Err(AppError::Forbidden("Only admins can create users".to_string()));
    }

    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    let user = state
        .user_service
        .create_user_by_admin(body.into_inner(), &auth.user_id, geo, user_agent)
        .await?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn get_user(
    state: web::Data<AppState>,
    path: web::Path<String>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let user = state.user_service.get_user(&user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn get_current_user(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user = state.user_service.get_user(&auth.user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    auth: AuthenticatedUser,
    body: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Parse role
    let role = auth.role.parse::<UserRole>()
        .map_err(|_| AppError::Unauthorized("Invalid role".to_string()))?;

    // Users can only update themselves unless they're admin
    if auth.user_id != user_id && !role.is_admin() {
        return Err(AppError::Forbidden(
            "You can only update your own profile".to_string(),
        ));
    }

    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    let user = state
        .user_service
        .update_user(&user_id, body.into_inner(), &auth.user_id, geo, user_agent)
        .await?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Parse role
    let role = auth.role.parse::<UserRole>()
        .map_err(|_| AppError::Unauthorized("Invalid role".to_string()))?;

    // Only admins can delete users
    if !role.can_delete_users() {
        return Err(AppError::Forbidden("Only admins can delete users".to_string()));
    }

    let user_id = path.into_inner();
    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    state
        .user_service
        .delete_user(&user_id, &auth.user_id, geo, user_agent)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn list_users(
    state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let users = state
        .user_service
        .list_users(query.limit, query.offset)
        .await?;

    let total = state.user_service.count_users().await?;

    let response = PaginatedResponse::new(users, total, query.limit, query.offset);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn search_users(
    state: web::Data<AppState>,
    query: web::Query<SearchParams>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let users = state
        .user_service
        .search_users(&query.query, query.limit, query.offset)
        .await?;

    let response = PaginatedResponse::new(users, users.len() as i64, query.limit, query.offset);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_users_by_network(
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<PaginationParams>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let network_id = path.into_inner();
    let users = state
        .user_service
        .get_users_by_network(&network_id, query.limit, query.offset)
        .await?;

    let response = PaginatedResponse::new(users, users.len() as i64, query.limit, query.offset);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_users_by_station(
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<PaginationParams>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let station_id = path.into_inner();
    let users = state
        .user_service
        .get_users_by_station(&station_id, query.limit, query.offset)
        .await?;

    let response = PaginatedResponse::new(users, users.len() as i64, query.limit, query.offset);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_users_by_role(
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<PaginationParams>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let role = path.into_inner();
    let users = state
        .user_service
        .get_users_by_role(&role, query.limit, query.offset)
        .await?;

    let response = PaginatedResponse::new(users, users.len() as i64, query.limit, query.offset);
    Ok(HttpResponse::Ok().json(response))
}

// Audit handlers
pub async fn get_user_audit_logs(
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<PaginationParams>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Parse role
    let role = auth.role.parse::<UserRole>()
        .map_err(|_| AppError::Unauthorized("Invalid role".to_string()))?;

    // Users can only view their own audit logs unless they're admin
    if auth.user_id != user_id && !role.is_admin() {
        return Err(AppError::Forbidden(
            "You can only view your own audit logs".to_string(),
        ));
    }

    let logs = state
        .user_service
        .get_user_audit_logs(&user_id, query.limit, query.offset)
        .await?;

    let total = state.user_service.count_user_audit_logs(&user_id).await?;

    let response = PaginatedResponse::new(logs, total, query.limit, query.offset);
    Ok(HttpResponse::Ok().json(response))
}