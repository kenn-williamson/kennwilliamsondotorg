use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::db::AccessRequest;

/// Repository trait for access request operations
#[async_trait]
pub trait AccessRequestRepository: Send + Sync {
    /// Create a new access request
    async fn create_request(
        &self,
        user_id: Uuid,
        message: String,
        requested_role: String,
    ) -> Result<AccessRequest>;

    /// Get access request by ID
    async fn get_request_by_id(&self, request_id: Uuid) -> Result<Option<AccessRequest>>;

    /// Get all requests for a specific user
    async fn get_user_requests(&self, user_id: Uuid) -> Result<Vec<AccessRequest>>;

    /// Get all pending access requests with user information (admin only)
    async fn get_pending_requests(&self) -> Result<Vec<PendingRequestWithUser>>;

    /// Approve an access request (admin only)
    async fn approve_request(
        &self,
        request_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()>;

    /// Reject an access request (admin only)
    async fn reject_request(
        &self,
        request_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()>;

    /// Count total access requests
    async fn count_all_requests(&self) -> Result<i64>;

    /// Count pending access requests
    async fn count_pending_requests(&self) -> Result<i64>;
}

/// Internal struct for pending requests with user info
#[derive(Debug, Clone)]
pub struct PendingRequestWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_email: String,
    pub user_display_name: String,
    pub message: String,
    pub requested_role: String,
    pub created_at: DateTime<Utc>,
}
