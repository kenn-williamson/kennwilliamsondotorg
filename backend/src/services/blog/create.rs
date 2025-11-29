use anyhow::{Result, anyhow};
use chrono::Utc;

use crate::models::api::CreateBlogPostRequest;
use crate::models::db::BlogPost;
use crate::repositories::traits::CreateBlogPost;

use super::BlogService;
use super::utils::{generate_excerpt, generate_slug};

/// Create new blog post
///
/// Business logic:
/// - Auto-generates slug from title if not provided
/// - Handles slug collisions by appending numeric suffix ("-2", "-3", etc.)
/// - Auto-generates excerpt from content if not provided (first 160 chars)
/// - Sets published_at timestamp if status is "published"
/// - Validates title is not empty
pub async fn create_post(
    service: &BlogService,
    request: CreateBlogPostRequest,
) -> Result<BlogPost> {
    // Validate title
    if request.title.trim().is_empty() {
        return Err(anyhow!("Title cannot be empty"));
    }

    // Validate status
    if request.status != "draft" && request.status != "published" {
        return Err(anyhow!(
            "Status must be 'draft' or 'published', got '{}'",
            request.status
        ));
    }

    // Auto-generate slug from title if not provided
    let base_slug = request
        .slug
        .clone()
        .unwrap_or_else(|| generate_slug(&request.title));

    // Handle slug collisions (append -2, -3, etc.)
    let slug = ensure_unique_slug(service, &base_slug).await?;

    // Auto-generate excerpt if not provided
    let excerpt = request
        .excerpt
        .clone()
        .or_else(|| Some(generate_excerpt(&request.content, 160)));

    // Set published_at if status is "published"
    let published_at = if request.status == "published" {
        Some(Utc::now())
    } else {
        None
    };

    // Remember if we're publishing for event emission
    let is_publishing = request.status == "published";

    // Create repository DTO
    let create_dto = CreateBlogPost {
        slug,
        title: request.title,
        excerpt,
        content: request.content,
        featured_image_url: request.featured_image_url,
        featured_image_alt: request.featured_image_alt,
        status: request.status,
        tags: request.tags,
        published_at,
        meta_description: request.meta_description,
    };

    // Call repository
    let post = service.repository.create_post(create_dto).await?;

    // Emit event if post was published
    if is_publishing {
        service.emit_blog_post_published_event(&post).await;
    }

    Ok(post)
}

/// Ensure slug is unique by appending numeric suffix if collision detected
///
/// Checks if slug exists, and if so, appends "-2", "-3", etc. until unique slug found.
async fn ensure_unique_slug(service: &BlogService, base_slug: &str) -> Result<String> {
    let mut slug = base_slug.to_string();
    let mut counter = 2;

    // Check if slug exists
    while service.repository.get_post_by_slug(&slug).await?.is_some() {
        slug = format!("{}-{}", base_slug, counter);
        counter += 1;

        // Safety check to prevent infinite loop
        if counter > 100 {
            return Err(anyhow!(
                "Unable to generate unique slug after 100 attempts for base '{}'",
                base_slug
            ));
        }
    }

    Ok(slug)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{MockBlogRepository, MockImageStorage};
    use crate::test_utils::BlogPostBuilder;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_create_post_auto_generates_slug() {
        // Given: A mock repository and request without slug
        let mut mock_repo = MockBlogRepository::new();

        // Expect slug check (should be available)
        mock_repo
            .expect_get_post_by_slug()
            .with(eq("hello-world"))
            .times(1)
            .returning(|_| Ok(None));

        // Expect create with generated slug
        mock_repo
            .expect_create_post()
            .withf(|post: &CreateBlogPost| post.slug == "hello-world")
            .times(1)
            .returning(|post| {
                Ok(BlogPostBuilder::new()
                    .with_title(&post.title)
                    .with_slug(&post.slug)
                    .draft()
                    .build())
            });

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        let request = CreateBlogPostRequest {
            title: "Hello World".to_string(),
            slug: None, // No slug provided
            content: "Test content".to_string(),
            excerpt: None,
            featured_image_url: None,
            featured_image_alt: None,
            tags: vec![],
            status: "draft".to_string(),
            meta_description: None,
        };

        // When: Creating post
        let result = service.create_post(request).await;

        // Then: Post created with auto-generated slug
        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.slug, "hello-world");
    }

    #[tokio::test]
    async fn test_create_post_handles_slug_collision() {
        // Given: A mock repository where "test-post" already exists
        let mut mock_repo = MockBlogRepository::new();

        // First check: slug exists (collision)
        mock_repo
            .expect_get_post_by_slug()
            .with(eq("test-post"))
            .times(1)
            .returning(|_| Ok(Some(BlogPostBuilder::new().with_slug("test-post").build())));

        // Second check: "test-post-2" is available
        mock_repo
            .expect_get_post_by_slug()
            .with(eq("test-post-2"))
            .times(1)
            .returning(|_| Ok(None));

        // Expect create with "-2" suffix
        mock_repo
            .expect_create_post()
            .withf(|post: &CreateBlogPost| post.slug == "test-post-2")
            .times(1)
            .returning(|post| Ok(BlogPostBuilder::new().with_slug(&post.slug).build()));

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        let request = CreateBlogPostRequest {
            title: "Test".to_string(),
            slug: Some("test-post".to_string()),
            content: "Content".to_string(),
            excerpt: None,
            featured_image_url: None,
            featured_image_alt: None,
            tags: vec![],
            status: "draft".to_string(),
            meta_description: None,
        };

        // When: Creating post with colliding slug
        let result = service.create_post(request).await;

        // Then: Post created with "-2" suffix
        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.slug, "test-post-2");
    }

    #[tokio::test]
    async fn test_create_post_generates_excerpt() {
        // Given: A mock repository and request without excerpt
        let mut mock_repo = MockBlogRepository::new();

        mock_repo.expect_get_post_by_slug().returning(|_| Ok(None));

        mock_repo
            .expect_create_post()
            .withf(|post: &CreateBlogPost| {
                // Verify excerpt was auto-generated from content
                post.excerpt.is_some() && post.excerpt.as_ref().unwrap().starts_with("This is")
            })
            .times(1)
            .returning(|post| {
                let mut builder = BlogPostBuilder::new().with_content(&post.content);
                if let Some(ref excerpt) = post.excerpt {
                    builder = builder.with_excerpt(excerpt);
                }
                Ok(builder.build())
            });

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        let long_content =
            "This is a very long content that should be truncated to 160 characters. ".repeat(10);

        let request = CreateBlogPostRequest {
            title: "Test".to_string(),
            slug: Some("test".to_string()),
            content: long_content,
            excerpt: None, // No excerpt provided
            featured_image_url: None,
            featured_image_alt: None,
            tags: vec![],
            status: "draft".to_string(),
            meta_description: None,
        };

        // When: Creating post
        let result = service.create_post(request).await;

        // Then: Excerpt auto-generated
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_post_sets_published_at() {
        // Given: A mock repository and request with "published" status
        let mut mock_repo = MockBlogRepository::new();

        mock_repo.expect_get_post_by_slug().returning(|_| Ok(None));

        mock_repo
            .expect_create_post()
            .withf(|post: &CreateBlogPost| {
                // Verify published_at is set
                post.published_at.is_some() && post.status == "published"
            })
            .times(1)
            .returning(|post| {
                let mut builder = BlogPostBuilder::new().with_status("published");
                if let Some(pub_at) = post.published_at {
                    builder = builder.published_at(pub_at);
                }
                Ok(builder.build())
            });

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        let request = CreateBlogPostRequest {
            title: "Published Post".to_string(),
            slug: None,
            content: "Content".to_string(),
            excerpt: None,
            featured_image_url: None,
            featured_image_alt: None,
            tags: vec![],
            status: "published".to_string(),
            meta_description: None,
        };

        // When: Creating published post
        let result = service.create_post(request).await;

        // Then: Post has published_at timestamp
        assert!(result.is_ok());
        let post = result.unwrap();
        assert!(post.published_at.is_some());
    }

    #[tokio::test]
    async fn test_create_post_validates_empty_title() {
        // Given: A service and request with empty title
        let service = BlogService::new(
            Box::new(MockBlogRepository::new()),
            Box::new(MockImageStorage::new()),
        );

        let request = CreateBlogPostRequest {
            title: "   ".to_string(), // Empty/whitespace title
            slug: None,
            content: "Content".to_string(),
            excerpt: None,
            featured_image_url: None,
            featured_image_alt: None,
            tags: vec![],
            status: "draft".to_string(),
            meta_description: None,
        };

        // When: Creating post
        let result = service.create_post(request).await;

        // Then: Returns error
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Title cannot be empty")
        );
    }

    #[tokio::test]
    async fn test_create_post_validates_status() {
        // Given: A service and request with invalid status
        let service = BlogService::new(
            Box::new(MockBlogRepository::new()),
            Box::new(MockImageStorage::new()),
        );

        let request = CreateBlogPostRequest {
            title: "Test".to_string(),
            slug: None,
            content: "Content".to_string(),
            excerpt: None,
            featured_image_url: None,
            featured_image_alt: None,
            tags: vec![],
            status: "invalid".to_string(),
            meta_description: None,
        };

        // When: Creating post
        let result = service.create_post(request).await;

        // Then: Returns error
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Status must be 'draft' or 'published'")
        );
    }
}
