/// Integration tests for OpenRouter API client
/// 
/// These tests verify the basic functionality of the client modules
/// Note: API tests require a valid OPENROUTER_API_KEY environment variable

use ro2::prelude::*;

#[test]
fn test_model_list() {
    let models = ModelList::get_available_models();
    assert!(!models.is_empty());
    assert!(models.contains(&"openai/gpt-4o"));
    assert!(models.contains(&"anthropic/claude-3-opus"));
}

#[test]
fn test_message_creation() {
    let user_msg = Message::user("Hello, world!");
    assert_eq!(user_msg.role, "user");
    assert_eq!(user_msg.content, "Hello, world!");
    
    let assistant_msg = Message::assistant("Hello back!");
    assert_eq!(assistant_msg.role, "assistant");
    assert_eq!(assistant_msg.content, "Hello back!");
    
    let system_msg = Message::system("You are a helpful assistant.");
    assert_eq!(system_msg.role, "system");
    assert_eq!(system_msg.content, "You are a helpful assistant.");
}

#[test]
fn test_config_creation_without_env() {
    // This should fail if no environment variable is set
    let result = Config::new();
    // We expect this to fail in test environment without API key
    assert!(result.is_err());
}

#[test]
fn test_config_with_custom() {
    let config = Config::with_custom(
        "test-api-key".to_string(),
        Some("https://example.com".to_string()),
        Some("Test Site".to_string()),
    );
    
    assert_eq!(config.api_key, "test-api-key");
    assert_eq!(config.base_url, "https://openrouter.ai/api/v1");
    assert_eq!(config.site_url, Some("https://example.com".to_string()));
    assert_eq!(config.site_name, Some("Test Site".to_string()));
}

#[tokio::test]
async fn test_client_creation_with_custom_config() {
    let config = Config::with_custom(
        "test-api-key".to_string(),
        None,
        None,
    );
    
    let client = OpenRouterClient::with_config(config);
    assert_eq!(client.get_config().api_key, "test-api-key");
}

// Note: This test requires a valid API key and will make an actual API call
// Uncomment and set OPENROUTER_API_KEY environment variable to test
/*
#[tokio::test]
async fn test_actual_api_call() {
    let client = OpenRouterClient::new().expect("Failed to create client");
    let response = client
        .send_chat_completion("openai/gpt-3.5-turbo", "Say hello!")
        .await;
    
    match response {
        Ok(content) => {
            println!("API Response: {}", content);
            assert!(!content.is_empty());
        }
        Err(e) => {
            println!("API call failed (this is expected without valid API key): {}", e);
        }
    }
}
*/