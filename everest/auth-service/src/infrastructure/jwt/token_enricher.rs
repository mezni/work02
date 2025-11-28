use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use crate::{
    domain::models::User,
    infrastructure::config::AppConfig,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichedClaims {
    pub sub: String, 
    pub exp: usize,  
    pub iat: usize,  
    pub role: String,
    pub organisation_name: Option<String>,
    pub station_name: Option<String>,
    pub preferred_username: String,
}

#[derive(Clone)]
pub struct JwtTokenEnricher {
    encoding_key: EncodingKey,
    expiration_hours: i64,
}

impl JwtTokenEnricher {
    pub fn new(config: &AppConfig) -> Self {
        let encoding_key = EncodingKey::from_secret(config.jwt.secret.as_ref());
        JwtTokenEnricher { 
            encoding_key,
            expiration_hours: config.jwt.expiration_hours,
        }
    }

    pub async fn enrich_and_encode(&self, user: &User) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now() + Duration::hours(self.expiration_hours);
        let iat = Utc::now().timestamp() as usize;

        // In a real implementation, you would fetch these from the database
        let (org_name, station_name) = match user.role {
            crate::domain::value_objects::Role::Admin => (None, None),
            crate::domain::value_objects::Role::Partner => (Some("PartnerOrg_A".to_string()), None),
            crate::domain::value_objects::Role::Operator => (Some("PartnerOrg_A".to_string()), Some("Station_X".to_string())),
            _ => (None, None),
        };

        let claims = EnrichedClaims {
            sub: user.id.to_string(),
            exp: expiration.timestamp() as usize,
            iat,
            role: user.role.to_string(),
            organisation_name: org_name,
            station_name: station_name,
            preferred_username: user.username.to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }
}
