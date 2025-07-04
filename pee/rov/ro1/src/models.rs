/// Data models for OpenRouter API
/// 
/// This module contains all the data structures used for API requests and responses.

use serde::{Deserialize, Serialize};

/// Request structure for chat completions
#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    /// The model to use for completion
    pub model: String,
    /// List of messages in the conversation
    pub messages: Vec<Message>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Temperature for randomness (0.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// A message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Role of the message sender (user, assistant, system)
    pub role: String,
    /// Content of the message
    pub content: String,
}

/// Response structure for chat completions
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    /// Unique identifier for the completion
    pub id: String,
    /// Object type (should be "chat.completion")
    pub object: String,
    /// Unix timestamp of creation
    pub created: u64,
    /// Model used for the completion
    pub model: String,
    /// List of completion choices
    pub choices: Vec<Choice>,
    /// Usage statistics
    pub usage: Usage,
}

/// A completion choice
#[derive(Debug, Deserialize)]
pub struct Choice {
    /// Index of the choice
    pub index: u32,
    /// The generated message
    pub message: Message,
    /// Reason for finishing
    pub finish_reason: Option<String>,
}

/// Token usage statistics
#[derive(Debug, Deserialize)]
pub struct Usage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the completion
    pub completion_tokens: u32,
    /// Total number of tokens used
    pub total_tokens: u32,
}

/// Available models response
#[derive(Debug, Deserialize)]
pub struct ModelsResponse {
    /// List of available models
    pub data: Vec<Model>,
}

/// Model information
#[derive(Debug, Deserialize)]
pub struct Model {
    /// Model identifier
    pub id: String,
    /// Object type
    pub object: String,
    /// Model owner/organization
    pub owned_by: String,
    /// Additional model details
    #[serde(flatten)]
    pub details: serde_json::Value,
}

impl ChatCompletionRequest {
    /// Create a new chat completion request
    /// 
    /// # Arguments
    /// * `model` - The model to use
    /// * `messages` - List of conversation messages
    pub fn new(model: String, messages: Vec<Message>) -> Self {
        Self {
            model,
            messages,
            max_tokens: None,
            temperature: None,
            stream: None,
        }
    }

    /// Set maximum tokens for the response
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set temperature for randomness
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Enable or disable streaming
    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
}

impl Message {
    /// Create a new user message
    pub fn user(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content,
        }
    }

    /// Create a new assistant message
    pub fn assistant(content: String) -> Self {
        Self {
            role: "assistant".to_string(),
            content,
        }
    }

    /// Create a new system message
    pub fn system(content: String) -> Self {
        Self {
            role: "system".to_string(),
            content,
        }
    }
}