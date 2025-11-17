use anyhow::{Result, anyhow};
use chrono::Utc;
use uuid::Uuid;

use crate::models::api::UpdateBlogPostRequest;
use crate::models::db::BlogPost;
use crate::repositories::traits::UpdateBlogPost;

use super::BlogService;

/// Update existing blog post
///
/// Business logic:
/// - Preserves published_at timestamp for already-published posts
/// - Sets published_at if changing status from draft to published
/// - Validates status values if provided
pub async fn update_post(
    service: &BlogService,
    id: Uuid,
    request: UpdateBlogPostRequest,
) -> Result<BlogPost> {
    // Get existing post
    let existing_post = service
        .repository
        .get_post_by_id(id)
        .await?
        .ok_or_else(|| anyhow!("Blog post not found with ID: {}", id))?;

    // Validate status if provided
    if let Some(ref status) = request.status
        && status != "draft"
        && status != "published"
    {
        return Err(anyhow!(
            "Status must be 'draft' or 'published', got '{}'",
            status
        ));
    }

    // Determine published_at logic:
    // - If post was already published, preserve existing published_at
    // - If changing from draft to published, set published_at to now
    // - If changing from published to draft, clear published_at
    let published_at = match (&existing_post.status[..], request.status.as_deref()) {
        // Already published -> keep existing published_at
        ("published", None) | ("published", Some("published")) => existing_post.published_at,

        // Changing from draft to published -> set now
        ("draft", Some("published")) => Some(Utc::now()),

        // Changing from published to draft -> clear published_at
        ("published", Some("draft")) => None,

        // Draft staying draft -> keep None
        ("draft", None) | ("draft", Some("draft")) => None,

        // Unknown states -> preserve existing
        _ => existing_post.published_at,
    };

    // Create update DTO
    let update_dto = UpdateBlogPost {
        slug: request.slug,
        title: request.title,
        excerpt: request.excerpt,
        content: request.content,
        featured_image_url: request.featured_image_url,
        featured_image_alt: request.featured_image_alt,
        status: request.status,
        tags: request.tags,
        published_at,
        meta_description: request.meta_description,
    };

    // Call repository
    service.repository.update_post(id, update_dto).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{MockBlogRepository, MockImageStorage};
    use crate::test_utils::BlogPostBuilder;
    use chrono::Duration;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_update_post_preserves_published_at() {
        // Given: An already-published post
        let mut mock_repo = MockBlogRepository::new();
        let test_id = Uuid::new_v4();
        let original_published_at = Utc::now() - Duration::days(7);

        mock_repo
            .expect_get_post_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(
                    BlogPostBuilder::new()
                        .with_id(test_id)
                        .published_at(original_published_at)
                        .build(),
                ))
            });

        mock_repo
            .expect_update_post()
            .withf(move |_, update: &UpdateBlogPost| {
                // Verify published_at was preserved (not changed)
                update.published_at == Some(original_published_at)
            })
            .times(1)
            .returning(move |_, _| {
                Ok(BlogPostBuilder::new()
                    .with_id(test_id)
                    .published_at(original_published_at)
                    .build())
            });

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        let request = UpdateBlogPostRequest {
            title: Some("Updated Title".to_string()),
            slug: None,
            content: None,
            excerpt: None,
            featured_image_url: None,
            featured_image_alt: None,
            tags: None,
            status: None, // Not changing status
            meta_description: None,
        };

        // When: Updating published post
        let result = service.update_post(test_id, request).await;

        // Then: published_at preserved
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.published_at, Some(original_published_at));
    }

    #[tokio::test]
    async fn test_update_post_sets_published_at_when_publishing() {
        // Given: A draft post
        let mut mock_repo = MockBlogRepository::new();
        let test_id = Uuid::new_v4();

        mock_repo
            .expect_get_post_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(
                    BlogPostBuilder::new()
                        .with_id(test_id)
                        .draft() // No published_at
                        .build(),
                ))
            });

        mock_repo
            .expect_update_post()
            .withf(move |_, update: &UpdateBlogPost| {
                // Verify published_at was set
                update.published_at.is_some() && update.status == Some("published".to_string())
            })
            .times(1)
            .returning(move |_, _| Ok(BlogPostBuilder::new().with_id(test_id).published().build()));

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        let request = UpdateBlogPostRequest {
            title: None,
            slug: None,
            content: None,
            excerpt: None,
            featured_image_url: None,
            featured_image_alt: None,
            tags: None,
            status: Some("published".to_string()), // Changing to published
            meta_description: None,
        };

        // When: Publishing draft
        let result = service.update_post(test_id, request).await;

        // Then: published_at is set
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert!(updated.published_at.is_some());
    }

    #[tokio::test]
    async fn test_update_post_not_found() {
        // Given: A mock repository with no post
        let mut mock_repo = MockBlogRepository::new();
        let test_id = Uuid::new_v4();

        mock_repo
            .expect_get_post_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        let request = UpdateBlogPostRequest {
            title: Some("Updated".to_string()),
            slug: None,
            content: None,
            excerpt: None,
            featured_image_url: None,
            featured_image_alt: None,
            tags: None,
            status: None,
            meta_description: None,
        };

        // When: Updating non-existent post
        let result = service.update_post(test_id, request).await;

        // Then: Error returned
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_update_post_validates_status() {
        // Given: A published post
        let mut mock_repo = MockBlogRepository::new();
        let test_id = Uuid::new_v4();

        mock_repo
            .expect_get_post_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(
                    BlogPostBuilder::new().with_id(test_id).published().build(),
                ))
            });

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        let request = UpdateBlogPostRequest {
            title: None,
            slug: None,
            content: None,
            excerpt: None,
            featured_image_url: None,
            featured_image_alt: None,
            tags: None,
            status: Some("invalid".to_string()), // Invalid status
            meta_description: None,
        };

        // When: Updating with invalid status
        let result = service.update_post(test_id, request).await;

        // Then: Error returned
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Status must be 'draft' or 'published'")
        );
    }
}
