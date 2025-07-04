/// OpenRouter API Client - Main Application
/// 
/// This is an example application demonstrating how to use the OpenRouter API client.

use openrouter_client::{Config, OpenRouterClient, Message};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with configuration
    let config = match Config::from_env() {
        Ok(config) => config,
        Err(_) => {
            // Fallback to manual configuration if env var not found
            println!("OPENROUTER_API_KEY environment variable not found.");
            println!("Please set it or provide the API key directly.");
            
            // For demo purposes, show how to create config manually
            let api_key = env::args()
                .nth(1)
                .unwrap_or_else(|| "your-api-key-here".to_string());
            
            Config::new(api_key)
                .with_timeout(60) // 60 seconds timeout
        }
    };

    // Create the client
    let client = OpenRouterClient::new(config)?;

    // Example 1: List available models
    println!("Fetching available models...");
    match list_available_models(&client).await {
        Ok(_) => println!("Successfully listed models"),
        Err(e) => println!("Failed to list models: {}", e),
    }

    // Example 2: Send a simple message
    println!("\nSending a simple message...");
    match send_simple_message(&client).await {
        Ok(_) => println!("Successfully sent message"),
        Err(e) => println!("Failed to send message: {}", e),
    }

    // Example 3: Send a conversation
    println!("\nSending a conversation...");
    match send_conversation_example(&client).await {
        Ok(_) => println!("Successfully sent conversation"),
        Err(e) => println!("Failed to send conversation: {}", e),
    }

    Ok(())
}

/// Example function to list available models
async fn list_available_models(client: &OpenRouterClient) -> Result<(), Box<dyn std::error::Error>> {
    let models = client.list_models().await?;
    
    println!("Available models (showing first 5):");
    for (i, model) in models.data.iter().take(5).enumerate() {
        println!("  {}. {} (by {})", i + 1, model.id, model.owned_by);
    }
    
    Ok(())
}

/// Example function to send a simple message
async fn send_simple_message(client: &OpenRouterClient) -> Result<(), Box<dyn std::error::Error>> {
    // Using a popular model - you might need to adjust based on available models
    let model = "openai/gpt-3.5-turbo";
    let message = "Hello! Can you tell me a fun fact about Rust programming language?";
    
    println!("Sending message to {}: {}", model, message);
    
    let response = client.send_message(model, message).await?;
    println!("Response: {}", response);
    
    Ok(())
}

/// Example function to send a conversation
async fn send_conversation_example(client: &OpenRouterClient) -> Result<(), Box<dyn std::error::Error>> {
    let model = "openai/gpt-3.5-turbo";
    
    let messages = vec![
        Message::system("You are a helpful assistant that explains programming concepts clearly.".to_string()),
        Message::user("What is async programming in Rust?".to_string()),
        Message::assistant("Async programming in Rust allows you to write concurrent code using async/await syntax...".to_string()),
        Message::user("Can you give me a simple example?".to_string()),
    ];
    
    println!("Sending conversation to {}...", model);
    
    let response = client.send_conversation(model, messages).await?;
    println!("Response: {}", response);
    
    Ok(())
}