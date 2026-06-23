use anyhow::{Result, anyhow};
use chrono::Utc;

use crate::models::api::CreateSongRequest;
use crate::models::db::Song;
use crate::repositories::traits::CreateSong;
use crate::services::blog::utils::generate_slug;

use super::MusicService;

/// Create a new song
///
/// Business logic:
/// - Auto-generates slug from title if not provided
/// - Handles slug collisions by appending numeric suffix ("-2", "-3", etc.)
/// - Sets published_at timestamp if status is "published"
/// - Validates title is not empty and status is valid
pub async fn create_song(service: &MusicService, request: CreateSongRequest) -> Result<Song> {
    // Validate title
    if request.title.trim().is_empty() {
        return Err(anyhow!("Title cannot be empty"));
    }

    // Validate status
    if request.status != "draft" && request.status != "published" {
        return Err(anyhow!(
            "Status must be 'draft' or 'published', got '{}'",
            request.status
        ));
    }

    // Auto-generate slug from title if not provided
    let base_slug = request
        .slug
        .clone()
        .unwrap_or_else(|| generate_slug(&request.title));

    // Handle slug collisions (append -2, -3, etc.)
    let slug = ensure_unique_slug(service, &base_slug).await?;

    // Set published_at if status is "published"
    let published_at = if request.status == "published" {
        Some(Utc::now())
    } else {
        None
    };

    let create_dto = CreateSong {
        slug,
        title: request.title,
        description: request.description,
        lyrics: request.lyrics,
        credits: request.credits,
        audio_url: request.audio_url,
        artwork_url: request.artwork_url,
        duration_seconds: request.duration_seconds,
        display_order: request.display_order.unwrap_or(0),
        status: request.status,
        published_at,
    };

    service.repository.create_song(create_dto).await
}

/// Ensure slug is unique by appending a numeric suffix if a collision is detected.
async fn ensure_unique_slug(service: &MusicService, base_slug: &str) -> Result<String> {
    let mut slug = base_slug.to_string();
    let mut counter = 2;

    while service.repository.get_song_by_slug(&slug).await?.is_some() {
        slug = format!("{}-{}", base_slug, counter);
        counter += 1;

        if counter > 100 {
            return Err(anyhow!(
                "Unable to generate unique slug after 100 attempts for base '{}'",
                base_slug
            ));
        }
    }

    Ok(slug)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{MockAudioStorage, MockImageStorage, MockSongRepository};
    use crate::repositories::traits::CreateSong;
    use crate::test_utils::SongBuilder;
    use mockall::predicate::*;

    fn service_with(repo: MockSongRepository) -> MusicService {
        MusicService::new(
            Box::new(repo),
            Box::new(MockAudioStorage::new()),
            Box::new(MockImageStorage::new()),
        )
    }

    fn request(title: &str, status: &str) -> CreateSongRequest {
        CreateSongRequest {
            title: title.to_string(),
            slug: None,
            description: None,
            lyrics: None,
            credits: None,
            audio_url: None,
            artwork_url: None,
            duration_seconds: None,
            display_order: None,
            status: status.to_string(),
        }
    }

    #[tokio::test]
    async fn test_create_song_auto_generates_slug() {
        let mut mock_repo = MockSongRepository::new();
        mock_repo
            .expect_get_song_by_slug()
            .with(eq("hello-world"))
            .times(1)
            .returning(|_| Ok(None));
        mock_repo
            .expect_create_song()
            .withf(|song: &CreateSong| song.slug == "hello-world")
            .times(1)
            .returning(|song| Ok(SongBuilder::new().with_slug(&song.slug).draft().build()));

        let service = service_with(mock_repo);
        let result = service.create_song(request("Hello World", "draft")).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().slug, "hello-world");
    }

    #[tokio::test]
    async fn test_create_song_handles_slug_collision() {
        let mut mock_repo = MockSongRepository::new();
        mock_repo
            .expect_get_song_by_slug()
            .with(eq("my-track"))
            .times(1)
            .returning(|_| Ok(Some(SongBuilder::new().with_slug("my-track").build())));
        mock_repo
            .expect_get_song_by_slug()
            .with(eq("my-track-2"))
            .times(1)
            .returning(|_| Ok(None));
        mock_repo
            .expect_create_song()
            .withf(|song: &CreateSong| song.slug == "my-track-2")
            .times(1)
            .returning(|song| Ok(SongBuilder::new().with_slug(&song.slug).build()));

        let mut req = request("My Track", "draft");
        req.slug = Some("my-track".to_string());

        let service = service_with(mock_repo);
        let result = service.create_song(req).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().slug, "my-track-2");
    }

    #[tokio::test]
    async fn test_create_song_sets_published_at() {
        let mut mock_repo = MockSongRepository::new();
        mock_repo.expect_get_song_by_slug().returning(|_| Ok(None));
        mock_repo
            .expect_create_song()
            .withf(|song: &CreateSong| song.published_at.is_some() && song.status == "published")
            .times(1)
            .returning(|song| {
                let mut b = SongBuilder::new().with_status("published");
                if let Some(p) = song.published_at {
                    b = b.published_at(p);
                }
                Ok(b.build())
            });

        let service = service_with(mock_repo);
        let result = service.create_song(request("Published Song", "published")).await;

        assert!(result.is_ok());
        assert!(result.unwrap().published_at.is_some());
    }

    #[tokio::test]
    async fn test_create_song_validates_empty_title() {
        let service = service_with(MockSongRepository::new());
        let result = service.create_song(request("   ", "draft")).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Title cannot be empty")
        );
    }

    #[tokio::test]
    async fn test_create_song_validates_status() {
        let service = service_with(MockSongRepository::new());
        let result = service.create_song(request("Song", "invalid")).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Status must be 'draft' or 'published'")
        );
    }
}
