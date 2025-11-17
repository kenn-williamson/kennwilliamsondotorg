use crate::models::db::blog_post::BlogPost;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// Builder for creating BlogPost instances in tests with sensible defaults.
///
/// # Examples
///
/// ```rust,ignore
/// // Minimal blog post with defaults
/// let post = BlogPostBuilder::new()
///     .persist(pool).await?;
///
/// // Published post with custom content
/// let post = BlogPostBuilder::new()
///     .with_title("My First Post")
///     .with_content("# Hello World\n\nThis is my first blog post.")
///     .published()
///     .persist(pool).await?;
///
/// // Draft post with tags
/// let post = BlogPostBuilder::new()
///     .with_title("Draft Post")
///     .with_tags(vec!["rust", "programming"])
///     .persist(pool).await?;
/// ```
#[derive(Clone)]
pub struct BlogPostBuilder {
    id: Option<Uuid>,
    slug: Option<String>,
    title: Option<String>,
    excerpt: Option<Option<String>>,
    content: Option<String>,
    featured_image_url: Option<Option<String>>,
    featured_image_alt: Option<Option<String>>,
    status: Option<String>,
    tags: Option<Vec<String>>,
    published_at: Option<Option<DateTime<Utc>>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    meta_description: Option<Option<String>>,
}

impl BlogPostBuilder {
    /// Create a new builder with sensible defaults
    pub fn new() -> Self {
        Self {
            id: None,
            slug: None,
            title: None,
            excerpt: None,
            content: None,
            featured_image_url: None,
            featured_image_alt: None,
            status: None,
            tags: None,
            published_at: None,
            created_at: None,
            updated_at: None,
            meta_description: None,
        }
    }

    /// Build the BlogPost with defaults for any unset fields (in-memory only, no database)
    pub fn build(self) -> BlogPost {
        let now = Utc::now();
        let uuid = Uuid::new_v4();
        let default_title = format!("Test Post {}", uuid);
        let default_slug = format!("test-post-{}", uuid);

        BlogPost {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            slug: self.slug.unwrap_or(default_slug),
            title: self.title.unwrap_or(default_title),
            excerpt: self.excerpt.unwrap_or(None),
            content: self
                .content
                .unwrap_or_else(|| "# Test Content\n\nThis is test content.".to_string()),
            featured_image_url: self.featured_image_url.unwrap_or(None),
            featured_image_alt: self.featured_image_alt.unwrap_or(None),
            status: self.status.unwrap_or_else(|| "draft".to_string()),
            tags: self.tags.unwrap_or_else(Vec::new),
            published_at: self.published_at.unwrap_or(None),
            created_at: self.created_at.unwrap_or(now),
            updated_at: self.updated_at.unwrap_or(now),
            meta_description: self.meta_description.unwrap_or(None),
        }
    }

    /// Persist BlogPost to database (for integration tests)
    pub async fn persist(self, pool: &PgPool) -> Result<BlogPost> {
        // Generate defaults
        let uuid = Uuid::new_v4();
        let default_title = format!("Test Post {}", uuid);
        let default_slug = format!("test-post-{}", uuid);

        let slug = self.slug.unwrap_or(default_slug);
        let title = self.title.unwrap_or(default_title);
        let excerpt = self.excerpt.unwrap_or(None);
        let content = self
            .content
            .unwrap_or_else(|| "# Test Content\n\nThis is test content.".to_string());
        let featured_image_url = self.featured_image_url.unwrap_or(None);
        let featured_image_alt = self.featured_image_alt.unwrap_or(None);
        let status = self.status.unwrap_or_else(|| "draft".to_string());
        let tags = self.tags.unwrap_or_else(Vec::new);
        let published_at = self.published_at.unwrap_or(None);
        let meta_description = self.meta_description.unwrap_or(None);

        let post = sqlx::query_as::<_, BlogPost>(
            "INSERT INTO blog_posts (slug, title, excerpt, content, featured_image_url, featured_image_alt, status, tags, published_at, meta_description)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             RETURNING *"
        )
        .bind(slug)
        .bind(title)
        .bind(excerpt)
        .bind(content)
        .bind(featured_image_url)
        .bind(featured_image_alt)
        .bind(status)
        .bind(tags)
        .bind(published_at)
        .bind(meta_description)
        .fetch_one(pool)
        .await?;

        Ok(post)
    }

    // ============================================================================
    // CONFIGURATION METHODS
    // ============================================================================

    /// Set a specific post ID
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the slug (URL-friendly identifier)
    pub fn with_slug(mut self, slug: impl Into<String>) -> Self {
        self.slug = Some(slug.into());
        self
    }

    /// Set the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the excerpt
    pub fn with_excerpt(mut self, excerpt: impl Into<String>) -> Self {
        self.excerpt = Some(Some(excerpt.into()));
        self
    }

    /// Set excerpt to None explicitly
    pub fn without_excerpt(mut self) -> Self {
        self.excerpt = Some(None);
        self
    }

    /// Set the content (markdown)
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set the featured image URL
    pub fn with_featured_image(mut self, url: impl Into<String>) -> Self {
        self.featured_image_url = Some(Some(url.into()));
        self
    }

    /// Set featured image URL to None explicitly
    pub fn without_featured_image(mut self) -> Self {
        self.featured_image_url = Some(None);
        self
    }

    /// Set the featured image alt text
    pub fn with_featured_image_alt(mut self, alt: impl Into<String>) -> Self {
        self.featured_image_alt = Some(Some(alt.into()));
        self
    }

    /// Set the status
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Set status to 'draft'
    pub fn draft(mut self) -> Self {
        self.status = Some("draft".to_string());
        self.published_at = Some(None);
        self
    }

    /// Set status to 'published' with published_at timestamp
    pub fn published(mut self) -> Self {
        self.status = Some("published".to_string());
        self.published_at = Some(Some(Utc::now()));
        self
    }

    /// Set status to 'published' with custom published_at timestamp
    pub fn published_at(mut self, published_at: DateTime<Utc>) -> Self {
        self.status = Some("published".to_string());
        self.published_at = Some(Some(published_at));
        self
    }

    /// Set the tags
    pub fn with_tags<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags = Some(tags.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Add a single tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        let mut tags = self.tags.unwrap_or_else(Vec::new);
        tags.push(tag.into());
        self.tags = Some(tags);
        self
    }

    /// Set the meta description
    pub fn with_meta_description(mut self, meta_description: impl Into<String>) -> Self {
        self.meta_description = Some(Some(meta_description.into()));
        self
    }

    /// Set meta description to None explicitly
    pub fn without_meta_description(mut self) -> Self {
        self.meta_description = Some(None);
        self
    }

    /// Set created_at timestamp
    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    /// Set updated_at timestamp
    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }
}

impl Default for BlogPostBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creates_valid_post_with_defaults() {
        let post = BlogPostBuilder::new().build();

        assert!(!post.id.is_nil());
        assert!(!post.slug.is_empty());
        assert!(!post.title.is_empty());
        assert_eq!(post.status, "draft");
        assert!(post.published_at.is_none());
        assert!(post.tags.is_empty());
    }

    #[test]
    fn test_builder_with_title() {
        let post = BlogPostBuilder::new().with_title("Custom Title").build();

        assert_eq!(post.title, "Custom Title");
    }

    #[test]
    fn test_builder_with_slug() {
        let post = BlogPostBuilder::new().with_slug("custom-slug").build();

        assert_eq!(post.slug, "custom-slug");
    }

    #[test]
    fn test_builder_with_content() {
        let content = "# Hello\n\nWorld";
        let post = BlogPostBuilder::new().with_content(content).build();

        assert_eq!(post.content, content);
    }

    #[test]
    fn test_builder_draft() {
        let post = BlogPostBuilder::new().draft().build();

        assert_eq!(post.status, "draft");
        assert!(post.published_at.is_none());
    }

    #[test]
    fn test_builder_published() {
        let post = BlogPostBuilder::new().published().build();

        assert_eq!(post.status, "published");
        assert!(post.published_at.is_some());
    }

    #[test]
    fn test_builder_with_tags() {
        let post = BlogPostBuilder::new()
            .with_tags(vec!["rust", "programming", "web"])
            .build();

        assert_eq!(post.tags, vec!["rust", "programming", "web"]);
    }

    #[test]
    fn test_builder_with_tag() {
        let post = BlogPostBuilder::new()
            .with_tag("rust")
            .with_tag("web")
            .build();

        assert_eq!(post.tags, vec!["rust", "web"]);
    }

    #[test]
    fn test_builder_with_excerpt() {
        let post = BlogPostBuilder::new()
            .with_excerpt("This is an excerpt")
            .build();

        assert_eq!(post.excerpt, Some("This is an excerpt".to_string()));
    }

    #[test]
    fn test_builder_without_excerpt() {
        let post = BlogPostBuilder::new().without_excerpt().build();

        assert!(post.excerpt.is_none());
    }

    #[test]
    fn test_builder_with_featured_image() {
        let post = BlogPostBuilder::new()
            .with_featured_image("https://example.com/image.jpg")
            .with_featured_image_alt("Alt text")
            .build();

        assert_eq!(
            post.featured_image_url,
            Some("https://example.com/image.jpg".to_string())
        );
        assert_eq!(post.featured_image_alt, Some("Alt text".to_string()));
    }

    #[test]
    fn test_builder_with_meta_description() {
        let post = BlogPostBuilder::new()
            .with_meta_description("SEO description")
            .build();

        assert_eq!(post.meta_description, Some("SEO description".to_string()));
    }

    #[test]
    fn test_builder_chaining() {
        let post = BlogPostBuilder::new()
            .with_title("My Post")
            .with_slug("my-post")
            .with_content("Content here")
            .with_tags(vec!["tag1", "tag2"])
            .published()
            .build();

        assert_eq!(post.title, "My Post");
        assert_eq!(post.slug, "my-post");
        assert_eq!(post.content, "Content here");
        assert_eq!(post.tags, vec!["tag1", "tag2"]);
        assert_eq!(post.status, "published");
        assert!(post.published_at.is_some());
    }
}
