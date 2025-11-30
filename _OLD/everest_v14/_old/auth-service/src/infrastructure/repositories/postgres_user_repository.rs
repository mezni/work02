use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::domain::{
    entities::User,
    repositories::UserRepository,
    value_objects::{Email, OrganisationName, Role},
    errors::DomainError,
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, keycloak_id, email, username, role, organisation_name, is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            keycloak_id: r.get("keycloak_id"),
            email: Email::new(r.get("email")).unwrap(),
            username: r.get("username"),
            role: Role::from_str(&r.get::<String, _>("role")).unwrap(),
            organisation_name: r.get::<Option<String>, _>("organisation_name")
                .map(|n| OrganisationName::new(n).unwrap()),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, keycloak_id, email, username, role, organisation_name, is_active, created_at, updated_at
            FROM users
            WHERE keycloak_id = $1
            "#,
        )
        .bind(keycloak_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            keycloak_id: r.get("keycloak_id"),
            email: Email::new(r.get("email")).unwrap(),
            username: r.get("username"),
            role: Role::from_str(&r.get::<String, _>("role")).unwrap(),
            organisation_name: r.get::<Option<String>, _>("organisation_name")
                .map(|n| OrganisationName::new(n).unwrap()),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, keycloak_id, email, username, role, organisation_name, is_active, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            keycloak_id: r.get("keycloak_id"),
            email: Email::new(r.get("email")).unwrap(),
            username: r.get("username"),
            role: Role::from_str(&r.get::<String, _>("role")).unwrap(),
            organisation_name: r.get::<Option<String>, _>("organisation_name")
                .map(|n| OrganisationName::new(n).unwrap()),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn save(&self, user: &User) -> Result<User, DomainError> {
        let role_str = user.role.to_string();
        let org_name = user.organisation_name.as_ref().map(|o| o.value());

        sqlx::query(
            r#"
            INSERT INTO users (id, keycloak_id, email, username, role, organisation_name, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(user.id)
        .bind(&user.keycloak_id)
        .bind(user.email.value())
        .bind(&user.username)
        .bind(&role_str)
        .bind(org_name)
        .bind(user.is_active)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(user.clone())
    }

    async fn update(&self, user: &User) -> Result<User, DomainError> {
        let role_str = user.role.to_string();
        let org_name = user.organisation_name.as_ref().map(|o| o.value());

        sqlx::query(
            r#"
            UPDATE users
            SET email = $2, username = $3, role = $4, organisation_name = $5, is_active = $6, updated_at = $7
            WHERE id = $1
            "#,
        )
        .bind(user.id)
        .bind(user.email.value())
        .bind(&user.username)
        .bind(&role_str)
        .bind(org_name)
        .bind(user.is_active)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(user.clone())
    }

    async fn list_by_organisation(&self, org_name: &str) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, keycloak_id, email, username, role, organisation_name, is_active, created_at, updated_at
            FROM users
            WHERE organisation_name = $1
            "#,
        )
        .bind(org_name)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| User {
                id: r.get("id"),
                keycloak_id: r.get("keycloak_id"),
                email: Email::new(r.get("email")).unwrap(),
                username: r.get("username"),
                role: Role::from_str(&r.get::<String, _>("role")).unwrap(),
                organisation_name: r.get::<Option<String>, _>("organisation_name")
                    .map(|n| OrganisationName::new(n).unwrap()),
                is_active: r.get("is_active"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })
            .collect())
    }

    async fn list_all(&self) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, keycloak_id, email, username, role, organisation_name, is_active, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| User {
                id: r.get("id"),
                keycloak_id: r.get("keycloak_id"),
                email: Email::new(r.get("email")).unwrap(),
                username: r.get("username"),
                role: Role::from_str(&r.get::<String, _>("role")).unwrap(),
                organisation_name: r.get::<Option<String>, _>("organisation_name")
                    .map(|n| OrganisationName::new(n).unwrap()),
                is_active: r.get("is_active"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })
            .collect())
    }
}