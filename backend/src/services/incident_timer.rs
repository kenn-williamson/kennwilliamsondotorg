use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::db::IncidentTimer;
use crate::models::api::{CreateIncidentTimer, UpdateIncidentTimer};
use crate::repositories::traits::IncidentTimerRepository;

#[derive(Clone)]
pub struct IncidentTimerService {
    repository: Arc<dyn IncidentTimerRepository>,
}

impl IncidentTimerService {
    pub fn new(repository: Box<dyn IncidentTimerRepository>) -> Self {
        Self { 
            repository: Arc::from(repository)
        }
    }

    pub async fn get_latest_by_user_slug(&self, user_slug: &str) -> Result<Option<(IncidentTimer, String)>> {
        // Use the new repository method that returns both timer and display name
        self.repository.find_latest_by_user_slug_with_display_name(user_slug).await
    }

    pub async fn get_all_by_user(&self, user_id: Uuid) -> Result<Vec<IncidentTimer>> {
        self.repository.find_by_user_id(user_id).await
    }

    pub async fn create(&self, user_id: Uuid, data: CreateIncidentTimer) -> Result<IncidentTimer> {
        let reset_timestamp = data.reset_timestamp.unwrap_or_else(|| {
            chrono::Utc::now()
        });

        // Validate that reset_timestamp is not in the future
        let now = chrono::Utc::now();
        if reset_timestamp > now {
            return Err(anyhow::anyhow!("Reset timestamp cannot be in the future"));
        }

        let timer_data = crate::repositories::traits::incident_timer_repository::CreateTimerData {
            user_id,
            reset_timestamp,
            notes: data.notes,
        };

        self.repository.create_timer(&timer_data).await
    }

    pub async fn update(&self, id: Uuid, user_id: Uuid, data: UpdateIncidentTimer) -> Result<Option<IncidentTimer>> {
        // Validate that reset_timestamp is not in the future
        if let Some(reset_timestamp) = &data.reset_timestamp {
            let now = chrono::Utc::now();
            if reset_timestamp > &now {
                return Err(anyhow::anyhow!("Reset timestamp cannot be in the future"));
            }
        }

        // Check if timer belongs to user
        let belongs_to_user = self.repository.timer_belongs_to_user(id, user_id).await?;
        if !belongs_to_user {
            return Ok(None);
        }

        let updates = crate::repositories::traits::incident_timer_repository::TimerUpdates {
            reset_timestamp: data.reset_timestamp,
            notes: data.notes,
        };

        let timer = self.repository.update_timer(id, &updates).await?;
        Ok(Some(timer))
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<bool> {
        // Check if timer belongs to user
        let belongs_to_user = self.repository.timer_belongs_to_user(id, user_id).await?;
        if !belongs_to_user {
            return Ok(false);
        }

        self.repository.delete_timer(id).await?;
        Ok(true)
    }

}