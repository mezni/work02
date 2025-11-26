use crate::domain::services::auth_service::AuthService;
use crate::application::dtos::requests::CreateUserRequest;
use crate::domain::models::user::User;

#[derive(Clone)]
pub struct UserAppService<T: AuthService> {
    auth_service: T,
}

impl<T: AuthService> UserAppService<T> {
    pub fn new(auth_service: T) -> Self {
        Self { auth_service }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<User> {
        self.auth_service.create_user(
            &request.username,
            request.email.as_deref(),
            request.first_name.as_deref(),
            request.last_name.as_deref(),
            &request.password,
        ).await
    }
}