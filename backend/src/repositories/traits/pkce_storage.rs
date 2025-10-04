use anyhow::Result;
use async_trait::async_trait;

/// Trait for storing and retrieving PKCE verifiers during OAuth flows
///
/// PKCE (Proof Key for Code Exchange) requires temporary storage of code verifiers
/// between the authorization request and token exchange steps.
#[async_trait]
pub trait PkceStorage: Send + Sync {
    /// Store a PKCE code verifier with the given state token
    ///
    /// # Arguments
    /// * `state` - OAuth state parameter (used as key)
    /// * `verifier` - PKCE code verifier to store
    /// * `ttl_seconds` - Time-to-live in seconds (typically 300 for 5 minutes)
    async fn store_pkce(&self, state: &str, verifier: &str, ttl_seconds: u64) -> Result<()>;

    /// Retrieve and delete a PKCE code verifier (single-use)
    ///
    /// # Arguments
    /// * `state` - OAuth state parameter
    ///
    /// # Returns
    /// * `Ok(Some(verifier))` if found and deleted
    /// * `Ok(None)` if not found or expired
    /// * `Err` on storage errors
    async fn retrieve_and_delete_pkce(&self, state: &str) -> Result<Option<String>>;
}
