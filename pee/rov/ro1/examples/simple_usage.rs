/// Simple usage example for OpenRouter API client
/// 
/// This example demonstrates basic usage of the OpenRouter client.
/// Run with: cargo run --example simple_usage

use openrouter_client::{Config, OpenRouterClient, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try to get API key from environment, fallback to placeholder
    let config = Config::from_env().unwrap_or_else(|_| {
        println!("Warning: OPENROUTER_API_KEY not found, using placeholder");
        Config::new("your-api-key-here".to_string())
    });

    // Create the client
    let client = OpenRouterClient::new(config)?;

    // Example: List models (this will fail without a real API key)
    println!("Attempting to list models...");
    match client.list_models().await {
        Ok(models) => {
            println!("Found {} models", models.data.len());
            for model in models.data.iter().take(3) {
                println!("  - {} (by {})", model.id, model.owned_by);
            }
        }
        Err(e) => println!("Failed to list models: {}", e),
    }

    // Example: Send a message (this will also fail without a real API key)
    println!("\nAttempting to send a message...");
    match client.send_message("openai/gpt-3.5-turbo", "Hello, world!").await {
        Ok(response) => println!("Response: {}", response),
        Err(e) => println!("Failed to send message: {}", e),
    }

    println!("\nTo use this example with a real API key:");
    println!("1. Get an API key from OpenRouter");
    println!("2. Set the environment variable: export OPENROUTER_API_KEY=your-key");
    println!("3. Run: cargo run --example simple_usage");

    Ok(())
}