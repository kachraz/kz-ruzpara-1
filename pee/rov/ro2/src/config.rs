/// Configuration module for OpenRouter API client
/// Handles API keys and base URLs

use std::env;
use anyhow::{Result, anyhow};

/// Configuration struct for OpenRouter API
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub site_url: Option<String>,
    pub site_name: Option<String>,
}

impl Config {
    /// Create a new configuration instance
    /// Reads API key from environment variable OPENROUTER_API_KEY
    /// Will first try to load from .env file, then fall back to system environment variables
    pub fn new() -> Result<Self> {
        // Try to load .env file (ignore errors if file doesn't exist)
        let _ = dotenv::dotenv();
        
        let api_key = env::var("OPENROUTER_API_KEY")
            .map_err(|_| anyhow!(
                "OPENROUTER_API_KEY environment variable not set. \
                Please set it in your .env file or as an environment variable."
            ))?;
        
        Ok(Config {
            api_key,
            base_url: "https://openrouter.ai/api/v1".to_string(),
            site_url: env::var("OPENROUTER_SITE_URL").ok(),
            site_name: env::var("OPENROUTER_SITE_NAME").ok(),
        })
    }
    
    /// Create configuration with custom values
    pub fn with_custom(api_key: String, site_url: Option<String>, site_name: Option<String>) -> Self {
        Config {
            api_key,
            base_url: "https://openrouter.ai/api/v1".to_string(),
            site_url,
            site_name,
        }
    }
}