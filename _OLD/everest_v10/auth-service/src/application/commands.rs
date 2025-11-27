use uuid::Uuid;

use crate::domain::enums::UserRole;

#[derive(Debug)]
pub struct CreateUserCommand {
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub company_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct UpdateUserCommand {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<UserRole>,
    pub company_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct DeleteUserCommand {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct CreateCompanyCommand {
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
}

#[derive(Debug)]
pub struct UpdateCompanyCommand {
    pub company_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct DeleteCompanyCommand {
    pub company_id: Uuid,
}

#[derive(Debug)]
pub struct AssignUserToCompanyCommand {
    pub user_id: Uuid,
    pub company_id: Uuid,
}

#[derive(Debug)]
pub struct ChangeUserRoleCommand {
    pub user_id: Uuid,
    pub new_role: UserRole,
}
