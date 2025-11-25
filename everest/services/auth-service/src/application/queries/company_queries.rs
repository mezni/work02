use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCompanyByIdQuery {
    pub company_id: Uuid,
    pub requested_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCompaniesQuery {
    pub page: u32,
    pub page_size: u32,
    pub requested_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCompanyUsersQuery {
    pub company_id: Uuid,
    pub page: u32,
    pub page_size: u32,
    pub requested_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchCompaniesQuery {
    pub query: String,
    pub page: u32,
    pub page_size: u32,
    pub requested_by: Uuid,
}