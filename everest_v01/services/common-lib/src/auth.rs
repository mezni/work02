

#[cfg(feature = "auth")]
pub mod authentication {
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};
    use chrono::{Duration, Utc};
    use actix_web::{
        dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
        Error, HttpMessage, error::ErrorUnauthorized,
    };
    use futures_util::future::LocalBoxFuture;
    use std::future::{ready, Ready};
    
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Claims {
        pub sub: String,
        pub exp: i64,
        pub iat: i64,
    }
    
    pub fn create_jwt(
        user_id: &str,
        secret: &str,
        expiry_hours: i64,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let expiry = now + Duration::hours(expiry_hours);
        
        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp(),
            exp: expiry.timestamp(),
        };
        
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }
    
    pub fn validate_jwt(
        token: &str,
        secret: &str,
    ) -> Result<Claims, jsonwebtoken::errors::Error> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )?;
        
        Ok(token_data.claims)
    }
    
    // JWT Authentication Middleware
    pub struct JwtAuth {
        secret: String,
    }
    
    impl JwtAuth {
        pub fn new(secret: String) -> Self {
            Self { secret }
        }
    }
    
    impl<S, B> Transform<S, ServiceRequest> for JwtAuth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
    {
        type Response = ServiceResponse<B>;
        type Error = Error;
        type InitError = ();
        type Transform = JwtAuthMiddleware<S>;
        type Future = Ready<Result<Self::Transform, Self::InitError>>;

        fn new_transform(&self, service: S) -> Self::Future {
            ready(Ok(JwtAuthMiddleware {
                service,
                secret: self.secret.clone(),
            }))
        }
    }
    
    pub struct JwtAuthMiddleware<S> {
        service: S,
        secret: String,
    }
    
    impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
    {
        type Response = ServiceResponse<B>;
        type Error = Error;
        type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

        forward_ready!(service);

        fn call(&self, req: ServiceRequest) -> Self::Future {
            let auth_header = req.headers().get("Authorization");
            
            let token = match auth_header {
                Some(header) => {
                    let auth_str = header.to_str().unwrap_or("");
                    if auth_str.starts_with("Bearer ") {
                        auth_str.trim_start_matches("Bearer ")
                    } else {
                        return Box::pin(async { Err(ErrorUnauthorized("Invalid auth header")) });
                    }
                }
                None => return Box::pin(async { Err(ErrorUnauthorized("Missing auth header")) }),
            };
            
            let claims = match validate_jwt(token, &self.secret) {
                Ok(c) => c,
                Err(_) => return Box::pin(async { Err(ErrorUnauthorized("Invalid token")) }),
            };
            
            req.extensions_mut().insert(claims);
            
            let fut = self.service.call(req);
            Box::pin(async move { fut.await })
        }
    }
}