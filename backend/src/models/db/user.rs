use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub real_name: Option<String>,
    pub google_user_id: Option<String>,
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User with timer data for public timer list
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserWithTimer {
    pub id: Uuid,
    pub display_name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub reset_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

/// User with roles from database (raw SQLx result)
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserWithRoles {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub real_name: Option<String>,
    pub google_user_id: Option<String>,
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub roles: Option<Vec<String>>,
}

/// Verification token for email verification
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct VerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Password reset token for password recovery flow
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
