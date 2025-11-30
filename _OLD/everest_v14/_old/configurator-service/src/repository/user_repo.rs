use crate::domain::user::{Role, User};
use crate::errors::AppError;
use sqlx::PgPool;

#[derive(Clone)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, user: &User) -> Result<User, AppError> {
        sqlx::query!(
            "INSERT INTO users (id, name, email, role, org_id, station_id) VALUES ($1, $2, $3, $4, $5, $6)",
            user.id,
            user.name,
            user.email,
            format!("{:?}", user.role),
            user.org_id,
            user.station_id
        )
        .execute(&self.pool)
        .await
        .map_err(|_| AppError::InternalError)?;

        Ok(user.clone())
    }
}
