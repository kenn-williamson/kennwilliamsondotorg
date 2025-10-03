use serde_json::json;

// Use consolidated test helpers from test_helpers module

#[actix_web::test]
async fn test_get_random_phrase_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper authentication
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";
    
    let register_request_body = json!({
        "email": email,
        "password": password,
        "display_name": display_name
    });
    
    let mut register_resp = srv.post("/backend/public/auth/register")
        .send_json(&register_request_body)
        .await
        .unwrap();
    
    assert!(register_resp.status().is_success());
    
    let register_body: serde_json::Value = register_resp.json().await.unwrap();
    let token = register_body.get("token").unwrap().as_str().unwrap();
    
    // Now test getting a random phrase
    let mut resp = srv.get("/backend/protected/phrases/random")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Get random phrase response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Get random phrase error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.is_string());
    assert!(!body.as_str().unwrap().is_empty());
}

#[actix_web::test]
async fn test_get_random_phrase_unauthorized() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let mut resp = srv.get("/backend/protected/phrases/random")
        .send()
        .await
        .unwrap();
    
    println!("Unauthorized response status: {}", resp.status());
    if resp.status() != 401 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Unexpected response: {:?}", body);
    }
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_get_user_phrases_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper authentication
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";
    
    let register_request_body = json!({
        "email": email,
        "password": password,
        "display_name": display_name
    });
    
    let mut register_resp = srv.post("/backend/public/auth/register")
        .send_json(&register_request_body)
        .await
        .unwrap();
    
    assert!(register_resp.status().is_success());
    
    let register_body: serde_json::Value = register_resp.json().await.unwrap();
    let token = register_body.get("token").unwrap().as_str().unwrap();
    
    // Now test getting user phrases
    let mut resp = srv.get("/backend/protected/phrases/user")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Get user phrases response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Get user phrases error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("phrases").is_some());
    assert!(body.get("total").is_some());
    
    let phrases = body.get("phrases").unwrap().as_array().unwrap();
    assert!(!phrases.is_empty()); // Should have the 20 initial phrases from migration
    
    // Verify phrase structure
    let first_phrase = &phrases[0];
    assert!(first_phrase.get("id").is_some());
    assert!(first_phrase.get("phrase_text").is_some());
    assert!(first_phrase.get("active").is_some());
    assert!(first_phrase.get("is_excluded").is_some());
}

#[actix_web::test]
async fn test_get_user_phrases_unauthorized() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let mut resp = srv.get("/backend/protected/phrases/user")
        .send()
        .await
        .unwrap();
    
    println!("Unauthorized response status: {}", resp.status());
    if resp.status() != 401 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Unexpected response: {:?}", body);
    }
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_exclude_phrase_success() {
    let (srv, pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper authentication
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";
    
    let register_request_body = json!({
        "email": email,
        "password": password,
        "display_name": display_name
    });
    
    let mut register_resp = srv.post("/backend/public/auth/register")
        .send_json(&register_request_body)
        .await
        .unwrap();
    
    assert!(register_resp.status().is_success());
    
    let register_body: serde_json::Value = register_resp.json().await.unwrap();
    let token = register_body.get("token").unwrap().as_str().unwrap();
    
    // Get a phrase ID from the database
    let phrase_id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM phrases LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    
    // Now test excluding a phrase
    let mut resp = srv.post(&format!("/backend/protected/phrases/exclude/{}", phrase_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Exclude phrase response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Exclude phrase error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("message").unwrap(), "Phrase excluded successfully");
}

#[actix_web::test]
async fn test_exclude_phrase_unauthorized() {
    let (srv, pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Get a phrase ID from the database
    let phrase_id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM phrases LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    
    let mut resp = srv.post(&format!("/backend/protected/phrases/exclude/{}", phrase_id))
        .send()
        .await
        .unwrap();
    
    println!("Unauthorized response status: {}", resp.status());
    if resp.status() != 401 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Unexpected response: {:?}", body);
    }
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
#[allow(unused_mut)]
async fn test_remove_phrase_exclusion_success() {
    let (srv, pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper authentication
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";
    
    let register_request_body = json!({
        "email": email,
        "password": password,
        "display_name": display_name
    });
    
    let mut register_resp = srv.post("/backend/public/auth/register")
        .send_json(&register_request_body)
        .await
        .unwrap();
    
    assert!(register_resp.status().is_success());
    
    let register_body: serde_json::Value = register_resp.json().await.unwrap();
    let token = register_body.get("token").unwrap().as_str().unwrap();
    let _user_id: uuid::Uuid = register_body.get("user").unwrap().get("id").unwrap().as_str().unwrap().parse().unwrap();
    
    // Get a phrase ID from the database
    let phrase_id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM phrases LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    
    // First exclude the phrase
    let mut exclude_resp = srv.post(&format!("/backend/protected/phrases/exclude/{}", phrase_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    assert!(exclude_resp.status().is_success());
    
    // Now test removing the exclusion
    let mut resp = srv.delete(&format!("/backend/protected/phrases/exclude/{}", phrase_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Remove phrase exclusion response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Remove phrase exclusion error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("message").unwrap(), "Phrase exclusion removed successfully");
}

#[actix_web::test]
async fn test_remove_phrase_exclusion_unauthorized() {
    let (srv, pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Get a phrase ID from the database
    let phrase_id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM phrases LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    
    let mut resp = srv.delete(&format!("/backend/protected/phrases/exclude/{}", phrase_id))
        .send()
        .await
        .unwrap();
    
    println!("Unauthorized response status: {}", resp.status());
    if resp.status() != 401 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Unexpected response: {:?}", body);
    }
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_submit_suggestion_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper authentication
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";
    
    let register_request_body = json!({
        "email": email,
        "password": password,
        "display_name": display_name
    });
    
    let mut register_resp = srv.post("/backend/public/auth/register")
        .send_json(&register_request_body)
        .await
        .unwrap();
    
    assert!(register_resp.status().is_success());
    
    let register_body: serde_json::Value = register_resp.json().await.unwrap();
    let token = register_body.get("token").unwrap().as_str().unwrap();
    
    // Now test submitting a suggestion
    let suggestion_request = json!({
        "phrase_text": "This is a test phrase suggestion"
    });
    
    let mut resp = srv.post("/backend/protected/phrases/suggestions")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&suggestion_request)
        .await
        .unwrap();
    
    println!("Submit suggestion response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Submit suggestion error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("id").is_some());
    assert_eq!(body.get("phrase_text").unwrap(), "This is a test phrase suggestion");
    assert_eq!(body.get("status").unwrap(), "pending");
}

#[actix_web::test]
async fn test_submit_suggestion_unauthorized() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let suggestion_request = json!({
        "phrase_text": "This is a test phrase suggestion"
    });
    
    let mut resp = srv.post("/backend/protected/phrases/suggestions")
        .send_json(&suggestion_request)
        .await
        .unwrap();
    
    println!("Unauthorized response status: {}", resp.status());
    if resp.status() != 401 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Unexpected response: {:?}", body);
    }
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
#[allow(unused_mut)]
async fn test_get_public_phrase_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get a slug
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";
    let slug = crate::test_helpers::unique_test_slug();
    
    let register_request_body = json!({
        "email": email,
        "password": password,
        "display_name": display_name,
        "slug": slug
    });
    
    let mut register_resp = srv.post("/backend/public/auth/register")
        .send_json(&register_request_body)
        .await
        .unwrap();
    
    assert!(register_resp.status().is_success());
    
    // Now test getting a public phrase by slug
    let mut resp = srv.get(&format!("/backend/public/{}/phrase", slug))
        .send()
        .await
        .unwrap();
    
    println!("Get public phrase response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Get public phrase error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.is_string());
    assert!(!body.as_str().unwrap().is_empty());
}

#[actix_web::test]
async fn test_get_public_phrase_nonexistent_user() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Test getting a phrase for a non-existent user
    let mut resp = srv.get("/backend/public/nonexistent-user/phrase")
        .send()
        .await
        .unwrap();
    
    println!("Get public phrase nonexistent user response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Unexpected response: {:?}", body);
    }
    assert!(resp.status().is_success()); // Should return a random phrase even for non-existent users
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.is_string());
    assert!(!body.as_str().unwrap().is_empty());
}
