use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::db::Song;

// Request/Response models for music (song) operations

#[derive(Debug, Serialize)]
pub struct SongResponse {
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
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SongListResponse {
    pub songs: Vec<SongResponse>,
    pub total: i64,
    pub page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CreateSongRequest {
    pub title: String,
    pub slug: Option<String>, // Auto-generated if not provided
    pub description: Option<String>,
    pub lyrics: Option<String>,
    pub credits: Option<String>,
    pub audio_url: Option<String>,
    pub artwork_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: Option<i32>,
    pub status: String, // 'draft' | 'published'
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UpdateSongRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub lyrics: Option<String>,
    pub credits: Option<String>,
    pub audio_url: Option<String>,
    pub artwork_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: Option<i32>,
    pub status: Option<String>,
}

/// Response after uploading an audio file to storage
#[derive(Debug, Serialize)]
pub struct AudioUploadResponse {
    pub url: String,
    pub duration_seconds: Option<i32>,
}

// Conversion implementations

impl From<Song> for SongResponse {
    fn from(song: Song) -> Self {
        SongResponse {
            id: song.id,
            slug: song.slug,
            title: song.title,
            description: song.description,
            lyrics: song.lyrics,
            credits: song.credits,
            audio_url: song.audio_url,
            artwork_url: song.artwork_url,
            duration_seconds: song.duration_seconds,
            display_order: song.display_order,
            status: song.status,
            published_at: song.published_at,
            created_at: song.created_at,
            updated_at: song.updated_at,
        }
    }
}
