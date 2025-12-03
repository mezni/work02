use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::sync::Arc;
use crate::{
    domain::User,
    application::{
        TokenService,
        service_traits::TokenServiceTrait,
        error::ApplicationError,
    },
    interfaces::error::InterfaceError,
};

// Auth Middleware
pub struct AuthMiddleware<S> {
    service: Rc<S>,
    token_service: Arc<dyn TokenServiceTrait>,
    require_admin: bool,
}

impl<S> AuthMiddleware<S> {
    pub fn admin() -> AuthMiddlewareFactory {
        AuthMiddlewareFactory { require_admin: true }
    }
}

pub struct AuthMiddlewareFactory {
    require_admin: bool,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // In real implementation, you would inject token_service
        let token_service = Arc::new(TokenService::new(
            todo!("Inject UserRepository"),
            todo!("Inject TokenRepository"),
        ));
        
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
            token_service,
            require_admin: self.require_admin,
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let token_service = Arc::clone(&self.token_service);
        let require_admin = self.require_admin;
        
        Box::pin(async move {
            // Extract token from Authorization header
            let token = extract_token_from_request(&req).await?;
            
            if token.is_none() {
                return Ok(req.error_response(InterfaceError::MissingAuthHeader));
            }
            
            let token = token.unwrap();
            
            // Validate token
            match token_service.validate_token(&token).await {
                Ok(user) => {
                    // Check admin requirement
                    if require_admin && !user.role.is_admin() {
                        return Ok(req.error_response(
                            actix_web::error::ErrorForbidden("Admin access required")
                        ));
                    }
                    
                    // Insert user into request extensions
                    req.extensions_mut().insert(user);
                    
                    // Call next middleware/service
                    service.call(req).await
                }
                Err(e) => {
                    // Handle specific token errors
                    let interface_error = match e {
                        ApplicationError::TokenExpired => InterfaceError::InvalidRequest("Token expired".to_string()),
                        ApplicationError::AccountDisabled => InterfaceError::InvalidRequest("Account disabled".to_string()),
                        ApplicationError::InvalidToken(msg) => InterfaceError::InvalidRequest(msg),
                        _ => InterfaceError::InvalidAuthHeader,
                    };
                    
                    Ok(req.error_response(interface_error))
                }
            }
        })
    }
}

async fn extract_token_from_request(req: &ServiceRequest) -> Result<Option<String>, InterfaceError> {
    // Try to get token from Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str[7..].trim().to_string();
                if !token.is_empty() {
                    return Ok(Some(token));
                }
            }
        }
        return Err(InterfaceError::InvalidAuthHeader);
    }
    
    // Try to get token from query parameter (for WebSocket connections, etc.)
    if let Some(token) = req.query_string().split('&')
        .find(|param| param.starts_with("token="))
        .and_then(|param| param.split('=').nth(1))
    {
        if !token.is_empty() {
            return Ok(Some(token.to_string()));
        }
    }
    
    Ok(None)
}

// Error Handler Middleware
pub struct ErrorHandler;

impl<S, B> Transform<S, ServiceRequest> for ErrorHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ErrorHandlerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorHandlerMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct ErrorHandlerMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ErrorHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        
        Box::pin(async move {
            let response = service.call(req).await;
            
            match response {
                Ok(resp) => Ok(resp),
                Err(err) => {
                    // Convert Actix error to our error response
                    let status = err.as_response_error().status_code();
                    let error_message = err.to_string();
                    
                    let error_response = match status {
                        actix_web::http::StatusCode::NOT_FOUND => {
                            InterfaceError::EndpointNotFound.error_response()
                        }
                        actix_web::http::StatusCode::METHOD_NOT_ALLOWED => {
                            InterfaceError::MethodNotAllowed.error_response()
                        }
                        actix_web::http::StatusCode::UNSUPPORTED_MEDIA_TYPE => {
                            InterfaceError::UnsupportedMediaType.error_response()
                        }
                        actix_web::http::StatusCode::PAYLOAD_TOO_LARGE => {
                            InterfaceError::PayloadTooLarge.error_response()
                        }
                        _ => {
                            // Check if it's already our InterfaceError
                            if let Some(interface_err) = err.as_error::<InterfaceError>() {
                                interface_err.error_response()
                            } else {
                                // Generic error
                                HttpResponse::InternalServerError().json(serde_json::json!({
                                    "error": "Internal Server Error",
                                    "message": error_message,
                                    "code": "INTERNAL_ERROR"
                                }))
                            }
                        }
                    };
                    
                    Ok(ServiceResponse::new(req.into_parts().0, error_response))
                }
            }
        })
    }
}

// Rate Limiter Middleware (simplified)
pub struct RateLimiter;

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // In production, you would:
        // 1. Extract client identifier (IP, user ID, API key)
        // 2. Check rate limit in Redis or similar store
        // 3. Increment counter
        // 4. Apply limits based on endpoint
        
        // For now, just pass through
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            service.call(req).await
        })
    }
}

// Helper to extract user from request extensions
pub fn extract_user_from_request(req: &ServiceRequest) -> Result<User, InterfaceError> {
    req.extensions()
        .get::<User>()
        .cloned()
        .ok_or_else(|| InterfaceError::InvalidAuthHeader)
}