use anyhow::{Result, anyhow};
use uuid::Uuid;

use super::BlogService;

/// Delete blog post and associated images
///
/// Business logic:
/// - Fetches post to get image URLs before deletion
/// - Deletes post from database
/// - Cleans up featured images from S3 storage
///
/// Note: If image deletion fails, the post is still deleted from database.
/// This prevents orphaned database records. Orphaned images can be cleaned up
/// separately via periodic S3 bucket cleanup.
pub async fn delete_post(service: &BlogService, id: Uuid) -> Result<()> {
    // Get post to find image URLs before deleting
    let post = service
        .repository
        .get_post_by_id(id)
        .await?
        .ok_or_else(|| anyhow!("Blog post not found with ID: {}", id))?;

    // Delete from database first
    service.repository.delete_post(id).await?;

    // Clean up images (best effort - don't fail if image deletion fails)
    if let Some(featured_url) = post.featured_image_url {
        // Log error but don't fail - post already deleted from DB
        if let Err(e) = service.image_storage.delete_image(&featured_url).await {
            eprintln!(
                "Warning: Failed to delete featured image '{}' for post {}: {}",
                featured_url, id, e
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{MockBlogRepository, MockImageStorage};
    use crate::test_utils::BlogPostBuilder;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_delete_post_removes_images() {
        // Given: A post with featured image
        let mut mock_repo = MockBlogRepository::new();
        let mut mock_storage = MockImageStorage::new();
        let test_id = Uuid::new_v4();
        let image_url = "https://s3.amazonaws.com/bucket/featured/image.jpg".to_string();
        let image_url_for_returning = image_url.clone();

        mock_repo
            .expect_get_post_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(
                    BlogPostBuilder::new()
                        .with_id(test_id)
                        .with_featured_image(&image_url_for_returning)
                        .build(),
                ))
            });

        mock_repo
            .expect_delete_post()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(()));

        // Expect image deletion
        mock_storage
            .expect_delete_image()
            .with(eq(image_url))
            .times(1)
            .returning(|_| Ok(()));

        let service = BlogService::new(Box::new(mock_repo), Box::new(mock_storage));

        // When: Deleting post
        let result = service.delete_post(test_id).await;

        // Then: Post and image deleted
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_post_without_image() {
        // Given: A post without featured image
        let mut mock_repo = MockBlogRepository::new();
        let mock_storage = MockImageStorage::new();
        let test_id = Uuid::new_v4();

        mock_repo
            .expect_get_post_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(
                    BlogPostBuilder::new()
                        .with_id(test_id)
                        .without_featured_image()
                        .build(),
                ))
            });

        mock_repo
            .expect_delete_post()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(()));

        // No image deletion expected
        let service = BlogService::new(Box::new(mock_repo), Box::new(mock_storage));

        // When: Deleting post
        let result = service.delete_post(test_id).await;

        // Then: Post deleted successfully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_post_not_found() {
        // Given: No post exists
        let mut mock_repo = MockBlogRepository::new();
        let mock_storage = MockImageStorage::new();
        let test_id = Uuid::new_v4();

        mock_repo
            .expect_get_post_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = BlogService::new(Box::new(mock_repo), Box::new(mock_storage));

        // When: Deleting non-existent post
        let result = service.delete_post(test_id).await;

        // Then: Error returned
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_delete_post_succeeds_even_if_image_deletion_fails() {
        // Given: A post with image, but image deletion will fail
        let mut mock_repo = MockBlogRepository::new();
        let mut mock_storage = MockImageStorage::new();
        let test_id = Uuid::new_v4();
        let image_url = "https://s3.amazonaws.com/bucket/featured/image.jpg".to_string();
        let image_url_for_returning = image_url.clone();

        mock_repo
            .expect_get_post_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(
                    BlogPostBuilder::new()
                        .with_id(test_id)
                        .with_featured_image(&image_url_for_returning)
                        .build(),
                ))
            });

        mock_repo
            .expect_delete_post()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(()));

        // Image deletion fails
        mock_storage
            .expect_delete_image()
            .with(eq(image_url))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("S3 error")));

        let service = BlogService::new(Box::new(mock_repo), Box::new(mock_storage));

        // When: Deleting post
        let result = service.delete_post(test_id).await;

        // Then: Post deletion still succeeds (image error logged but not propagated)
        assert!(result.is_ok());
    }
}
