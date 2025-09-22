use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;
use sqlx::PgPool;

use crate::repositories::traits::PhraseRepository;
use crate::models::db::{Phrase, PhraseSuggestion, PhraseWithUserExclusionView};
use crate::models::api::{
    CreatePhraseRequest, UpdatePhraseRequest, PhraseSuggestionRequest
};

pub struct PostgresPhraseRepository {
    pool: PgPool,
}

impl PostgresPhraseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PhraseRepository for PostgresPhraseRepository {
    async fn get_random_phrase_by_slug(&self, user_slug: &str) -> Result<String> {
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
        .fetch_one(&self.pool)
        .await?;

        Ok(phrase_text)
    }

    async fn get_random_phrase(&self, user_id: Uuid) -> Result<String> {
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
        .fetch_one(&self.pool)
        .await?;

        Ok(phrase_text)
    }

    async fn get_user_phrases(
        &self, 
        user_id: Uuid, 
        limit: Option<i64>, 
        offset: Option<i64>
    ) -> Result<Vec<Phrase>> {
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
        .fetch_all(&self.pool)
        .await?;

        Ok(phrases)
    }

    async fn get_user_phrases_with_exclusions(&self, user_id: Uuid) -> Result<Vec<PhraseWithUserExclusionView>> {
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
        .fetch_all(&self.pool)
        .await?;

        let phrases_with_exclusions: Vec<PhraseWithUserExclusionView> = phrases
            .into_iter()
            .map(|row| {
                let phrase = Phrase {
                    id: row.id,
                    phrase_text: row.phrase_text,
                    active: row.active,
                    created_by: row.created_by,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                let is_excluded = row.is_excluded.unwrap_or(false);
                PhraseWithUserExclusionView {
                    phrase,
                    is_excluded,
                }
            })
            .collect();

        Ok(phrases_with_exclusions)
    }

    async fn get_all_phrases(
        &self, 
        include_inactive: bool, 
        limit: Option<i64>, 
        offset: Option<i64>
    ) -> Result<Vec<Phrase>> {
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
            .fetch_all(&self.pool)
            .await?;

        Ok(phrases)
    }

    async fn create_phrase(&self, request: CreatePhraseRequest, created_by: Uuid) -> Result<Phrase> {
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
        .fetch_one(&self.pool)
        .await?;

        Ok(phrase)
    }

    async fn update_phrase(&self, phrase_id: Uuid, request: UpdatePhraseRequest) -> Result<Phrase> {
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
        .fetch_one(&self.pool)
        .await?;

        Ok(phrase)
    }

    async fn exclude_phrase_for_user(&self, user_id: Uuid, phrase_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_excluded_phrases (user_id, phrase_id, excluded_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, phrase_id) DO NOTHING
            "#,
            user_id,
            phrase_id,
            chrono::Utc::now()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_phrase_exclusion(&self, user_id: Uuid, phrase_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM user_excluded_phrases 
            WHERE user_id = $1 AND phrase_id = $2
            "#,
            user_id,
            phrase_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_user_excluded_phrases(&self, user_id: Uuid) -> Result<Vec<(Uuid, String, chrono::DateTime<chrono::Utc>)>> {
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
        .fetch_all(&self.pool)
        .await?;

        Ok(exclusions
            .into_iter()
            .map(|row| (row.id, row.phrase_text, row.excluded_at))
            .collect())
    }

    async fn submit_phrase_suggestion(&self, user_id: Uuid, request: PhraseSuggestionRequest) -> Result<PhraseSuggestion> {
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
        .fetch_one(&self.pool)
        .await?;

        Ok(suggestion)
    }

    async fn get_user_suggestions(&self, user_id: Uuid) -> Result<Vec<PhraseSuggestion>> {
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
        .fetch_all(&self.pool)
        .await?;

        Ok(suggestions)
    }

    async fn get_pending_suggestions(&self) -> Result<Vec<crate::repositories::traits::phrase_repository::PendingSuggestionWithUser>> {
        let suggestions = sqlx::query!(
            r#"
            SELECT 
                ps.id,
                ps.phrase_text,
                ps.created_at,
                u.display_name as user_display_name,
                u.email as user_email
            FROM phrase_suggestions ps
            JOIN users u ON ps.user_id = u.id
            WHERE ps.status = 'pending'
            ORDER BY ps.created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(suggestions
            .into_iter()
            .map(|row| crate::repositories::traits::phrase_repository::PendingSuggestionWithUser {
                id: row.id,
                phrase_text: row.phrase_text,
                created_at: row.created_at,
                user_display_name: Some(row.user_display_name),
                user_email: Some(row.user_email),
            })
            .collect())
    }

    async fn approve_suggestion(
        &self,
        suggestion_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()> {
        // Start a transaction
        let mut tx = self.pool.begin().await?;

        // Get the suggestion details
        let suggestion = sqlx::query!(
            r#"
            SELECT user_id, phrase_text
            FROM phrase_suggestions
            WHERE id = $1 AND status = 'pending'
            "#,
            suggestion_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        let suggestion = match suggestion {
            Some(s) => s,
            None => {
                return Err(anyhow::anyhow!("Suggestion not found or already processed"));
            }
        };

        // Create the phrase
        let _phrase = sqlx::query_as!(
            Phrase,
            r#"
            INSERT INTO phrases (phrase_text, active, created_by)
            VALUES ($1, true, $2)
            RETURNING id, phrase_text, active, created_by, created_at, updated_at
            "#,
            suggestion.phrase_text,
            suggestion.user_id
        )
        .fetch_one(&mut *tx)
        .await?;

        // Update the suggestion status
        sqlx::query!(
            r#"
            UPDATE phrase_suggestions
            SET status = 'approved', admin_id = $1, admin_reason = $2, updated_at = NOW()
            WHERE id = $3
            "#,
            admin_id,
            admin_reason,
            suggestion_id
        )
        .execute(&mut *tx)
        .await?;

        // Commit the transaction
        tx.commit().await?;

        Ok(())
    }

    async fn reject_suggestion(
        &self,
        suggestion_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE phrase_suggestions
            SET status = 'rejected', admin_id = $1, admin_reason = $2, updated_at = NOW()
            WHERE id = $3 AND status = 'pending'
            "#,
            admin_id,
            admin_reason,
            suggestion_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn count_all_phrases(&self) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM phrases"
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(count)
    }

    async fn count_pending_suggestions(&self) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM phrase_suggestions WHERE status = 'pending'"
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(count)
    }
}
