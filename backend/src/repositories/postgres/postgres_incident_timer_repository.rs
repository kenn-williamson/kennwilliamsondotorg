use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

use crate::models::db::incident_timer::IncidentTimer;
use crate::repositories::traits::incident_timer_repository::{IncidentTimerRepository, CreateTimerData, TimerUpdates};

/// PostgreSQL implementation of IncidentTimerRepository
pub struct PostgresIncidentTimerRepository {
    pool: PgPool,
}

impl PostgresIncidentTimerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IncidentTimerRepository for PostgresIncidentTimerRepository {
    async fn create_timer(&self, timer_data: &CreateTimerData) -> Result<IncidentTimer> {
        // Match exact query from IncidentTimerService::create
        let timer = sqlx::query_as!(
            IncidentTimer,
            r#"
            INSERT INTO incident_timers (user_id, reset_timestamp, notes)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, reset_timestamp, notes, created_at, updated_at
            "#,
            timer_data.user_id,
            timer_data.reset_timestamp,
            timer_data.notes
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(timer)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<IncidentTimer>> {
        // Match exact query from IncidentTimerService::get_all_by_user
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

    async fn find_by_id(&self, id: Uuid) -> Result<Option<IncidentTimer>> {
        let timer = sqlx::query_as::<_, IncidentTimer>(
            "SELECT * FROM incident_timers WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(timer)
    }

    async fn find_latest_by_user_slug(&self, slug: &str) -> Result<Option<IncidentTimer>> {
        // Match exact query from IncidentTimerService::get_latest_by_user_slug
        let timer = sqlx::query_as!(
            IncidentTimer,
            r#"
            SELECT it.id, it.user_id, it.reset_timestamp, it.notes, it.created_at, it.updated_at
            FROM incident_timers it
            JOIN users u ON it.user_id = u.id
            WHERE u.slug = $1
            ORDER BY it.reset_timestamp DESC
            LIMIT 1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(timer)
    }

    async fn find_latest_by_user_slug_with_display_name(&self, slug: &str) -> Result<Option<(IncidentTimer, String)>> {
        // Query to get both timer and user display name
        let result = sqlx::query!(
            r#"
            SELECT it.id, it.user_id, it.reset_timestamp, it.notes, it.created_at, it.updated_at, u.display_name
            FROM incident_timers it
            JOIN users u ON it.user_id = u.id
            WHERE u.slug = $1
            ORDER BY it.reset_timestamp DESC
            LIMIT 1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let timer = IncidentTimer {
                    id: row.id,
                    user_id: row.user_id,
                    reset_timestamp: row.reset_timestamp,
                    notes: row.notes,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                Ok(Some((timer, row.display_name)))
            }
            None => Ok(None),
        }
    }

    async fn update_timer(&self, id: Uuid, updates: &TimerUpdates) -> Result<IncidentTimer> {
        // Match exact query from IncidentTimerService::update
        let timer = sqlx::query_as!(
            IncidentTimer,
            r#"
            UPDATE incident_timers
            SET 
                reset_timestamp = COALESCE($1, reset_timestamp),
                notes = COALESCE($2, notes),
                updated_at = NOW()
            WHERE id = $3
            RETURNING id, user_id, reset_timestamp, notes, created_at, updated_at
            "#,
            updates.reset_timestamp,
            updates.notes,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match timer {
            Some(timer) => Ok(timer),
            None => Err(anyhow::anyhow!("Timer not found")),
        }
    }

    async fn delete_timer(&self, id: Uuid) -> Result<()> {
        // Match exact query from IncidentTimerService::delete
        sqlx::query!(
            r#"
            DELETE FROM incident_timers
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn timer_belongs_to_user(&self, timer_id: Uuid, user_id: Uuid) -> Result<bool> {
        // Check if timer exists and belongs to user - should be SELECT, not DELETE
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM incident_timers 
                WHERE id = $1 AND user_id = $2
            )
            "#,
            timer_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}
