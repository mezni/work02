use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    pub realm_access: Option<RealmAccess>,
    // Keycloak attributes
    pub network_id: Option<Vec<String>>,
    pub station_id: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

impl Claims {
    pub fn has_role(&self, role: &str) -> bool {
        self.realm_access
            .as_ref()
            .map(|r| r.roles.contains(&role.to_string()))
            .unwrap_or(false)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("ADMIN")
    }

    pub fn is_partner(&self) -> bool {
        self.has_role("PARTNER")
    }

    pub fn is_operator(&self) -> bool {
        self.has_role("OPERATOR")
    }

    pub fn get_network_id(&self) -> Option<String> {
        self.network_id.as_ref()?.first().cloned()
    }

    pub fn get_station_id(&self) -> Option<String> {
        self.station_id.as_ref()?.first().cloned()
    }
}

// Implement JWT middleware similar to station-service
// with JWKS fetching from Keycloak