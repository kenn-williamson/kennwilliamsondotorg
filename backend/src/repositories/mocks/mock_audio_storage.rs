use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;

use crate::repositories::traits::audio_storage::{AudioStorage, AudioUpload};

// Generate mock for AudioStorage trait
mock! {
    pub AudioStorage {}

    #[async_trait]
    impl AudioStorage for AudioStorage {
        async fn upload_audio(&self, audio_data: Vec<u8>, filename: String) -> Result<AudioUpload>;
        async fn delete_audio(&self, url: &str) -> Result<()>;
    }
}
