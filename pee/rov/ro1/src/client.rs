/// OpenRouter API client implementation
/// 
/// This module provides the main client for interacting with the OpenRouter API.

use crate::{config::Config, error::OpenRouterError, models::*};
use reqwest::Client;
use std::time::Duration;

/// Main client for OpenRouter API
#[derive(Debug)]
pub struct OpenRouterClient {
    /// HTTP client
    client: Client,
    /// Configuration
    config: Config,
}

impl OpenRouterClient {
    /// Create a new OpenRouter client
    /// 
    /// # Arguments
    /// * `config` - Configuration for the client
    /// 
    /// # Returns
    /// Result containing the client or an error
    pub fn new(config: Config) -> Result<Self, OpenRouterError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(OpenRouterError::RequestError)?;

        Ok(Self { client, config })
    }

    /// Create a chat completion
    /// 
    /// # Arguments
    /// * `request` - The chat completion request
    /// 
    /// # Returns
    /// Result containing the completion response or an error
    pub async fn create_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, OpenRouterError> {
        let url = format!("{}/chat/completions", self.config.base_url);
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// List available models
    /// 
    /// # Returns
    /// Result containing the models response or an error
    pub async fn list_models(&self) -> Result<ModelsResponse, OpenRouterError> {
        let url = format!("{}/models", self.config.base_url);
        
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Handle HTTP response and convert to appropriate type
    /// 
    /// # Arguments
    /// * `response` - The HTTP response
    /// 
    /// # Returns
    /// Result containing the parsed response or an error
    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T, OpenRouterError>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let status = response.status();
        
        if status.is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text).map_err(OpenRouterError::SerializationError)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(OpenRouterError::ApiError {
                status: status.as_u16(),
                message: error_text,
            })
        }
    }
}

/// Helper functions for common operations
impl OpenRouterClient {
    /// Send a simple text message and get a response
    /// 
    /// # Arguments
    /// * `model` - The model to use
    /// * `message` - The user message
    /// 
    /// # Returns
    /// Result containing the assistant's response or an error
    pub async fn send_message(
        &self,
        model: &str,
        message: &str,
    ) -> Result<String, OpenRouterError> {
        let request = ChatCompletionRequest::new(
            model.to_string(),
            vec![Message::user(message.to_string())],
        );

        let response = self.create_chat_completion(request).await?;
        
        response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| OpenRouterError::Other("No response choices available".to_string()))
    }

    /// Send a conversation and get a response
    /// 
    /// # Arguments
    /// * `model` - The model to use
    /// * `messages` - The conversation messages
    /// 
    /// # Returns
    /// Result containing the assistant's response or an error
    pub async fn send_conversation(
        &self,
        model: &str,
        messages: Vec<Message>,
    ) -> Result<String, OpenRouterError> {
        let request = ChatCompletionRequest::new(model.to_string(), messages);

        let response = self.create_chat_completion(request).await?;
        
        response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| OpenRouterError::Other("No response choices available".to_string()))
    }
}