/// Blog API route handlers
///
/// Provides HTTP endpoints for blog post management, including:
/// - Public endpoints for viewing published posts
/// - Admin endpoints for CRUD operations
/// - Image upload handling
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Result as ActixResult, web};
use serde::Deserialize;
use uuid::Uuid;

use crate::middleware::auth::AuthContext;
use crate::models::api::{
    BlogPostListResponse, BlogPostResponse, CreateBlogPostRequest, TagListResponse,
    UpdateBlogPostRequest,
};
use crate::services::blog::BlogService;

// ============================================================================
// PATH AND QUERY EXTRACTORS
// ============================================================================

#[derive(Deserialize)]
pub struct PostIdPath {
    id: Uuid,
}

#[derive(Deserialize)]
pub struct PostSlugPath {
    slug: String,
}

#[derive(Deserialize)]
pub struct ListPostsQuery {
    page: Option<i32>,
    limit: Option<i32>,
    status: Option<String>,
    tag: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
    page: Option<i32>,
    limit: Option<i32>,
}

#[derive(Deserialize)]
pub struct TagsQuery {
    status: Option<String>,
}

// ============================================================================
// PUBLIC ENDPOINTS (No auth required)
// ============================================================================

/// GET /backend/public/blog/posts
/// List posts with optional filtering and pagination
pub async fn get_published_posts(
    query: web::Query<ListPostsQuery>,
    service: web::Data<BlogService>,
) -> ActixResult<HttpResponse> {
    use crate::repositories::traits::BlogPostFilters;

    let filters = BlogPostFilters {
        status: query.status.clone(),
        tag: query.tag.clone(),
        page: query.page.unwrap_or(1),
        limit: query.limit.unwrap_or(10),
    };

    match service.list_posts(filters).await {
        Ok(result) => {
            let response = BlogPostListResponse {
                posts: result.posts.into_iter().map(|p| p.into()).collect(),
                total: result.total,
                page: result.page,
                total_pages: result.total_pages,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            log::error!("Failed to list posts: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

/// GET /backend/public/blog/posts/{slug}
/// Get single post by slug
pub async fn get_post_by_slug(
    path: web::Path<PostSlugPath>,
    service: web::Data<BlogService>,
) -> ActixResult<HttpResponse> {
    match service.get_post_by_slug(&path.slug).await {
        Ok(Some(post)) => {
            let response: BlogPostResponse = post.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        }))),
        Err(err) => {
            log::error!("Failed to get post by slug: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

/// GET /backend/public/blog/tags
/// Get all tags with counts
pub async fn get_all_tags(
    query: web::Query<TagsQuery>,
    service: web::Data<BlogService>,
) -> ActixResult<HttpResponse> {
    match service.get_all_tags(query.status.clone()).await {
        Ok(tags) => {
            let response = TagListResponse {
                tags: tags.into_iter().map(|t| t.into()).collect(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            log::error!("Failed to get tags: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

/// GET /backend/public/blog/search
/// Search posts by full-text query
pub async fn search_posts(
    query: web::Query<SearchQuery>,
    service: web::Data<BlogService>,
) -> ActixResult<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    match service.search_posts(&query.q, page, limit).await {
        Ok(result) => {
            let response = BlogPostListResponse {
                posts: result.posts.into_iter().map(|p| p.into()).collect(),
                total: result.total,
                page: result.page,
                total_pages: result.total_pages,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            log::error!("Failed to search posts: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

// ============================================================================
// ADMIN PROTECTED ENDPOINTS (Requires admin role)
// ============================================================================

/// POST /backend/protected/admin/blog/posts
/// Create new blog post (admin only)
pub async fn create_post(
    req: HttpRequest,
    data: web::Json<CreateBlogPostRequest>,
    service: web::Data<BlogService>,
) -> ActixResult<HttpResponse> {
    let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();

    // Require admin role
    auth_ctx.require_role("admin")?;

    match service.create_post(data.into_inner()).await {
        Ok(post) => {
            let response: BlogPostResponse = post.into();
            Ok(HttpResponse::Created().json(response))
        }
        Err(err) => {
            log::error!("Failed to create post: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

/// PUT /backend/protected/admin/blog/posts/{id}
/// Update existing blog post (admin only)
pub async fn update_post(
    req: HttpRequest,
    path: web::Path<PostIdPath>,
    data: web::Json<UpdateBlogPostRequest>,
    service: web::Data<BlogService>,
) -> ActixResult<HttpResponse> {
    let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();

    // Require admin role
    auth_ctx.require_role("admin")?;

    match service.update_post(path.id, data.into_inner()).await {
        Ok(post) => {
            let response: BlogPostResponse = post.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            log::error!("Failed to update post: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

/// DELETE /backend/protected/admin/blog/posts/{id}
/// Delete blog post (admin only)
pub async fn delete_post(
    req: HttpRequest,
    path: web::Path<PostIdPath>,
    service: web::Data<BlogService>,
) -> ActixResult<HttpResponse> {
    let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();

    // Require admin role
    auth_ctx.require_role("admin")?;

    match service.delete_post(path.id).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => {
            log::error!("Failed to delete post: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}
