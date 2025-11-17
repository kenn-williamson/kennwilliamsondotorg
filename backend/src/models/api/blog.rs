use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::db::BlogPost;

// Request/Response models for blog operations

#[derive(Debug, Serialize)]
pub struct BlogPostResponse {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub featured_image_url: Option<String>,
    pub featured_image_alt: Option<String>,
    pub status: String,
    pub tags: Vec<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BlogPostListResponse {
    pub posts: Vec<BlogPostResponse>,
    pub total: i64,
    pub page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CreateBlogPostRequest {
    pub title: String,
    pub slug: Option<String>, // Auto-generated if not provided
    pub content: String,
    pub excerpt: Option<String>, // Auto-generated if not provided
    pub featured_image_url: Option<String>,
    pub featured_image_alt: Option<String>,
    pub tags: Vec<String>,
    pub status: String, // 'draft' | 'published'
    pub meta_description: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UpdateBlogPostRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub featured_image_url: Option<String>,
    pub featured_image_alt: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub tag: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct TagListResponse {
    pub tags: Vec<TagResponse>,
}

#[derive(Debug, Serialize)]
pub struct ImageUploadResponse {
    pub featured_url: String,
    pub original_url: String,
}

// Conversion implementations

impl From<BlogPost> for BlogPostResponse {
    fn from(post: BlogPost) -> Self {
        BlogPostResponse {
            id: post.id,
            slug: post.slug,
            title: post.title,
            excerpt: post.excerpt,
            content: post.content,
            featured_image_url: post.featured_image_url,
            featured_image_alt: post.featured_image_alt,
            status: post.status,
            tags: post.tags,
            published_at: post.published_at,
            created_at: post.created_at,
            updated_at: post.updated_at,
            meta_description: post.meta_description,
        }
    }
}
