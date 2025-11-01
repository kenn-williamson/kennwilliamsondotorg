// JWT token creation fixtures for testing

use anyhow::Result;

/// Creates a JWT token for testing with appropriate roles from test context
/// For most tests that don't set up roles in DB, includes email-verified for backward compatibility
/// For admin tests that use create_test_app_with_admin_user, will include admin role
#[allow(dead_code)]
pub async fn create_test_jwt_token(user: &backend::models::db::user::User) -> Result<String> {
    use backend::services::auth::jwt::JwtService;

    let jwt_secret = "test-jwt-secret-for-api-tests".to_string();
    let jwt_service = JwtService::new(jwt_secret);

    // For tests, we default to email-verified role for backward compatibility
    // Admin tests should explicitly create tokens with admin role, or we could fetch from DB
    // For simplicity in tests: include both email-verified and admin roles
    // Real production code fetches roles from DB
    jwt_service.generate_token(user, &["email-verified".to_string(), "admin".to_string()])
}
