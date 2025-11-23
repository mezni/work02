// src/application/queries/get_user_by_id.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetUserByIdQuery {
    pub user_id: String,
}

impl GetUserByIdQuery {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}