use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::LocalBoxFuture;
use std::collections::HashMap;
use std::future::{ready, Ready};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    requests_per_minute: usize,
    state: Arc<Mutex<RateLimiterState>>,
}

struct RateLimiterState {
    requests: HashMap<String, Vec<Instant>>,
}

impl RateLimiter {
    pub fn new(requests_per_minute: usize) -> Self {
        Self {
            requests_per_minute,
            state: Arc::new(Mutex::new(RateLimiterState {
                requests: HashMap::new(),
            })),
        }
    }

    fn is_allowed(&self, key: &str) -> bool {
        let mut state = self.state.lock().unwrap();
        let now = Instant::now();
        let one_minute_ago = now - Duration::from_secs(60);

        // Get or create request history for this key
        let requests = state.requests.entry(key.to_string()).or_insert_with(Vec::new);

        // Remove old requests
        requests.retain(|&time| time > one_minute_ago);

        // Check if under limit
        if requests.len() < self.requests_per_minute {
            requests.push(now);
            true
        } else {
            false
        }
    }

    // Cleanup old entries periodically
    pub fn cleanup(&self) {
        let mut state = self.state.lock().unwrap();
        let now = Instant::now();
        let one_minute_ago = now - Duration::from_secs(60);

        state.requests.retain(|_, requests| {
            requests.retain(|&time| time > one_minute_ago);
            !requests.is_empty()
        });
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service,
            limiter: self.clone(),
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    limiter: RateLimiter,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
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
        // Get client IP as rate limit key
        let client_ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        if !self.limiter.is_allowed(&client_ip) {
            let response = HttpResponse::TooManyRequests()
                .json(serde_json::json!({
                    "error": "RATE_LIMIT_EXCEEDED",
                    "message": "Too many requests. Please try again later."
                }));

            let (req, _) = req.into_parts();
            return Box::pin(async move {
                Ok(ServiceResponse::new(req, response.map_into_boxed_body()))
            });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allows_requests() {
        let limiter = RateLimiter::new(5);

        // Should allow first 5 requests
        for _ in 0..5 {
            assert!(limiter.is_allowed("test_ip"));
        }

        // Should deny 6th request
        assert!(!limiter.is_allowed("test_ip"));
    }

    #[test]
    fn test_rate_limiter_different_keys() {
        let limiter = RateLimiter::new(2);

        assert!(limiter.is_allowed("ip1"));
        assert!(limiter.is_allowed("ip1"));
        assert!(!limiter.is_allowed("ip1"));

        // Different IP should have its own limit
        assert!(limiter.is_allowed("ip2"));
        assert!(limiter.is_allowed("ip2"));
        assert!(!limiter.is_allowed("ip2"));
    }

    #[test]
    fn test_rate_limiter_cleanup() {
        let limiter = RateLimiter::new(5);

        limiter.is_allowed("test_ip");
        limiter.cleanup();

        // Cleanup should not affect recent requests
        assert!(limiter.is_allowed("test_ip"));
    }
}