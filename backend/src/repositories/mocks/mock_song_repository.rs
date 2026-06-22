use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::Song;
use crate::repositories::traits::song_repository::{
    CreateSong, SongFilters, SongList, SongRepository, UpdateSong,
};

// Generate mock for SongRepository trait
mock! {
    pub SongRepository {}

    #[async_trait]
    impl SongRepository for SongRepository {
        async fn create_song(&self, song: CreateSong) -> Result<Song>;
        async fn get_song_by_id(&self, id: Uuid) -> Result<Option<Song>>;
        async fn get_song_by_slug(&self, slug: &str) -> Result<Option<Song>>;
        async fn list_songs(&self, filters: SongFilters) -> Result<SongList>;
        async fn update_song(&self, id: Uuid, song: UpdateSong) -> Result<Song>;
        async fn delete_song(&self, id: Uuid) -> Result<()>;
        async fn search_songs(&self, query: &str, page: i32, limit: i32) -> Result<SongList>;
    }
}
