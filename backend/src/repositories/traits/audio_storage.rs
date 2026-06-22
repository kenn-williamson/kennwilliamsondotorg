use anyhow::Result;
use async_trait::async_trait;

/// Result of uploading an audio file, containing the public URL and optional
/// duration metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioUpload {
    /// Public URL for direct browser streaming (S3 supports HTTP range requests)
    pub url: String,
    /// Track length in whole seconds, if known at upload time
    pub duration_seconds: Option<i32>,
}

impl AudioUpload {
    pub fn new(url: impl Into<String>, duration_seconds: Option<i32>) -> Self {
        Self {
            url: url.into(),
            duration_seconds,
        }
    }
}

/// Trait for audio file storage operations (S3, local filesystem, etc.)
///
/// Mirrors the URL-driven design of [`super::image_storage::ImageStorage`]:
/// uploads return a public URL stored directly on the `songs` row, and deletes
/// accept that URL (translated to a storage key internally).
///
/// # Security
///
/// Implementations MUST validate:
/// - File size limits
/// - File extension / format allow-list
/// - Filename sanitization (prevent path traversal)
#[async_trait]
pub trait AudioStorage: Send + Sync {
    /// Upload an audio file and return its public URL (plus duration if known).
    ///
    /// # Arguments
    /// * `audio_data` - Raw audio bytes
    /// * `filename` - Original filename (for extension detection)
    async fn upload_audio(&self, audio_data: Vec<u8>, filename: String) -> Result<AudioUpload>;

    /// Delete an audio file from storage (idempotent).
    ///
    /// # Arguments
    /// * `url` - Full public URL of the audio file to delete
    async fn delete_audio(&self, url: &str) -> Result<()>;
}
