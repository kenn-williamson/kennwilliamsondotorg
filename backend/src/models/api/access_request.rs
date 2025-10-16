use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::db::AccessRequest;

// Request/Response models for access request operations

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CreateAccessRequestRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct AccessRequestResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub message: String,
    pub requested_role: String,
    pub status: String,
    pub admin_id: Option<Uuid>,
    pub admin_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AccessRequestWithUserResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_email: String,
    pub user_display_name: String,
    pub message: String,
    pub requested_role: String,
    pub status: String,
    pub admin_id: Option<Uuid>,
    pub admin_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AccessRequestListResponse {
    pub requests: Vec<AccessRequestWithUserResponse>,
    pub total: i64,
}

// Conversion implementations

impl From<AccessRequest> for AccessRequestResponse {
    fn from(request: AccessRequest) -> Self {
        AccessRequestResponse {
            id: request.id,
            user_id: request.user_id,
            message: request.message,
            requested_role: request.requested_role,
            status: request.status,
            admin_id: request.admin_id,
            admin_reason: request.admin_reason,
            created_at: request.created_at,
            updated_at: request.updated_at,
        }
    }
}
