/// Configuration module for OpenRouter API client
/// 
/// This module handles configuration settings including API keys and base URLs.

use std::env;

/// Configuration struct for OpenRouter API client
#[derive(Debug, Clone)]
pub struct Config {
    /// OpenRouter API key
    pub api_key: String,
    /// Base URL for OpenRouter API
    pub base_url: String,
    /// HTTP client timeout in seconds
    pub timeout_seconds: u64,
}

impl Config {
    /// Create a new configuration with default values
    /// 
    /// # Arguments
    /// * `api_key` - The OpenRouter API key
    /// 
    /// # Returns
    /// A new Config instance with default settings
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://openrouter.ai/api/v1".to_string(),
            timeout_seconds: 30,
        }
    }

    /// Create configuration from environment variables
    /// 
    /// Looks for OPENROUTER_API_KEY environment variable
    /// 
    /// # Returns
    /// Result containing Config or error message
    pub fn from_env() -> Result<Self, String> {
        let api_key = env::var("OPENROUTER_API_KEY")
            .map_err(|_| "OPENROUTER_API_KEY environment variable not found".to_string())?;
        
        Ok(Self::new(api_key))
    }

    /// Set custom base URL
    /// 
    /// # Arguments
    /// * `url` - The base URL to use
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    /// Set custom timeout
    /// 
    /// # Arguments
    /// * `seconds` - Timeout in seconds
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
}