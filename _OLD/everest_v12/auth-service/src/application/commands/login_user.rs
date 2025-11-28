use crate::application::dto::auth_request::LoginRequest;

pub struct LoginUserCommand {
    pub username: String,
    pub password: String,
}

impl From<LoginRequest> for LoginUserCommand {
    fn from(req: LoginRequest) -> Self {
        LoginUserCommand {
            username: req.username,
            password: req.password,
        }
    }
}
