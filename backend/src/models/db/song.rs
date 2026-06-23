use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Song {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub lyrics: Option<String>,
    pub credits: Option<String>,
    pub audio_url: Option<String>,
    pub artwork_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: i32,
    pub status: String, // 'draft' | 'published'
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Note: search_vector is a generated column, not included in struct
}
