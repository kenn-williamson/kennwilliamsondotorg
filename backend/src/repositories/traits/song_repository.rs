use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::db::Song;

/// Data structures for song repository operations

#[derive(Debug, Clone)]
pub struct CreateSong {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub lyrics: Option<String>,
    pub credits: Option<String>,
    pub audio_url: Option<String>,
    pub artwork_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: i32,
    pub status: String,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub struct UpdateSong {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub lyrics: Option<String>,
    pub credits: Option<String>,
    pub audio_url: Option<String>,
    pub artwork_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: Option<i32>,
    pub status: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct SongFilters {
    pub status: Option<String>,
    pub page: i32,
    pub limit: i32,
}

#[derive(Debug, Clone)]
pub struct SongList {
    pub songs: Vec<Song>,
    pub total: i64,
    pub page: i32,
    pub total_pages: i32,
}

/// Repository trait for song (music) operations
#[async_trait]
pub trait SongRepository: Send + Sync {
    /// Create a new song
    async fn create_song(&self, song: CreateSong) -> Result<Song>;

    /// Get a song by ID
    async fn get_song_by_id(&self, id: Uuid) -> Result<Option<Song>>;

    /// Get a song by slug
    async fn get_song_by_slug(&self, slug: &str) -> Result<Option<Song>>;

    /// List songs with filters and pagination (ordered by display_order, then newest)
    async fn list_songs(&self, filters: SongFilters) -> Result<SongList>;

    /// Update a song
    async fn update_song(&self, id: Uuid, song: UpdateSong) -> Result<Song>;

    /// Delete a song
    async fn delete_song(&self, id: Uuid) -> Result<()>;

    /// Search songs using full-text search
    async fn search_songs(&self, query: &str, page: i32, limit: i32) -> Result<SongList>;
}
