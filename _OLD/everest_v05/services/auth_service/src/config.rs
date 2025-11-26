use dotenvy::dotenv;
use std::env;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct KeycloakConfig {
    pub url: String,
    pub admin_user: String,
    pub admin_password: String,
    pub realm: String,
    pub client: String,
    pub initial_admin_username: String,
    pub initial_admin_email: String,
    pub initial_admin_password: String,
}

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub db: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u16,
    pub jwt: JwtConfig,
    pub keycloak: KeycloakConfig,
    pub postgres: PostgresConfig,
}

impl ServiceConfig {
    pub fn from_env() -> Self {
        // Load .env file
        dotenv().ok();

        // Keycloak
        let keycloak = KeycloakConfig {
            url: env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set"),
            admin_user: env::var("KEYCLOAK_ADMIN").expect("KEYCLOAK_ADMIN must be set"),
            admin_password: env::var("KEYCLOAK_ADMIN_PASSWORD").expect("KEYCLOAK_ADMIN_PASSWORD must be set"),
            realm: env::var("REALM_NAME").expect("REALM_NAME must be set"),
            client: env::var("CLIENT_NAME").expect("CLIENT_NAME must be set"),
            initial_admin_username: env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set"),
            initial_admin_email: env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set"),
            initial_admin_password: env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set"),
        };

        // Postgres
        let postgres = PostgresConfig {
            host: env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".into()),
            port: env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".into()).parse().unwrap_or(5432),
            db: env::var("POSTGRES_DB").unwrap_or_else(|_| "auth_service".into()),
            user: env::var("POSTGRES_USER").unwrap_or_else(|_| "auth_user".into()),
            password: env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "secret".into()),
        };

        // JWT
        let jwt = JwtConfig {
            secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            expiration_seconds: env::var("JWT_EXPIRATION_SECONDS")
                .unwrap_or_else(|_| "3600".into())
                .parse()
                .unwrap_or(3600),
        };

        // Service
        ServiceConfig {
            host: env::var("SERVICE_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("SERVICE_PORT").unwrap_or_else(|_| "8080".into()).parse().unwrap_or(8080),
            jwt,
            keycloak,
            postgres,
        }
    }

    /// Example helper: build Postgres connection string
    pub fn postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres.user, self.postgres.password, self.postgres.host, self.postgres.port, self.postgres.db
        )
    }

    /// JWT expiration as Duration
    pub fn jwt_expiration(&self) -> Duration {
        Duration::from_secs(self.jwt.expiration_seconds)
    }
}
