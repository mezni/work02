use crate::AppState;
use std::sync::Arc;

#[derive(Clone)]
pub struct RegistrationService {
    state: Arc<AppState>,
}

impl RegistrationService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn register_user(&self) -> String {
        // Use self.state.db_pool or self.state.config here
        format!(
            "register_user for realm: {}",
            self.state.config.keycloak_realm
        )
    }

    pub async fn verify_registration(&self) -> String {
        "verify_registration placeholder".to_string()
    }

    pub async fn resend_verification(&self) -> String {
        "resend_verification placeholder".to_string()
    }
}
