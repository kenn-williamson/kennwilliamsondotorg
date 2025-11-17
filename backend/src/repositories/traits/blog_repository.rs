use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::db::BlogPost;

/// Data structures for repository operations

#[derive(Debug, Clone)]
pub struct CreateBlogPost {
    pub slug: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub featured_image_url: Option<String>,
    pub featured_image_alt: Option<String>,
    pub status: String,
    pub tags: Vec<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateBlogPost {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub featured_image_url: Option<String>,
    pub featured_image_alt: Option<String>,
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BlogPostFilters {
    pub status: Option<String>,
    pub tag: Option<String>,
    pub page: i32,
    pub limit: i32,
}

#[derive(Debug, Clone)]
pub struct BlogPostList {
    pub posts: Vec<BlogPost>,
    pub total: i64,
    pub page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TagCount {
    pub tag: String,
    pub count: i64,
}

/// Repository trait for blog post operations
#[async_trait]
pub trait BlogRepository: Send + Sync {
    /// Create a new blog post
    async fn create_post(&self, post: CreateBlogPost) -> Result<BlogPost>;

    /// Get a blog post by ID
    async fn get_post_by_id(&self, id: Uuid) -> Result<Option<BlogPost>>;

    /// Get a blog post by slug
    async fn get_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>>;

    /// List blog posts with filters and pagination
    async fn list_posts(&self, filters: BlogPostFilters) -> Result<BlogPostList>;

    /// Update a blog post
    async fn update_post(&self, id: Uuid, post: UpdateBlogPost) -> Result<BlogPost>;

    /// Delete a blog post
    async fn delete_post(&self, id: Uuid) -> Result<()>;

    /// Search blog posts using full-text search
    async fn search_posts(&self, query: &str, page: i32, limit: i32) -> Result<BlogPostList>;

    /// Get all tags with counts (optionally filter by status)
    async fn get_all_tags(&self, status: Option<String>) -> Result<Vec<TagCount>>;
}
