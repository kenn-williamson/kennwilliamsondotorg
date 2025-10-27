use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::repositories::traits::email_suppression_repository::{
    CreateSuppressionData, EmailSuppressionRepository,
};

/// SNS message structure from AWS
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SnsMessage {
    #[serde(rename = "Type")]
    pub message_type: String,
    pub message_id: String,
    pub topic_arn: String,
    pub message: String,
    pub timestamp: String,
    pub signature_version: String,
    pub signature: String,
    #[serde(rename = "SigningCertURL")]
    pub signing_cert_url: String,
    #[serde(rename = "SubscribeURL")]
    pub subscribe_url: Option<String>,
    pub token: Option<String>,
}

/// SES Notification (nested in SNS Message)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SesNotification {
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce: Option<BounceDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaint: Option<ComplaintDetails>,
    pub mail: MailMetadata,
}

/// Bounce details from SES
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BounceDetails {
    pub bounce_type: String, // "Permanent" or "Transient"
    pub bounce_sub_type: Option<String>,
    pub bounced_recipients: Vec<BouncedRecipient>,
    pub timestamp: String,
    pub feedback_id: String, // Required by AWS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_mta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_mta_ip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BouncedRecipient {
    pub email_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic_code: Option<String>,
}

/// Complaint details from SES
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplaintDetails {
    pub complained_recipients: Vec<ComplainedRecipient>,
    pub timestamp: String,
    pub feedback_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaint_feedback_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplainedRecipient {
    pub email_address: String,
}

/// Mail metadata from SES
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailMetadata {
    pub message_id: String,
    pub source: String,
}

/// Bounce type classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BounceType {
    Hard,    // Permanent bounce
    Soft,    // Transient bounce
    Unknown,
}

impl BounceType {
    pub fn from_ses(bounce_type: &str) -> Self {
        match bounce_type {
            "Permanent" => BounceType::Hard,
            "Transient" => BounceType::Soft,
            _ => BounceType::Unknown,
        }
    }
}

/// SNS webhook handler
pub struct SnsHandler {
    suppression_repo: Box<dyn EmailSuppressionRepository>,
}

impl SnsHandler {
    pub fn new(suppression_repo: Box<dyn EmailSuppressionRepository>) -> Self {
        Self { suppression_repo }
    }

    /// Get the message type from SNS message

    /// Handle SNS notification (bounce or complaint)
    pub async fn handle_notification(&self, sns_message: &SnsMessage) -> Result<()> {
        // Parse the nested SES notification from the SNS message
        let ses_notification: SesNotification = serde_json::from_str(&sns_message.message)
            .context("Failed to parse SES notification from SNS message")?;

        match ses_notification.event_type.as_str() {
            "Bounce" => self.handle_bounce(ses_notification).await,
            "Complaint" => self.handle_complaint(ses_notification).await,
            _ => {
                log::warn!(
                    "Unknown SES notification type: {}",
                    ses_notification.event_type
                );
                Ok(())
            }
        }
    }

    /// Handle bounce notification
    async fn handle_bounce(&self, notification: SesNotification) -> Result<()> {
        let bounce = notification
            .bounce
            .ok_or_else(|| anyhow!("Bounce notification missing bounce details"))?;

        let bounce_type = BounceType::from_ses(&bounce.bounce_type);

        for recipient in bounce.bounced_recipients {
            let email = &recipient.email_address;

            match bounce_type {
                BounceType::Hard => {
                    // Hard bounce: Immediate full suppression
                    self.create_or_update_suppression(
                        email,
                        "bounce",
                        Some(format!(
                            "Hard bounce: {} - {}",
                            recipient.status.unwrap_or_default(),
                            recipient.diagnostic_code.unwrap_or_default()
                        )),
                        true, // suppress transactional
                        true, // suppress marketing
                    )
                    .await?;

                    log::warn!("Created hard bounce suppression for {}", email);
                }
                BounceType::Soft => {
                    // Soft bounce: Track count, suppress after threshold
                    self.handle_soft_bounce(email, &recipient).await?;
                }
                BounceType::Unknown => {
                    log::warn!("Unknown bounce type for {}: {}", email, bounce.bounce_type);
                }
            }
        }

        Ok(())
    }

    /// Handle soft bounce with threshold logic
    async fn handle_soft_bounce(&self, email: &str, recipient: &BouncedRecipient) -> Result<()> {
        // Check if suppression already exists
        let existing = self.suppression_repo.find_by_email(email).await?;

        match existing {
            Some(suppression) => {
                // Check if we've hit the threshold BEFORE incrementing (3 soft bounces)
                if suppression.bounce_count + 1 >= 3 {
                    let new_count = suppression.bounce_count + 1;

                    // Delete the soft_bounce tracking entry
                    self.suppression_repo.delete_suppression(email).await?;

                    // Create full bounce suppression with preserved count
                    let data = CreateSuppressionData {
                        email: email.to_string(),
                        suppression_type: "bounce".to_string(),
                        reason: Some(format!("Soft bounce threshold reached (3+ bounces)")),
                        suppress_transactional: true,
                        suppress_marketing: true,
                    };
                    self.suppression_repo.create_suppression(&data).await?;

                    // Set the bounce count to the new count
                    for _ in 0..new_count {
                        self.suppression_repo.increment_bounce_count(email, Utc::now()).await?;
                    }

                    log::warn!(
                        "Converted soft bounce to suppression for {} (threshold reached)",
                        email
                    );
                } else {
                    // Increment bounce count
                    self.suppression_repo
                        .increment_bounce_count(email, Utc::now())
                        .await?;

                    log::info!(
                        "Soft bounce tracked for {} (count: {})",
                        email,
                        suppression.bounce_count + 1
                    );
                }
            }
            None => {
                // First soft bounce: Create tracking entry without suppression
                self.create_or_update_suppression(
                    email,
                    "soft_bounce",
                    Some(format!(
                        "Soft bounce: {}",
                        recipient.diagnostic_code.clone().unwrap_or_default()
                    )),
                    false, // don't suppress yet
                    false, // don't suppress yet
                )
                .await?;

                // Increment count to 1
                self.suppression_repo
                    .increment_bounce_count(email, Utc::now())
                    .await?;

                log::info!("First soft bounce tracked for {}", email);
            }
        }

        Ok(())
    }

    /// Handle complaint notification
    async fn handle_complaint(&self, notification: SesNotification) -> Result<()> {
        let complaint = notification
            .complaint
            .ok_or_else(|| anyhow!("Complaint notification missing complaint details"))?;

        for recipient in complaint.complained_recipients {
            let email = &recipient.email_address;

            // Complaint: Immediate full suppression (AWS SES requirement)
            self.create_or_update_suppression(
                email,
                "complaint",
                Some(format!(
                    "User marked as spam: {}",
                    complaint
                        .complaint_feedback_type
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string())
                )),
                true, // suppress transactional
                true, // suppress marketing
            )
            .await?;

            log::warn!("Created complaint suppression for {}", email);
        }

        Ok(())
    }

    /// Create or update suppression entry
    async fn create_or_update_suppression(
        &self,
        email: &str,
        suppression_type: &str,
        reason: Option<String>,
        suppress_transactional: bool,
        suppress_marketing: bool,
    ) -> Result<()> {
        // Check if suppression already exists
        let existing = self.suppression_repo.find_by_email(email).await?;

        if existing.is_some() {
            // Already suppressed, just increment bounce count if it's a bounce
            if suppression_type == "bounce" || suppression_type == "soft_bounce" {
                self.suppression_repo
                    .increment_bounce_count(email, Utc::now())
                    .await?;
            }
            return Ok(());
        }

        // Create new suppression
        let data = CreateSuppressionData {
            email: email.to_string(),
            suppression_type: suppression_type.to_string(),
            reason,
            suppress_transactional,
            suppress_marketing,
        };

        self.suppression_repo.create_suppression(&data).await?;

        Ok(())
    }

    /// Handle SNS subscription confirmation
    pub async fn handle_subscription_confirmation(&self, sns_message: &SnsMessage) -> Result<()> {
        let subscribe_url = sns_message
            .subscribe_url
            .as_ref()
            .ok_or_else(|| anyhow!("Subscription confirmation missing SubscribeURL"))?;

        log::info!(
            "Auto-confirming SNS subscription for topic: {}",
            sns_message.topic_arn
        );

        // Make HTTP GET request to subscribe URL to confirm subscription
        let client = reqwest::Client::new();
        let response = client
            .get(subscribe_url)
            .send()
            .await
            .context("Failed to confirm SNS subscription")?;

        if response.status().is_success() {
            log::info!("SNS subscription confirmed successfully");
            Ok(())
        } else {
            Err(anyhow!(
                "SNS subscription confirmation failed with status: {}",
                response.status()
            ))
        }
    }
}
