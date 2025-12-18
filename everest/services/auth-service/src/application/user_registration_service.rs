use crate::application::user_registration_dto::{RegisterUserRequest, RegisterUserResponse};
use crate::core::errors::AppError;
use chrono::{Duration, Utc};
use sqlx::PgPool;

pub struct UserRegistrationService;

impl UserRegistrationService {
    pub async fn execute(
        db: &PgPool,
        req: RegisterUserRequest,
    ) -> Result<RegisterUserResponse, AppError> {
        // 2. Process Registration logic (e.g., call Keycloak API here)
        // For now, we simulate a successful record creation
        let registration_id = "XXX".to_string();

        Ok(RegisterUserResponse {
            registration_id,
            status: "PENDING_VERIFICATION".to_string(),
            expires_at: (Utc::now() + Duration::hours(24)).to_rfc3339(),
        })
    }
}
