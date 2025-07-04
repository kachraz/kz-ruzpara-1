1. [OpenRouter API Client (ro2)](#openrouter-api-client-ro2)
   1. [Project Structure \& File Descriptions](#project-structure--file-descriptions)
      1. [Core Source Files (src/)](#core-source-files-src)
         1. [`src/lib.rs` - Library Entry Point](#srclibrs---library-entry-point)
         2. [`src/main.rs` - CLI Application](#srcmainrs---cli-application)
         3. [`src/config.rs` - Configuration Management](#srcconfigrs---configuration-management)
         4. [`src/models.rs` - **PRIMARY MODEL CONFIGURATION FILE**](#srcmodelsrs---primary-model-configuration-file)
         5. [`src/openrouter_client.rs` - HTTP Client Implementation](#srcopenrouter_clientrs---http-client-implementation)
      2. [Examples \& Tests](#examples--tests)
         1. [`examples/basic_usage.rs` - Usage Examples](#examplesbasic_usagers---usage-examples)
         2. [`tests/integration_tests.rs` - Test Suite](#testsintegration_testsrs---test-suite)
      3. [Configuration \& Documentation](#configuration--documentation)
         1. [`Cargo.toml` - Project Configuration](#cargotoml---project-configuration)
         2. [`.env` / `.env.example` - Environment Variables](#env--envexample---environment-variables)
         3. [`prompt.txt` - Input File](#prompttxt---input-file)
         4. [`results/` - Output Directory](#results---output-directory)
   2. [How to Change AI Models](#how-to-change-ai-models)
      1. [Quick Model Change (CLI):](#quick-model-change-cli)
      2. [Add New Models to Available List:](#add-new-models-to-available-list)
      3. [Current Available Models:](#current-available-models)
   3. [Quick Start](#quick-start)
   4. [Usage Examples](#usage-examples)
      1. [Basic Library Usage](#basic-library-usage)
      2. [Multi-Message Conversation](#multi-message-conversation)
      3. [Custom Parameters](#custom-parameters)
      4. [Custom Configuration](#custom-configuration)
   5. [Features](#features)
   6. [Installation](#installation)
   7. [Testing](#testing)
   8. [Error Handling](#error-handling)
   9. [Contributing](#contributing)
   10. [License](#license)

# OpenRouter API Client (ro2)

A modular, async Rust client for the OpenRouter API that provides easy access to various AI models.

## Project Structure & File Descriptions

This section explains what each file does and where to make specific changes:

### Core Source Files (src/)

#### `src/lib.rs` - Library Entry Point

- **Purpose**: Main library file that exports all public modules and types
- **What it does**: Defines the public API surface and re-exports commonly used types
- **When to modify**: When adding new modules or changing the public API

#### `src/main.rs` - CLI Application

- **Purpose**: Command-line interface for the OpenRouter client
- **What it does**:
  - Reads prompts from `prompt.txt`
  - Sends requests to OpenRouter API
  - Saves responses to timestamped markdown files in `results/`
- **MODEL CHANGES**: **Line 41** - Change `selected_model` variable to use different AI models
- **When to modify**: To change default model, add CLI arguments, or modify output format

#### `src/config.rs` - Configuration Management

- **Purpose**: Handles API keys, URLs, and client configuration
- **What it does**:
  - Loads API keys from environment variables or `.env` file
  - Manages OpenRouter base URL and optional site headers
  - Provides configuration validation
- **When to modify**: To add new configuration options or change API endpoints

#### `src/models.rs` - **PRIMARY MODEL CONFIGURATION FILE**

- **Purpose**: **THIS IS WHERE YOU MODIFY AVAILABLE MODELS**
- **What it does**:
  - **Lines 12-24**: `get_available_models()` - **ADD/REMOVE MODELS HERE**
  - Defines data structures for API requests and responses
  - Provides helper methods for creating messages
- **MODEL CHANGES**:
  - **Add new models**: Add model strings to the `vec!` in `get_available_models()`
  - **Remove models**: Remove model strings from the same vector
  - **Model format**: Use format `"provider/model-name"` (e.g., `"openai/gpt-4o"`)

#### `src/openrouter_client.rs` - HTTP Client Implementation

- **Purpose**: Handles all HTTP communication with OpenRouter API
- **What it does**:
  - Builds HTTP headers with authentication
  - Sends chat completion requests
  - Handles API responses and errors
  - Provides multiple methods for different request types
- **When to modify**: To add new API endpoints, change request parameters, or modify error handling

### Examples & Tests

#### `examples/basic_usage.rs` - Usage Examples

- **Purpose**: Demonstrates how to use the client library
- **What it does**: Shows basic usage, multi-message conversations, and custom parameters
- **MODEL CHANGES**: **Lines 44, 65, 85** - Examples use specific models that you can change
- **When to modify**: To add new usage examples or demonstrate new features

#### `tests/integration_tests.rs` - Test Suite

- **Purpose**: Validates library functionality
- **What it does**: Tests model lists, message creation, configuration, and client setup
- **When to modify**: When adding new features that need testing

### Configuration & Documentation

#### `Cargo.toml` - Project Configuration

- **Purpose**: Rust project manifest
- **What it does**: Defines dependencies, project metadata, and build configuration
- **When to modify**: To add new dependencies or change project settings

#### `.env` / `.env.example` - Environment Variables

- **Purpose**: Store API keys and configuration
- **What it does**: Contains `OPENROUTER_API_KEY` and optional site headers
- **When to modify**: To add your actual API key (in `.env`, not `.env.example`)

#### `prompt.txt` - Input File

- **Purpose**: Contains the prompt/question for the CLI application
- **What it does**: The CLI reads this file and sends its contents to the AI model
- **When to modify**: Change this file to ask different questions

#### `results/` - Output Directory

- **Purpose**: Stores AI responses from the CLI application
- **What it does**: Contains timestamped markdown files with AI responses
- **When to modify**: These are auto-generated; you can delete old files if needed

## How to Change AI Models

### Quick Model Change (CLI):

1. **Edit `src/main.rs` line 41**:
   ```rust
   let selected_model = "anthropic/claude-3-opus";  // Change this line
   ```

### Add New Models to Available List:

1. **Edit `src/models.rs` lines 12-24**:
   ```rust
   pub fn get_available_models() -> Vec<&'static str> {
       vec![
           "openai/gpt-4o",
           "openai/gpt-4o-mini",
           "your-new-model/model-name",  // Add new models here
           // ... existing models
       ]
   }
   ```

### Current Available Models:

- `openai/gpt-4o` - Latest GPT-4 model
- `openai/gpt-4o-mini` - Faster, cheaper GPT-4
- `openai/gpt-4-turbo` - GPT-4 Turbo
- `openai/gpt-3.5-turbo` - GPT-3.5 Turbo
- `anthropic/claude-3-opus` - Claude 3 Opus (most capable)
- `anthropic/claude-3-sonnet` - Claude 3 Sonnet (balanced)
- `anthropic/claude-3-haiku` - Claude 3 Haiku (fastest)
- `google/gemini-pro` - Google's Gemini Pro
- `meta-llama/llama-2-70b-chat` - Meta's Llama 2
- `mistralai/mistral-7b-instruct` - Mistral AI model

## Quick Start

1. **Set up API key**:

   ```bash
   export OPENROUTER_API_KEY="your-api-key-here"
   ```

2. **Create a prompt**:

   ```bash
   echo "What is the meaning of life?" > prompt.txt
   ```

3. **Run the CLI**:

   ```bash
   cargo run
   ```

4. **Check results**:
   ```bash
   ls results/
   ```

## Usage Examples

### Basic Library Usage

```rust
use ro2::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = OpenRouterClient::new()?;
    let response = client
        .send_chat_completion("openai/gpt-4o", "Hello!")
        .await?;
    println!("Response: {}", response);
    Ok(())
}
```

### Multi-Message Conversation

```rust
let messages = vec![
    Message::system("You are a helpful assistant."),
    Message::user("What is Rust?"),
];
let response = client
    .send_chat_completion_with_messages("openai/gpt-4o", messages)
    .await?;
```

### Custom Parameters

```rust
let response = client
    .send_chat_completion_with_params(
        "openai/gpt-4o",
        messages,
        Some(0.8),      // temperature
        Some(100),      // max_tokens
    )
    .await?;
```

### Custom Configuration

```rust
let config = Config::with_custom(
    "your-api-key".to_string(),
    Some("https://your-site.com".to_string()),
    Some("Your App Name".to_string()),
);

let client = OpenRouterClient::with_config(config);
```

## Features

- **Modular Design**: Clean separation of concerns with dedicated modules
- **Async Support**: Built with `tokio` for efficient async operations
- **Easy to Use**: Simple API with sensible defaults
- **Configurable**: Support for custom headers and multiple models
- **Well Documented**: Comprehensive code annotations and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
dotenv = "0.15"
chrono = { version = "0.4", features = ["serde"] }
```

## Testing

```bash
cargo test
```

## Error Handling

The client uses `anyhow::Result` for error handling. Common errors include:

- Missing API key
- Network connectivity issues
- Invalid API responses
- Rate limiting

## Contributing

1. Ensure your code is well-documented
2. Add tests for new functionality
3. Run `cargo test` before submitting
4. Follow the existing modular structure
5. Update this README when adding new files

## License

This project is open source and available under the MIT License.
