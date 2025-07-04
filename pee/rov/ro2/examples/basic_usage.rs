/// Basic usage example for OpenRouter API client
/// 
/// This example demonstrates how to use the OpenRouter client to send
/// chat completion requests to various AI models.
/// 
/// To run this example:
/// 1. Set your OPENROUTER_API_KEY environment variable
/// 2. Run: cargo run --example basic_usage

use ro2::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 OpenRouter API Client Example");
    println!("================================");
    
    // Display available models
    println!("\n📋 Available Models:");
    let models = ModelList::get_available_models();
    for (i, model) in models.iter().enumerate() {
        println!("  {}. {}", i + 1, model);
    }
    
    // Try to create a client
    println!("\n🔧 Creating OpenRouter client...");
    let client = match OpenRouterClient::new() {
        Ok(client) => {
            println!("✅ Client created successfully!");
            client
        }
        Err(e) => {
            println!("❌ Failed to create client: {}", e);
            println!("💡 Make sure to set your OPENROUTER_API_KEY environment variable");
            println!("   Example: export OPENROUTER_API_KEY=\"your-api-key-here\"");
            return Ok(());
        }
    };
    
    // Example 1: Simple chat completion
    println!("\n💬 Example 1: Simple Chat Completion");
    println!("Model: openai/gpt-3.5-turbo");
    println!("Message: 'Hello! Can you tell me a fun fact about Rust programming?'");
    
    match client.send_chat_completion(
        "openai/gpt-3.5-turbo", 
        "Hello! Can you tell me a fun fact about Rust programming?"
    ).await {
        Ok(response) => {
            println!("✅ Response received:");
            println!("   {}", response);
        }
        Err(e) => {
            println!("❌ Request failed: {}", e);
        }
    }
    
    // Example 2: Multi-message conversation
    println!("\n🗣️  Example 2: Multi-Message Conversation");
    println!("Model: openai/gpt-4o-mini");
    
    let messages = vec![
        Message::system("You are a helpful programming assistant that gives concise answers."),
        Message::user("What is the difference between Vec and Array in Rust?"),
    ];
    
    match client.send_chat_completion_with_messages("openai/gpt-4o-mini", messages).await {
        Ok(response) => {
            println!("✅ Response received:");
            println!("   {}", response);
        }
        Err(e) => {
            println!("❌ Request failed: {}", e);
        }
    }
    
    // Example 3: Custom parameters
    println!("\n⚙️  Example 3: Custom Parameters");
    println!("Model: anthropic/claude-3-haiku");
    println!("Temperature: 0.3, Max tokens: 50");
    
    let messages = vec![
        Message::user("Write a very short haiku about programming in Rust."),
    ];
    
    match client.send_chat_completion_with_params(
        "anthropic/claude-3-haiku",
        messages,
        Some(0.3),  // Lower temperature for more focused output
        Some(50),   // Limit tokens for a short response
    ).await {
        Ok(response) => {
            println!("✅ Response received:");
            println!("   {}", response);
        }
        Err(e) => {
            println!("❌ Request failed: {}", e);
        }
    }
    
    println!("\n🎉 Example completed!");
    println!("💡 Try modifying the examples above to experiment with different models and parameters.");
    
    Ok(())
}