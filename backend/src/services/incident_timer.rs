use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::models::db::IncidentTimer;
use crate::models::api::{CreateIncidentTimer, UpdateIncidentTimer};

#[derive(Clone)]
pub struct IncidentTimerService {
    pool: PgPool,
}

impl IncidentTimerService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_latest_by_user_slug(&self, user_slug: &str) -> Result<Option<(IncidentTimer, String)>> {
        let row = sqlx::query(
            r#"
            SELECT it.id, it.user_id, it.reset_timestamp, it.notes, it.created_at, it.updated_at, u.display_name
            FROM incident_timers it
            JOIN users u ON it.user_id = u.id
            WHERE u.slug = $1
            ORDER BY it.reset_timestamp DESC
            LIMIT 1
            "#,
        )
        .bind(user_slug)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let timer = IncidentTimer {
                id: row.get("id"),
                user_id: row.get("user_id"),
                reset_timestamp: row.get("reset_timestamp"),
                notes: row.get("notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            let display_name: String = row.get("display_name");
            Ok(Some((timer, display_name)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_by_user(&self, user_id: Uuid) -> Result<Vec<IncidentTimer>> {
        let timers = sqlx::query_as!(
            IncidentTimer,
            r#"
            SELECT id, user_id, reset_timestamp, notes, created_at, updated_at
            FROM incident_timers
            WHERE user_id = $1
            ORDER BY reset_timestamp DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(timers)
    }

    pub async fn create(&self, user_id: Uuid, data: CreateIncidentTimer) -> Result<IncidentTimer> {
        let reset_timestamp = data.reset_timestamp.unwrap_or_else(|| {
            sqlx::types::chrono::Utc::now()
        });

        // Validate that reset_timestamp is not in the future
        let now = sqlx::types::chrono::Utc::now();
        if reset_timestamp > now {
            return Err(anyhow::anyhow!("Reset timestamp cannot be in the future"));
        }

        let timer = sqlx::query_as!(
            IncidentTimer,
            r#"
            INSERT INTO incident_timers (user_id, reset_timestamp, notes)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, reset_timestamp, notes, created_at, updated_at
            "#,
            user_id,
            reset_timestamp,
            data.notes
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(timer)
    }

    pub async fn update(&self, id: Uuid, user_id: Uuid, data: UpdateIncidentTimer) -> Result<Option<IncidentTimer>> {
        // Validate that reset_timestamp is not in the future
        if let Some(reset_timestamp) = &data.reset_timestamp {
            let now = sqlx::types::chrono::Utc::now();
            if reset_timestamp > &now {
                return Err(anyhow::anyhow!("Reset timestamp cannot be in the future"));
            }
        }

        let timer = sqlx::query_as!(
            IncidentTimer,
            r#"
            UPDATE incident_timers
            SET 
                reset_timestamp = COALESCE($1, reset_timestamp),
                notes = COALESCE($2, notes),
                updated_at = NOW()
            WHERE id = $3 AND user_id = $4
            RETURNING id, user_id, reset_timestamp, notes, created_at, updated_at
            "#,
            data.reset_timestamp,
            data.notes,
            id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(timer)
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM incident_timers
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

}