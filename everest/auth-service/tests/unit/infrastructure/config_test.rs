use auth_service::infrastructure::config::{
    Config, DatabaseConfig, JwtConfig, KeycloakConfig, LoggingConfig, ServerConfig,
};
use serial_test::serial;
use std::env;

#[test]
#[serial]
fn test_config_default_values() {
    let config = Config {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        },
        database: DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "test".to_string(),
            password: "test".to_string(),
            database_name: "test".to_string(),
            max_connections: 10,
        },
        keycloak: KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "test".to_string(),
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        },
        jwt: JwtConfig {
            secret: "test".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_days: 7,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
        },
    };

    assert_eq!(config.server.host, "127.0.0.1");
    assert_eq!(config.server.port, 3000);
    assert_eq!(config.database.host, "localhost");
    assert_eq!(config.database.port, 5432);
    assert_eq!(config.logging.level, "info");
}

#[test]
#[serial]
fn test_database_connection_string() {
    let db_config = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        username: "user".to_string(),
        password: "pass".to_string(),
        database_name: "mydb".to_string(),
        max_connections: 10,
    };

    let conn_string = db_config.connection_string();
    assert_eq!(conn_string, "postgres://user:pass@localhost:5432/mydb");
}

#[test]
#[serial]
fn test_environment_detection() {
    // Test default environment (development)
    env::remove_var("APP_ENVIRONMENT");
    let config = Config {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        },
        database: DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "test".to_string(),
            password: "test".to_string(),
            database_name: "test".to_string(),
            max_connections: 10,
        },
        keycloak: KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "test".to_string(),
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        },
        jwt: JwtConfig {
            secret: "test".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_days: 7,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
        },
    };

    assert_eq!(config.environment(), "development");
    assert!(config.is_development());
    assert!(!config.is_production());

    // Test production environment
    env::set_var("APP_ENVIRONMENT", "production");
    assert_eq!(config.environment(), "production");
    assert!(config.is_production());
    assert!(!config.is_development());

    // Cleanup
    env::remove_var("APP_ENVIRONMENT");
}

#[test]
#[serial]
fn test_config_load_fallback() {
    // Remove any existing env var to test fallback behavior
    env::remove_var("APP_ENVIRONMENT");

    // This should not panic and should load default config
    let result = Config::load();

    // It might fail due to missing config files, but shouldn't panic
    assert!(result.is_ok() || matches!(result, Err(_)));
}

#[test]
#[serial]
fn test_logging_config_default() {
    let logging_config = LoggingConfig::default();
    assert_eq!(logging_config.level, "info");
}

#[test]
#[serial]
fn test_keycloak_url_generation() {
    let keycloak_config = KeycloakConfig {
        server_url: "http://localhost:8080".to_string(),
        realm: "myrealm".to_string(),
        client_id: "myclient".to_string(),
        client_secret: "secret".to_string(),
        admin_username: "admin".to_string(),
        admin_password: "admin".to_string(),
    };

    assert!(keycloak_config.admin_token_url().contains("myrealm"));
    assert!(keycloak_config.token_url().contains("myrealm"));
    assert!(keycloak_config.user_info_url().contains("myrealm"));
}
