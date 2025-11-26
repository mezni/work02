use uuid::Uuid;

use crate::domain::enums::UserRole;

#[derive(Debug)]
pub struct GetUserByIdQuery {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct GetUserByEmailQuery {
    pub email: String,
}

#[derive(Debug)]
pub struct GetUserByKeycloakIdQuery {
    pub keycloak_id: String,
}

#[derive(Debug)]
pub struct ListUsersQuery {
    pub company_id: Option<Uuid>,
    pub role: Option<UserRole>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug)]
pub struct GetCompanyByIdQuery {
    pub company_id: Uuid,
}

#[derive(Debug)]
pub struct ListCompaniesQuery {
    pub user_id: Option<Uuid>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug)]
pub struct ListCompanyUsersQuery {
    pub company_id: Uuid,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug)]
pub struct ValidateTokenQuery {
    pub token: String,
}
