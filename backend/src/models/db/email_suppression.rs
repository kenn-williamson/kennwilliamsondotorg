use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Email suppression record for AWS SES compliance
/// Tracks bounces, complaints, unsubscribes, and manual suppressions
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmailSuppression {
    pub id: Uuid,
    pub email: String,
    pub suppression_type: String, // 'bounce', 'complaint', 'unsubscribe', 'manual'
    pub reason: Option<String>,

    // Scope of suppression
    pub suppress_transactional: bool, // Blocks verification, password reset, etc.
    pub suppress_marketing: bool,     // Blocks newsletters, announcements, etc.

    // Metadata
    pub bounce_count: i32,
    pub last_bounce_at: Option<DateTime<Utc>>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Email type for suppression checks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailType {
    Transactional, // Verification, password reset, security alerts
    Marketing,     // Newsletters, announcements, promotional
}

impl EmailType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmailType::Transactional => "transactional",
            EmailType::Marketing => "marketing",
        }
    }
}
