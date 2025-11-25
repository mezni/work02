use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserByIdQuery {
    pub user_id: Uuid,
    pub requested_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserByEmailQuery {
    pub email: String,
    pub requested_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersQuery {
    pub page: u32,
    pub page_size: u32,
    pub company_id: Option<Uuid>,
    pub role: Option<String>,
    pub requested_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserProfileQuery {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUsersQuery {
    pub query: String,
    pub page: u32,
    pub page_size: u32,
    pub requested_by: Uuid,
}