// shared/src/telemetry.rs
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    // Check if JSON logging is enabled
    if std::env::var("JSON_LOGGING").is_ok() {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer())
            .init();
    }
    
    tracing::info!("Telemetry initialized");
}

// Health check response structures
#[derive(Debug, serde::Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ReadinessResponse {
    pub status: String,
    pub timestamp: String,
    pub dependencies: DependenciesHealth,
}

#[derive(Debug, serde::Serialize)]
pub struct DependenciesHealth {
    pub database: DependencyStatus,
    pub keycloak: DependencyStatus,
}

#[derive(Debug, serde::Serialize)]
pub struct DependencyStatus {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl HealthResponse {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl ReadinessResponse {
    pub fn ready(dependencies: DependenciesHealth) -> Self {
        let overall_status = if dependencies.database.status == "healthy" 
            && dependencies.keycloak.status == "healthy" {
            "ready"
        } else {
            "not_ready"
        };
        
        Self {
            status: overall_status.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            dependencies,
        }
    }
}

impl DependenciesHealth {
    pub fn new(database: DependencyStatus, keycloak: DependencyStatus) -> Self {
        Self { database, keycloak }
    }
}

impl DependencyStatus {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            error: None,
        }
    }
    
    pub fn unhealthy(error: String) -> Self {
        Self {
            status: "unhealthy".to_string(),
            error: Some(error),
        }
    }
}