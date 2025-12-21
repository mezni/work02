use crate::AppState;
use crate::application::health_dto::HealthResponseDto;

pub struct HealthService;

impl HealthService {
    pub async fn check_health(state: &AppState) -> HealthResponseDto {
        // 1. Check Database
        let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db).await {
            Ok(_) => "up",
            Err(_) => "down",
        };

        // 2. Check Keycloak
        let keycloak_status = match state.keycloak.get_user_info("ping").await {
            Err(crate::infrastructure::keycloak_client::AppError::NetworkError(_)) => "down",
            _ => "up",
        };

        let overall_status = if db_status == "up" && keycloak_status == "up" {
            "ok"
        } else {
            "unhealthy"
        };

        HealthResponseDto {
            status: overall_status.to_string(),
            database: db_status.to_string(),
            keycloak: keycloak_status.to_string(),
        }
    }
}
