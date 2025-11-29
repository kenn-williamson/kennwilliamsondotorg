use crate::events::EventHandler;
use crate::events::types::{
    AccessRequestApprovedEvent, AccessRequestCreatedEvent, AccessRequestRejectedEvent,
    BlogPostPublishedEvent, PasswordChangedEvent, PhraseSuggestionApprovedEvent,
    PhraseSuggestionCreatedEvent, PhraseSuggestionRejectedEvent, ProfileUpdatedEvent,
    UserRegisteredEvent,
};
use crate::repositories::traits::{
    AdminRepository, UnsubscribeTokenRepository, UserPreferencesRepository, UserRepository,
    VerificationTokenRepository,
};
use crate::services::email::EmailService;
use crate::services::email::templates::{
    AccessRequestApprovedTemplate, AccessRequestNotificationTemplate,
    AccessRequestRejectedTemplate, BlogPostPublishedTemplate, Email, EmailTemplate,
    PasswordChangedEmailTemplate, PhraseSuggestionApprovedTemplate,
    PhraseSuggestionNotificationTemplate, PhraseSuggestionRejectedTemplate,
    ProfileUpdatedEmailTemplate, VerificationEmailTemplate,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// Email notification handler for access request created events
///
/// Sends email notifications to all admin users when a new access request is created.
pub struct AccessRequestEmailNotificationHandler {
    admin_repository: Arc<dyn AdminRepository>,
    email_service: Arc<dyn EmailService>,
    frontend_url: String,
}

impl AccessRequestEmailNotificationHandler {
    /// Create a new AccessRequestEmailNotificationHandler
    ///
    /// # Arguments
    /// * `admin_repository` - Repository for fetching admin email addresses
    /// * `email_service` - Service for sending emails
    /// * `frontend_url` - Base URL for frontend links (e.g., "https://kennwilliamson.org")
    pub fn new(
        admin_repository: Arc<dyn AdminRepository>,
        email_service: Arc<dyn EmailService>,
        frontend_url: impl Into<String>,
    ) -> Self {
        Self {
            admin_repository,
            email_service,
            frontend_url: frontend_url.into(),
        }
    }
}

#[async_trait]
impl EventHandler<AccessRequestCreatedEvent> for AccessRequestEmailNotificationHandler {
    async fn handle(&self, event: &AccessRequestCreatedEvent) -> Result<()> {
        log::info!(
            "Handling AccessRequestCreatedEvent for user '{}' ({})",
            event.user_display_name,
            event.user_email
        );

        // Fetch admin emails
        let admin_emails = self.admin_repository.get_admin_emails().await?;

        if admin_emails.is_empty() {
            log::warn!("No admin emails found - cannot send access request notification");
            return Ok(());
        }

        // Build email template
        let template = AccessRequestNotificationTemplate::new(
            &event.user_display_name,
            Some(event.message.clone()),
            &event.requested_role,
            &self.frontend_url,
        );

        // Render template
        let html_body = template.render_html()?;
        let text_body = template.render_plain_text();
        let subject = template.subject();

        // Build email
        let email = Email::builder()
            .with_recipients(admin_emails.clone())
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()?;

        // Send email
        self.email_service.send_email(email).await?;

        log::info!(
            "Sent access request notification for user '{}' ({}) to {} admin(s)",
            event.user_display_name,
            event.user_email,
            admin_emails.len()
        );

        Ok(())
    }

    fn handler_name(&self) -> &'static str {
        "AccessRequestEmailNotificationHandler"
    }
}

/// Email notification handler for phrase suggestion created events
///
/// Sends email notifications to all admin users when a new phrase suggestion is submitted.
pub struct PhraseSuggestionEmailNotificationHandler {
    admin_repository: Arc<dyn AdminRepository>,
    user_repository: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
    frontend_url: String,
}

impl PhraseSuggestionEmailNotificationHandler {
    /// Create a new PhraseSuggestionEmailNotificationHandler
    ///
    /// # Arguments
    /// * `admin_repository` - Repository for fetching admin email addresses
    /// * `user_repository` - Repository for fetching user details
    /// * `email_service` - Service for sending emails
    /// * `frontend_url` - Base URL for frontend links (e.g., "https://kennwilliamson.org")
    pub fn new(
        admin_repository: Arc<dyn AdminRepository>,
        user_repository: Arc<dyn UserRepository>,
        email_service: Arc<dyn EmailService>,
        frontend_url: impl Into<String>,
    ) -> Self {
        Self {
            admin_repository,
            user_repository,
            email_service,
            frontend_url: frontend_url.into(),
        }
    }
}

#[async_trait]
impl EventHandler<PhraseSuggestionCreatedEvent> for PhraseSuggestionEmailNotificationHandler {
    async fn handle(&self, event: &PhraseSuggestionCreatedEvent) -> Result<()> {
        log::info!(
            "Handling PhraseSuggestionCreatedEvent from user_id {}",
            event.user_id
        );

        // Fetch user details
        let user = self
            .user_repository
            .find_by_id(event.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found for id {}", event.user_id))?;

        // Fetch admin emails
        let admin_emails = self.admin_repository.get_admin_emails().await?;

        if admin_emails.is_empty() {
            log::warn!("No admin emails found - cannot send phrase suggestion notification");
            return Ok(());
        }

        // Build email template
        let template = PhraseSuggestionNotificationTemplate::new(
            &user.display_name,
            &event.phrase_text,
            &self.frontend_url,
        );

        // Render template
        let html_body = template.render_html()?;
        let text_body = template.render_plain_text();
        let subject = template.subject();

        // Build email
        let email = Email::builder()
            .with_recipients(admin_emails.clone())
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()?;

        // Send email
        self.email_service.send_email(email).await?;

        log::info!(
            "Sent phrase suggestion notification from user '{}' ({}) to {} admin(s)",
            user.display_name,
            user.email,
            admin_emails.len()
        );

        Ok(())
    }

    fn handler_name(&self) -> &'static str {
        "PhraseSuggestionEmailNotificationHandler"
    }
}

/// Email notification handler for access request approved events
///
/// Sends email notification to the user when their access request is approved.
pub struct AccessRequestApprovedEmailHandler {
    user_repository: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
    frontend_url: String,
}

impl AccessRequestApprovedEmailHandler {
    /// Create a new AccessRequestApprovedEmailHandler
    ///
    /// # Arguments
    /// * `user_repository` - Repository for fetching user details
    /// * `email_service` - Service for sending emails
    /// * `frontend_url` - Base URL for frontend links
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        email_service: Arc<dyn EmailService>,
        frontend_url: impl Into<String>,
    ) -> Self {
        Self {
            user_repository,
            email_service,
            frontend_url: frontend_url.into(),
        }
    }
}

#[async_trait]
impl EventHandler<AccessRequestApprovedEvent> for AccessRequestApprovedEmailHandler {
    async fn handle(&self, event: &AccessRequestApprovedEvent) -> Result<()> {
        log::info!(
            "Handling AccessRequestApprovedEvent for user_id {}",
            event.user_id
        );

        // Fetch user details
        let user = self
            .user_repository
            .find_by_id(event.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found for id {}", event.user_id))?;

        // Build email template
        let template = AccessRequestApprovedTemplate::new(
            &user.display_name,
            &event.granted_role,
            event.admin_reason.clone(),
            &self.frontend_url,
        );

        // Render email content
        let html_body = template.render_html()?;
        let text_body = template.render_plain_text();
        let subject = template.subject();

        // Build email
        let email = Email::builder()
            .to(&user.email)
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()?;

        // Send email
        self.email_service.send_email(email).await?;

        log::info!(
            "Sent access request approved notification to user '{}' ({})",
            user.display_name,
            user.email
        );

        Ok(())
    }

    fn handler_name(&self) -> &'static str {
        "AccessRequestApprovedEmailHandler"
    }
}

/// Email notification handler for access request rejected events
///
/// Sends email notification to the user when their access request is rejected.
pub struct AccessRequestRejectedEmailHandler {
    user_repository: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
    frontend_url: String,
}

impl AccessRequestRejectedEmailHandler {
    /// Create a new AccessRequestRejectedEmailHandler
    ///
    /// # Arguments
    /// * `user_repository` - Repository for fetching user details
    /// * `email_service` - Service for sending emails
    /// * `frontend_url` - Base URL for frontend links
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        email_service: Arc<dyn EmailService>,
        frontend_url: impl Into<String>,
    ) -> Self {
        Self {
            user_repository,
            email_service,
            frontend_url: frontend_url.into(),
        }
    }
}

#[async_trait]
impl EventHandler<AccessRequestRejectedEvent> for AccessRequestRejectedEmailHandler {
    async fn handle(&self, event: &AccessRequestRejectedEvent) -> Result<()> {
        log::info!(
            "Handling AccessRequestRejectedEvent for user_id {}",
            event.user_id
        );

        // Fetch user details
        let user = self
            .user_repository
            .find_by_id(event.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found for id {}", event.user_id))?;

        // Build email template
        let template = AccessRequestRejectedTemplate::new(
            &user.display_name,
            event.admin_reason.clone(),
            &self.frontend_url,
        );

        // Render email content
        let html_body = template.render_html()?;
        let text_body = template.render_plain_text();
        let subject = template.subject();

        // Build email
        let email = Email::builder()
            .to(&user.email)
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()?;

        // Send email
        self.email_service.send_email(email).await?;

        log::info!(
            "Sent access request rejected notification to user '{}' ({})",
            user.display_name,
            user.email
        );

        Ok(())
    }

    fn handler_name(&self) -> &'static str {
        "AccessRequestRejectedEmailHandler"
    }
}

/// Email notification handler for phrase suggestion approved events
///
/// Sends email notification to the user when their phrase suggestion is approved.
pub struct PhraseSuggestionApprovedEmailHandler {
    user_repository: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
    frontend_url: String,
}

impl PhraseSuggestionApprovedEmailHandler {
    /// Create a new PhraseSuggestionApprovedEmailHandler
    ///
    /// # Arguments
    /// * `user_repository` - Repository for fetching user details
    /// * `email_service` - Service for sending emails
    /// * `frontend_url` - Base URL for frontend links
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        email_service: Arc<dyn EmailService>,
        frontend_url: impl Into<String>,
    ) -> Self {
        Self {
            user_repository,
            email_service,
            frontend_url: frontend_url.into(),
        }
    }
}

#[async_trait]
impl EventHandler<PhraseSuggestionApprovedEvent> for PhraseSuggestionApprovedEmailHandler {
    async fn handle(&self, event: &PhraseSuggestionApprovedEvent) -> Result<()> {
        log::info!(
            "Handling PhraseSuggestionApprovedEvent for user_id {}",
            event.user_id
        );

        // Fetch user details
        let user = self
            .user_repository
            .find_by_id(event.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found for id {}", event.user_id))?;

        // Build email template
        let template = PhraseSuggestionApprovedTemplate::new(
            &user.display_name,
            &event.phrase_text,
            event.admin_reason.clone(),
            &self.frontend_url,
        );

        // Render email content
        let html_body = template.render_html()?;
        let text_body = template.render_plain_text();
        let subject = template.subject();

        // Build email
        let email = Email::builder()
            .to(&user.email)
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()?;

        // Send email
        self.email_service.send_email(email).await?;

        log::info!(
            "Sent phrase suggestion approved notification to user '{}' ({})",
            user.display_name,
            user.email
        );

        Ok(())
    }

    fn handler_name(&self) -> &'static str {
        "PhraseSuggestionApprovedEmailHandler"
    }
}

/// Email notification handler for phrase suggestion rejected events
///
/// Sends email notification to the user when their phrase suggestion is rejected.
pub struct PhraseSuggestionRejectedEmailHandler {
    user_repository: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
    frontend_url: String,
}

impl PhraseSuggestionRejectedEmailHandler {
    /// Create a new PhraseSuggestionRejectedEmailHandler
    ///
    /// # Arguments
    /// * `user_repository` - Repository for fetching user details
    /// * `email_service` - Service for sending emails
    /// * `frontend_url` - Base URL for frontend links
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        email_service: Arc<dyn EmailService>,
        frontend_url: impl Into<String>,
    ) -> Self {
        Self {
            user_repository,
            email_service,
            frontend_url: frontend_url.into(),
        }
    }
}

#[async_trait]
impl EventHandler<PhraseSuggestionRejectedEvent> for PhraseSuggestionRejectedEmailHandler {
    async fn handle(&self, event: &PhraseSuggestionRejectedEvent) -> Result<()> {
        log::info!(
            "Handling PhraseSuggestionRejectedEvent for user_id {}",
            event.user_id
        );

        // Fetch user details
        let user = self
            .user_repository
            .find_by_id(event.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found for id {}", event.user_id))?;

        // Build email template
        let template = PhraseSuggestionRejectedTemplate::new(
            &user.display_name,
            &event.phrase_text,
            event.admin_reason.clone(),
            &self.frontend_url,
        );

        // Render email content
        let html_body = template.render_html()?;
        let text_body = template.render_plain_text();
        let subject = template.subject();

        // Build email
        let email = Email::builder()
            .to(&user.email)
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()?;

        // Send email
        self.email_service.send_email(email).await?;

        log::info!(
            "Sent phrase suggestion rejected notification to user '{}' ({})",
            user.display_name,
            user.email
        );

        Ok(())
    }

    fn handler_name(&self) -> &'static str {
        "PhraseSuggestionRejectedEmailHandler"
    }
}

/// Email notification handler for password changed events
///
/// Sends security alert email to the user when their password is changed.
pub struct PasswordChangedEmailHandler {
    user_repository: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
    frontend_url: String,
}

impl PasswordChangedEmailHandler {
    /// Create a new PasswordChangedEmailHandler
    ///
    /// # Arguments
    /// * `user_repository` - Repository for fetching user details
    /// * `email_service` - Service for sending emails
    /// * `frontend_url` - Base URL for frontend links
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        email_service: Arc<dyn EmailService>,
        frontend_url: impl Into<String>,
    ) -> Self {
        Self {
            user_repository,
            email_service,
            frontend_url: frontend_url.into(),
        }
    }
}

#[async_trait]
impl EventHandler<PasswordChangedEvent> for PasswordChangedEmailHandler {
    async fn handle(&self, event: &PasswordChangedEvent) -> Result<()> {
        log::info!(
            "Handling PasswordChangedEvent for user_id {}",
            event.user_id
        );

        // Fetch user details
        let user = self
            .user_repository
            .find_by_id(event.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found for id {}", event.user_id))?;

        // Format timestamp for email
        let changed_at = event
            .occurred_at
            .format("%B %d, %Y at %I:%M %P UTC")
            .to_string();

        // Build email template
        let template =
            PasswordChangedEmailTemplate::new(&user.display_name, changed_at, &self.frontend_url);

        // Render email content
        let html_body = template.render_html()?;
        let text_body = template.render_plain_text();
        let subject = template.subject();

        // Build email
        let email = Email::builder()
            .to(&user.email)
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()?;

        // Send email
        self.email_service.send_email(email).await?;

        log::info!(
            "Sent password changed notification to user '{}' ({})",
            user.display_name,
            user.email
        );

        Ok(())
    }

    fn handler_name(&self) -> &'static str {
        "PasswordChangedEmailHandler"
    }
}

/// Email notification handler for profile updated events
///
/// Sends security notification email to the user when their profile is updated.
pub struct ProfileUpdatedEmailHandler {
    user_repository: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
    frontend_url: String,
}

impl ProfileUpdatedEmailHandler {
    /// Create a new ProfileUpdatedEmailHandler
    ///
    /// # Arguments
    /// * `user_repository` - Repository for fetching user details
    /// * `email_service` - Service for sending emails
    /// * `frontend_url` - Base URL for frontend links
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        email_service: Arc<dyn EmailService>,
        frontend_url: impl Into<String>,
    ) -> Self {
        Self {
            user_repository,
            email_service,
            frontend_url: frontend_url.into(),
        }
    }
}

#[async_trait]
impl EventHandler<ProfileUpdatedEvent> for ProfileUpdatedEmailHandler {
    async fn handle(&self, event: &ProfileUpdatedEvent) -> Result<()> {
        log::info!("Handling ProfileUpdatedEvent for user_id {}", event.user_id);

        // Fetch user details
        let user = self
            .user_repository
            .find_by_id(event.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found for id {}", event.user_id))?;

        // Format timestamp for email
        let updated_at = event
            .occurred_at
            .format("%B %d, %Y at %I:%M %P UTC")
            .to_string();

        // Determine if values actually changed (only show arrows if changed)
        let old_name = if event.old_display_name != event.new_display_name {
            Some(event.old_display_name.clone())
        } else {
            None
        };
        let old_slug_val = if event.old_slug != event.new_slug {
            Some(event.old_slug.clone())
        } else {
            None
        };

        // Build email template
        let template = ProfileUpdatedEmailTemplate::new(
            &user.display_name,
            old_name,
            &event.new_display_name,
            old_slug_val,
            &event.new_slug,
            updated_at,
            &self.frontend_url,
        );

        // Render email content
        let html_body = template.render_html()?;
        let text_body = template.render_plain_text();
        let subject = template.subject();

        // Build email
        let email = Email::builder()
            .to(&user.email)
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()?;

        // Send email
        self.email_service.send_email(email).await?;

        log::info!(
            "Sent profile updated notification to user '{}' ({})",
            user.display_name,
            user.email
        );

        Ok(())
    }

    fn handler_name(&self) -> &'static str {
        "ProfileUpdatedEmailHandler"
    }
}

/// Email notification handler for user registered events
///
/// Sends verification email to the user when they register.
pub struct UserRegisteredEmailHandler {
    verification_token_repository: Arc<dyn VerificationTokenRepository>,
    email_service: Arc<dyn EmailService>,
    frontend_url: String,
}

impl UserRegisteredEmailHandler {
    /// Create a new UserRegisteredEmailHandler
    ///
    /// # Arguments
    /// * `verification_token_repository` - Repository for storing verification tokens
    /// * `email_service` - Service for sending emails
    /// * `frontend_url` - Base URL for frontend links
    pub fn new(
        verification_token_repository: Arc<dyn VerificationTokenRepository>,
        email_service: Arc<dyn EmailService>,
        frontend_url: impl Into<String>,
    ) -> Self {
        Self {
            verification_token_repository,
            email_service,
            frontend_url: frontend_url.into(),
        }
    }
}

#[async_trait]
impl EventHandler<UserRegisteredEvent> for UserRegisteredEmailHandler {
    async fn handle(&self, event: &UserRegisteredEvent) -> Result<()> {
        log::info!(
            "Handling UserRegisteredEvent for user '{}' ({})",
            event.user_display_name,
            event.user_email
        );

        // Generate secure verification token
        let token = generate_verification_token();
        let token_hash = hash_verification_token(&token);

        // Create token data with 24-hour expiration
        use crate::repositories::traits::verification_token_repository::CreateVerificationTokenData;
        use chrono::{Duration, Utc};

        let expires_at = Utc::now() + Duration::hours(24);
        let token_data = CreateVerificationTokenData {
            user_id: event.user_id,
            token_hash,
            expires_at,
        };

        // Store hashed token in database
        self.verification_token_repository
            .create_token(&token_data)
            .await?;

        // Build email template
        let template =
            VerificationEmailTemplate::new(&event.user_display_name, &token, &self.frontend_url);

        // Render template
        let html_body = template.render_html()?;
        let text_body = template.render_plain_text();
        let subject = template.subject();

        // Build email
        let email = Email::builder()
            .to(&event.user_email)
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()?;

        // Send email
        self.email_service.send_email(email).await?;

        log::info!(
            "Sent verification email to user '{}' ({})",
            event.user_display_name,
            event.user_email
        );

        Ok(())
    }

    fn handler_name(&self) -> &'static str {
        "UserRegisteredEmailHandler"
    }
}

/// Generate a secure random verification token (32 bytes = 256 bits)
fn generate_verification_token() -> String {
    use rand::Rng;
    let mut token_bytes = [0u8; 32];
    rand::rng().fill(&mut token_bytes);
    hex::encode(token_bytes)
}

/// Hash verification token using SHA-256 for storage
fn hash_verification_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// Generate a secure random unsubscribe token (32 bytes = 256 bits)
fn generate_unsubscribe_token() -> String {
    use rand::Rng;
    let mut token_bytes = [0u8; 32];
    rand::rng().fill(&mut token_bytes);
    hex::encode(token_bytes)
}

/// Hash unsubscribe token using SHA-256 for storage
fn hash_unsubscribe_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// Email notification handler for blog post published events
///
/// Sends email notifications to all users who have opted in to blog notifications.
/// Each email includes a unique unsubscribe token for one-click unsubscribe.
pub struct BlogPostPublishedEmailHandler {
    user_repository: Arc<dyn UserRepository>,
    user_preferences_repository: Arc<dyn UserPreferencesRepository>,
    unsubscribe_token_repository: Arc<dyn UnsubscribeTokenRepository>,
    email_service: Arc<dyn EmailService>,
    frontend_url: String,
}

impl BlogPostPublishedEmailHandler {
    /// Create a new BlogPostPublishedEmailHandler
    ///
    /// # Arguments
    /// * `user_repository` - Repository for fetching user details
    /// * `user_preferences_repository` - Repository for finding users with notifications enabled
    /// * `unsubscribe_token_repository` - Repository for creating unsubscribe tokens
    /// * `email_service` - Service for sending emails
    /// * `frontend_url` - Base URL for frontend links
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        user_preferences_repository: Arc<dyn UserPreferencesRepository>,
        unsubscribe_token_repository: Arc<dyn UnsubscribeTokenRepository>,
        email_service: Arc<dyn EmailService>,
        frontend_url: impl Into<String>,
    ) -> Self {
        Self {
            user_repository,
            user_preferences_repository,
            unsubscribe_token_repository,
            email_service,
            frontend_url: frontend_url.into(),
        }
    }
}

#[async_trait]
impl EventHandler<BlogPostPublishedEvent> for BlogPostPublishedEmailHandler {
    async fn handle(&self, event: &BlogPostPublishedEvent) -> Result<()> {
        use crate::models::db::unsubscribe_token::email_types;

        log::info!(
            "Handling BlogPostPublishedEvent for post '{}' ({})",
            event.title,
            event.slug
        );

        // Get all users with blog notifications enabled
        let user_ids = self
            .user_preferences_repository
            .find_users_with_blog_notifications()
            .await?;

        if user_ids.is_empty() {
            log::info!("No users have blog notifications enabled");
            return Ok(());
        }

        log::info!(
            "Sending blog post notification to {} user(s)",
            user_ids.len()
        );

        let mut success_count = 0;
        let mut error_count = 0;

        for user_id in user_ids {
            // Fetch user details
            let user = match self.user_repository.find_by_id(user_id).await? {
                Some(u) => u,
                None => {
                    log::warn!("User {} not found, skipping notification", user_id);
                    continue;
                }
            };

            // Generate unsubscribe token
            let raw_token = generate_unsubscribe_token();
            let token_hash = hash_unsubscribe_token(&raw_token);

            // Store the hashed token
            if let Err(e) = self
                .unsubscribe_token_repository
                .create_or_replace(user_id, email_types::BLOG_NOTIFICATIONS, &token_hash)
                .await
            {
                log::error!(
                    "Failed to create unsubscribe token for user {}: {}",
                    user_id,
                    e
                );
                error_count += 1;
                continue;
            }

            // Build email template
            let template = BlogPostPublishedTemplate::new(
                &user.display_name,
                &event.title,
                &event.slug,
                event.excerpt.clone(),
                event.featured_image_url.clone(),
                &raw_token,
                &self.frontend_url,
            );

            // Render email content
            let html_body = match template.render_html() {
                Ok(html) => html,
                Err(e) => {
                    log::error!("Failed to render HTML for user {}: {}", user_id, e);
                    error_count += 1;
                    continue;
                }
            };
            let text_body = template.render_plain_text();
            let subject = template.subject();

            // Build email
            let email = match Email::builder()
                .to(&user.email)
                .subject(subject)
                .text_body(text_body)
                .html_body(html_body)
                .build()
            {
                Ok(e) => e,
                Err(e) => {
                    log::error!("Failed to build email for user {}: {}", user_id, e);
                    error_count += 1;
                    continue;
                }
            };

            // Send email
            if let Err(e) = self.email_service.send_email(email).await {
                log::error!("Failed to send email to user {}: {}", user_id, e);
                error_count += 1;
                continue;
            }

            success_count += 1;
            log::debug!(
                "Sent blog post notification to user '{}' ({})",
                user.display_name,
                user.email
            );
        }

        log::info!(
            "Blog post notification complete: {} sent, {} failed",
            success_count,
            error_count
        );

        Ok(())
    }

    fn handler_name(&self) -> &'static str {
        "BlogPostPublishedEmailHandler"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventHandler;
    use crate::repositories::mocks::{
        MockAdminRepository, MockUnsubscribeTokenRepository, MockUserPreferencesRepository,
        MockUserRepository,
    };
    use crate::services::email::MockEmailService;
    use crate::test_utils::UserBuilder;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_access_request_handler_sends_emails() {
        // Setup mocks
        let mut mock_admin_repo = MockAdminRepository::new();
        let mock_email_service = MockEmailService::new();

        // Configure expectations
        mock_admin_repo
            .expect_get_admin_emails()
            .times(1)
            .returning(|| Ok(vec!["admin@example.com".to_string()]));

        // Clone for verification
        let email_service_clone = mock_email_service.clone();

        // Create handler
        let handler = AccessRequestEmailNotificationHandler::new(
            Arc::new(mock_admin_repo),
            Arc::new(mock_email_service),
            "https://kennwilliamson.org",
        );

        // Create event
        let event = AccessRequestCreatedEvent::new(
            Uuid::new_v4(),
            "user@example.com",
            "Test User",
            "I need access",
            "trusted-contact",
        );

        // Handle event
        let result = handler.handle(&event).await;
        assert!(result.is_ok());

        // Verify email was sent
        assert_eq!(email_service_clone.count(), 1);
        let sent_emails = email_service_clone.get_sent_emails();
        assert_eq!(sent_emails[0].to, vec!["admin@example.com"]);
        assert!(sent_emails[0].subject.contains("Access Request"));
    }

    #[tokio::test]
    async fn test_access_request_handler_no_admins() {
        // Setup mocks
        let mut mock_admin_repo = MockAdminRepository::new();
        let mock_email_service = MockEmailService::new();

        // Configure expectations - no admins
        mock_admin_repo
            .expect_get_admin_emails()
            .times(1)
            .returning(|| Ok(vec![]));

        let email_service_clone = mock_email_service.clone();

        // Create handler
        let handler = AccessRequestEmailNotificationHandler::new(
            Arc::new(mock_admin_repo),
            Arc::new(mock_email_service),
            "https://kennwilliamson.org",
        );

        // Create event
        let event = AccessRequestCreatedEvent::new(
            Uuid::new_v4(),
            "user@example.com",
            "Test User",
            "I need access",
            "trusted-contact",
        );

        // Handle event
        let result = handler.handle(&event).await;
        assert!(result.is_ok());

        // Verify no email was sent
        assert_eq!(email_service_clone.count(), 0);
    }

    #[tokio::test]
    async fn test_phrase_suggestion_handler_sends_emails() {
        // Setup mocks
        let mut mock_admin_repo = MockAdminRepository::new();
        let mut mock_user_repo = MockUserRepository::new();
        let mock_email_service = MockEmailService::new();

        // Configure expectations
        mock_admin_repo
            .expect_get_admin_emails()
            .times(1)
            .returning(|| Ok(vec!["admin@example.com".to_string()]));

        // Configure user repository to return user details
        mock_user_repo.expect_find_by_id().times(1).returning(|_| {
            Ok(Some(
                crate::test_utils::UserBuilder::new()
                    .with_display_name("Test User")
                    .build(),
            ))
        });

        // Clone for verification
        let email_service_clone = mock_email_service.clone();

        // Create handler
        let handler = PhraseSuggestionEmailNotificationHandler::new(
            Arc::new(mock_admin_repo),
            Arc::new(mock_user_repo),
            Arc::new(mock_email_service),
            "https://kennwilliamson.org",
        );

        // Create event
        let event = PhraseSuggestionCreatedEvent::new(Uuid::new_v4(), "Time is an illusion");

        // Handle event
        let result = handler.handle(&event).await;
        assert!(result.is_ok());

        // Verify email was sent
        assert_eq!(email_service_clone.count(), 1);
        let sent_emails = email_service_clone.get_sent_emails();
        assert_eq!(sent_emails[0].to, vec!["admin@example.com"]);
        assert!(sent_emails[0].subject.contains("Phrase Suggestion"));
    }

    #[tokio::test]
    async fn test_phrase_suggestion_handler_no_admins() {
        // Setup mocks
        let mut mock_admin_repo = MockAdminRepository::new();
        let mut mock_user_repo = MockUserRepository::new();
        let mock_email_service = MockEmailService::new();

        // Configure expectations
        let user_id = Uuid::new_v4();

        mock_user_repo
            .expect_find_by_id()
            .times(1)
            .returning(|_| Ok(Some(UserBuilder::new().build())));

        mock_admin_repo
            .expect_get_admin_emails()
            .times(1)
            .returning(|| Ok(vec![]));

        let email_service_clone = mock_email_service.clone();

        // Create handler
        let handler = PhraseSuggestionEmailNotificationHandler::new(
            Arc::new(mock_admin_repo),
            Arc::new(mock_user_repo),
            Arc::new(mock_email_service),
            "https://kennwilliamson.org",
        );

        // Create event
        let event = PhraseSuggestionCreatedEvent::new(user_id, "Test phrase");

        // Handle event
        let result = handler.handle(&event).await;
        assert!(result.is_ok());

        // Verify no email was sent
        assert_eq!(email_service_clone.count(), 0);
    }

    #[tokio::test]
    async fn test_blog_post_published_handler_sends_emails() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_prefs_repo = MockUserPreferencesRepository::new();
        let mut mock_unsubscribe_repo = MockUnsubscribeTokenRepository::new();
        let mock_email_service = MockEmailService::new();

        let user_id = Uuid::new_v4();

        // Configure user preferences repo to return one user
        mock_prefs_repo
            .expect_find_users_with_blog_notifications()
            .times(1)
            .returning(move || Ok(vec![user_id]));

        // Configure user repository to return user details
        mock_user_repo.expect_find_by_id().times(1).returning(|_| {
            Ok(Some(
                UserBuilder::new()
                    .with_email("subscriber@example.com")
                    .with_display_name("Blog Subscriber")
                    .build(),
            ))
        });

        // Configure unsubscribe token repo
        mock_unsubscribe_repo
            .expect_create_or_replace()
            .times(1)
            .returning(|_, _, _| Ok(true));

        // Clone for verification
        let email_service_clone = mock_email_service.clone();

        // Create handler
        let handler = BlogPostPublishedEmailHandler::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_prefs_repo),
            Arc::new(mock_unsubscribe_repo),
            Arc::new(mock_email_service),
            "https://kennwilliamson.org",
        );

        // Create event
        let event = BlogPostPublishedEvent::new(
            Uuid::new_v4(),
            "my-awesome-post",
            "My Awesome Post",
            Some("This is an exciting new post!".to_string()),
            Some("https://example.com/image.jpg".to_string()),
        );

        // Handle event
        let result = handler.handle(&event).await;
        assert!(result.is_ok());

        // Verify email was sent
        assert_eq!(email_service_clone.count(), 1);
        let sent_emails = email_service_clone.get_sent_emails();
        assert_eq!(sent_emails[0].to, vec!["subscriber@example.com"]);
        assert!(sent_emails[0].subject.contains("My Awesome Post"));
    }

    #[tokio::test]
    async fn test_blog_post_published_handler_no_subscribers() {
        // Setup mocks
        let mock_user_repo = MockUserRepository::new();
        let mut mock_prefs_repo = MockUserPreferencesRepository::new();
        let mock_unsubscribe_repo = MockUnsubscribeTokenRepository::new();
        let mock_email_service = MockEmailService::new();

        // Configure user preferences repo to return no users
        mock_prefs_repo
            .expect_find_users_with_blog_notifications()
            .times(1)
            .returning(|| Ok(vec![]));

        let email_service_clone = mock_email_service.clone();

        // Create handler
        let handler = BlogPostPublishedEmailHandler::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_prefs_repo),
            Arc::new(mock_unsubscribe_repo),
            Arc::new(mock_email_service),
            "https://kennwilliamson.org",
        );

        // Create event
        let event =
            BlogPostPublishedEvent::new(Uuid::new_v4(), "test-post", "Test Post", None, None);

        // Handle event
        let result = handler.handle(&event).await;
        assert!(result.is_ok());

        // Verify no email was sent
        assert_eq!(email_service_clone.count(), 0);
    }

    #[tokio::test]
    async fn test_blog_post_published_handler_multiple_subscribers() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_prefs_repo = MockUserPreferencesRepository::new();
        let mut mock_unsubscribe_repo = MockUnsubscribeTokenRepository::new();
        let mock_email_service = MockEmailService::new();

        let user_id1 = Uuid::new_v4();
        let user_id2 = Uuid::new_v4();

        // Configure user preferences repo to return two users
        mock_prefs_repo
            .expect_find_users_with_blog_notifications()
            .times(1)
            .returning(move || Ok(vec![user_id1, user_id2]));

        // Configure user repository to return user details for each call
        mock_user_repo.expect_find_by_id().times(2).returning(|id| {
            Ok(Some(
                UserBuilder::new()
                    .with_id(id)
                    .with_email(format!("user-{}@example.com", &id.to_string()[..8]))
                    .build(),
            ))
        });

        // Configure unsubscribe token repo for multiple calls
        mock_unsubscribe_repo
            .expect_create_or_replace()
            .times(2)
            .returning(|_, _, _| Ok(true));

        // Clone for verification
        let email_service_clone = mock_email_service.clone();

        // Create handler
        let handler = BlogPostPublishedEmailHandler::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_prefs_repo),
            Arc::new(mock_unsubscribe_repo),
            Arc::new(mock_email_service),
            "https://kennwilliamson.org",
        );

        // Create event
        let event = BlogPostPublishedEvent::new(
            Uuid::new_v4(),
            "multi-subscriber-post",
            "Multi Subscriber Post",
            None,
            None,
        );

        // Handle event
        let result = handler.handle(&event).await;
        assert!(result.is_ok());

        // Verify both emails were sent
        assert_eq!(email_service_clone.count(), 2);
    }
}
