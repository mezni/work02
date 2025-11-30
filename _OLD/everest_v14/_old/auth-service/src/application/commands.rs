use crate::domain::value_objects::Role;

#[derive(Debug)]
pub struct CreateUserCommand {
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: Role,
    pub organisation_name: Option<String>,
    pub requester_role: Role,
    pub keycloak_id: Option<String>, // Add this field    
}
