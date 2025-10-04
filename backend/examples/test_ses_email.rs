/// Test script for sending verification email via AWS SES
///
/// Usage: cargo run --example test_ses_email
///
/// Prerequisites:
/// - AWS credentials configured (~/.aws/credentials)
/// - SES_FROM_EMAIL in .env.development
/// - Domain verified in SES
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env.development from parent directory
    dotenv::from_filename("../.env.development").ok();
    env_logger::init();

    let from_email = env::var("SES_FROM_EMAIL")
        .expect("SES_FROM_EMAIL must be set in .env.development");
    let reply_to_email = env::var("SES_REPLY_TO_EMAIL").ok();
    let frontend_url = env::var("FRONTEND_URL")
        .expect("FRONTEND_URL must be set in .env.development");

    println!("🚀 Testing AWS SES Email Service");
    println!("From: {}", from_email);
    println!("Reply-To: {:?}", reply_to_email);
    println!("Frontend URL: {}", frontend_url);
    println!();

    // Create SES service
    let ses_service = backend::services::email::ses_email_service::SesEmailService::new(
        from_email,
        reply_to_email,
    );

    // Test email details
    let test_email = "kenngineering@gmail.com"; // Your verified test email
    let test_token = "test-token-abc123xyz789";

    println!("📧 Sending test verification email to: {}", test_email);
    println!("Token: {}", test_token);
    println!();

    // Send test email
    use backend::services::email::EmailService;
    ses_service
        .send_verification_email(test_email, Some("Test User"), test_token, &frontend_url)
        .await?;

    println!("✅ Email sent successfully!");
    println!();
    println!("Check your inbox at: {}", test_email);
    println!("Expected verification URL: {}/verify-email?token={}", frontend_url, test_token);

    Ok(())
}
