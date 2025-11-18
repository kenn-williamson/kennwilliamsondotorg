/// Blog API route handlers
///
/// Provides HTTP endpoints for blog post management, including:
/// - Public endpoints for viewing published posts
/// - Admin endpoints for CRUD operations
/// - Image upload handling
use actix_multipart::Multipart;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Result as ActixResult, web};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
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

/// Response for image upload
#[derive(Serialize)]
struct ImageUploadResponse {
    url: String,
    original_url: String,
}

/// POST /backend/protected/admin/blog/upload-image
/// Upload blog featured image (admin only)
pub async fn upload_image(
    req: HttpRequest,
    mut payload: Multipart,
    service: web::Data<BlogService>,
) -> ActixResult<HttpResponse> {
    let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();

    // Require admin role
    auth_ctx.require_role("admin")?;

    // Extract image data from multipart form
    let mut image_data: Vec<u8> = Vec::new();
    let mut filename = String::from("upload.jpg");

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            log::error!("Failed to read multipart field: {}", e);
            actix_web::error::ErrorBadRequest("Invalid multipart data")
        })?;

        let content_disposition = field.content_disposition();

        // Get the field name
        if let Some(name) = content_disposition.get_name()
            && name == "image"
        {
            // Get filename if available
            if let Some(fname) = content_disposition.get_filename() {
                filename = fname.to_string();
            }

            // Read the field data
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| {
                    log::error!("Failed to read chunk: {}", e);
                    actix_web::error::ErrorBadRequest("Failed to read image data")
                })?;
                image_data.extend_from_slice(&data);
            }
        }
    }

    // Validate that we received image data
    if image_data.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No image data received"
        })));
    }

    // Upload image via ImageStorage (S3)
    match service.upload_image(image_data, filename).await {
        Ok(urls) => {
            let response = ImageUploadResponse {
                url: urls.featured_url,
                original_url: urls.original_url,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            let error_msg = err.to_string();
            log::error!("Failed to upload image: {}", error_msg);

            // Return 400 for validation errors, 500 for server errors
            if error_msg.contains("exceeds") || error_msg.contains("Invalid") {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": error_msg
                })))
            } else {
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to upload image"
                })))
            }
        }
    }
}
