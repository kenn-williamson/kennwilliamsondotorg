use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

use crate::models::db::incident_timer::IncidentTimer;

/// Data structure for creating a new incident timer
#[derive(Debug, Clone)]
pub struct CreateTimerData {
    pub user_id: Uuid,
    pub reset_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

/// Data structure for updating an incident timer
#[derive(Debug, Clone)]
pub struct TimerUpdates {
    pub reset_timestamp: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

/// Repository trait for incident timer operations
#[async_trait]
pub trait IncidentTimerRepository: Send + Sync {
    /// Create a new incident timer
    async fn create_timer(&self, timer_data: &CreateTimerData) -> Result<IncidentTimer>;
    
    /// Find all timers for a user
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<IncidentTimer>>;
    
    /// Find timer by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<IncidentTimer>>;
    
    /// Find latest timer for a user by slug
    async fn find_latest_by_user_slug(&self, slug: &str) -> Result<Option<IncidentTimer>>;
    
    /// Find latest timer for a user by slug with display name (for public display)
    async fn find_latest_by_user_slug_with_display_name(&self, slug: &str) -> Result<Option<(IncidentTimer, String)>>;
    
    /// Update an incident timer
    async fn update_timer(&self, id: Uuid, updates: &TimerUpdates) -> Result<IncidentTimer>;
    
    /// Delete an incident timer
    async fn delete_timer(&self, id: Uuid) -> Result<()>;
    
    /// Check if timer belongs to user
    async fn timer_belongs_to_user(&self, timer_id: Uuid, user_id: Uuid) -> Result<bool>;
}
