use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CompanyDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    #[schema(example = "Acme Corporation")]
    pub name: String,
    
    #[schema(example = "A leading technology company specializing in innovative solutions")]
    pub description: Option<String>,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub created_by: Uuid,
    
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCompanyRequest {
    #[schema(example = "Acme Corporation")]
    pub name: String,
    
    #[schema(example = "A leading technology company specializing in innovative solutions")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateCompanyRequest {
    #[schema(example = "Acme Corporation Ltd.")]
    pub name: Option<String>,
    
    #[schema(example = "A global leader in innovative technology solutions")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CompanyListResponse {
    pub companies: Vec<CompanyDto>,
    
    #[schema(example = 50)]
    pub total: u64,
    
    #[schema(example = 1)]
    pub page: u32,
    
    #[schema(example = 20)]
    pub page_size: u32,
    
    #[schema(example = 3)]
    pub total_pages: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CompanyUsersResponse {
    pub users: Vec<super::user_dto::UserDto>,
    
    #[schema(example = 25)]
    pub total: u64,
    
    #[schema(example = 1)]
    pub page: u32,
    
    #[schema(example = 20)]
    pub page_size: u32,
    
    #[schema(example = 2)]
    pub total_pages: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CompanySearchRequest {
    #[schema(example = "Acme")]
    pub query: String,
    
    #[schema(example = 1)]
    pub page: u32,
    
    #[schema(example = 20)]
    pub page_size: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransferOwnershipRequest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub new_owner_id: Uuid,
}