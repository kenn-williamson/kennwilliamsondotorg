use anyhow::{Context, Result};
use async_trait::async_trait;
use redis::{Client, Commands};

use crate::repositories::traits::PkceStorage;

/// Redis-based PKCE storage implementation
///
/// Stores PKCE code verifiers temporarily with TTL for OAuth flows.
/// Keys are prefixed with "oauth:google:state:" to namespace them.
#[derive(Clone)]
pub struct RedisPkceStorage {
    redis_client: Client,
}

impl RedisPkceStorage {
    /// Create a new Redis PKCE storage instance
    ///
    /// # Arguments
    /// * `redis_url` - Redis connection URL (e.g., "redis://localhost:6379")
    pub fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)
            .context("Failed to create Redis client for PKCE storage")?;
        Ok(Self {
            redis_client: client,
        })
    }

    /// Generate Redis key for PKCE verifier
    fn pkce_key(state: &str) -> String {
        format!("oauth:google:state:{}", state)
    }
}

#[async_trait]
impl PkceStorage for RedisPkceStorage {
    async fn store_pkce(&self, state: &str, verifier: &str, ttl_seconds: u64) -> Result<()> {
        let mut conn = self
            .redis_client
            .get_connection()
            .context("Failed to get Redis connection")?;

        let key = Self::pkce_key(state);

        // Store verifier with TTL
        let _: () = conn
            .set(&key, verifier)
            .context("Failed to store PKCE verifier in Redis")?;

        let _: () = conn
            .expire(&key, ttl_seconds as i64)
            .context("Failed to set TTL on PKCE verifier")?;

        log::debug!("Stored PKCE verifier for state {} with TTL {}s", state, ttl_seconds);

        Ok(())
    }

    async fn retrieve_and_delete_pkce(&self, state: &str) -> Result<Option<String>> {
        let mut conn = self
            .redis_client
            .get_connection()
            .context("Failed to get Redis connection")?;

        let key = Self::pkce_key(state);

        // Get the verifier
        let verifier: Option<String> = conn
            .get(&key)
            .context("Failed to retrieve PKCE verifier from Redis")?;

        // If found, delete it (single-use)
        if verifier.is_some() {
            let _: () = conn
                .del(&key)
                .context("Failed to delete PKCE verifier from Redis")?;

            log::debug!("Retrieved and deleted PKCE verifier for state {}", state);
        } else {
            log::warn!("No PKCE verifier found for state {} (expired or invalid)", state);
        }

        Ok(verifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkce_key_format() {
        let state = "test-state-123";
        let key = RedisPkceStorage::pkce_key(state);
        assert_eq!(key, "oauth:google:state:test-state-123");
    }
}
