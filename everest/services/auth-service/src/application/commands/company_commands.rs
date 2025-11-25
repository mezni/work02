use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCompanyCommand {
    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCompanyCommand {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCompanyCommand {
    pub company_id: Uuid,
    pub deleted_by: Uuid,
}