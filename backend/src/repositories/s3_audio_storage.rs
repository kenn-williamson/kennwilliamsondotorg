use crate::repositories::traits::audio_storage::{AudioStorage, AudioUpload};
use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use aws_sdk_s3::Client as S3Client;
use uuid::Uuid;

/// Allowed audio file extensions (lowercase, without the dot)
const ALLOWED_AUDIO_EXTENSIONS: &[&str] = &["mp3", "wav", "flac", "m4a", "aac", "ogg", "oga"];

/// Maximum audio upload size (50MB)
const MAX_AUDIO_SIZE: usize = 50 * 1024 * 1024;

pub struct S3AudioStorage {
    bucket_name: String,
}

impl S3AudioStorage {
    pub fn new(bucket_name: String) -> Self {
        Self { bucket_name }
    }

    /// Create S3 client from environment (credentials loaded from environment or EC2 instance role)
    async fn create_s3_client() -> S3Client {
        let config = aws_config::load_from_env().await;
        S3Client::new(&config)
    }

    /// Sanitize filename to prevent path traversal attacks
    fn sanitize_filename(filename: &str) -> String {
        filename
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '_')
            .collect()
    }

    /// Extract lowercase file extension from filename
    fn get_extension(filename: &str) -> Option<String> {
        filename.rsplit('.').next().map(|e| e.to_ascii_lowercase())
    }

    /// Map an audio extension to its MIME content type
    fn content_type_for(extension: &str) -> &'static str {
        match extension {
            "mp3" => "audio/mpeg",
            "wav" => "audio/wav",
            "flac" => "audio/flac",
            "m4a" | "aac" => "audio/mp4",
            "ogg" | "oga" => "audio/ogg",
            _ => "application/octet-stream",
        }
    }
}

#[async_trait]
impl AudioStorage for S3AudioStorage {
    async fn upload_audio(&self, audio_data: Vec<u8>, filename: String) -> Result<AudioUpload> {
        let s3_client = Self::create_s3_client().await;

        // 1. Validate file size
        if audio_data.len() > MAX_AUDIO_SIZE {
            bail!("Audio exceeds 50MB limit");
        }

        // 2. Sanitize filename and validate extension against allow-list
        let sanitized_filename = Self::sanitize_filename(&filename);
        let extension = Self::get_extension(&sanitized_filename).unwrap_or_else(|| "mp3".to_string());
        if !ALLOWED_AUDIO_EXTENSIONS.contains(&extension.as_str()) {
            bail!(
                "Invalid audio format: .{} (allowed: mp3, wav, flac, m4a, aac, ogg)",
                extension
            );
        }

        // 3. Generate UUID for unique storage and upload
        let audio_id = Uuid::new_v4();
        let key = format!("music/audio/{}.{}", audio_id, extension);
        let content_type = Self::content_type_for(&extension);

        s3_client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&key)
            .body(audio_data.into())
            .content_type(content_type)
            .send()
            .await
            .context("Failed to upload audio to S3")?;

        let url = format!("https://{}.s3.amazonaws.com/{}", self.bucket_name, key);

        // Duration is captured client-side (from the browser audio element) for now;
        // server-side extraction can be added later without changing this API.
        Ok(AudioUpload {
            url,
            duration_seconds: None,
        })
    }

    async fn delete_audio(&self, url: &str) -> Result<()> {
        let s3_client = Self::create_s3_client().await;

        let key = url
            .split(&format!("{}.s3.amazonaws.com/", self.bucket_name))
            .nth(1)
            .context("Invalid S3 URL format")?;

        s3_client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .context("Failed to delete audio from S3")?;

        Ok(())
    }
}
