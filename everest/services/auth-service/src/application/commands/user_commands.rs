use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::domain::enums::UserRole;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserCommand {
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    pub role: UserRole,

    pub company_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserCommand {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    pub company_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeUserRoleCommand {
    pub user_id: Uuid,
    pub new_role: UserRole,
    pub changed_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignUserToCompanyCommand {
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub assigned_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUserFromCompanyCommand {
    pub user_id: Uuid,
    pub removed_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProfileCommand {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,

    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChangePasswordCommand {
    pub current_password: String,

    #[validate(length(min = 8))]
    pub new_password: String,

    #[validate(must_match = "new_password")]
    pub confirm_password: String,
}