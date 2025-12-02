use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct User {
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        Self { username, email }
    }
}