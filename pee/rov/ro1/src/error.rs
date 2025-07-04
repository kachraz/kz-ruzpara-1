/// Error handling module for OpenRouter API client
/// 
/// This module defines custom error types for better error handling.

use std::fmt;

/// Custom error type for OpenRouter API operations
#[derive(Debug)]
pub enum OpenRouterError {
    /// HTTP request failed
    RequestError(reqwest::Error),
    /// JSON serialization/deserialization failed
    SerializationError(serde_json::Error),
    /// API returned an error response
    ApiError { status: u16, message: String },
    /// Configuration error
    ConfigError(String),
    /// Generic error with message
    Other(String),
}

impl fmt::Display for OpenRouterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenRouterError::RequestError(e) => write!(f, "Request error: {}", e),
            OpenRouterError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            OpenRouterError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            OpenRouterError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            OpenRouterError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for OpenRouterError {}

impl From<reqwest::Error> for OpenRouterError {
    fn from(error: reqwest::Error) -> Self {
        OpenRouterError::RequestError(error)
    }
}

impl From<serde_json::Error> for OpenRouterError {
    fn from(error: serde_json::Error) -> Self {
        OpenRouterError::SerializationError(error)
    }
}