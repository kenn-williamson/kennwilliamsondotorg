use anyhow::Result;
use uuid::Uuid;

use crate::models::db::Song;
use crate::repositories::traits::{SongFilters, SongList};

use super::MusicService;

/// Get song by ID
pub async fn get_song_by_id(service: &MusicService, id: Uuid) -> Result<Option<Song>> {
    service.repository.get_song_by_id(id).await
}

/// Get song by slug
pub async fn get_song_by_slug(service: &MusicService, slug: &str) -> Result<Option<Song>> {
    service.repository.get_song_by_slug(slug).await
}

/// List songs with filters and pagination
pub async fn list_songs(service: &MusicService, filters: SongFilters) -> Result<SongList> {
    service.repository.list_songs(filters).await
}

/// Search songs using full-text search
pub async fn search_songs(
    service: &MusicService,
    query: &str,
    page: i32,
    limit: i32,
) -> Result<SongList> {
    service.repository.search_songs(query, page, limit).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{MockAudioStorage, MockImageStorage, MockSongRepository};
    use crate::test_utils::SongBuilder;
    use mockall::predicate::*;

    fn service_with(repo: MockSongRepository) -> MusicService {
        MusicService::new(
            Box::new(repo),
            Box::new(MockAudioStorage::new()),
            Box::new(MockImageStorage::new()),
        )
    }

    #[tokio::test]
    async fn test_get_song_by_id_found() {
        let mut mock_repo = MockSongRepository::new();
        let test_id = Uuid::new_v4();
        mock_repo
            .expect_get_song_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| Ok(Some(SongBuilder::new().with_id(test_id).build())));

        let service = service_with(mock_repo);
        let result = service.get_song_by_id(test_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().id, test_id);
    }

    #[tokio::test]
    async fn test_get_song_by_id_not_found() {
        let mut mock_repo = MockSongRepository::new();
        let test_id = Uuid::new_v4();
        mock_repo
            .expect_get_song_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = service_with(mock_repo);
        let result = service.get_song_by_id(test_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_list_songs() {
        let mut mock_repo = MockSongRepository::new();
        mock_repo.expect_list_songs().times(1).returning(|_| {
            Ok(SongList {
                songs: vec![
                    SongBuilder::new().with_title("Track 1").build(),
                    SongBuilder::new().with_title("Track 2").build(),
                ],
                total: 2,
                page: 1,
                total_pages: 1,
            })
        });

        let service = service_with(mock_repo);
        let filters = SongFilters {
            status: Some("published".to_string()),
            page: 1,
            limit: 10,
        };
        let result = service.list_songs(filters).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().songs.len(), 2);
    }
}
