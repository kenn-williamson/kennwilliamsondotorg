use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};
use uuid::Uuid;

use crate::models::api::{
    ExcludedPhrasesResponse, PhraseListResponse, PhraseSuggestionRequest, PhraseSuggestionResponse,
    SuggestionListResponse, UserExcludedPhraseResponse,
};
use crate::services::phrase::PhraseService;

/// Get a random phrase for a specific user (public endpoint)
pub async fn get_random_phrase_for_user(
    phrase_service: web::Data<PhraseService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let user_slug = path.into_inner();

    match phrase_service.get_random_phrase_by_slug(&user_slug).await {
        Ok(phrase_text) => Ok(HttpResponse::Ok().json(phrase_text)),
        Err(e) => {
            log::error!("Failed to get random phrase for user {}: {}", user_slug, e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get random phrase"
            })))
        }
    }
}

/// Get a random phrase for the authenticated user (protected endpoint)
pub async fn get_random_phrase_for_auth_user(
    phrase_service: web::Data<PhraseService>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    match phrase_service.get_random_phrase(user_id).await {
        Ok(phrase_text) => Ok(HttpResponse::Ok().json(phrase_text)),
        Err(e) => {
            log::error!("Failed to get random phrase for authenticated user: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get random phrase"
            })))
        }
    }
}

/// Get all active phrases for authenticated user
pub async fn get_user_phrases(
    phrase_service: web::Data<PhraseService>,
    req: HttpRequest,
    query: web::Query<PhraseListQuery>,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    let limit = query.limit;
    let offset = query.offset;

    match phrase_service
        .get_user_phrases(user_id, limit, offset)
        .await
    {
        Ok(phrases) => {
            let total = phrases.len() as i64;
            let response = PhraseListResponse { phrases, total };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to get user phrases: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get user phrases"
            })))
        }
    }
}

/// Get all phrases for user with exclusion status (single API call)
pub async fn get_user_phrases_with_exclusions(
    phrase_service: web::Data<PhraseService>,
    req: HttpRequest,
    query: web::Query<PhraseListQuery>,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    let limit = query.limit;
    let offset = query.offset;
    let search = query.search.clone();

    match phrase_service
        .get_user_phrases_with_exclusions(user_id, limit, offset, search)
        .await
    {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => {
            log::error!("Failed to get user phrases with exclusions: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get user phrases"
            })))
        }
    }
}

/// Exclude a phrase for the authenticated user
pub async fn exclude_phrase(
    phrase_service: web::Data<PhraseService>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    let phrase_id = path.into_inner();

    match phrase_service
        .exclude_phrase_for_user(user_id, phrase_id)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Phrase excluded successfully"
        }))),
        Err(e) => {
            log::error!("Failed to exclude phrase: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to exclude phrase"
            })))
        }
    }
}

/// Remove phrase exclusion for the authenticated user
pub async fn remove_phrase_exclusion(
    phrase_service: web::Data<PhraseService>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    let phrase_id = path.into_inner();

    match phrase_service
        .remove_phrase_exclusion(user_id, phrase_id)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Phrase exclusion removed successfully"
        }))),
        Err(e) => {
            log::error!("Failed to remove phrase exclusion: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to remove phrase exclusion"
            })))
        }
    }
}

/// Get user's excluded phrases
pub async fn get_excluded_phrases(
    phrase_service: web::Data<PhraseService>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match phrase_service.get_user_excluded_phrases(user_id).await {
        Ok(exclusions) => {
            let excluded_phrases: Vec<UserExcludedPhraseResponse> = exclusions
                .into_iter()
                .map(
                    |(id, phrase_text, excluded_at)| UserExcludedPhraseResponse {
                        id,
                        phrase_id: id, // This should be the phrase_id, but we're using id for now
                        phrase_text,
                        excluded_at,
                    },
                )
                .collect();

            let total = excluded_phrases.len() as i64;
            let response = ExcludedPhrasesResponse {
                excluded_phrases,
                total,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to get excluded phrases: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get excluded phrases"
            })))
        }
    }
}

/// Submit a phrase suggestion
pub async fn submit_suggestion(
    phrase_service: web::Data<PhraseService>,
    req: HttpRequest,
    request: web::Json<PhraseSuggestionRequest>,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match phrase_service
        .submit_phrase_suggestion(user_id, request.into_inner())
        .await
    {
        Ok(suggestion) => {
            let response: PhraseSuggestionResponse = suggestion.into();
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => {
            log::error!("Failed to submit suggestion: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to submit suggestion"
            })))
        }
    }
}

/// Get user's phrase suggestions
pub async fn get_user_suggestions(
    phrase_service: web::Data<PhraseService>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match phrase_service.get_user_suggestions(user_id).await {
        Ok(suggestions) => {
            let total = suggestions.len() as i64;
            let response = SuggestionListResponse {
                suggestions: suggestions.into_iter().map(|s| s.into()).collect(),
                total,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to get user suggestions: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get user suggestions"
            })))
        }
    }
}

// Query parameters for list endpoints
#[derive(serde::Deserialize)]
pub struct PhraseListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub search: Option<String>,
}
