use sqlx::PgPool;
use uuid::Uuid;

/// Phrase moderation service for admin operations
pub struct PhraseModerationService;

impl PhraseModerationService {
    /// Get pending phrase suggestions with submitter info
    pub async fn get_pending_suggestions(pool: &PgPool) -> Result<Vec<PendingSuggestion>, sqlx::Error> {
        let suggestions = sqlx::query_as!(
            PendingSuggestion,
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
        .fetch_all(pool)
        .await?;

        Ok(suggestions)
    }

    /// Approve a phrase suggestion
    pub async fn approve_suggestion(
        pool: &PgPool,
        suggestion_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        // Get the suggestion
        let suggestion = sqlx::query!(
            "SELECT phrase_text, user_id FROM phrase_suggestions WHERE id = $1 AND status = 'pending'",
            suggestion_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        let suggestion = match suggestion {
            Some(s) => s,
            None => return Err(sqlx::Error::RowNotFound),
        };

        // Create the phrase
        sqlx::query!(
            r#"
            INSERT INTO phrases (id, phrase_text, active, created_by, created_at, updated_at)
            VALUES (uuid_generate_v7(), $1, true, $2, NOW(), NOW())
            "#,
            suggestion.phrase_text,
            suggestion.user_id
        )
        .execute(&mut *tx)
        .await?;

        // Update suggestion status
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

        tx.commit().await?;
        Ok(())
    }

    /// Reject a phrase suggestion
    pub async fn reject_suggestion(
        pool: &PgPool,
        suggestion_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<(), sqlx::Error> {
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
        .execute(pool)
        .await?;

        Ok(())
    }
}

/// Pending suggestion with submitter info
#[derive(Debug, Clone, serde::Serialize)]
pub struct PendingSuggestion {
    pub id: Uuid,
    pub phrase_text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user_display_name: String,
    pub user_email: String,
}

impl From<PendingSuggestion> for crate::models::api::PendingSuggestionResponse {
    fn from(suggestion: PendingSuggestion) -> Self {
        Self {
            id: suggestion.id,
            phrase_text: suggestion.phrase_text,
            created_at: suggestion.created_at,
            user_display_name: suggestion.user_display_name,
            user_email: suggestion.user_email,
        }
    }
}
