/// Music (song) API route handlers
///
/// Provides HTTP endpoints for the music section:
/// - Public endpoints for viewing published songs
/// - Admin endpoints for CRUD operations and audio/artwork uploads
use actix_multipart::Multipart;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Result as ActixResult, web};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::middleware::auth::AuthContext;
use crate::models::api::{
    AudioUploadResponse, CreateSongRequest, SongListResponse, SongResponse, UpdateSongRequest,
};
use crate::repositories::traits::SongFilters;
use crate::services::music::MusicService;

// ============================================================================
// PATH AND QUERY EXTRACTORS
// ============================================================================

#[derive(Deserialize)]
pub struct SongIdPath {
    id: Uuid,
}

#[derive(Deserialize)]
pub struct SongSlugPath {
    slug: String,
}

#[derive(Deserialize)]
pub struct ListSongsQuery {
    page: Option<i32>,
    limit: Option<i32>,
    status: Option<String>,
}

// ============================================================================
// PUBLIC ENDPOINTS (No auth required) - published songs only
// ============================================================================

/// GET /backend/public/music/songs
/// List published songs (ordered by display_order, then newest)
pub async fn get_published_songs(
    query: web::Query<ListSongsQuery>,
    service: web::Data<MusicService>,
) -> ActixResult<HttpResponse> {
    // Force published status so drafts never leak through the public endpoint.
    let filters = SongFilters {
        status: Some("published".to_string()),
        page: query.page.unwrap_or(1),
        limit: query.limit.unwrap_or(50),
    };

    match service.list_songs(filters).await {
        Ok(result) => Ok(HttpResponse::Ok().json(SongListResponse {
            songs: result.songs.into_iter().map(|s| s.into()).collect(),
            total: result.total,
            page: result.page,
            total_pages: result.total_pages,
        })),
        Err(err) => {
            log::error!("Failed to list songs: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

/// GET /backend/public/music/songs/{slug}
/// Get a single published song by slug
pub async fn get_song_by_slug(
    path: web::Path<SongSlugPath>,
    service: web::Data<MusicService>,
) -> ActixResult<HttpResponse> {
    match service.get_song_by_slug(&path.slug).await {
        // Only expose published songs publicly
        Ok(Some(song)) if song.status == "published" => {
            let response: SongResponse = song.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(_) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Song not found"
        }))),
        Err(err) => {
            log::error!("Failed to get song by slug: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

// ============================================================================
// ADMIN PROTECTED ENDPOINTS (Requires admin role)
// ============================================================================

/// GET /backend/protected/admin/music/songs
/// List all songs (drafts + published) with optional status filter
pub async fn get_all_songs(
    req: HttpRequest,
    query: web::Query<ListSongsQuery>,
    service: web::Data<MusicService>,
) -> ActixResult<HttpResponse> {
    let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();
    auth_ctx.require_role("admin")?;

    let filters = SongFilters {
        status: query.status.clone(),
        page: query.page.unwrap_or(1),
        limit: query.limit.unwrap_or(100),
    };

    match service.list_songs(filters).await {
        Ok(result) => Ok(HttpResponse::Ok().json(SongListResponse {
            songs: result.songs.into_iter().map(|s| s.into()).collect(),
            total: result.total,
            page: result.page,
            total_pages: result.total_pages,
        })),
        Err(err) => {
            log::error!("Failed to list songs (admin): {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

/// POST /backend/protected/admin/music/songs
/// Create a new song (admin only)
pub async fn create_song(
    req: HttpRequest,
    data: web::Json<CreateSongRequest>,
    service: web::Data<MusicService>,
) -> ActixResult<HttpResponse> {
    let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();
    auth_ctx.require_role("admin")?;

    match service.create_song(data.into_inner()).await {
        Ok(song) => {
            let response: SongResponse = song.into();
            Ok(HttpResponse::Created().json(response))
        }
        Err(err) => {
            log::error!("Failed to create song: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

/// PUT /backend/protected/admin/music/songs/{id}
/// Update an existing song (admin only)
pub async fn update_song(
    req: HttpRequest,
    path: web::Path<SongIdPath>,
    data: web::Json<UpdateSongRequest>,
    service: web::Data<MusicService>,
) -> ActixResult<HttpResponse> {
    let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();
    auth_ctx.require_role("admin")?;

    match service.update_song(path.id, data.into_inner()).await {
        Ok(song) => {
            let response: SongResponse = song.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            log::error!("Failed to update song: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

/// DELETE /backend/protected/admin/music/songs/{id}
/// Delete a song (admin only)
pub async fn delete_song(
    req: HttpRequest,
    path: web::Path<SongIdPath>,
    service: web::Data<MusicService>,
) -> ActixResult<HttpResponse> {
    let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();
    auth_ctx.require_role("admin")?;

    match service.delete_song(path.id).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => {
            log::error!("Failed to delete song: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

/// Response for artwork upload
#[derive(Serialize)]
struct ArtworkUploadResponse {
    url: String,
    original_url: String,
}

/// Collect a single named file field from a multipart payload into memory.
async fn read_file_field(
    mut payload: Multipart,
    field_name: &str,
    default_filename: &str,
) -> ActixResult<(Vec<u8>, String)> {
    let mut data: Vec<u8> = Vec::new();
    let mut filename = default_filename.to_string();

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            log::error!("Failed to read multipart field: {}", e);
            actix_web::error::ErrorBadRequest("Invalid multipart data")
        })?;

        let content_disposition = field.content_disposition();

        if let Some(name) = content_disposition.get_name()
            && name == field_name
        {
            if let Some(fname) = content_disposition.get_filename() {
                filename = fname.to_string();
            }

            while let Some(chunk) = field.next().await {
                let chunk = chunk.map_err(|e| {
                    log::error!("Failed to read chunk: {}", e);
                    actix_web::error::ErrorBadRequest("Failed to read upload data")
                })?;
                data.extend_from_slice(&chunk);
            }
        }
    }

    Ok((data, filename))
}

/// POST /backend/protected/admin/music/upload-audio
/// Upload a song's audio file (admin only)
pub async fn upload_audio(
    req: HttpRequest,
    payload: Multipart,
    service: web::Data<MusicService>,
) -> ActixResult<HttpResponse> {
    let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();
    auth_ctx.require_role("admin")?;

    let (audio_data, filename) = read_file_field(payload, "audio", "upload.mp3").await?;

    if audio_data.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No audio data received"
        })));
    }

    match service.upload_audio(audio_data, filename).await {
        Ok(upload) => Ok(HttpResponse::Ok().json(AudioUploadResponse {
            url: upload.url,
            duration_seconds: upload.duration_seconds,
        })),
        Err(err) => {
            let error_msg = err.to_string();
            log::error!("Failed to upload audio: {}", error_msg);
            if error_msg.contains("exceeds") || error_msg.contains("Invalid") {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({ "error": error_msg })))
            } else {
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to upload audio"
                })))
            }
        }
    }
}

/// POST /backend/protected/admin/music/upload-artwork
/// Upload a song's cover artwork (admin only)
pub async fn upload_artwork(
    req: HttpRequest,
    payload: Multipart,
    service: web::Data<MusicService>,
) -> ActixResult<HttpResponse> {
    let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();
    auth_ctx.require_role("admin")?;

    let (image_data, filename) = read_file_field(payload, "artwork", "artwork.jpg").await?;

    if image_data.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No artwork data received"
        })));
    }

    match service.upload_artwork(image_data, filename).await {
        Ok(urls) => Ok(HttpResponse::Ok().json(ArtworkUploadResponse {
            url: urls.featured_url,
            original_url: urls.original_url,
        })),
        Err(err) => {
            let error_msg = err.to_string();
            log::error!("Failed to upload artwork: {}", error_msg);
            if error_msg.contains("exceeds") || error_msg.contains("Invalid") {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({ "error": error_msg })))
            } else {
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to upload artwork"
                })))
            }
        }
    }
}
