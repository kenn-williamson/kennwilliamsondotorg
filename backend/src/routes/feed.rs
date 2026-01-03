use actix_web::{web, HttpResponse, Result as ActixResult};

use crate::services::feed::FeedService;

/// GET /backend/public/feed/rss
///
/// Generate RSS 2.0 feed of published blog posts
pub async fn get_rss_feed(service: web::Data<FeedService>) -> ActixResult<HttpResponse> {
    match service.generate_rss().await {
        Ok(rss) => Ok(HttpResponse::Ok()
            .content_type("application/rss+xml; charset=utf-8")
            .insert_header(("Cache-Control", "public, max-age=3600"))
            .body(rss)),
        Err(err) => {
            log::error!("Failed to generate RSS feed: {}", err);
            Ok(HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("Failed to generate RSS feed"))
        }
    }
}

/// GET /backend/public/feed/atom
///
/// Generate Atom feed of published blog posts
pub async fn get_atom_feed(service: web::Data<FeedService>) -> ActixResult<HttpResponse> {
    match service.generate_atom().await {
        Ok(atom) => Ok(HttpResponse::Ok()
            .content_type("application/atom+xml; charset=utf-8")
            .insert_header(("Cache-Control", "public, max-age=3600"))
            .body(atom)),
        Err(err) => {
            log::error!("Failed to generate Atom feed: {}", err);
            Ok(HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("Failed to generate Atom feed"))
        }
    }
}

/// GET /backend/public/feed/json
///
/// Generate JSON Feed 1.1 of published blog posts
pub async fn get_json_feed(service: web::Data<FeedService>) -> ActixResult<HttpResponse> {
    match service.generate_json().await {
        Ok(json) => Ok(HttpResponse::Ok()
            .content_type("application/feed+json; charset=utf-8")
            .insert_header(("Cache-Control", "public, max-age=3600"))
            .body(json)),
        Err(err) => {
            log::error!("Failed to generate JSON feed: {}", err);
            Ok(HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("Failed to generate JSON feed"))
        }
    }
}

/// GET /backend/public/feed
///
/// Redirect to RSS feed (default feed format)
pub async fn get_default_feed() -> HttpResponse {
    HttpResponse::TemporaryRedirect()
        .insert_header(("Location", "/backend/public/feed/rss"))
        .finish()
}
