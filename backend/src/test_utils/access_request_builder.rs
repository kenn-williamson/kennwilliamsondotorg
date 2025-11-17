use crate::models::db::AccessRequest;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// Builder for creating AccessRequest instances in tests with sensible defaults.
#[derive(Clone)]
pub struct AccessRequestBuilder {
    user_id: Option<Uuid>,
    message: Option<String>,
    requested_role: Option<String>,
    status: Option<String>,
    admin_id: Option<Option<Uuid>>,
    admin_reason: Option<Option<String>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl AccessRequestBuilder {
    /// Create a new builder with sensible defaults
    pub fn new() -> Self {
        Self {
            user_id: None,
            message: None,
            requested_role: None,
            status: None,
            admin_id: None,
            admin_reason: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Persist AccessRequest to database (for integration tests)
    pub async fn persist(self, pool: &PgPool) -> Result<AccessRequest> {
        let now = Utc::now();
        let user_id = self
            .user_id
            .ok_or_else(|| anyhow::anyhow!("user_id is required"))?;
        let message = self
            .message
            .unwrap_or_else(|| "I would like access please".to_string());
        let requested_role = self
            .requested_role
            .unwrap_or_else(|| "trusted-contact".to_string());
        let status = self.status.unwrap_or_else(|| "pending".to_string());

        let request = sqlx::query_as::<_, AccessRequest>(
            r#"
            INSERT INTO access_requests (user_id, message, requested_role, status, admin_id, admin_reason, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(message)
        .bind(requested_role)
        .bind(status)
        .bind(self.admin_id.flatten())
        .bind(self.admin_reason.flatten())
        .bind(self.created_at.unwrap_or(now))
        .bind(self.updated_at.unwrap_or(now))
        .fetch_one(pool)
        .await?;

        Ok(request)
    }

    // ============================================================================
    // CONFIGURATION METHODS
    // ============================================================================

    /// Set the user_id (required)
    pub fn with_user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set the message
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Set the requested role
    pub fn with_requested_role(mut self, requested_role: impl Into<String>) -> Self {
        self.requested_role = Some(requested_role.into());
        self
    }

    /// Set the status
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Set admin_id
    pub fn with_admin_id(mut self, admin_id: Uuid) -> Self {
        self.admin_id = Some(Some(admin_id));
        self
    }

    /// Set admin_reason
    pub fn with_admin_reason(mut self, admin_reason: impl Into<String>) -> Self {
        self.admin_reason = Some(Some(admin_reason.into()));
        self
    }

    /// Set created_at timestamp
    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    /// Set updated_at timestamp
    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }

    // ============================================================================
    // CONVENIENCE PRESETS
    // ============================================================================

    /// Create a pending request (default)
    pub fn pending(self) -> Self {
        self.with_status("pending")
    }

    /// Create an approved request
    pub fn approved(self, admin_id: Uuid, admin_reason: impl Into<String>) -> Self {
        self.with_status("approved")
            .with_admin_id(admin_id)
            .with_admin_reason(admin_reason)
    }

    /// Create a rejected request
    pub fn rejected(self, admin_id: Uuid, admin_reason: impl Into<String>) -> Self {
        self.with_status("rejected")
            .with_admin_id(admin_id)
            .with_admin_reason(admin_reason)
    }
}

impl Default for AccessRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
