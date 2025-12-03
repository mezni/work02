use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;
use std::sync::Arc;
use crate::domain::token::{TokenClaims, Token};
use crate::config::AppConfig;
use super::error::{InfrastructureError, InfrastructureResult};

#[derive(Clone)]
pub struct JwtTokenGenerator {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    issuer: String,
    audience: String,
    expiration_hours: i64,
}

impl JwtTokenGenerator {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(config.jwt.secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.jwt.secret.as_bytes()),
            issuer: config.jwt.issuer.clone(),
            audience: "auth-service".to_string(),
            expiration_hours: config.jwt.expiration_hours,
        }
    }
    
    pub fn generate(&self, claims: &TokenClaims) -> InfrastructureResult<String> {
        let header = Header::new(Algorithm::HS256);
        
        encode(&header, claims, &self.encoding_key)
            .map_err(|e| InfrastructureError::TokenEncoding(e.into()))
    }
    
    pub fn validate(&self, token: &str) -> InfrastructureResult<TokenClaims> {
        let validation = Validation::new(Algorithm::HS256);
        
        let token_data = decode::<TokenClaims>(token, &self.decoding_key, &validation)
            .map_err(|e| InfrastructureError::TokenDecoding(e.to_string()))?;
        
        Ok(token_data.claims)
    }
    
    pub fn create_claims(
        &self,
        user_id: Uuid,
        email: String,
        role: crate::domain::value_objects::UserRole,
        company_name: String,
        station_name: String,
        is_active: bool,
        email_verified: bool,
    ) -> TokenClaims {
        TokenClaims::new(
            user_id,
            email,
            role,
            company_name,
            station_name,
            is_active,
            email_verified,
            self.issuer.clone(),
            self.audience.clone(),
            self.expiration_hours,
        )
    }
    
    pub fn generate_token_pair(
        &self,
        claims: &TokenClaims,
        refresh_token_days: i64,
    ) -> InfrastructureResult<(Token, String)> {
        let access_token = self.generate(claims)?;
        let refresh_token = Uuid::new_v4().to_string();
        
        let token = Token::new(
            access_token,
            refresh_token.clone(),
            (claims.exp - claims.iat) as i64,
        );
        
        Ok((token, refresh_token))
    }
    
    pub fn is_token_expired(&self, token: &str) -> bool {
        match self.validate(token) {
            Ok(claims) => {
                let now = Utc::now().timestamp();
                now >= claims.exp
            }
            Err(_) => true,
        }
    }
    
    pub fn get_remaining_time(&self, token: &str) -> Option<i64> {
        match self.validate(token) {
            Ok(claims) => {
                let now = Utc::now().timestamp();
                let remaining = claims.exp - now;
                if remaining > 0 {
                    Some(remaining)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}