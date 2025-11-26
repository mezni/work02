use crate::domain::enums::UserRole;

pub struct CreateUserCommand {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub company: Option<String>,
}
