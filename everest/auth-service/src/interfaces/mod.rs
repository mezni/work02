pub mod handlers;
pub mod routes;

use crate::application::services::user_service::UserService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}

impl AppState {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }
}