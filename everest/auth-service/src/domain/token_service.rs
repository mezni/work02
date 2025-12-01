// domain/token_service.rs
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::domain::errors::DomainError;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub struct TokenService;

impl TokenService {
    pub fn create_token(
        user_id: &str,
        secret: &[u8],
        expires_in_seconds: i64,
    ) -> Result<String, DomainError> {
        if user_id.is_empty() {
            return Err(DomainError::InvalidSubject);
        }

        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::seconds(expires_in_seconds)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            sub: user_id.to_string(),
            exp,
            iat,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret),
        )
        .map_err(|_| DomainError::TokenCreationError)
    }

    pub fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, DomainError> {
        let decoded = decode::<TokenClaims>(
            &token.into(),
            &DecodingKey::from_secret(secret),
            &Validation::new(Algorithm::HS256),
        );
        match decoded {
            Ok(token) => Ok(token.claims.sub),
            Err(_) => Err(DomainError::InvalidToken),
        }
    }
}
