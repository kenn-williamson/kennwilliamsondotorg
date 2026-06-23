/// Music service module
///
/// Implements business logic for song management including:
/// - CRUD operations (create, read, update, delete)
/// - Slug generation and collision handling (shared with the blog service)
/// - Publishing workflow (draft → published)
/// - Audio + artwork upload/cleanup via storage abstractions
use anyhow::Result;
use std::sync::Arc;

use crate::models::api::{CreateSongRequest, UpdateSongRequest};
use crate::models::db::Song;
use crate::repositories::traits::{AudioStorage, AudioUpload, ImageStorage, SongRepository};

pub mod create;
pub mod delete;
pub mod read;
pub mod update;

/// MusicService provides business logic for song operations
///
/// Uses dependency injection with Arc-wrapped trait objects for testability.
/// Audio files are stored via [`AudioStorage`]; cover artwork reuses the
/// existing [`ImageStorage`] (a square image fits within the 1200x630 bound).
pub struct MusicService {
    repository: Arc<dyn SongRepository>,
    audio_storage: Arc<dyn AudioStorage>,
    image_storage: Arc<dyn ImageStorage>,
}

/// Builder for MusicService with validation
pub struct MusicServiceBuilder {
    repository: Option<Box<dyn SongRepository>>,
    audio_storage: Option<Box<dyn AudioStorage>>,
    image_storage: Option<Box<dyn ImageStorage>>,
}

impl MusicServiceBuilder {
    pub fn new() -> Self {
        Self {
            repository: None,
            audio_storage: None,
            image_storage: None,
        }
    }

    pub fn with_repository(mut self, repository: Box<dyn SongRepository>) -> Self {
        self.repository = Some(repository);
        self
    }

    pub fn with_audio_storage(mut self, audio_storage: Box<dyn AudioStorage>) -> Self {
        self.audio_storage = Some(audio_storage);
        self
    }

    pub fn with_image_storage(mut self, image_storage: Box<dyn ImageStorage>) -> Self {
        self.image_storage = Some(image_storage);
        self
    }

    /// Build MusicService, validating that all dependencies are present.
    pub fn build(self) -> Result<MusicService> {
        Ok(MusicService {
            repository: Arc::from(
                self.repository
                    .ok_or_else(|| anyhow::anyhow!("SongRepository is required"))?,
            ),
            audio_storage: Arc::from(
                self.audio_storage
                    .ok_or_else(|| anyhow::anyhow!("AudioStorage is required"))?,
            ),
            image_storage: Arc::from(
                self.image_storage
                    .ok_or_else(|| anyhow::anyhow!("ImageStorage is required"))?,
            ),
        })
    }
}

impl Default for MusicServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicService {
    pub fn builder() -> MusicServiceBuilder {
        MusicServiceBuilder::new()
    }

    /// Create MusicService directly (useful for testing)
    pub fn new(
        repository: Box<dyn SongRepository>,
        audio_storage: Box<dyn AudioStorage>,
        image_storage: Box<dyn ImageStorage>,
    ) -> Self {
        Self {
            repository: Arc::from(repository),
            audio_storage: Arc::from(audio_storage),
            image_storage: Arc::from(image_storage),
        }
    }

    // --- Create Operations ---

    /// Create a new song.
    ///
    /// Auto-generates a slug from the title (with collision handling) and sets
    /// `published_at` when the status is "published".
    pub async fn create_song(&self, request: CreateSongRequest) -> Result<Song> {
        create::create_song(self, request).await
    }

    // --- Read Operations ---

    pub async fn get_song_by_id(&self, id: uuid::Uuid) -> Result<Option<Song>> {
        read::get_song_by_id(self, id).await
    }

    pub async fn get_song_by_slug(&self, slug: &str) -> Result<Option<Song>> {
        read::get_song_by_slug(self, slug).await
    }

    pub async fn list_songs(
        &self,
        filters: crate::repositories::traits::SongFilters,
    ) -> Result<crate::repositories::traits::SongList> {
        read::list_songs(self, filters).await
    }

    pub async fn search_songs(
        &self,
        query: &str,
        page: i32,
        limit: i32,
    ) -> Result<crate::repositories::traits::SongList> {
        read::search_songs(self, query, page, limit).await
    }

    // --- Update Operations ---

    /// Update an existing song. Preserves `published_at` for already-published
    /// songs and sets it when transitioning draft → published.
    pub async fn update_song(
        &self,
        id: uuid::Uuid,
        request: UpdateSongRequest,
    ) -> Result<Song> {
        update::update_song(self, id, request).await
    }

    // --- Delete Operations ---

    /// Delete a song and best-effort clean up its audio + artwork from storage.
    pub async fn delete_song(&self, id: uuid::Uuid) -> Result<()> {
        delete::delete_song(self, id).await
    }

    // --- Upload Operations ---

    /// Upload an audio file via [`AudioStorage`] and return its public URL.
    pub async fn upload_audio(
        &self,
        audio_data: Vec<u8>,
        filename: String,
    ) -> Result<AudioUpload> {
        self.audio_storage.upload_audio(audio_data, filename).await
    }

    /// Upload cover artwork via [`ImageStorage`] (returns processed + original URLs).
    pub async fn upload_artwork(
        &self,
        image_data: Vec<u8>,
        filename: String,
    ) -> Result<crate::repositories::traits::ImageUrls> {
        self.image_storage.upload_image(image_data, filename).await
    }
}
