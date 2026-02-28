//! Universal API Client - Unified interface for Anthropic and OpenAI-compatible APIs
//!
//! This is a minimal stub implementation. Full implementation coming soon.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("not implemented yet")]
    NotImplemented,
}

/// API format type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiFormat {
    Anthropic,
    OpenAI,
}

/// Minimal stub - full implementation coming in Phase 2
pub struct ApiClient;

impl ApiClient {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}
