use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::models::db::{EmailSuppression, EmailType};
use crate::repositories::traits::email_suppression_repository::{
    CreateSuppressionData, EmailSuppressionRepository,
};

/// PostgreSQL implementation of EmailSuppressionRepository
pub struct PostgresEmailSuppressionRepository {
    pool: PgPool,
}

impl PostgresEmailSuppressionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmailSuppressionRepository for PostgresEmailSuppressionRepository {
    async fn create_suppression(
        &self,
        data: &CreateSuppressionData,
    ) -> Result<EmailSuppression> {
        let suppression = sqlx::query_as!(
            EmailSuppression,
            r#"
            INSERT INTO email_suppressions (
                email,
                suppression_type,
                reason,
                suppress_transactional,
                suppress_marketing
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                email,
                suppression_type,
                reason,
                suppress_transactional,
                suppress_marketing,
                bounce_count,
                last_bounce_at,
                created_at,
                updated_at
            "#,
            data.email,
            data.suppression_type,
            data.reason,
            data.suppress_transactional,
            data.suppress_marketing
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(suppression)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<EmailSuppression>> {
        let suppression = sqlx::query_as!(
            EmailSuppression,
            r#"
            SELECT
                id,
                email,
                suppression_type,
                reason,
                suppress_transactional,
                suppress_marketing,
                bounce_count,
                last_bounce_at,
                created_at,
                updated_at
            FROM email_suppressions
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(suppression)
    }

    async fn is_email_suppressed(&self, email: &str, email_type: EmailType) -> Result<bool> {
        let result = match email_type {
            EmailType::Transactional => {
                sqlx::query!(
                    r#"
                    SELECT suppress_transactional
                    FROM email_suppressions
                    WHERE email = $1
                    "#,
                    email
                )
                .fetch_optional(&self.pool)
                .await?
                .map(|row| row.suppress_transactional)
                .unwrap_or(false)
            }
            EmailType::Marketing => {
                sqlx::query!(
                    r#"
                    SELECT suppress_marketing
                    FROM email_suppressions
                    WHERE email = $1
                    "#,
                    email
                )
                .fetch_optional(&self.pool)
                .await?
                .map(|row| row.suppress_marketing)
                .unwrap_or(false)
            }
        };

        Ok(result)
    }

    async fn increment_bounce_count(
        &self,
        email: &str,
        bounced_at: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE email_suppressions
            SET
                bounce_count = bounce_count + 1,
                last_bounce_at = $2,
                updated_at = NOW()
            WHERE email = $1
            "#,
            email,
            bounced_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_suppression(&self, email: &str) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM email_suppressions
            WHERE email = $1
            "#,
            email
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
