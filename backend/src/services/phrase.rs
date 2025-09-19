use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::{Phrase, PhraseSuggestion};
use crate::models::api::{
    CreatePhraseRequest, PhraseResponse, PhraseSuggestionRequest, 
    UpdatePhraseRequest, PhraseWithExclusion, UserPhrasesResponse
};

pub struct PhraseService;

impl PhraseService {
    /// Get a random active phrase for a user by slug, excluding phrases the user has excluded
    pub async fn get_random_phrase_by_slug(
        pool: &PgPool,
        user_slug: &str,
    ) -> Result<String, sqlx::Error> {
        // Single query with JOIN to get phrase text directly
        let phrase_text = sqlx::query_scalar!(
            r#"
            SELECT p.phrase_text
            FROM phrases p
            WHERE p.active = true 
            AND p.id NOT IN (
                SELECT uep.phrase_id 
                FROM user_excluded_phrases uep
                JOIN users u ON uep.user_id = u.id
                WHERE u.slug = $1
            )
            ORDER BY RANDOM()
            LIMIT 1
            "#,
            user_slug
        )
        .fetch_one(pool)
        .await?;

        Ok(phrase_text)
    }

    /// Get a random active phrase, excluding phrases the user has excluded (for authenticated users)
    pub async fn get_random_phrase(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<String, sqlx::Error> {
        // Get random phrase excluding user's excluded phrases
        let phrase_text = sqlx::query_scalar!(
            r#"
            SELECT p.phrase_text
            FROM phrases p
            WHERE p.active = true 
            AND p.id NOT IN (
                SELECT phrase_id 
                FROM user_excluded_phrases 
                WHERE user_id = $1
            )
            ORDER BY RANDOM()
            LIMIT 1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(phrase_text)
    }

    /// Get all active phrases for a user (excluding their excluded phrases)
    pub async fn get_user_phrases(
        pool: &PgPool,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<PhraseResponse>, sqlx::Error> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let phrases = sqlx::query_as!(
            Phrase,
            r#"
            SELECT p.id, p.phrase_text, p.active, p.created_by, p.created_at, p.updated_at
            FROM phrases p
            WHERE p.active = true 
            AND p.id NOT IN (
                SELECT phrase_id 
                FROM user_excluded_phrases 
                WHERE user_id = $1
            )
            ORDER BY p.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(phrases.into_iter().map(PhraseResponse::from).collect())
    }

    /// Get all active phrases for a user with exclusion status (single API call)
    pub async fn get_user_phrases_with_exclusions(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<UserPhrasesResponse, sqlx::Error> {
        let phrases = sqlx::query!(
            r#"
            SELECT 
                p.id,
                p.phrase_text,
                p.active,
                p.created_by,
                p.created_at,
                p.updated_at,
                CASE WHEN uep.phrase_id IS NOT NULL THEN true ELSE false END as is_excluded
            FROM phrases p
            LEFT JOIN user_excluded_phrases uep ON p.id = uep.phrase_id AND uep.user_id = $1
            WHERE p.active = true
            ORDER BY p.created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        let phrases_with_exclusions: Vec<PhraseWithExclusion> = phrases
            .into_iter()
            .map(|row| PhraseWithExclusion {
                id: row.id,
                phrase_text: row.phrase_text,
                active: row.active,
                created_by: row.created_by,
                created_at: row.created_at,
                updated_at: row.updated_at,
                is_excluded: row.is_excluded.unwrap_or(false),
            })
            .collect();

        let total = phrases_with_exclusions.len() as i64;

        Ok(UserPhrasesResponse {
            phrases: phrases_with_exclusions,
            total,
        })
    }

    /// Get all phrases (admin only)
    pub async fn get_all_phrases(
        pool: &PgPool,
        include_inactive: bool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<PhraseResponse>, sqlx::Error> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let where_clause = if include_inactive {
            "1=1"
        } else {
            "active = true"
        };

        let query = format!(
            r#"
            SELECT id, phrase_text, active, created_by, created_at, updated_at
            FROM phrases
            WHERE {}
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            where_clause
        );

        let phrases = sqlx::query_as::<_, Phrase>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok(phrases.into_iter().map(PhraseResponse::from).collect())
    }

    /// Create a new phrase (admin only)
    pub async fn create_phrase(
        pool: &PgPool,
        request: CreatePhraseRequest,
        created_by: Uuid,
    ) -> Result<PhraseResponse, sqlx::Error> {
        let active = request.active.unwrap_or(true);

        let phrase = sqlx::query_as!(
            Phrase,
            r#"
            INSERT INTO phrases (phrase_text, active, created_by)
            VALUES ($1, $2, $3)
            RETURNING id, phrase_text, active, created_by, created_at, updated_at
            "#,
            request.phrase_text,
            active,
            created_by
        )
        .fetch_one(pool)
        .await?;

        Ok(PhraseResponse::from(phrase))
    }

    /// Update a phrase (admin only)
    pub async fn update_phrase(
        pool: &PgPool,
        phrase_id: Uuid,
        request: UpdatePhraseRequest,
    ) -> Result<PhraseResponse, sqlx::Error> {
        let phrase = sqlx::query_as!(
            Phrase,
            r#"
            UPDATE phrases 
            SET 
                phrase_text = COALESCE($2, phrase_text),
                active = COALESCE($3, active),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, phrase_text, active, created_by, created_at, updated_at
            "#,
            phrase_id,
            request.phrase_text,
            request.active
        )
        .fetch_one(pool)
        .await?;

        Ok(PhraseResponse::from(phrase))
    }

    /// Exclude a phrase for a user
    pub async fn exclude_phrase_for_user(
        pool: &PgPool,
        user_id: Uuid,
        phrase_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_excluded_phrases (user_id, phrase_id, excluded_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, phrase_id) DO NOTHING
            "#,
            user_id,
            phrase_id,
            Utc::now()
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Remove phrase exclusion for a user
    pub async fn remove_phrase_exclusion(
        pool: &PgPool,
        user_id: Uuid,
        phrase_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM user_excluded_phrases 
            WHERE user_id = $1 AND phrase_id = $2
            "#,
            user_id,
            phrase_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get user's excluded phrases
    pub async fn get_user_excluded_phrases(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<(Uuid, String, chrono::DateTime<Utc>)>, sqlx::Error> {
        let exclusions = sqlx::query!(
            r#"
            SELECT uep.id, p.phrase_text, uep.excluded_at
            FROM user_excluded_phrases uep
            JOIN phrases p ON uep.phrase_id = p.id
            WHERE uep.user_id = $1
            ORDER BY uep.excluded_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(exclusions
            .into_iter()
            .map(|row| (row.id, row.phrase_text, row.excluded_at))
            .collect())
    }

    /// Submit a phrase suggestion
    pub async fn submit_phrase_suggestion(
        pool: &PgPool,
        user_id: Uuid,
        request: PhraseSuggestionRequest,
    ) -> Result<PhraseSuggestion, sqlx::Error> {
        let suggestion = sqlx::query_as!(
            PhraseSuggestion,
            r#"
            INSERT INTO phrase_suggestions (user_id, phrase_text, status)
            VALUES ($1, $2, 'pending')
            RETURNING id, user_id, phrase_text, status, admin_id, admin_reason, created_at, updated_at
            "#,
            user_id,
            request.phrase_text
        )
        .fetch_one(pool)
        .await?;

        Ok(suggestion)
    }

    /// Get user's phrase suggestions
    pub async fn get_user_suggestions(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<PhraseSuggestion>, sqlx::Error> {
        let suggestions = sqlx::query_as!(
            PhraseSuggestion,
            r#"
            SELECT id, user_id, phrase_text, status, admin_id, admin_reason, created_at, updated_at
            FROM phrase_suggestions
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(suggestions)
    }

}
