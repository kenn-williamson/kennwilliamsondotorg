use anyhow::Result;
use async_trait::async_trait;

/// Result of uploading an image, containing public URLs for direct browser access
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageUrls {
    /// Public URL for the processed featured image (1200x630px, optimized)
    pub featured_url: String,
    /// Public URL for the original uploaded image (backup for future re-processing)
    pub original_url: String,
}

impl ImageUrls {
    /// Create new ImageUrls
    pub fn new(featured_url: impl Into<String>, original_url: impl Into<String>) -> Self {
        Self {
            featured_url: featured_url.into(),
            original_url: original_url.into(),
        }
    }
}

/// Trait for image storage operations (S3, local filesystem, etc.)
///
/// # Design Philosophy
///
/// This trait uses a **URL-driven approach**:
/// - Upload returns public URLs (not database IDs)
/// - URLs are stored directly in blog_posts table
/// - Delete accepts URLs (translates to storage keys internally)
///
/// This keeps the blog feature simple (no separate images table) while
/// remaining flexible enough to migrate to a full media library later.
///
/// # Migration Path to Media Library
///
/// If we later need image reuse, metadata, or a media library:
/// 1. Create an `images` table with id, url, metadata
/// 2. Create `ImageRepository` for database operations
/// 3. Keep `ImageStorage` for S3/storage operations
/// 4. Service layer coordinates both (insert DB record, upload to storage)
/// 5. Existing URL-based code continues to work
///
/// # Security
///
/// Implementations MUST validate:
/// - File size limits
/// - File type/MIME validation
/// - Filename sanitization (prevent path traversal)
///
#[async_trait]
pub trait ImageStorage: Send + Sync {
    /// Upload an image with processing and return public URLs
    ///
    /// # Processing Steps
    /// 1. Validate file size (<5MB)
    /// 2. Sanitize filename (remove path traversal attempts)
    /// 3. Validate image format (load with image crate to verify)
    /// 4. Save original to storage (backup for future re-processing)
    /// 5. Resize to 1200x630px (optimal for social media preview)
    /// 6. Compress to JPEG 80% quality (balance quality/size)
    /// 7. Save processed version to storage
    /// 8. Return public URLs for both versions
    ///
    /// # Arguments
    /// * `image_data` - Raw image bytes
    /// * `filename` - Original filename (for extension detection)
    ///
    /// # Returns
    /// * `ImageUrls` with public URLs for direct browser access
    ///
    /// # Errors
    /// * File too large (>5MB)
    /// * Invalid image format
    /// * Storage upload failure
    async fn upload_image(&self, image_data: Vec<u8>, filename: String) -> Result<ImageUrls>;

    /// Delete an image from storage
    ///
    /// # Arguments
    /// * `url` - Full public URL of the image to delete
    ///
    /// # Implementation Notes
    /// - Should extract storage key from URL internally
    /// - Should delete both featured and original versions
    /// - Should be idempotent (no error if already deleted)
    ///
    /// # Errors
    /// * Invalid URL format
    /// * Storage deletion failure (unless already deleted)
    async fn delete_image(&self, url: &str) -> Result<()>;
}
