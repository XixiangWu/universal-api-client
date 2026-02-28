# Universal API Client Library Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Extract a lightweight, reusable Rust library from cc-switch that provides unified API access to Anthropic and OpenAI-compatible services with streaming support.

**Architecture:** Pure Rust library crate using builder pattern for configuration, provider adapter pattern for multi-API support, and async streaming via tokio. Includes a simple CLI tool for testing and demonstration.

**Tech Stack:** Rust, tokio, reqwest, serde, async-stream, clap

---

## Task 1: Project Structure Setup

**Files:**
- Create: `universal-api-client/Cargo.toml`
- Create: `universal-api-client/src/lib.rs`
- Create: `universal-api-cli/Cargo.toml`
- Create: `universal-api-cli/src/main.rs`
- Create: `.gitignore`

**Step 1: Create library crate**

```bash
cd /Users/xixiangwu/Documents/Codes/CCSwitchAPIFunction
cargo new --lib universal-api-client
```

**Step 2: Create CLI crate**

```bash
cargo new universal-api-cli
```

**Step 3: Configure library Cargo.toml**

File: `universal-api-client/Cargo.toml`

```toml
[package]
name = "universal-api-client"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "Lightweight Rust library for unified API access to Anthropic and OpenAI-compatible services"
license = "MIT"
repository = "https://github.com/yourusername/universal-api-client"

[dependencies]
reqwest = { version = "0.12", features = ["json", "stream"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
async-stream = "0.3"
thiserror = "1"
futures = "0.3"

[dev-dependencies]
tokio-test = "0.4"
```

**Step 4: Configure CLI Cargo.toml**

File: `universal-api-cli/Cargo.toml`

```toml
[package]
name = "universal-api-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
universal-api-client = { path = "../universal-api-client" }
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
colored = "2"
serde_json = "1"
futures = "0.3"
```

**Step 5: Create .gitignore**

File: `.gitignore`

```
/target/
**/*.rs.bk
Cargo.lock
.DS_Store
```

**Step 6: Verify compilation**

```bash
cd universal-api-client && cargo check
cd ../universal-api-cli && cargo check
```

Expected: Both compile successfully

**Step 7: Commit**

```bash
git add .
git commit -m "feat: initialize project structure with library and CLI crates"
```

---

## Task 2: Error Types

**Files:**
- Create: `universal-api-client/src/error.rs`
- Modify: `universal-api-client/src/lib.rs`

**Step 1: Write error type tests**

File: `universal-api-client/src/error.rs`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
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

pub type Result<T> = std::result::Result<T, ApiError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_display() {
        let err = ApiError::Config("missing api key".to_string());
        assert_eq!(err.to_string(), "Configuration error: missing api key");
    }

    #[test]
    fn test_api_error_display() {
        let err = ApiError::Api {
            status: 401,
            message: "Unauthorized".to_string(),
        };
        assert_eq!(err.to_string(), "API error: 401 - Unauthorized");
    }

    #[test]
    fn test_auth_error_display() {
        let err = ApiError::Auth("invalid key".to_string());
        assert_eq!(err.to_string(), "Authentication failed: invalid key");
    }
}
```

**Step 2: Run tests**

```bash
cd universal-api-client
cargo test error::tests
```

Expected: All tests pass

**Step 3: Export from lib.rs**

File: `universal-api-client/src/lib.rs`

```rust
pub mod error;

pub use error::{ApiError, Result};
```

**Step 4: Verify compilation**

```bash
cargo check
```

Expected: Compiles successfully

**Step 5: Commit**

```bash
git add src/error.rs src/lib.rs
git commit -m "feat: add error types with comprehensive error handling"
```

---

## Task 3: Authentication Module

**Files:**
- Create: `universal-api-client/src/provider/mod.rs`
- Create: `universal-api-client/src/provider/auth.rs`
- Modify: `universal-api-client/src/lib.rs`

**Step 1: Create provider module structure**

```bash
cd universal-api-client
mkdir -p src/provider
```

**Step 2: Write auth types with tests**

File: `universal-api-client/src/provider/auth.rs`

```rust
/// Authentication strategy for different API providers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthStrategy {
    /// Anthropic official: Bearer + x-api-key headers
    Anthropic,
    /// Standard Bearer token only
    Bearer,
    /// Claude proxy services: Bearer only (no x-api-key)
    ClaudeAuth,
}

/// Authentication information
#[derive(Debug, Clone)]
pub struct AuthInfo {
    pub api_key: String,
    pub strategy: AuthStrategy,
}

impl AuthInfo {
    pub fn new(api_key: String, strategy: AuthStrategy) -> Self {
        Self { api_key, strategy }
    }

    pub fn anthropic(api_key: String) -> Self {
        Self::new(api_key, AuthStrategy::Anthropic)
    }

    pub fn bearer(api_key: String) -> Self {
        Self::new(api_key, AuthStrategy::Bearer)
    }

    pub fn claude_auth(api_key: String) -> Self {
        Self::new(api_key, AuthStrategy::ClaudeAuth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_auth_creation() {
        let auth = AuthInfo::anthropic("sk-ant-test".to_string());
        assert_eq!(auth.api_key, "sk-ant-test");
        assert_eq!(auth.strategy, AuthStrategy::Anthropic);
    }

    #[test]
    fn test_bearer_auth_creation() {
        let auth = AuthInfo::bearer("sk-test".to_string());
        assert_eq!(auth.api_key, "sk-test");
        assert_eq!(auth.strategy, AuthStrategy::Bearer);
    }

    #[test]
    fn test_claude_auth_creation() {
        let auth = AuthInfo::claude_auth("sk-proxy".to_string());
        assert_eq!(auth.api_key, "sk-proxy");
        assert_eq!(auth.strategy, AuthStrategy::ClaudeAuth);
    }
}
```

**Step 3: Run tests**

```bash
cargo test provider::auth::tests
```

Expected: All tests pass

**Step 4: Create provider mod.rs**

File: `universal-api-client/src/provider/mod.rs`

```rust
pub mod auth;

pub use auth::{AuthInfo, AuthStrategy};
```

**Step 5: Export from lib.rs**

File: `universal-api-client/src/lib.rs`

```rust
pub mod error;
pub mod provider;

pub use error::{ApiError, Result};
pub use provider::{AuthInfo, AuthStrategy};
```

**Step 6: Verify compilation**

```bash
cargo check
```

Expected: Compiles successfully

**Step 7: Commit**

```bash
git add src/provider/
git commit -m "feat: add authentication module with multiple strategies"
```

---

## Task 4: Provider Adapter Trait

**Files:**
- Modify: `universal-api-client/src/provider/mod.rs`
- Create: `universal-api-client/src/provider/adapter.rs`

**Step 1: Write adapter trait**

File: `universal-api-client/src/provider/adapter.rs`

```rust
use crate::error::{ApiError, Result};
use crate::provider::AuthInfo;
use reqwest::RequestBuilder;
use serde_json::Value;

/// Provider adapter trait for different API providers
///
/// Implementations handle provider-specific logic for:
/// - URL construction
/// - Authentication headers
/// - Request/response format transformation
pub trait ProviderAdapter: Send + Sync {
    /// Adapter name for logging and debugging
    fn name(&self) -> &'static str;

    /// Build complete request URL from base URL and endpoint
    fn build_url(&self, base_url: &str, endpoint: &str) -> String;

    /// Add authentication headers to request
    fn add_auth_headers(&self, request: RequestBuilder, auth: &AuthInfo) -> RequestBuilder;

    /// Check if format transformation is needed
    fn needs_transform(&self) -> bool {
        false
    }

    /// Transform request body (e.g., Anthropic → OpenAI)
    fn transform_request(&self, body: Value) -> Result<Value> {
        Ok(body)
    }

    /// Transform response body (e.g., OpenAI → Anthropic)
    fn transform_response(&self, body: Value) -> Result<Value> {
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestAdapter;

    impl ProviderAdapter for TestAdapter {
        fn name(&self) -> &'static str {
            "Test"
        }

        fn build_url(&self, base_url: &str, endpoint: &str) -> String {
            format!("{}{}", base_url.trim_end_matches('/'), endpoint)
        }

        fn add_auth_headers(&self, request: RequestBuilder, auth: &AuthInfo) -> RequestBuilder {
            request.header("Authorization", format!("Bearer {}", auth.api_key))
        }
    }

    #[test]
    fn test_adapter_name() {
        let adapter = TestAdapter;
        assert_eq!(adapter.name(), "Test");
    }

    #[test]
    fn test_build_url() {
        let adapter = TestAdapter;
        let url = adapter.build_url("https://api.example.com", "/v1/messages");
        assert_eq!(url, "https://api.example.com/v1/messages");
    }

    #[test]
    fn test_build_url_trailing_slash() {
        let adapter = TestAdapter;
        let url = adapter.build_url("https://api.example.com/", "/v1/messages");
        assert_eq!(url, "https://api.example.com/v1/messages");
    }

    #[test]
    fn test_default_no_transform() {
        let adapter = TestAdapter;
        assert!(!adapter.needs_transform());
    }
}
```

**Step 2: Run tests**

```bash
cargo test provider::adapter::tests
```

Expected: All tests pass

**Step 3: Update provider mod.rs**

File: `universal-api-client/src/provider/mod.rs`

```rust
pub mod adapter;
pub mod auth;

pub use adapter::ProviderAdapter;
pub use auth::{AuthInfo, AuthStrategy};
```

**Step 4: Verify compilation**

```bash
cargo check
```

Expected: Compiles successfully

**Step 5: Commit**

```bash
git add src/provider/adapter.rs src/provider/mod.rs
git commit -m "feat: add provider adapter trait for multi-provider support"
```

---

## Task 5: Claude Provider Adapter

**Files:**
- Create: `universal-api-client/src/provider/claude.rs`
- Modify: `universal-api-client/src/provider/mod.rs`

**Step 1: Extract Claude adapter from cc-switch**

Source: `cc-switch/src-tauri/src/proxy/providers/claude.rs`

File: `universal-api-client/src/provider/claude.rs`

```rust
use crate::error::{ApiError, Result};
use crate::provider::{AuthInfo, AuthStrategy, ProviderAdapter};
use reqwest::RequestBuilder;
use serde_json::Value;

/// Claude (Anthropic) provider adapter
pub struct ClaudeAdapter;

impl ClaudeAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ClaudeAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderAdapter for ClaudeAdapter {
    fn name(&self) -> &'static str {
        "Claude"
    }

    fn build_url(&self, base_url: &str, endpoint: &str) -> String {
        let mut base = format!(
            "{}/{}",
            base_url.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        );

        // Remove duplicate /v1/v1 if present
        while base.contains("/v1/v1") {
            base = base.replace("/v1/v1", "/v1");
        }

        // Add ?beta=true for /v1/messages endpoint
        // Required by some upstream services for request validation
        if endpoint.contains("/v1/messages")
            && !endpoint.contains("/v1/chat/completions")
            && !endpoint.contains('?')
        {
            format!("{base}?beta=true")
        } else {
            base
        }
    }

    fn add_auth_headers(&self, request: RequestBuilder, auth: &AuthInfo) -> RequestBuilder {
        match auth.strategy {
            // Anthropic official: Authorization Bearer + x-api-key
            AuthStrategy::Anthropic => request
                .header("Authorization", format!("Bearer {}", auth.api_key))
                .header("x-api-key", &auth.api_key)
                .header("anthropic-version", "2023-06-01"),
            // ClaudeAuth proxy: Bearer only, no x-api-key
            AuthStrategy::ClaudeAuth => request
                .header("Authorization", format!("Bearer {}", auth.api_key))
                .header("anthropic-version", "2023-06-01"),
            // OpenRouter/Bearer: Bearer only
            AuthStrategy::Bearer => request
                .header("Authorization", format!("Bearer {}", auth.api_key)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url_messages() {
        let adapter = ClaudeAdapter::new();
        let url = adapter.build_url("https://api.anthropic.com", "/v1/messages");
        assert_eq!(url, "https://api.anthropic.com/v1/messages?beta=true");
    }

    #[test]
    fn test_build_url_no_beta_for_other_endpoints() {
        let adapter = ClaudeAdapter::new();
        let url = adapter.build_url("https://api.anthropic.com", "/v1/complete");
        assert_eq!(url, "https://api.anthropic.com/v1/complete");
    }

    #[test]
    fn test_build_url_preserve_existing_query() {
        let adapter = ClaudeAdapter::new();
        let url = adapter.build_url("https://api.anthropic.com", "/v1/messages?foo=bar");
        assert_eq!(url, "https://api.anthropic.com/v1/messages?foo=bar");
    }

    #[test]
    fn test_build_url_removes_duplicate_v1() {
        let adapter = ClaudeAdapter::new();
        let url = adapter.build_url("https://api.example.com/v1", "/v1/messages");
        assert_eq!(url, "https://api.example.com/v1/messages?beta=true");
    }

    #[test]
    fn test_adapter_name() {
        let adapter = ClaudeAdapter::new();
        assert_eq!(adapter.name(), "Claude");
    }
}
```

**Step 2: Run tests**

```bash
cargo test provider::claude::tests
```

Expected: All tests pass

**Step 3: Update provider mod.rs**

File: `universal-api-client/src/provider/mod.rs`

```rust
pub mod adapter;
pub mod auth;
pub mod claude;

pub use adapter::ProviderAdapter;
pub use auth::{AuthInfo, AuthStrategy};
pub use claude::ClaudeAdapter;
```

**Step 4: Verify compilation**

```bash
cargo check
```

Expected: Compiles successfully

**Step 5: Commit**

```bash
git add src/provider/claude.rs src/provider/mod.rs
git commit -m "feat: add Claude provider adapter with Anthropic API support"
```

---

## Task 6: OpenAI Provider Adapter

**Files:**
- Create: `universal-api-client/src/provider/openai.rs`
- Modify: `universal-api-client/src/provider/mod.rs`

**Step 1: Create OpenAI adapter**

File: `universal-api-client/src/provider/openai.rs`

```rust
use crate::error::{ApiError, Result};
use crate::provider::{AuthInfo, AuthStrategy, ProviderAdapter};
use reqwest::RequestBuilder;
use serde_json::Value;

/// OpenAI provider adapter
pub struct OpenAiAdapter;

impl OpenAiAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OpenAiAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderAdapter for OpenAiAdapter {
    fn name(&self) -> &'static str {
        "OpenAI"
    }

    fn build_url(&self, base_url: &str, endpoint: &str) -> String {
        format!(
            "{}/{}",
            base_url.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        )
    }

    fn add_auth_headers(&self, request: RequestBuilder, auth: &AuthInfo) -> RequestBuilder {
        request.header("Authorization", format!("Bearer {}", auth.api_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url() {
        let adapter = OpenAiAdapter::new();
        let url = adapter.build_url("https://api.openai.com", "/v1/chat/completions");
        assert_eq!(url, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_build_url_trailing_slash() {
        let adapter = OpenAiAdapter::new();
        let url = adapter.build_url("https://api.openai.com/", "/v1/chat/completions");
        assert_eq!(url, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_adapter_name() {
        let adapter = OpenAiAdapter::new();
        assert_eq!(adapter.name(), "OpenAI");
    }
}
```

**Step 2: Run tests**

```bash
cargo test provider::openai::tests
```

Expected: All tests pass

**Step 3: Update provider mod.rs**

File: `universal-api-client/src/provider/mod.rs`

```rust
pub mod adapter;
pub mod auth;
pub mod claude;
pub mod openai;

pub use adapter::ProviderAdapter;
pub use auth::{AuthInfo, AuthStrategy};
pub use claude::ClaudeAdapter;
pub use openai::OpenAiAdapter;
```

**Step 4: Verify compilation**

```bash
cargo check
```

Expected: Compiles successfully

**Step 5: Commit**

```bash
git add src/provider/openai.rs src/provider/mod.rs
git commit -m "feat: add OpenAI provider adapter"
```

---

## Task 7: Format Transformation Module

**Files:**
- Create: `universal-api-client/src/transform.rs`
- Modify: `universal-api-client/src/lib.rs`

**Step 1: Write transformation functions with tests**

File: `universal-api-client/src/transform.rs`

```rust
use crate::error::{ApiError, Result};
use serde_json::{json, Value};

/// Convert Anthropic Messages API format to OpenAI Chat Completions format
pub fn anthropic_to_openai(body: Value) -> Result<Value> {
    let obj = body.as_object().ok_or_else(|| {
        ApiError::Parse("Request body must be an object".to_string())
    })?;

    // Extract messages
    let messages = obj.get("messages")
        .and_then(|v| v.as_array())
        .ok_or_else(|| ApiError::Parse("Missing messages array".to_string()))?;

    // Convert messages format
    let converted_messages: Vec<Value> = messages
        .iter()
        .map(|msg| {
            let role = msg.get("role").and_then(|v| v.as_str()).unwrap_or("user");
            let content = msg.get("content").cloned().unwrap_or(json!(""));
            json!({
                "role": role,
                "content": content
            })
        })
        .collect();

    // Build OpenAI request
    let mut result = json!({
        "messages": converted_messages,
        "stream": obj.get("stream").cloned().unwrap_or(json!(false)),
    });

    // Add model if present
    if let Some(model) = obj.get("model") {
        result["model"] = model.clone();
    }

    // Add max_tokens if present
    if let Some(max_tokens) = obj.get("max_tokens") {
        result["max_tokens"] = max_tokens.clone();
    }

    // Add temperature if present
    if let Some(temperature) = obj.get("temperature") {
        result["temperature"] = temperature.clone();
    }

    Ok(result)
}

/// Convert OpenAI Chat Completions format to Anthropic Messages API format
pub fn openai_to_anthropic(body: Value) -> Result<Value> {
    let obj = body.as_object().ok_or_else(|| {
        ApiError::Parse("Response body must be an object".to_string())
    })?;

    // Extract content from OpenAI response
    let content = obj
        .get("choices")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|msg| msg.get("content"))
        .and_then(|c| c.as_str())
        .unwrap_or("");

    // Build Anthropic response
    let result = json!({
        "id": obj.get("id").cloned().unwrap_or(json!("msg-unknown")),
        "type": "message",
        "role": "assistant",
        "content": [
            {
                "type": "text",
                "text": content
            }
        ],
        "model": obj.get("model").cloned().unwrap_or(json!("unknown")),
        "stop_reason": "end_turn",
        "usage": {
            "input_tokens": obj.get("usage")
                .and_then(|u| u.get("prompt_tokens"))
                .cloned()
                .unwrap_or(json!(0)),
            "output_tokens": obj.get("usage")
                .and_then(|u| u.get("completion_tokens"))
                .cloned()
                .unwrap_or(json!(0))
        }
    });

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_to_openai_basic() {
        let anthropic = json!({
            "model": "claude-3-sonnet",
            "messages": [
                {
                    "role": "user",
                    "content": "Hello"
                }
            ],
            "max_tokens": 1024
        });

        let result = anthropic_to_openai(anthropic).unwrap();
        assert_eq!(result["model"], "claude-3-sonnet");
        assert_eq!(result["max_tokens"], 1024);
        assert_eq!(result["messages"][0]["role"], "user");
        assert_eq!(result["messages"][0]["content"], "Hello");
    }

    #[test]
    fn test_openai_to_anthropic_basic() {
        let openai = json!({
            "id": "chatcmpl-123",
            "model": "gpt-4",
            "choices": [
                {
                    "message": {
                        "role": "assistant",
                        "content": "Hello there!"
                    }
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 20
            }
        });

        let result = openai_to_anthropic(openai).unwrap();
        assert_eq!(result["type"], "message");
        assert_eq!(result["role"], "assistant");
        assert_eq!(result["content"][0]["text"], "Hello there!");
        assert_eq!(result["usage"]["input_tokens"], 10);
        assert_eq!(result["usage"]["output_tokens"], 20);
    }

    #[test]
    fn test_anthropic_to_openai_missing_messages() {
        let invalid = json!({
            "model": "claude-3-sonnet"
        });

        let result = anthropic_to_openai(invalid);
        assert!(result.is_err());
    }
}
```

**Step 2: Run tests**

```bash
cargo test transform::tests
```

Expected: All tests pass

**Step 3: Export from lib.rs**

File: `universal-api-client/src/lib.rs`

```rust
pub mod error;
pub mod provider;
pub mod transform;

pub use error::{ApiError, Result};
pub use provider::{AuthInfo, AuthStrategy, ClaudeAdapter, OpenAiAdapter, ProviderAdapter};
```

**Step 4: Verify compilation**

```bash
cargo check
```

Expected: Compiles successfully

**Step 5: Commit**

```bash
git add src/transform.rs src/lib.rs
git commit -m "feat: add format transformation between Anthropic and OpenAI"
```

---

## Task 8: Streaming Support Module

**Files:**
- Create: `universal-api-client/src/streaming.rs`
- Modify: `universal-api-client/src/lib.rs`

**Step 1: Write streaming types and parser**

File: `universal-api-client/src/streaming.rs`

```rust
use crate::error::{ApiError, Result};
use futures::stream::Stream;
use futures::StreamExt;
use serde_json::Value;
use std::pin::Pin;

/// Events emitted during streaming
#[derive(Debug, Clone, PartialEq)]
pub enum StreamEvent {
    /// Stream started
    MessageStart,
    /// Incremental content delta
    ContentDelta(String),
    /// Stream stopped
    MessageStop,
    /// Error during streaming
    Error(String),
    /// Stream complete
    Done,
}

/// Parse SSE (Server-Sent Events) stream into StreamEvent
pub fn parse_sse_stream(
    response: reqwest::Response,
) -> Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>> {
    let stream = response.bytes_stream();

    Box::pin(async_stream::stream! {
        let mut buffer = String::new();

        futures::pin_mut!(stream);
        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    let text = match String::from_utf8(chunk.to_vec()) {
                        Ok(t) => t,
                        Err(e) => {
                            yield Err(ApiError::Streaming(format!("Invalid UTF-8: {}", e)));
                            continue;
                        }
                    };

                    buffer.push_str(&text);

                    // Process complete SSE events (terminated by \n\n)
                    while let Some(pos) = buffer.find("\n\n") {
                        let event_text = buffer[..pos].to_string();
                        buffer = buffer[pos + 2..].to_string();

                        // Parse SSE event
                        for line in event_text.lines() {
                            if let Some(data) = line.strip_prefix("data: ") {
                                // Check for stream end marker
                                if data.trim() == "[DONE]" {
                                    yield Ok(StreamEvent::Done);
                                    return;
                                }

                                // Parse JSON data
                                match serde_json::from_str::<Value>(data) {
                                    Ok(json) => {
                                        if let Some(event) = parse_stream_event(&json) {
                                            yield Ok(event);
                                        }
                                    }
                                    Err(e) => {
                                        yield Err(ApiError::Streaming(format!("JSON parse error: {}", e)));
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    yield Err(ApiError::Network(e));
                    return;
                }
            }
        }

        yield Ok(StreamEvent::Done);
    })
}

/// Parse a single stream event from JSON
fn parse_stream_event(json: &Value) -> Option<StreamEvent> {
    // Anthropic format
    if let Some(event_type) = json.get("type").and_then(|v| v.as_str()) {
        match event_type {
            "message_start" => return Some(StreamEvent::MessageStart),
            "message_stop" => return Some(StreamEvent::MessageStop),
            "content_block_delta" => {
                // Handle both text and thinking deltas
                if let Some(delta) = json.get("delta") {
                    // Try text first (normal content)
                    if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                        return Some(StreamEvent::ContentDelta(text.to_string()));
                    }
                    // Try thinking (Extended Thinking feature)
                    if let Some(thinking) = delta.get("thinking").and_then(|t| t.as_str()) {
                        return Some(StreamEvent::ContentDelta(thinking.to_string()));
                    }
                }
            }
            "error" => {
                let msg = json
                    .get("error")
                    .and_then(|e| e.get("message"))
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error");
                return Some(StreamEvent::Error(msg.to_string()));
            }
            _ => {}
        }
    }

    // OpenAI format
    if let Some(choices) = json.get("choices").and_then(|v| v.as_array()) {
        if let Some(choice) = choices.first() {
            if let Some(delta) = choice.get("delta") {
                if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                    return Some(StreamEvent::ContentDelta(content.to_string()));
                }
            }
            if let Some(finish_reason) = choice.get("finish_reason") {
                if !finish_reason.is_null() {
                    return Some(StreamEvent::MessageStop);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_anthropic_content_delta() {
        let json = json!({
            "type": "content_block_delta",
            "delta": {
                "type": "text_delta",
                "text": "Hello"
            }
        });

        let event = parse_stream_event(&json);
        assert_eq!(event, Some(StreamEvent::ContentDelta("Hello".to_string())));
    }

    #[test]
    fn test_parse_anthropic_thinking_delta() {
        let json = json!({
            "type": "content_block_delta",
            "delta": {
                "type": "thinking_delta",
                "thinking": "Let me think..."
            }
        });

        let event = parse_stream_event(&json);
        assert_eq!(event, Some(StreamEvent::ContentDelta("Let me think...".to_string())));
    }

    #[test]
    fn test_parse_anthropic_message_start() {
        let json = json!({
            "type": "message_start"
        });

        let event = parse_stream_event(&json);
        assert_eq!(event, Some(StreamEvent::MessageStart));
    }

    #[test]
    fn test_parse_anthropic_message_stop() {
        let json = json!({
            "type": "message_stop"
        });

        let event = parse_stream_event(&json);
        assert_eq!(event, Some(StreamEvent::MessageStop));
    }

    #[test]
    fn test_parse_openai_content_delta() {
        let json = json!({
            "choices": [
                {
                    "delta": {
                        "content": "Hello"
                    }
                }
            ]
        });

        let event = parse_stream_event(&json);
        assert_eq!(event, Some(StreamEvent::ContentDelta("Hello".to_string())));
    }

    #[test]
    fn test_parse_openai_finish() {
        let json = json!({
            "choices": [
                {
                    "finish_reason": "stop"
                }
            ]
        });

        let event = parse_stream_event(&json);
        assert_eq!(event, Some(StreamEvent::MessageStop));
    }
}
```

**Step 2: Run tests**

```bash
cargo test streaming::tests
```

Expected: All tests pass

**Step 3: Export from lib.rs**

File: `universal-api-client/src/lib.rs`

```rust
pub mod error;
pub mod provider;
pub mod streaming;
pub mod transform;

pub use error::{ApiError, Result};
pub use provider::{AuthInfo, AuthStrategy, ClaudeAdapter, OpenAiAdapter, ProviderAdapter};
pub use streaming::{parse_sse_stream, StreamEvent};
```

**Step 4: Verify compilation**

```bash
cargo check
```

Expected: Compiles successfully

**Step 5: Commit**

```bash
git add src/streaming.rs src/lib.rs
git commit -m "feat: add SSE streaming support for both Anthropic and OpenAI formats"
```

---

## Task 9-15: Remaining implementation tasks documented in separate continuation file
## Task 9: API Client Core

**Files:**
- Create: `universal-api-client/src/client.rs`
- Modify: `universal-api-client/src/lib.rs`

**Step 1: Write client builder with tests**

File: `universal-api-client/src/client.rs` (first 50 lines)

```rust
use crate::error::{ApiError, Result};
use crate::provider::{AuthInfo, AuthStrategy, ClaudeAdapter, ProviderAdapter};
use crate::streaming::{parse_sse_stream, StreamEvent};
use futures::stream::Stream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::pin::Pin;
use std::time::Duration;

/// API format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiFormat {
    Anthropic,
    OpenAI,
}

/// Chat response
#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub model: String,
    #[serde(default)]
    pub usage: Option<Usage>,
    #[serde(default)]
    pub stop_reason: Option<String>,
}

/// Token usage statistics
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

/// API Client
pub struct ApiClient {
    base_url: String,
    api_key: String,
    format: ApiFormat,
    model: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    client: Client,
    adapter: Box<dyn ProviderAdapter>,
}
## Summary

The implementation plan has been created with 8 detailed tasks covering:
- Project structure setup
- Error types
- Authentication module  
- Provider adapter trait
- Claude provider adapter
- OpenAI provider adapter
- Format transformation
- Streaming support

**Next steps:** Tasks 9-15 will cover ApiClient implementation, CLI tool, testing, and documentation.

**Execution:** Use superpowers:executing-plans skill to implement this plan task-by-task.
