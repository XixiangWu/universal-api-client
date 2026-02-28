# Universal API Client Library - Design Document

**Date:** 2026-03-01
**Project:** Extract API communication layer from cc-switch
**Approach:** Minimal Extraction (Approach 1)

## Overview

Extract a lightweight, reusable Rust library from the cc-switch project that provides a unified interface for calling AI APIs (Anthropic, OpenAI-compatible providers). The library will be a standalone crate that can be embedded into any Rust project with minimal dependencies.

## Goals

1. **Portability**: Pure Rust library crate, no Tauri/database dependencies
2. **Simplicity**: Easy to integrate with builder pattern API
3. **Streaming**: Full support for SSE streaming responses
4. **Format Conversion**: Automatic translation between Anthropic ↔ OpenAI formats
5. **Testing**: CLI tool for validation and demonstration

## Non-Goals

- Circuit breakers and failover (too complex for minimal approach)
- Provider health tracking and databases
- Multiple provider management
- UI components

## Architecture

### Project Structure

```
universal-api-client/          # Library crate
├── src/
│   ├── lib.rs                 # Public API exports
│   ├── client.rs              # ApiClient + Builder pattern
│   ├── provider/
│   │   ├── mod.rs             # ProviderAdapter trait
│   │   ├── claude.rs          # Anthropic adapter
│   │   ├── openai.rs          # OpenAI adapter
│   │   └── auth.rs            # Authentication strategies
│   ├── transform.rs           # Format conversion (Anthropic ↔ OpenAI)
│   ├── streaming.rs           # SSE streaming support
│   └── error.rs               # Error types
├── examples/
│   └── simple_chat.rs         # Usage example
└── Cargo.toml

universal-api-cli/             # CLI testing tool (separate crate)
├── src/
│   └── main.rs                # Simple CLI with clap
└── Cargo.toml
```

### Core Components

#### 1. ApiClient (client.rs)

Main entry point for users. Provides builder pattern for configuration.

**Public API:**
```rust
pub struct ApiClient { /* private fields */ }

impl ApiClient {
    pub fn builder() -> ApiClientBuilder;
    pub async fn chat(&self, prompt: &str) -> Result<ChatResponse, ApiError>;
    pub async fn chat_stream(&self, prompt: &str) -> Result<impl Stream<Item = Result<StreamEvent>>, ApiError>;
}

pub struct ApiClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    format: ApiFormat,
    model: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    timeout: Duration,
}

impl ApiClientBuilder {
    pub fn base_url(mut self, url: impl Into<String>) -> Self;
    pub fn api_key(mut self, key: impl Into<String>) -> Self;
    pub fn format(mut self, format: ApiFormat) -> Self;
    pub fn model(mut self, model: impl Into<String>) -> Self;
    pub fn max_tokens(mut self, tokens: u32) -> Self;
    pub fn temperature(mut self, temp: f32) -> Self;
    pub fn timeout(mut self, duration: Duration) -> Self;
    pub fn build(self) -> Result<ApiClient, ApiError>;
}
```

#### 2. Provider Adapters (provider/)

Abstraction layer for different API providers.

**ProviderAdapter trait:**
```rust
pub trait ProviderAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn build_url(&self, base_url: &str, endpoint: &str) -> String;
    fn add_auth_headers(&self, request: RequestBuilder, auth: &AuthInfo) -> RequestBuilder;
    fn needs_transform(&self) -> bool;
    fn transform_request(&self, body: Value) -> Result<Value, ApiError>;
    fn transform_response(&self, body: Value) -> Result<Value, ApiError>;
}
```

**Implementations:**
- `ClaudeAdapter`: Anthropic Messages API
  - Supports `x-api-key` + `Authorization: Bearer` headers
  - Adds `anthropic-version` header
  - Adds `?beta=true` query parameter for `/v1/messages`

- `OpenAiAdapter`: OpenAI Chat Completions API
  - Supports `Authorization: Bearer` header
  - Standard OpenAI endpoint structure

#### 3. Authentication (provider/auth.rs)

**Auth strategies:**
```rust
pub enum AuthStrategy {
    Anthropic,      // Bearer + x-api-key
    Bearer,         // Bearer only
    ClaudeAuth,     // Bearer only (for proxies)
}

pub struct AuthInfo {
    pub api_key: String,
    pub strategy: AuthStrategy,
}
```

#### 4. Format Transformation (transform.rs)

Converts between Anthropic and OpenAI message formats.

**Functions:**
```rust
pub fn anthropic_to_openai(body: Value) -> Result<Value, ApiError>;
pub fn openai_to_anthropic(body: Value) -> Result<Value, ApiError>;
```

**Transformations:**
- Message structure: `messages` array format differences
- Role mapping: `user`/`assistant` vs `user`/`assistant`/`system`
- Model names: provider-specific model identifiers
- Streaming events: SSE format differences

#### 5. Streaming Support (streaming.rs)

Parses Server-Sent Events (SSE) for streaming responses.

**Stream events:**
```rust
pub enum StreamEvent {
    MessageStart,
    ContentDelta(String),
    MessageStop,
    Error(String),
    Done,
}
```

**Implementation:**
- Uses `async-stream` for `Stream` trait
- Parses SSE format: `data: {...}\n\n`
- Handles both Anthropic and OpenAI streaming formats
- Graceful error handling for malformed events

#### 6. Error Handling (error.rs)

Comprehensive error types with helpful messages.

```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Streaming error: {0}")]
    Streaming(String),
}
```

### Data Structures

#### ApiFormat

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiFormat {
    Anthropic,
    OpenAI,
}
```

#### ChatResponse

```rust
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub model: String,
    pub usage: Option<Usage>,
    pub stop_reason: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}
```

## CLI Tool Design

### Command Structure

```bash
universal-api-cli \
  --base-url <URL> \
  --api-key <KEY> \
  --format <anthropic|openai> \
  [--model <MODEL>] \
  [--stream] \
  [--max-tokens <N>] \
  [--temperature <T>] \
  [--json] \
  [--verbose] \
  --prompt <TEXT>
```

### Features

1. **Basic Mode**: Single request/response
2. **Streaming Mode**: Real-time output with `--stream`
3. **JSON Output**: Machine-readable with `--json`
4. **Verbose Mode**: Debug info (headers, timing) with `--verbose`
5. **Color Output**: Success (green), errors (red), streaming (cyan)

### Example Usage

```bash
# Test with provided endpoint
universal-api-cli \
  --base-url "https://code.z-daha.cc" \
  --api-key "sk-53d75f3e9e9bc771af279702663e524adbceb698cf0d45e5ce7db3ee0907efd8" \
  --format anthropic \
  --stream \
  --prompt "Hello, how are you?"
```

## Dependencies

### Library (universal-api-client)

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json", "stream"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
async-stream = "0.3"
thiserror = "1"
futures = "0.3"
```

### CLI (universal-api-cli)

```toml
[dependencies]
universal-api-client = { path = "../universal-api-client" }
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
colored = "2"
serde_json = "1"
```

## Implementation Plan

### Phase 1: Core Library Setup
1. Create library crate structure
2. Implement error types (`error.rs`)
3. Implement auth strategies (`provider/auth.rs`)
4. Define `ProviderAdapter` trait (`provider/mod.rs`)

### Phase 2: Provider Adapters
5. Extract and adapt `ClaudeAdapter` from cc-switch
6. Extract and adapt `OpenAiAdapter` from cc-switch (codex)
7. Implement format transformation (`transform.rs`)

### Phase 3: Client Implementation
8. Implement streaming support (`streaming.rs`)
9. Build `ApiClient` and builder pattern (`client.rs`)
10. Create public API exports (`lib.rs`)

### Phase 4: CLI Tool
11. Create CLI crate structure
12. Implement argument parsing with clap
13. Add colored output and progress indicators
14. Implement JSON output mode

### Phase 5: Testing & Documentation
15. Write unit tests for all components
16. Integration test with provided endpoint
17. Write comprehensive README
18. Add inline documentation and examples

## Testing Strategy

### Unit Tests

- Format conversion correctness
- Auth header construction
- URL building logic
- Error handling paths

### Integration Tests

**Test Endpoint:**
- Base URL: `https://code.z-daha.cc`
- API Key: `sk-53d75f3e9e9bc771af279702663e524adbceb698cf0d45e5ce7db3ee0907efd8`
- Format: Anthropic

**Test Cases:**
1. Non-streaming request
2. Streaming request with real-time output
3. Error handling (invalid key, network errors)
4. Both Anthropic and OpenAI format modes

### CLI Tests

- Command parsing
- Output formatting
- Streaming display
- Error messages

## Documentation

### README.md Structure

1. **Quick Start** (5-minute setup)
2. **Installation** (`cargo add universal-api-client`)
3. **Basic Examples**
   - Non-streaming chat
   - Streaming chat
   - Custom configuration
4. **API Reference**
   - `ApiClient` methods
   - Builder options
   - Error types
5. **Supported Providers**
   - Anthropic (official)
   - OpenAI (official)
   - Compatible services (OpenRouter, proxies)
6. **CLI Tool Usage**
   - Installation
   - Command reference
   - Examples
7. **Integration Guide**
   - How to embed in projects
   - Best practices
   - Common patterns
8. **Troubleshooting**
   - Common errors
   - Debug tips

### Inline Documentation

- Every public API has doc comments
- Examples in doc comments
- Links to API documentation
- Usage warnings where appropriate

## Extraction Guidelines

### From cc-switch Source Files

**Extract from:**
- `src-tauri/src/proxy/providers/adapter.rs` → `provider/mod.rs`
- `src-tauri/src/proxy/providers/claude.rs` → `provider/claude.rs`
- `src-tauri/src/proxy/providers/codex.rs` → `provider/openai.rs`
- `src-tauri/src/proxy/providers/auth.rs` → `provider/auth.rs`
- `src-tauri/src/proxy/providers/transform.rs` → `transform.rs`

**Remove:**
- All Tauri dependencies (`tauri::*`)
- Database code (SQLite, `Database` struct)
- Circuit breaker logic
- Provider router and failover
- Health tracking
- UI-related code
- `Provider` struct (replace with simple config)

**Keep:**
- Core adapter trait and implementations
- Auth strategies and header construction
- Format transformation logic
- URL building logic
- Request/response handling
- Streaming SSE parsing

**Adapt:**
- Replace `Provider` struct with builder pattern config
- Remove database lookups, use direct config values
- Simplify error types (remove app-specific errors)
- Make all APIs async with tokio

## Success Criteria

1. ✅ Library compiles without Tauri/database dependencies
2. ✅ Successfully calls test endpoint with Anthropic format
3. ✅ Streaming responses work correctly
4. ✅ CLI tool displays real-time streaming output
5. ✅ Format conversion works bidirectionally
6. ✅ Comprehensive documentation and examples
7. ✅ Easy to integrate into other projects

## Future Enhancements (Out of Scope)

- Circuit breakers and retry logic
- Multiple provider management
- Health tracking and metrics
- Configuration file support
- Additional provider adapters (Gemini, etc.)
- Rate limiting
- Request caching

---

**Next Steps:** Proceed to implementation planning with the `writing-plans` skill.
