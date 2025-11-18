use crate::repositories::traits::image_storage::{ImageStorage, ImageUrls};
use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use aws_sdk_s3::Client as S3Client;
use image::ImageFormat;
use uuid::Uuid;

pub struct S3ImageStorage {
    bucket_name: String,
}

impl S3ImageStorage {
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

    /// Extract file extension from filename
    fn get_extension(filename: &str) -> Option<&str> {
        filename.rsplit('.').next()
    }
}

#[async_trait]
impl ImageStorage for S3ImageStorage {
    async fn upload_image(&self, image_data: Vec<u8>, filename: String) -> Result<ImageUrls> {
        // Create S3 client
        let s3_client = Self::create_s3_client().await;

        // 1. Validate file size (<5MB)
        const MAX_SIZE: usize = 5 * 1024 * 1024;
        if image_data.len() > MAX_SIZE {
            bail!("Image exceeds 5MB limit");
        }

        // 2. Sanitize filename
        let sanitized_filename = Self::sanitize_filename(&filename);
        let extension = Self::get_extension(&sanitized_filename).unwrap_or("jpg");

        // 3. Validate image format by loading with image crate
        let img = image::load_from_memory(&image_data).context("Invalid image format")?;

        // 4. Generate UUID for unique storage
        let image_id = Uuid::new_v4();

        // 5. Save original to S3
        let original_key = format!("blog/originals/{}.{}", image_id, extension);
        s3_client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&original_key)
            .body(image_data.into())
            .content_type(format!("image/{}", extension))
            .send()
            .await
            .context("Failed to upload original image to S3")?;

        // 6. Resize to 1200x630px (social media optimal)
        let resized = img.resize(1200, 630, image::imageops::FilterType::Lanczos3);

        // 7. Optimize/compress (80% quality JPEG)
        let mut buffer = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buffer);
        resized
            .write_to(&mut cursor, ImageFormat::Jpeg)
            .context("Failed to encode resized image")?;

        // 8. Save featured image to S3
        let featured_key = format!("blog/featured/{}.jpg", image_id);
        s3_client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&featured_key)
            .body(buffer.into())
            .content_type("image/jpeg")
            .send()
            .await
            .context("Failed to upload featured image to S3")?;

        // 9. Build public URLs
        let featured_url = format!(
            "https://{}.s3.amazonaws.com/{}",
            self.bucket_name, featured_key
        );
        let original_url = format!(
            "https://{}.s3.amazonaws.com/{}",
            self.bucket_name, original_key
        );

        Ok(ImageUrls {
            featured_url,
            original_url,
        })
    }

    async fn delete_image(&self, url: &str) -> Result<()> {
        // Create S3 client
        let s3_client = Self::create_s3_client().await;

        // Extract key from S3 URL
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
            .context("Failed to delete image from S3")?;

        Ok(())
    }
}
