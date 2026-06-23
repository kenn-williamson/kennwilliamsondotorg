use crate::models::db::song::Song;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// Builder for creating Song instances in tests with sensible defaults.
///
/// Mirrors [`super::blog_post_builder::BlogPostBuilder`]: `build()` constructs
/// an in-memory value, `persist()` inserts into the database for integration tests.
#[derive(Clone)]
pub struct SongBuilder {
    id: Option<Uuid>,
    slug: Option<String>,
    title: Option<String>,
    description: Option<Option<String>>,
    lyrics: Option<Option<String>>,
    credits: Option<Option<String>>,
    audio_url: Option<Option<String>>,
    artwork_url: Option<Option<String>>,
    duration_seconds: Option<Option<i32>>,
    display_order: Option<i32>,
    status: Option<String>,
    published_at: Option<Option<DateTime<Utc>>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl SongBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            slug: None,
            title: None,
            description: None,
            lyrics: None,
            credits: None,
            audio_url: None,
            artwork_url: None,
            duration_seconds: None,
            display_order: None,
            status: None,
            published_at: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Build the Song with defaults for any unset fields (in-memory only).
    pub fn build(self) -> Song {
        let now = Utc::now();
        let uuid = Uuid::new_v4();
        let default_title = format!("Test Song {}", uuid);
        let default_slug = format!("test-song-{}", uuid);

        Song {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            slug: self.slug.unwrap_or(default_slug),
            title: self.title.unwrap_or(default_title),
            description: self.description.unwrap_or(None),
            lyrics: self.lyrics.unwrap_or(None),
            credits: self.credits.unwrap_or(None),
            audio_url: self.audio_url.unwrap_or(None),
            artwork_url: self.artwork_url.unwrap_or(None),
            duration_seconds: self.duration_seconds.unwrap_or(None),
            display_order: self.display_order.unwrap_or(0),
            status: self.status.unwrap_or_else(|| "draft".to_string()),
            published_at: self.published_at.unwrap_or(None),
            created_at: self.created_at.unwrap_or(now),
            updated_at: self.updated_at.unwrap_or(now),
        }
    }

    /// Persist Song to database (for integration tests).
    pub async fn persist(self, pool: &PgPool) -> Result<Song> {
        let uuid = Uuid::new_v4();
        let slug = self.slug.unwrap_or_else(|| format!("test-song-{}", uuid));
        let title = self.title.unwrap_or_else(|| format!("Test Song {}", uuid));
        let description = self.description.unwrap_or(None);
        let lyrics = self.lyrics.unwrap_or(None);
        let credits = self.credits.unwrap_or(None);
        let audio_url = self.audio_url.unwrap_or(None);
        let artwork_url = self.artwork_url.unwrap_or(None);
        let duration_seconds = self.duration_seconds.unwrap_or(None);
        let display_order = self.display_order.unwrap_or(0);
        let status = self.status.unwrap_or_else(|| "draft".to_string());
        let published_at = self.published_at.unwrap_or(None);

        let song = sqlx::query_as::<_, Song>(
            "INSERT INTO songs (slug, title, description, lyrics, credits, audio_url, artwork_url, duration_seconds, display_order, status, published_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             RETURNING *",
        )
        .bind(slug)
        .bind(title)
        .bind(description)
        .bind(lyrics)
        .bind(credits)
        .bind(audio_url)
        .bind(artwork_url)
        .bind(duration_seconds)
        .bind(display_order)
        .bind(status)
        .bind(published_at)
        .fetch_one(pool)
        .await?;

        Ok(song)
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_slug(mut self, slug: impl Into<String>) -> Self {
        self.slug = Some(slug.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(Some(description.into()));
        self
    }

    pub fn with_audio_url(mut self, url: impl Into<String>) -> Self {
        self.audio_url = Some(Some(url.into()));
        self
    }

    pub fn without_audio_url(mut self) -> Self {
        self.audio_url = Some(None);
        self
    }

    pub fn with_artwork_url(mut self, url: impl Into<String>) -> Self {
        self.artwork_url = Some(Some(url.into()));
        self
    }

    pub fn with_duration(mut self, seconds: i32) -> Self {
        self.duration_seconds = Some(Some(seconds));
        self
    }

    pub fn with_display_order(mut self, order: i32) -> Self {
        self.display_order = Some(order);
        self
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn draft(mut self) -> Self {
        self.status = Some("draft".to_string());
        self.published_at = Some(None);
        self
    }

    pub fn published(mut self) -> Self {
        self.status = Some("published".to_string());
        self.published_at = Some(Some(Utc::now()));
        self
    }

    pub fn published_at(mut self, published_at: DateTime<Utc>) -> Self {
        self.status = Some("published".to_string());
        self.published_at = Some(Some(published_at));
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }
}

impl Default for SongBuilder {
    fn default() -> Self {
        Self::new()
    }
}
