use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AccessRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub message: String,
    pub requested_role: String,
    pub status: String, // "pending", "approved", "rejected"
    pub admin_id: Option<Uuid>,
    pub admin_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
