use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;

use crate::repositories::traits::image_storage::{ImageStorage, ImageUrls};

// Generate mock for ImageStorage trait
mock! {
    pub ImageStorage {}

    #[async_trait]
    impl ImageStorage for ImageStorage {
        async fn upload_image(&self, image_data: Vec<u8>, filename: String) -> Result<ImageUrls>;
        async fn delete_image(&self, url: &str) -> Result<()>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_mock_upload_image() {
        let mut mock_storage = MockImageStorage::new();

        let image_data = vec![0u8; 1024]; // 1KB fake image
        let filename = "test.jpg".to_string();

        // Setup mock expectation
        mock_storage
            .expect_upload_image()
            .times(1)
            .with(eq(image_data.clone()), eq(filename.clone()))
            .returning(|_, _| {
                Ok(ImageUrls::new(
                    "https://example.s3.amazonaws.com/blog/featured/test-123.jpg",
                    "https://example.s3.amazonaws.com/blog/originals/test-123.jpg",
                ))
            });

        // Test the mock
        let result = mock_storage.upload_image(image_data, filename).await;
        assert!(result.is_ok());

        let urls = result.unwrap();
        assert!(urls.featured_url.contains("featured"));
        assert!(urls.original_url.contains("originals"));
    }

    #[tokio::test]
    async fn test_mock_delete_image() {
        let mut mock_storage = MockImageStorage::new();

        let image_url = "https://example.s3.amazonaws.com/blog/featured/test-123.jpg";

        // Setup mock expectation
        mock_storage
            .expect_delete_image()
            .times(1)
            .with(eq(image_url))
            .returning(|_| Ok(()));

        // Test the mock
        let result = mock_storage.delete_image(image_url).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_upload_error() {
        let mut mock_storage = MockImageStorage::new();

        // Setup mock to return an error
        mock_storage
            .expect_upload_image()
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("File too large")));

        // Test error handling
        let result = mock_storage
            .upload_image(vec![0u8; 1024], "test.jpg".to_string())
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("File too large"));
    }

    #[tokio::test]
    async fn test_mock_delete_error() {
        let mut mock_storage = MockImageStorage::new();

        // Setup mock to return an error
        mock_storage
            .expect_delete_image()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("S3 connection failed")));

        // Test error handling
        let result = mock_storage
            .delete_image("https://example.com/image.jpg")
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("S3 connection failed")
        );
    }

    #[tokio::test]
    async fn test_image_urls_equality() {
        let urls1 = ImageUrls::new(
            "https://example.com/featured.jpg",
            "https://example.com/original.jpg",
        );

        let urls2 = ImageUrls::new(
            "https://example.com/featured.jpg",
            "https://example.com/original.jpg",
        );

        assert_eq!(urls1, urls2);
    }

    #[tokio::test]
    async fn test_image_urls_creation() {
        let urls = ImageUrls::new(
            "https://example.com/featured.jpg",
            "https://example.com/original.jpg",
        );

        assert_eq!(urls.featured_url, "https://example.com/featured.jpg");
        assert_eq!(urls.original_url, "https://example.com/original.jpg");
    }
}
