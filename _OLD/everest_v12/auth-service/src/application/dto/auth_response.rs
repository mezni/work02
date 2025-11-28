use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct AuthResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
    #[schema(example = "bearer")]
    pub token_type: String,
    #[schema(example = 3600)]
    pub expires_in: i64,
    pub user: UserDTO,
}

impl AuthResponse {
    pub fn new(token: String, user: UserDTO) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user,
        }
    }
}
