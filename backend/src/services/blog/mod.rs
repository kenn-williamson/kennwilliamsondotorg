/// Blog service module
///
/// Implements business logic for blog post management including:
/// - CRUD operations (create, read, update, delete)
/// - Slug generation and collision handling
/// - Excerpt generation from markdown content
/// - Image management (upload, delete)
/// - Publishing workflow (draft → published)
/// - Event emission when posts are published
use anyhow::Result;
use std::sync::Arc;

use crate::events::EventPublisher;
use crate::models::api::{CreateBlogPostRequest, UpdateBlogPostRequest};
use crate::models::db::BlogPost;
use crate::repositories::traits::{BlogRepository, ImageStorage};

pub mod create;
pub mod delete;
pub mod read;
pub mod update;
pub mod utils;

/// BlogService provides business logic for blog post operations
///
/// Uses dependency injection pattern with Arc-wrapped trait objects
/// for testability and flexibility.
pub struct BlogService {
    repository: Arc<dyn BlogRepository>,
    image_storage: Arc<dyn ImageStorage>,
    event_bus: Option<Arc<dyn EventPublisher>>,
}

/// Builder for BlogService with validation
pub struct BlogServiceBuilder {
    repository: Option<Box<dyn BlogRepository>>,
    image_storage: Option<Box<dyn ImageStorage>>,
    event_bus: Option<Arc<dyn EventPublisher>>,
}

impl BlogServiceBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            repository: None,
            image_storage: None,
            event_bus: None,
        }
    }

    /// Set blog repository implementation
    pub fn with_repository(mut self, repository: Box<dyn BlogRepository>) -> Self {
        self.repository = Some(repository);
        self
    }

    /// Set image storage implementation
    pub fn with_image_storage(mut self, image_storage: Box<dyn ImageStorage>) -> Self {
        self.image_storage = Some(image_storage);
        self
    }

    /// Set event bus for publishing domain events
    pub fn with_event_bus(mut self, event_bus: Arc<dyn EventPublisher>) -> Self {
        self.event_bus = Some(event_bus);
        self
    }

    /// Build BlogService with validation
    ///
    /// # Errors
    ///
    /// Returns error if required dependencies (repository, image_storage) are missing
    pub fn build(self) -> Result<BlogService> {
        Ok(BlogService {
            repository: Arc::from(
                self.repository
                    .ok_or_else(|| anyhow::anyhow!("BlogRepository is required"))?,
            ),
            image_storage: Arc::from(
                self.image_storage
                    .ok_or_else(|| anyhow::anyhow!("ImageStorage is required"))?,
            ),
            event_bus: self.event_bus,
        })
    }
}

impl Default for BlogServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BlogService {
    /// Create new builder instance
    pub fn builder() -> BlogServiceBuilder {
        BlogServiceBuilder::new()
    }

    /// Create BlogService directly (useful for testing)
    pub fn new(repository: Box<dyn BlogRepository>, image_storage: Box<dyn ImageStorage>) -> Self {
        Self {
            repository: Arc::from(repository),
            image_storage: Arc::from(image_storage),
            event_bus: None,
        }
    }

    /// Helper method to publish a blog post published event
    pub(crate) async fn emit_blog_post_published_event(&self, post: &BlogPost) {
        if let Some(event_bus) = &self.event_bus {
            let event = crate::events::types::BlogPostPublishedEvent::new(
                post.id,
                &post.slug,
                &post.title,
                post.excerpt.clone(),
                post.featured_image_url.clone(),
            );

            // Fire-and-forget event publishing
            if let Err(e) = event_bus.publish(Box::new(event)).await {
                log::error!("Failed to publish BlogPostPublishedEvent: {}", e);
            } else {
                log::debug!(
                    "Published BlogPostPublishedEvent for post '{}' ({})",
                    post.title,
                    post.slug
                );
            }
        }
    }

    // --- Create Operations ---

    /// Create new blog post
    ///
    /// Handles:
    /// - Auto-generates slug from title if not provided
    /// - Handles slug collisions by appending numeric suffix
    /// - Auto-generates excerpt from content if not provided
    /// - Sets published_at timestamp if status is "published"
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Title is empty
    /// - Slug collision cannot be resolved
    /// - Repository operation fails
    pub async fn create_post(&self, request: CreateBlogPostRequest) -> Result<BlogPost> {
        create::create_post(self, request).await
    }

    // --- Read Operations ---

    /// Get blog post by ID
    pub async fn get_post_by_id(&self, id: uuid::Uuid) -> Result<Option<BlogPost>> {
        read::get_post_by_id(self, id).await
    }

    /// Get blog post by slug
    pub async fn get_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>> {
        read::get_post_by_slug(self, slug).await
    }

    /// List blog posts with filters and pagination
    pub async fn list_posts(
        &self,
        filters: crate::repositories::traits::BlogPostFilters,
    ) -> Result<crate::repositories::traits::BlogPostList> {
        read::list_posts(self, filters).await
    }

    /// Search blog posts using full-text search
    pub async fn search_posts(
        &self,
        query: &str,
        page: i32,
        limit: i32,
    ) -> Result<crate::repositories::traits::BlogPostList> {
        read::search_posts(self, query, page, limit).await
    }

    /// Get all tags with optional status filter
    pub async fn get_all_tags(
        &self,
        status: Option<String>,
    ) -> Result<Vec<crate::repositories::traits::TagCount>> {
        read::get_all_tags(self, status).await
    }

    // --- Update Operations ---

    /// Update existing blog post
    ///
    /// Preserves published_at timestamp for already-published posts.
    /// Sets published_at if changing from draft to published.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Post not found
    /// - Slug collision (if changing slug)
    /// - Repository operation fails
    pub async fn update_post(
        &self,
        id: uuid::Uuid,
        request: UpdateBlogPostRequest,
    ) -> Result<BlogPost> {
        update::update_post(self, id, request).await
    }

    // --- Delete Operations ---

    /// Delete blog post and associated images
    ///
    /// Cleans up featured images from S3 storage after deleting post from database.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Post not found
    /// - Repository operation fails
    /// - Image deletion fails (post still deleted from DB)
    pub async fn delete_post(&self, id: uuid::Uuid) -> Result<()> {
        delete::delete_post(self, id).await
    }

    // --- Image Operations ---

    /// Upload blog featured image
    ///
    /// Handles:
    /// - Image validation (size, format)
    /// - Filename sanitization
    /// - Image processing (resize, compress)
    /// - S3 upload (featured + original)
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Image exceeds 5MB limit
    /// - Invalid image format
    /// - S3 upload fails
    pub async fn upload_image(
        &self,
        image_data: Vec<u8>,
        filename: String,
    ) -> Result<crate::repositories::traits::ImageUrls> {
        self.image_storage.upload_image(image_data, filename).await
    }
}
