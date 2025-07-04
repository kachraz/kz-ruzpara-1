/// OpenRouter API Client Library
/// 
/// This library provides a modular and easy-to-use interface for calling the OpenRouter API.
/// It supports async operations and is structured with clear separation of concerns.

pub mod config;
pub mod models;
pub mod openrouter_client;

pub use config::Config;
pub use models::{ModelList, Message, ChatCompletionRequest, ChatCompletionResponse};
pub use openrouter_client::OpenRouterClient;

/// Re-export commonly used types for convenience
pub mod prelude {
    pub use crate::{Config, OpenRouterClient, ModelList, Message};
    pub use anyhow::Result;
}