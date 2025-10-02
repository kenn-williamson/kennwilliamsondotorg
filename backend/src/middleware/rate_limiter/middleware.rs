use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    HttpMessage, Error, Result,
    web::Data,
};
use uuid::Uuid;

use super::config::{RateLimitConfig, get_rate_limit_configs};
use super::trait_def::RateLimitServiceTrait;

/// Extract client identifier from request
fn get_client_identifier(req: &ServiceRequest) -> String {
    // Try to get user ID first (for authenticated requests)
    if let Some(user_id) = req.extensions().get::<Uuid>() {
        return format!("user:{}", user_id);
    }

    // Fall back to IP address for anonymous requests
    if let Some(peer_addr) = req.peer_addr() {
        return format!("ip:{}", peer_addr.ip());
    }

    // Last resort - use a default identifier
    "unknown".to_string()
}

/// Determine endpoint type from request path
fn get_endpoint_type(path: &str) -> String {
    if path.contains("/auth/register") {
        "register".to_string()
    } else if path.contains("/auth/login") {
        "login".to_string()
    } else if path.contains("/phrases") {
        "phrases".to_string()
    } else if path.contains("/incident-timers") {
        "timers".to_string()
    } else {
        "general".to_string()
    }
}

/// Rate limiting middleware
pub async fn rate_limit_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    log::debug!("Rate limit middleware called for: {} {}", req.method(), req.path());

    // Get rate limit service from app data
    let rate_limit_service = match req.app_data::<Data<dyn RateLimitServiceTrait>>() {
        Some(service) => {
            log::debug!("Rate limit service found in app data");
            service
        },
        None => {
            log::error!("Rate limit service not found in app data");
            log::error!("Looking for type: {}", std::any::type_name::<Data<dyn RateLimitServiceTrait>>());
            return Ok(next.call(req).await?);
        }
    };

    let identifier = get_client_identifier(&req);
    let endpoint_type = get_endpoint_type(req.path());

    // Get rate limit configuration
    let configs = get_rate_limit_configs();
    let config = match configs.get(&endpoint_type) {
        Some(config) => config,
        None => {
            log::debug!("No rate limit config for endpoint: {}", endpoint_type);
            return Ok(next.call(req).await?);
        }
    };

    // Check rate limit
    match rate_limit_service.check_rate_limit(&identifier, &endpoint_type, config).await {
        Ok(true) => {
            log::warn!("Rate limit exceeded for {} on {}", identifier, endpoint_type);
            return Err(actix_web::error::ErrorTooManyRequests(
                "Rate limit exceeded"
            ));
        }
        Ok(false) => {
            // Increment counters
            if let Err(e) = rate_limit_service.increment_rate_limit(&identifier, &endpoint_type, config).await {
                log::error!("Failed to increment rate limit: {}", e);
            }
        }
        Err(e) => {
            log::error!("Rate limit check failed: {}", e);
            // Fail open - allow request if Redis is down
        }
    }

    next.call(req).await
}

/// Admin rate limiting middleware (more restrictive)
pub async fn admin_rate_limit_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    // Get rate limit service from app data
    let rate_limit_service = match req.app_data::<Data<dyn RateLimitServiceTrait>>() {
        Some(service) => service,
        None => {
            log::error!("Admin rate limit service not found in app data");
            return Ok(next.call(req).await?);
        }
    };

    let identifier = get_client_identifier(&req);

    // Admin endpoints have stricter limits
    let config = RateLimitConfig {
        requests_per_hour: 50,
        burst_limit: 10,
        burst_window: 300, // 5 minutes
    };

    // Check rate limit
    match rate_limit_service.check_rate_limit(&identifier, "admin", &config).await {
        Ok(true) => {
            log::warn!("Admin rate limit exceeded for {}", identifier);
            return Err(actix_web::error::ErrorTooManyRequests(
                "Admin rate limit exceeded"
            ));
        }
        Ok(false) => {
            // Increment counters
            if let Err(e) = rate_limit_service.increment_rate_limit(&identifier, "admin", &config).await {
                log::error!("Failed to increment admin rate limit: {}", e);
            }
        }
        Err(e) => {
            log::error!("Admin rate limit check failed: {}", e);
            // Fail open - allow request if Redis is down
        }
    }

    next.call(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_type_detection() {
        assert_eq!(get_endpoint_type("/backend/public/auth/register"), "register");
        assert_eq!(get_endpoint_type("/backend/public/auth/login"), "login");
        assert_eq!(get_endpoint_type("/backend/protected/phrases/random"), "phrases");
        assert_eq!(get_endpoint_type("/backend/protected/incident-timers"), "timers");
        assert_eq!(get_endpoint_type("/backend/public/health"), "general");
        assert_eq!(get_endpoint_type("/backend/protected/admin/users"), "general");
    }
}
