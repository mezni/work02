use crate::domain::user::{Role, User};
use crate::errors::AppError;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_user(
        &self,
        name: String,
        email: String,
        role: Role,
        org_id: Option<Uuid>,
        station_id: Option<Uuid>,
    ) -> Result<User, AppError> {
        Ok(User::new(name, email, role, org_id, station_id))
    }
}
