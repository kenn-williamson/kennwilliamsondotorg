use anyhow::{Result, anyhow};
use chrono::Utc;
use uuid::Uuid;

use crate::models::api::UpdateSongRequest;
use crate::models::db::Song;
use crate::repositories::traits::UpdateSong;

use super::MusicService;

/// Update an existing song
///
/// Business logic:
/// - Preserves published_at for already-published songs
/// - Sets published_at when transitioning draft → published
/// - Validates status values and prevents unpublishing
pub async fn update_song(
    service: &MusicService,
    id: Uuid,
    request: UpdateSongRequest,
) -> Result<Song> {
    let existing = service
        .repository
        .get_song_by_id(id)
        .await?
        .ok_or_else(|| anyhow!("Song not found with ID: {}", id))?;

    // Validate status if provided
    if let Some(ref status) = request.status
        && status != "draft"
        && status != "published"
    {
        return Err(anyhow!(
            "Status must be 'draft' or 'published', got '{}'",
            status
        ));
    }

    // Prevent unpublishing - once published, stays published
    if existing.status == "published" && request.status.as_deref() == Some("draft") {
        return Err(anyhow!(
            "Cannot unpublish a published song. Edit it in place or delete it."
        ));
    }

    // Determine published_at:
    // - already published -> preserve existing
    // - draft -> published -> set now
    // - draft staying draft -> keep None
    let published_at = match (&existing.status[..], request.status.as_deref()) {
        ("published", None) | ("published", Some("published")) => existing.published_at,
        ("draft", Some("published")) => Some(Utc::now()),
        ("draft", None) | ("draft", Some("draft")) => None,
        _ => existing.published_at,
    };

    let update_dto = UpdateSong {
        slug: request.slug,
        title: request.title,
        description: request.description,
        lyrics: request.lyrics,
        credits: request.credits,
        audio_url: request.audio_url,
        artwork_url: request.artwork_url,
        duration_seconds: request.duration_seconds,
        display_order: request.display_order,
        status: request.status,
        published_at,
    };

    service.repository.update_song(id, update_dto).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{MockAudioStorage, MockImageStorage, MockSongRepository};
    use crate::repositories::traits::UpdateSong;
    use crate::test_utils::SongBuilder;
    use chrono::Duration;
    use mockall::predicate::*;

    fn service_with(repo: MockSongRepository) -> MusicService {
        MusicService::new(
            Box::new(repo),
            Box::new(MockAudioStorage::new()),
            Box::new(MockImageStorage::new()),
        )
    }

    fn empty_update() -> UpdateSongRequest {
        UpdateSongRequest {
            title: None,
            slug: None,
            description: None,
            lyrics: None,
            credits: None,
            audio_url: None,
            artwork_url: None,
            duration_seconds: None,
            display_order: None,
            status: None,
        }
    }

    #[tokio::test]
    async fn test_update_song_preserves_published_at() {
        let mut mock_repo = MockSongRepository::new();
        let test_id = Uuid::new_v4();
        let original = Utc::now() - Duration::days(7);

        mock_repo
            .expect_get_song_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| Ok(Some(SongBuilder::new().with_id(test_id).published_at(original).build())));
        mock_repo
            .expect_update_song()
            .withf(move |_, u: &UpdateSong| u.published_at == Some(original))
            .times(1)
            .returning(move |_, _| Ok(SongBuilder::new().with_id(test_id).published_at(original).build()));

        let service = service_with(mock_repo);
        let mut req = empty_update();
        req.title = Some("New Title".to_string());
        let result = service.update_song(test_id, req).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().published_at, Some(original));
    }

    #[tokio::test]
    async fn test_update_song_sets_published_at_when_publishing() {
        let mut mock_repo = MockSongRepository::new();
        let test_id = Uuid::new_v4();

        mock_repo
            .expect_get_song_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| Ok(Some(SongBuilder::new().with_id(test_id).draft().build())));
        mock_repo
            .expect_update_song()
            .withf(move |_, u: &UpdateSong| {
                u.published_at.is_some() && u.status == Some("published".to_string())
            })
            .times(1)
            .returning(move |_, _| Ok(SongBuilder::new().with_id(test_id).published().build()));

        let service = service_with(mock_repo);
        let mut req = empty_update();
        req.status = Some("published".to_string());
        let result = service.update_song(test_id, req).await;

        assert!(result.is_ok());
        assert!(result.unwrap().published_at.is_some());
    }

    #[tokio::test]
    async fn test_update_song_not_found() {
        let mut mock_repo = MockSongRepository::new();
        let test_id = Uuid::new_v4();
        mock_repo
            .expect_get_song_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = service_with(mock_repo);
        let result = service.update_song(test_id, empty_update()).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_update_song_prevents_unpublishing() {
        let mut mock_repo = MockSongRepository::new();
        let test_id = Uuid::new_v4();
        mock_repo
            .expect_get_song_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| Ok(Some(SongBuilder::new().with_id(test_id).published().build())));

        let service = service_with(mock_repo);
        let mut req = empty_update();
        req.status = Some("draft".to_string());
        let result = service.update_song(test_id, req).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot unpublish"));
    }
}
