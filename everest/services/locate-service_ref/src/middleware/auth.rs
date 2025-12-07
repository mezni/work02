use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: Option<Vec<String>>,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub realm_access: Option<RealmAccess>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Clone)]
pub struct JwtMiddleware {
    pub keycloak_url: String,
    pub realm: String,
    pub required_roles: Vec<String>,
}

impl JwtMiddleware {
    pub fn new(keycloak_url: String, realm: String) -> Self {
        Self {
            keycloak_url,
            realm,
            required_roles: Vec::new(),
        }
    }

    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        self.required_roles = roles;
        self
    }

    async fn fetch_jwks(&self) -> Result<jsonwebtoken::jwk::JwkSet, String> {
        let jwks_url = format!(
            "{}/realms/{}/protocol/openid-connect/certs",
            self.keycloak_url, self.realm
        );

        let response = reqwest::get(&jwks_url)
            .await
            .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("JWKS endpoint returned: {}", response.status()));
        }

        response
            .json::<jsonwebtoken::jwk::JwkSet>()
            .await
            .map_err(|e| format!("Failed to parse JWKS: {}", e))
    }

    fn validate_token(&self, token: &str, jwks: &jsonwebtoken::jwk::JwkSet) -> Result<Claims, String> {
        let header = decode_header(token)
            .map_err(|e| format!("Invalid token header: {}", e))?;

        let kid = header.kid.ok_or("Token missing kid")?;

        let jwk = jwks
            .find(&kid)
            .ok_or("JWK not found for kid")?;

        let decoding_key = DecodingKey::from_jwk(jwk)
            .map_err(|e| format!("Failed to create decoding key: {}", e))?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[format!(
            "{}/realms/{}",
            self.keycloak_url, self.realm
        )]);

        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|e| format!("Token validation failed: {}", e))?;

        // Check required roles if specified
        if !self.required_roles.is_empty() {
            if let Some(realm_access) = &token_data.claims.realm_access {
                let has_required_role = self
                    .required_roles
                    .iter()
                    .any(|required| realm_access.roles.contains(required));

                if !has_required_role {
                    return Err("User does not have required role".to_string());
                }
            } else {
                return Err("No realm access in token".to_string());
            }
        }

        Ok(token_data.claims)
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            service: Rc::new(service),
            keycloak_url: self.keycloak_url.clone(),
            realm: self.realm.clone(),
            required_roles: self.required_roles.clone(),
        }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
    keycloak_url: String,
    realm: String,
    required_roles: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let keycloak_url = self.keycloak_url.clone();
        let realm = self.realm.clone();
        let required_roles = self.required_roles.clone();

        Box::pin(async move {
            // Extract token from Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok());

            let token = match auth_header {
                Some(header) if header.starts_with("Bearer ") => {
                    header.trim_start_matches("Bearer ")
                }
                _ => {
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "UNAUTHORIZED",
                                "message": "Missing or invalid Authorization header"
                            }))
                            .into_body(),
                    ));
                }
            };

            // Create middleware instance for validation
            let middleware = JwtMiddleware {
                keycloak_url,
                realm,
                required_roles,
            };

            // Fetch JWKS
            let jwks = match middleware.fetch_jwks().await {
                Ok(jwks) => jwks,
                Err(e) => {
                    tracing::error!("Failed to fetch JWKS: {}", e);
                    return Ok(req.into_response(
                        HttpResponse::InternalServerError()
                            .json(serde_json::json!({
                                "error": "INTERNAL_ERROR",
                                "message": "Failed to validate token"
                            }))
                            .into_body(),
                    ));
                }
            };

            // Validate token
            let claims = match middleware.validate_token(token, &jwks) {
                Ok(claims) => claims,
                Err(e) => {
                    tracing::warn!("Token validation failed: {}", e);
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "UNAUTHORIZED",
                                "message": e
                            }))
                            .into_body(),
                    ));
                }
            };

            // Add claims to request extensions
            req.extensions_mut().insert(claims);

            // Continue with the request
            svc.call(req).await
        })
    }
}

// Helper to extract claims from request
pub fn get_claims(req: &ServiceRequest) -> Option<Claims> {
    req.extensions().get::<Claims>().cloned()
}

// Helper for actix handlers to extract claims
pub mod extractors {
    use super::Claims;
    use actix_web::{dev::Payload, FromRequest, HttpRequest};
    use futures_util::future::{ready, Ready};

    impl FromRequest for Claims {
        type Error = actix_web::Error;
        type Future = Ready<Result<Self, Self::Error>>;

        fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
            match req.extensions().get::<Claims>() {
                Some(claims) => ready(Ok(claims.clone())),
                None => ready(Err(actix_web::error::ErrorUnauthorized(
                    "No claims found in request",
                ))),
            }
        }
    }
}