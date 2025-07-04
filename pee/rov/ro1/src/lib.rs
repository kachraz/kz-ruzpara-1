/// OpenRouter API Client Library
/// 
/// This library provides a modular interface for interacting with the OpenRouter API.
/// It includes modules for configuration, API client, and data models.

pub mod config;
pub mod client;
pub mod models;
pub mod error;

pub use client::OpenRouterClient;
pub use config::Config;
pub use models::*;
pub use error::OpenRouterError;