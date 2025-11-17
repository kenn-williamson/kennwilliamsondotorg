use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::repositories::traits::PkceStorage;

/// Entry in the mock PKCE storage with expiration time
#[derive(Clone)]
struct PkceEntry {
    verifier: String,
    expires_at: u64, // Unix timestamp
}

/// Mock PKCE storage for testing
///
/// Uses an in-memory HashMap with manual expiration checking.
/// Thread-safe via Arc<Mutex>.
#[derive(Clone)]
pub struct MockPkceStorage {
    storage: Arc<Mutex<HashMap<String, PkceEntry>>>,
}

impl MockPkceStorage {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get current Unix timestamp in seconds
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    /// Clean up expired entries (manual cleanup for testing)
    #[allow(dead_code)] // Part of testing infrastructure API
    pub fn cleanup_expired(&self) {
        let now = Self::current_timestamp();
        let mut storage = self.storage.lock().unwrap();
        storage.retain(|_, entry| entry.expires_at > now);
    }
}

impl Default for MockPkceStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PkceStorage for MockPkceStorage {
    async fn store_pkce(&self, state: &str, verifier: &str, ttl_seconds: u64) -> Result<()> {
        let expires_at = Self::current_timestamp() + ttl_seconds;
        let entry = PkceEntry {
            verifier: verifier.to_string(),
            expires_at,
        };

        let mut storage = self.storage.lock().unwrap();
        storage.insert(state.to_string(), entry);

        log::debug!(
            "[MockPkceStorage] Stored PKCE verifier for state {} with TTL {}s",
            state,
            ttl_seconds
        );

        Ok(())
    }

    async fn retrieve_and_delete_pkce(&self, state: &str) -> Result<Option<String>> {
        let now = Self::current_timestamp();
        let mut storage = self.storage.lock().unwrap();

        // Check if entry exists and is not expired
        if let Some(entry) = storage.get(state) {
            if entry.expires_at > now {
                // Not expired, retrieve and delete
                let verifier = entry.verifier.clone();
                storage.remove(state);

                log::debug!(
                    "[MockPkceStorage] Retrieved and deleted PKCE verifier for state {}",
                    state
                );

                return Ok(Some(verifier));
            } else {
                // Expired, delete and return None
                storage.remove(state);

                log::debug!(
                    "[MockPkceStorage] PKCE verifier for state {} was expired",
                    state
                );

                return Ok(None);
            }
        }

        log::debug!(
            "[MockPkceStorage] No PKCE verifier found for state {}",
            state
        );

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let storage = MockPkceStorage::new();
        let state = "test-state";
        let verifier = "test-verifier";

        // Store
        storage.store_pkce(state, verifier, 300).await.unwrap();

        // Retrieve
        let result = storage.retrieve_and_delete_pkce(state).await.unwrap();
        assert_eq!(result, Some(verifier.to_string()));

        // Should be deleted now
        let result = storage.retrieve_and_delete_pkce(state).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_expiration() {
        let storage = MockPkceStorage::new();
        let state = "test-state";
        let verifier = "test-verifier";

        // Store with 0 TTL (immediately expired)
        storage.store_pkce(state, verifier, 0).await.unwrap();

        // Should be expired
        let result = storage.retrieve_and_delete_pkce(state).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_not_found() {
        let storage = MockPkceStorage::new();

        let result = storage
            .retrieve_and_delete_pkce("nonexistent")
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        let storage = MockPkceStorage::new();

        // Store one that expires immediately and one that doesn't
        storage.store_pkce("expired", "verifier1", 0).await.unwrap();
        storage.store_pkce("valid", "verifier2", 300).await.unwrap();

        // Cleanup
        storage.cleanup_expired();

        // Check results
        let expired = storage.retrieve_and_delete_pkce("expired").await.unwrap();
        assert_eq!(expired, None);

        let valid = storage.retrieve_and_delete_pkce("valid").await.unwrap();
        assert_eq!(valid, Some("verifier2".to_string()));
    }
}
