use async_trait::async_trait;
use std::sync::Arc;
use serde::Serialize;
use super::{
    keycloak_client::KeycloakClient,
    cache::{Cache, RedisCache},
    error::{InfrastructureError, InfrastructureResult},
};

#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub service: String,
    pub status: HealthState,
    pub message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum HealthState {
    Healthy,
    Unhealthy,
    Degraded,
}

pub struct HealthChecker {
    keycloak_client: Arc<KeycloakClient>,
    cache: Option<Arc<dyn Cache>>,
}

impl HealthChecker {
    pub fn new(keycloak_client: Arc<KeycloakClient>, cache: Option<Arc<dyn Cache>>) -> Self {
        Self {
            keycloak_client,
            cache,
        }
    }
    
    pub async fn check_all(&self) -> Vec<HealthStatus> {
        let mut results = Vec::new();
        
        // Check Keycloak
        results.push(self.check_keycloak().await);
        
        // Check cache if available
        if let Some(cache) = &self.cache {
            results.push(self.check_cache(cache).await);
        }
        
        // Check application health
        results.push(self.check_application().await);
        
        results
    }
    
    pub async fn check_keycloak(&self) -> HealthStatus {
        let start_time = chrono::Utc::now();
        
        match self.keycloak_client.health_check().await {
            Ok(_) => HealthStatus {
                service: "keycloak".to_string(),
                status: HealthState::Healthy,
                message: Some("Keycloak is reachable".to_string()),
                timestamp: start_time,
            },
            Err(e) => HealthStatus {
                service: "keycloak".to_string(),
                status: HealthState::Unhealthy,
                message: Some(format!("Keycloak error: {}", e)),
                timestamp: start_time,
            },
        }
    }
    
    pub async fn check_cache(&self, cache: &Arc<dyn Cache>) -> HealthStatus {
        let start_time = chrono::Utc::now();
        
        // Try to perform a simple cache operation
        let test_key = "health_check";
        let test_value = "test_value";
        
        match cache.set(test_key, &test_value, Some(1)).await {
            Ok(_) => {
                match cache.get::<String>(test_key).await {
                    Ok(Some(value)) if value == test_value => {
                        let _ = cache.delete(test_key).await; // Clean up
                        HealthStatus {
                            service: "cache".to_string(),
                            status: HealthState::Healthy,
                            message: Some("Cache is working".to_string()),
                            timestamp: start_time,
                        }
                    }
                    Ok(_) => HealthStatus {
                        service: "cache".to_string(),
                        status: HealthState::Degraded,
                        message: Some("Cache returned incorrect value".to_string()),
                        timestamp: start_time,
                    },
                    Err(e) => HealthStatus {
                        service: "cache".to_string(),
                        status: HealthState::Unhealthy,
                        message: Some(format!("Cache error: {}", e)),
                        timestamp: start_time,
                    },
                }
            }
            Err(e) => HealthStatus {
                service: "cache".to_string(),
                status: HealthState::Unhealthy,
                message: Some(format!("Cache error: {}", e)),
                timestamp: start_time,
            },
        }
    }
    
    pub async fn check_application(&self) -> HealthStatus {
        HealthStatus {
            service: "application".to_string(),
            status: HealthState::Healthy,
            message: Some("Application is running".to_string()),
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub async fn is_healthy(&self) -> bool {
        let results = self.check_all().await;
        results.iter().all(|status| status.status == HealthState::Healthy)
    }
    
    pub async fn get_health_summary(&self) -> HealthSummary {
        let results = self.check_all().await;
        let total_services = results.len();
        let healthy_services = results.iter()
            .filter(|status| status.status == HealthState::Healthy)
            .count();
        
        HealthSummary {
            status: if healthy_services == total_services {
                HealthState::Healthy
            } else if healthy_services > 0 {
                HealthState::Degraded
            } else {
                HealthState::Unhealthy
            },
            services: results,
            total_services,
            healthy_services,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HealthSummary {
    pub status: HealthState,
    pub services: Vec<HealthStatus>,
    pub total_services: usize,
    pub healthy_services: usize,
}

impl HealthSummary {
    pub fn is_healthy(&self) -> bool {
        self.status == HealthState::Healthy
    }
}