use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// User list response for admin
#[derive(Debug, Clone, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserWithRoles>,
    pub total: i64,
}

/// User with roles for admin display
#[derive(Debug, Clone, Serialize)]
pub struct UserWithRoles {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub roles: Vec<String>,
}

/// System statistics response
#[derive(Debug, Clone, Serialize)]
pub struct SystemStatsResponse {
    pub total_users: i64,
    pub active_users: i64,
    pub pending_suggestions: i64,
    pub total_phrases: i64,
}

/// Pending suggestion response
#[derive(Debug, Clone, Serialize)]
pub struct PendingSuggestionResponse {
    pub id: Uuid,
    pub phrase_text: String,
    pub created_at: DateTime<Utc>,
    pub user_display_name: String,
    pub user_email: String,
}

/// Pending suggestions list response
#[derive(Debug, Clone, Serialize)]
pub struct PendingSuggestionsResponse {
    pub suggestions: Vec<PendingSuggestionResponse>,
    pub total: i64,
}

/// Password reset response
#[derive(Debug, Clone, Serialize)]
pub struct PasswordResetResponse {
    pub new_password: String,
}

/// Admin action request
#[derive(Debug, Clone, Deserialize)]
pub struct AdminActionRequest {
    pub admin_reason: Option<String>,
}

/// User search query parameters
#[derive(Debug, Clone, Deserialize)]
pub struct UserSearchQuery {
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
