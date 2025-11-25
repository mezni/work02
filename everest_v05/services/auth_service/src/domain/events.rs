// You can expand events later
#[derive(Debug, Clone)]
pub enum UserEvent {
    Created { user_id: String },
    RoleAssigned { user_id: String, role: String },
}

#[derive(Debug, Clone)]
pub enum CompanyEvent {
    Created { company_id: String },
}
