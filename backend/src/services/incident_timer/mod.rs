use crate::repositories::traits::IncidentTimerRepository;
use std::sync::Arc;

// Module declarations
pub mod create;
pub mod delete;
pub mod read;
pub mod update;

#[derive(Clone)]
pub struct IncidentTimerService {
    repository: Arc<dyn IncidentTimerRepository>,
}

impl IncidentTimerService {
    pub fn new(repository: Box<dyn IncidentTimerRepository>) -> Self {
        Self {
            repository: Arc::from(repository),
        }
    }

    // Delegate to modules - these methods are implemented in the respective module files
    // The actual implementations are in create.rs, read.rs, update.rs, delete.rs
}
