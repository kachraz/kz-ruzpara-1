# OpenRouter API Client

A modular, async Rust client for the OpenRouter API that provides easy access to various AI models through a unified interface.

## Features

- **Modular Design**: Clean separation of concerns with dedicated modules for configuration, client, models, and error handling
- **Async/Await Support**: Built with Tokio for efficient async operations
- **Type Safety**: Strongly typed request/response models using Serde
- **Error Handling**: Comprehensive error types for better debugging
- **Easy Configuration**: Support for environment variables and manual configuration
- **Well Documented**: Extensive code documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```

## Quick Start

### 1. Set up your API key

```bash
export OPENROUTER_API_KEY="your-api-key-here"
```

### 2. Basic usage

```rust
use openrouter_client::{Config, OpenRouterClient, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let config = Config::from_env()?;
    let client = OpenRouterClient::new(config)?;
    
    // Send a simple message
    let response = client.send_message(
        "openai/gpt-3.5-turbo",
        "Hello, how are you?"
    ).await?;
    
    println!("Response: {}", response);
    Ok(())
}
```

## API Reference

### Configuration

```rust
// From environment variable
let config = Config::from_env()?;

// Manual configuration
let config = Config::new("your-api-key".to_string())
    .with_base_url("https://openrouter.ai/api/v1".to_string())
    .with_timeout(60);
```

### Client Operations

```rust
// List available models
let models = client.list_models().await?;

// Send a simple message
let response = client.send_message("model-name", "your message").await?;

// Send a conversation
let messages = vec![
    Message::system("You are a helpful assistant".to_string()),
    Message::user("Hello!".to_string()),
];
let response = client.send_conversation("model-name", messages).await?;

// Advanced chat completion
let request = ChatCompletionRequest::new("model-name".to_string(), messages)
    .with_max_tokens(100)
    .with_temperature(0.7);
let response = client.create_chat_completion(request).await?;
```

## Examples

### List Available Models

```rust
let models = client.list_models().await?;
for model in models.data {
    println!("{} by {}", model.id, model.owned_by);
}
```

### Conversation with Context

```rust
let messages = vec![
    Message::system("You are a Rust programming expert.".to_string()),
    Message::user("Explain async/await in Rust".to_string()),
    Message::assistant("Async/await in Rust allows...".to_string()),
    Message::user("Can you show an example?".to_string()),
];

let response = client.send_conversation("openai/gpt-4", messages).await?;
```

## Error Handling

The client provides comprehensive error handling:

```rust
match client.send_message("model", "message").await {
    Ok(response) => println!("Success: {}", response),
    Err(OpenRouterError::ApiError { status, message }) => {
        println!("API Error {}: {}", status, message);
    },
    Err(OpenRouterError::RequestError(e)) => {
        println!("Request failed: {}", e);
    },
    Err(e) => println!("Other error: {}", e),
}
```

## Testing

Run tests with:

```bash
cargo nextest run
```

For integration tests with a real API key:

```bash
OPENROUTER_API_KEY=your-key cargo nextest run
```

## Module Structure

- `config`: Configuration management
- `client`: Main API client implementation  
- `models`: Request/response data structures
- `error`: Error types and handling
- `lib.rs`: Public API exports

## License

This project is licensed under the MIT License.