use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::models::db::{EmailSuppression, EmailType};

/// Data for creating a new email suppression
pub struct CreateSuppressionData {
    pub email: String,
    pub suppression_type: String,
    pub reason: Option<String>,
    pub suppress_transactional: bool,
    pub suppress_marketing: bool,
}

/// Repository trait for managing email suppressions
#[async_trait]
#[allow(dead_code)]
pub trait EmailSuppressionRepository: Send + Sync {
    /// Create a new email suppression
    async fn create_suppression(&self, data: &CreateSuppressionData) -> Result<EmailSuppression>;

    /// Find suppression by email address
    async fn find_by_email(&self, email: &str) -> Result<Option<EmailSuppression>>;

    /// Check if an email is suppressed for a given email type
    /// Returns true if the email should NOT be sent
    async fn is_email_suppressed(&self, email: &str, email_type: EmailType) -> Result<bool>;

    /// Increment bounce count for an email
    async fn increment_bounce_count(&self, email: &str, bounced_at: DateTime<Utc>) -> Result<()>;

    /// Delete a suppression (admin override)
    async fn delete_suppression(&self, email: &str) -> Result<()>;
}
