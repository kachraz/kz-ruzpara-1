/// Integration tests for OpenRouter API client
/// 
/// These tests verify the functionality of the OpenRouter client.
/// Note: These tests require a valid API key to run against the real API.

use openrouter_client::{Config, OpenRouterClient, Message, ChatCompletionRequest};

/// Test configuration creation
#[test]
fn test_config_creation() {
    let api_key = "test-api-key".to_string();
    let config = Config::new(api_key.clone());
    
    assert_eq!(config.api_key, api_key);
    assert_eq!(config.base_url, "https://openrouter.ai/api/v1");
    assert_eq!(config.timeout_seconds, 30);
}

/// Test configuration with custom settings
#[test]
fn test_config_with_custom_settings() {
    let config = Config::new("test-key".to_string())
        .with_base_url("https://custom.api.com".to_string())
        .with_timeout(60);
    
    assert_eq!(config.base_url, "https://custom.api.com");
    assert_eq!(config.timeout_seconds, 60);
}

/// Test message creation
#[test]
fn test_message_creation() {
    let user_msg = Message::user("Hello".to_string());
    assert_eq!(user_msg.role, "user");
    assert_eq!(user_msg.content, "Hello");
    
    let assistant_msg = Message::assistant("Hi there!".to_string());
    assert_eq!(assistant_msg.role, "assistant");
    assert_eq!(assistant_msg.content, "Hi there!");
    
    let system_msg = Message::system("You are helpful".to_string());
    assert_eq!(system_msg.role, "system");
    assert_eq!(system_msg.content, "You are helpful");
}

/// Test chat completion request creation
#[test]
fn test_chat_completion_request() {
    let messages = vec![Message::user("Test message".to_string())];
    let request = ChatCompletionRequest::new("test-model".to_string(), messages.clone())
        .with_max_tokens(100)
        .with_temperature(0.7)
        .with_stream(false);
    
    assert_eq!(request.model, "test-model");
    assert_eq!(request.messages.len(), 1);
    assert_eq!(request.max_tokens, Some(100));
    assert_eq!(request.temperature, Some(0.7));
    assert_eq!(request.stream, Some(false));
}

/// Test client creation
#[test]
fn test_client_creation() {
    let config = Config::new("test-api-key".to_string());
    let client = OpenRouterClient::new(config);
    
    assert!(client.is_ok());
}

// Note: The following tests would require a real API key and network access
// They are commented out to avoid failures in CI/CD environments

/*
#[tokio::test]
async fn test_list_models_integration() {
    let config = Config::from_env().expect("OPENROUTER_API_KEY must be set");
    let client = OpenRouterClient::new(config).expect("Failed to create client");
    
    let result = client.list_models().await;
    assert!(result.is_ok());
    
    let models = result.unwrap();
    assert!(!models.data.is_empty());
}

#[tokio::test]
async fn test_send_message_integration() {
    let config = Config::from_env().expect("OPENROUTER_API_KEY must be set");
    let client = OpenRouterClient::new(config).expect("Failed to create client");
    
    let result = client.send_message("openai/gpt-3.5-turbo", "Hello, world!").await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(!response.is_empty());
}
*/