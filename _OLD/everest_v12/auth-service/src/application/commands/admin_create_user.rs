use crate::application::dto::auth_request::AdminCreateUserRequest;
use uuid::Uuid;

pub struct AdminCreateUserCommand {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub organisation_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
}

impl From<AdminCreateUserRequest> for AdminCreateUserCommand {
    fn from(req: AdminCreateUserRequest) -> Self {
        AdminCreateUserCommand {
            username: req.username,
            email: req.email,
            password: req.password,
            role: req.role,
            organisation_id: req.organisation_id,
            station_id: req.station_id,
        }
    }
}
