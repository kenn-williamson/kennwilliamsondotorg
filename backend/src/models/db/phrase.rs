use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Phrase {
    pub id: Uuid,
    pub phrase_text: String,
    pub active: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PhraseSuggestion {
    pub id: Uuid,
    pub user_id: Uuid,
    pub phrase_text: String,
    pub status: String, // "pending", "approved", "rejected"
    pub admin_id: Option<Uuid>,
    pub admin_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PhraseSearchResultWithUserExclusionView {
    pub id: Uuid,
    pub phrase_text: String,
    pub active: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_excluded: Option<bool>, // SQLx infers Option due to LEFT JOIN, but we handle conversion in service layer
    pub rank: Option<f32>, // For ordering in search queries
}

