use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::db::{Phrase, PhraseSuggestion};

// Request/Response models for phrase operations

#[derive(Debug, Serialize)]
pub struct PhraseResponse {
    pub id: Uuid,
    pub phrase_text: String,
    pub active: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Serialize)]
pub struct PhraseListResponse {
    pub phrases: Vec<PhraseResponse>,
    pub total: i64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CreatePhraseRequest {
    pub phrase_text: String,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UpdatePhraseRequest {
    pub phrase_text: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PhraseSuggestionRequest {
    pub phrase_text: String,
}

#[derive(Debug, Serialize)]
pub struct PhraseSuggestionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub phrase_text: String,
    pub status: String,
    pub admin_id: Option<Uuid>,
    pub admin_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SuggestionListResponse {
    pub suggestions: Vec<PhraseSuggestionResponse>,
    pub total: i64,
}


#[derive(Debug, Serialize)]
pub struct UserExcludedPhraseResponse {
    pub id: Uuid,
    pub phrase_id: Uuid,
    pub phrase_text: String,
    pub excluded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ExcludedPhrasesResponse {
    pub excluded_phrases: Vec<UserExcludedPhraseResponse>,
    pub total: i64,
}

// New DTO for combined phrase data with exclusion status
#[derive(Debug, Serialize)]
pub struct PhraseWithExclusion {
    pub id: Uuid,
    pub phrase_text: String,
    pub active: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_excluded: bool,
}

#[derive(Debug, Serialize)]
pub struct UserPhrasesResponse {
    pub phrases: Vec<PhraseWithExclusion>,
    pub total: i64,
}

// Conversion implementations

impl From<Phrase> for PhraseResponse {
    fn from(phrase: Phrase) -> Self {
        PhraseResponse {
            id: phrase.id,
            phrase_text: phrase.phrase_text,
            active: phrase.active,
            created_by: phrase.created_by,
            created_at: phrase.created_at,
            updated_at: phrase.updated_at,
        }
    }
}

impl From<PhraseSuggestion> for PhraseSuggestionResponse {
    fn from(suggestion: PhraseSuggestion) -> Self {
        PhraseSuggestionResponse {
            id: suggestion.id,
            user_id: suggestion.user_id,
            phrase_text: suggestion.phrase_text,
            status: suggestion.status,
            admin_id: suggestion.admin_id,
            admin_reason: suggestion.admin_reason,
            created_at: suggestion.created_at,
            updated_at: suggestion.updated_at,
        }
    }
}
