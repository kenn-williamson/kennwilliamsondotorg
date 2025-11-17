use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BlogPost {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub featured_image_url: Option<String>,
    pub featured_image_alt: Option<String>,
    pub status: String, // 'draft' | 'published'
    pub tags: Vec<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub meta_description: Option<String>,
    // Note: search_vector is generated column, not included in struct
}
