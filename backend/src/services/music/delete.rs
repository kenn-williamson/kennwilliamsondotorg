use anyhow::{Result, anyhow};
use uuid::Uuid;

use super::MusicService;

/// Delete a song and best-effort clean up its audio + artwork from storage.
///
/// The song is removed from the database first to avoid orphaned records; if
/// storage cleanup fails it is logged but not propagated (orphaned objects can
/// be reaped by a periodic bucket cleanup).
pub async fn delete_song(service: &MusicService, id: Uuid) -> Result<()> {
    let song = service
        .repository
        .get_song_by_id(id)
        .await?
        .ok_or_else(|| anyhow!("Song not found with ID: {}", id))?;

    // Delete from database first
    service.repository.delete_song(id).await?;

    // Clean up audio file (best effort)
    if let Some(audio_url) = song.audio_url
        && let Err(e) = service.audio_storage.delete_audio(&audio_url).await
    {
        log::warn!(
            "Failed to delete audio '{}' for song {}: {}",
            audio_url,
            id,
            e
        );
    }

    // Clean up artwork (best effort)
    if let Some(artwork_url) = song.artwork_url
        && let Err(e) = service.image_storage.delete_image(&artwork_url).await
    {
        log::warn!(
            "Failed to delete artwork '{}' for song {}: {}",
            artwork_url,
            id,
            e
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{MockAudioStorage, MockImageStorage, MockSongRepository};
    use crate::test_utils::SongBuilder;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_delete_song_removes_media() {
        let mut mock_repo = MockSongRepository::new();
        let mut mock_audio = MockAudioStorage::new();
        let mut mock_image = MockImageStorage::new();
        let test_id = Uuid::new_v4();
        let audio_url = "https://bucket.s3.amazonaws.com/music/audio/x.mp3".to_string();
        let artwork_url = "https://bucket.s3.amazonaws.com/blog/featured/y.jpg".to_string();
        let (a, w) = (audio_url.clone(), artwork_url.clone());

        mock_repo
            .expect_get_song_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(
                    SongBuilder::new()
                        .with_id(test_id)
                        .with_audio_url(&a)
                        .with_artwork_url(&w)
                        .build(),
                ))
            });
        mock_repo
            .expect_delete_song()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(()));
        mock_audio
            .expect_delete_audio()
            .with(eq(audio_url))
            .times(1)
            .returning(|_| Ok(()));
        mock_image
            .expect_delete_image()
            .with(eq(artwork_url))
            .times(1)
            .returning(|_| Ok(()));

        let service = MusicService::new(Box::new(mock_repo), Box::new(mock_audio), Box::new(mock_image));
        assert!(service.delete_song(test_id).await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_song_without_media() {
        let mut mock_repo = MockSongRepository::new();
        let test_id = Uuid::new_v4();

        mock_repo
            .expect_get_song_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(
                    SongBuilder::new()
                        .with_id(test_id)
                        .without_audio_url()
                        .build(),
                ))
            });
        mock_repo
            .expect_delete_song()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(()));

        let service = MusicService::new(
            Box::new(mock_repo),
            Box::new(MockAudioStorage::new()),
            Box::new(MockImageStorage::new()),
        );
        assert!(service.delete_song(test_id).await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_song_not_found() {
        let mut mock_repo = MockSongRepository::new();
        let test_id = Uuid::new_v4();
        mock_repo
            .expect_get_song_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = MusicService::new(
            Box::new(mock_repo),
            Box::new(MockAudioStorage::new()),
            Box::new(MockImageStorage::new()),
        );
        let result = service.delete_song(test_id).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_delete_song_succeeds_even_if_storage_fails() {
        let mut mock_repo = MockSongRepository::new();
        let mut mock_audio = MockAudioStorage::new();
        let test_id = Uuid::new_v4();
        let audio_url = "https://bucket.s3.amazonaws.com/music/audio/x.mp3".to_string();
        let a = audio_url.clone();

        mock_repo
            .expect_get_song_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| Ok(Some(SongBuilder::new().with_id(test_id).with_audio_url(&a).build())));
        mock_repo
            .expect_delete_song()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(()));
        mock_audio
            .expect_delete_audio()
            .with(eq(audio_url))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("S3 error")));

        let service = MusicService::new(
            Box::new(mock_repo),
            Box::new(mock_audio),
            Box::new(MockImageStorage::new()),
        );
        // Storage error is logged but not propagated
        assert!(service.delete_song(test_id).await.is_ok());
    }
}
