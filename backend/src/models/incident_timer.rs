use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct IncidentTimer {
    pub id: Uuid,
    pub user_id: Uuid,
    pub reset_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIncidentTimer {
    pub reset_timestamp: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIncidentTimer {
    pub reset_timestamp: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IncidentTimerResponse {
    pub id: Uuid,
    pub reset_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<IncidentTimer> for IncidentTimerResponse {
    fn from(timer: IncidentTimer) -> Self {
        Self {
            id: timer.id,
            reset_timestamp: timer.reset_timestamp,
            notes: timer.notes,
            created_at: timer.created_at,
            updated_at: timer.updated_at,
        }
    }
}