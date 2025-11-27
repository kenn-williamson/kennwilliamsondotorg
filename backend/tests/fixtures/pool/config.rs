//! Pool configuration with sensible defaults and environment variable overrides.

use std::time::Duration;

/// Configuration for the testcontainer pool.
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of containers in the pool.
    pub max_size: usize,

    /// Timeout waiting for container checkout.
    pub checkout_timeout: Duration,

    /// Timeout for container creation.
    pub create_timeout: Duration,

    /// Timeout for database reset (recycle).
    pub recycle_timeout: Duration,

    /// PostgreSQL image to use.
    pub postgres_image: String,

    /// Image tag.
    pub postgres_tag: String,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_size: std::env::var("TESTCONTAINER_POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(|| std::cmp::min(num_cpus::get(), 4)),
            checkout_timeout: Duration::from_secs(30),
            create_timeout: Duration::from_secs(60),
            recycle_timeout: Duration::from_secs(30), // Migrations take time
            postgres_image: "ghcr.io/fboulnois/pg_uuidv7".to_string(),
            postgres_tag: "1.6.0".to_string(),
        }
    }
}

impl PoolConfig {
    /// Configuration optimized for CI (fewer resources).
    pub fn ci() -> Self {
        Self {
            max_size: 2,
            checkout_timeout: Duration::from_secs(60),
            create_timeout: Duration::from_secs(120),
            recycle_timeout: Duration::from_secs(60),
            ..Default::default()
        }
    }

    /// Detect if running in CI and return appropriate config.
    pub fn auto() -> Self {
        if std::env::var("CI").is_ok() || std::env::var("GITHUB_ACTIONS").is_ok() {
            Self::ci()
        } else {
            Self::default()
        }
    }
}
