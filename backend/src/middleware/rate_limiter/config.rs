use std::collections::HashMap;

/// Rate limiting configuration for different endpoints
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RateLimitConfig {
    pub requests_per_hour: u32,
    pub burst_limit: u32,
    pub burst_window: u32, // seconds
}

/// Rate limit configurations for different endpoints
#[allow(dead_code)]
pub fn get_rate_limit_configs() -> HashMap<String, RateLimitConfig> {
    let mut configs = HashMap::new();

    // Registration - with CAPTCHA protection, can be less restrictive
    // Turnstile provides primary bot defense, rate limiting is secondary
    configs.insert(
        "register".to_string(),
        RateLimitConfig {
            requests_per_hour: 10,  // Increased from 3 (better UX for shared IPs)
            burst_limit: 2,          // Increased from 1
            burst_window: 300,       // 5 minutes (unchanged)
        },
    );

    // Login - restrictive but allows retries
    configs.insert(
        "login".to_string(),
        RateLimitConfig {
            requests_per_hour: 10,
            burst_limit: 3,
            burst_window: 300, // 5 minutes
        },
    );

    // Phrase requests - protect against API abuse
    configs.insert(
        "phrases".to_string(),
        RateLimitConfig {
            requests_per_hour: 100,
            burst_limit: 20,
            burst_window: 600, // 10 minutes
        },
    );

    // General API - allow normal usage
    configs.insert(
        "general".to_string(),
        RateLimitConfig {
            requests_per_hour: 300,
            burst_limit: 50,
            burst_window: 300, // 5 minutes
        },
    );

    // Timer operations - moderate protection
    configs.insert(
        "timers".to_string(),
        RateLimitConfig {
            requests_per_hour: 200,
            burst_limit: 30,
            burst_window: 600, // 10 minutes
        },
    );

    configs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_configs() {
        let configs = get_rate_limit_configs();

        // Test that all expected endpoints have configurations
        assert!(configs.contains_key("register"));
        assert!(configs.contains_key("login"));
        assert!(configs.contains_key("phrases"));
        assert!(configs.contains_key("general"));
        assert!(configs.contains_key("timers"));

        // Test registration rate limits (with CAPTCHA protection)
        let register_config = configs.get("register").unwrap();
        assert_eq!(register_config.requests_per_hour, 10);
        assert_eq!(register_config.burst_limit, 2);

        // Test that general API is least restrictive
        let general_config = configs.get("general").unwrap();
        assert_eq!(general_config.requests_per_hour, 300);
        assert_eq!(general_config.burst_limit, 50);
    }

    #[test]
    fn test_rate_limit_config_creation() {
        let config = RateLimitConfig {
            requests_per_hour: 100,
            burst_limit: 20,
            burst_window: 600,
        };

        assert_eq!(config.requests_per_hour, 100);
        assert_eq!(config.burst_limit, 20);
        assert_eq!(config.burst_window, 600);
    }
}
