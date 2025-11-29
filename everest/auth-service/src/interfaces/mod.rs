pub mod handlers;
pub mod middleware;
pub mod routes;

// src/interfaces/mod.rs
use crate::application::services::auth_service::AuthService;
use crate::application::services::user_service::UserService;
use std::sync::Arc;

pub struct AppState {
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub fn new(user_service: Arc<UserService>, auth_service: Arc<AuthService>) -> Self {
        Self {
            user_service,
            auth_service,
        }
    }
}
