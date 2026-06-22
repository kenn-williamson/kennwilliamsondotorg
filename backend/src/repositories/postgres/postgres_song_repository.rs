use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::Song;
use crate::repositories::traits::song_repository::{
    CreateSong, SongFilters, SongList, SongRepository, UpdateSong,
};

pub struct PostgresSongRepository {
    pool: PgPool,
}

impl PostgresSongRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SongRepository for PostgresSongRepository {
    async fn create_song(&self, song: CreateSong) -> Result<Song> {
        let created = sqlx::query_as!(
            Song,
            r#"
            INSERT INTO songs (
                slug, title, description, lyrics, credits, audio_url, artwork_url,
                duration_seconds, display_order, status, published_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING
                id, slug, title, description, lyrics, credits, audio_url, artwork_url,
                duration_seconds, display_order, status, published_at, created_at, updated_at
            "#,
            song.slug,
            song.title,
            song.description,
            song.lyrics,
            song.credits,
            song.audio_url,
            song.artwork_url,
            song.duration_seconds,
            song.display_order,
            song.status,
            song.published_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }

    async fn get_song_by_id(&self, id: Uuid) -> Result<Option<Song>> {
        let song = sqlx::query_as!(
            Song,
            r#"
            SELECT
                id, slug, title, description, lyrics, credits, audio_url, artwork_url,
                duration_seconds, display_order, status, published_at, created_at, updated_at
            FROM songs
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(song)
    }

    async fn get_song_by_slug(&self, slug: &str) -> Result<Option<Song>> {
        let song = sqlx::query_as!(
            Song,
            r#"
            SELECT
                id, slug, title, description, lyrics, credits, audio_url, artwork_url,
                duration_seconds, display_order, status, published_at, created_at, updated_at
            FROM songs
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(song)
    }

    async fn list_songs(&self, filters: SongFilters) -> Result<SongList> {
        let limit = filters.limit as i64;
        let offset = ((filters.page - 1) * filters.limit) as i64;

        let (total, songs) = match &filters.status {
            Some(status) => {
                let total = sqlx::query_scalar!(
                    "SELECT COUNT(*) FROM songs WHERE status = $1",
                    status
                )
                .fetch_one(&self.pool)
                .await?
                .unwrap_or(0);

                let songs = sqlx::query_as!(
                    Song,
                    r#"
                    SELECT
                        id, slug, title, description, lyrics, credits, audio_url, artwork_url,
                        duration_seconds, display_order, status, published_at, created_at, updated_at
                    FROM songs
                    WHERE status = $1
                    ORDER BY display_order ASC, created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                    status,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await?;

                (total, songs)
            }
            None => {
                let total = sqlx::query_scalar!("SELECT COUNT(*) FROM songs")
                    .fetch_one(&self.pool)
                    .await?
                    .unwrap_or(0);

                let songs = sqlx::query_as!(
                    Song,
                    r#"
                    SELECT
                        id, slug, title, description, lyrics, credits, audio_url, artwork_url,
                        duration_seconds, display_order, status, published_at, created_at, updated_at
                    FROM songs
                    ORDER BY display_order ASC, created_at DESC
                    LIMIT $1 OFFSET $2
                    "#,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await?;

                (total, songs)
            }
        };

        let total_pages = ((total as f64) / (filters.limit as f64)).ceil() as i32;

        Ok(SongList {
            songs,
            total,
            page: filters.page,
            total_pages,
        })
    }

    async fn update_song(&self, id: Uuid, song: UpdateSong) -> Result<Song> {
        // COALESCE keeps the existing value when a field is not provided (None).
        let updated = sqlx::query_as!(
            Song,
            r#"
            UPDATE songs
            SET
                slug = COALESCE($2, slug),
                title = COALESCE($3, title),
                description = COALESCE($4, description),
                lyrics = COALESCE($5, lyrics),
                credits = COALESCE($6, credits),
                audio_url = COALESCE($7, audio_url),
                artwork_url = COALESCE($8, artwork_url),
                duration_seconds = COALESCE($9, duration_seconds),
                display_order = COALESCE($10, display_order),
                status = COALESCE($11, status),
                published_at = COALESCE($12, published_at),
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, slug, title, description, lyrics, credits, audio_url, artwork_url,
                duration_seconds, display_order, status, published_at, created_at, updated_at
            "#,
            id,
            song.slug,
            song.title,
            song.description,
            song.lyrics,
            song.credits,
            song.audio_url,
            song.artwork_url,
            song.duration_seconds,
            song.display_order,
            song.status,
            song.published_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }

    async fn delete_song(&self, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM songs WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn search_songs(&self, query: &str, page: i32, limit: i32) -> Result<SongList> {
        let limit_i64 = limit as i64;
        let offset = ((page - 1) * limit) as i64;

        // Prefix-match full-text search using the search_vector generated column
        let search_query = query
            .split_whitespace()
            .map(|word| format!("{}:*", word))
            .collect::<Vec<_>>()
            .join(" & ");

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM songs WHERE search_vector @@ to_tsquery('english', $1)",
            search_query
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        let songs = sqlx::query_as!(
            Song,
            r#"
            SELECT
                id, slug, title, description, lyrics, credits, audio_url, artwork_url,
                duration_seconds, display_order, status, published_at, created_at, updated_at
            FROM songs
            WHERE search_vector @@ to_tsquery('english', $1)
            ORDER BY display_order ASC, created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            search_query,
            limit_i64,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        let total_pages = ((total as f64) / (limit as f64)).ceil() as i32;

        Ok(SongList {
            songs,
            total,
            page,
            total_pages,
        })
    }
}
