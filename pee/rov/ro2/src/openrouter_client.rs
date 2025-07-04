/// OpenRouter API client module
/// Handles HTTP requests to the OpenRouter API

use anyhow::{Result, anyhow};
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
use crate::config::Config;
use crate::models::{ChatCompletionRequest, ChatCompletionResponse, Message};

/// OpenRouter API client
pub struct OpenRouterClient {
    client: Client,
    config: Config,
}

impl OpenRouterClient {
    /// Create a new OpenRouter client
    pub fn new() -> Result<Self> {
        let config = Config::new()?;
        let client = Client::new();
        
        Ok(OpenRouterClient {
            client,
            config,
        })
    }
    
    /// Create a new OpenRouter client with custom configuration
    pub fn with_config(config: Config) -> Self {
        let client = Client::new();
        
        OpenRouterClient {
            client,
            config,
        }
    }
    
    /// Build headers for API requests
    fn build_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        
        // Authorization header
        let auth_value = format!("Bearer {}", self.config.api_key);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value)
                .map_err(|e| anyhow!("Invalid API key format: {}", e))?
        );
        
        // Content-Type header
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        // Optional site headers for rankings
        if let Some(ref site_url) = self.config.site_url {
            headers.insert(
                "HTTP-Referer",
                HeaderValue::from_str(site_url)
                    .map_err(|e| anyhow!("Invalid site URL: {}", e))?
            );
        }
        
        if let Some(ref site_name) = self.config.site_name {
            headers.insert(
                "X-Title",
                HeaderValue::from_str(site_name)
                    .map_err(|e| anyhow!("Invalid site name: {}", e))?
            );
        }
        
        Ok(headers)
    }
    
    /// Send a chat completion request
    pub async fn send_chat_completion(&self, model: &str, user_message: &str) -> Result<String> {
        let messages = vec![Message::user(user_message)];
        self.send_chat_completion_with_messages(model, messages).await
    }
    
    /// Send a chat completion request with multiple messages
    pub async fn send_chat_completion_with_messages(&self, model: &str, messages: Vec<Message>) -> Result<String> {
        let request = ChatCompletionRequest {
            model: model.to_string(),
            messages,
            temperature: None,
            max_tokens: None,
        };
        
        self.send_chat_completion_request(request).await
    }
    
    /// Send a chat completion request with custom parameters
    pub async fn send_chat_completion_with_params(
        &self,
        model: &str,
        messages: Vec<Message>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<String> {
        let request = ChatCompletionRequest {
            model: model.to_string(),
            messages,
            temperature,
            max_tokens,
        };
        
        self.send_chat_completion_request(request).await
    }
    
    /// Send the actual HTTP request to OpenRouter API
    async fn send_chat_completion_request(&self, request: ChatCompletionRequest) -> Result<String> {
        let headers = self.build_headers()?;
        let url = format!("{}/chat/completions", self.config.base_url);
        
        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send request: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!("API request failed with status {}: {}", status, error_text));
        }
        
        let completion_response: ChatCompletionResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse response: {}", e))?;
        
        // Extract the content from the first choice
        completion_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| anyhow!("No response content received"))
    }
    
    /// Get the current configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }
}