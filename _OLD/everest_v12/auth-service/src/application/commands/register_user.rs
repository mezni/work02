use crate::application::dto::auth_request::RegisterRequest;

pub struct RegisterUserCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<RegisterRequest> for RegisterUserCommand {
    fn from(req: RegisterRequest) -> Self {
        RegisterUserCommand {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}
