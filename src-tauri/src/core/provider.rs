use serde::{Deserialize, Serialize};

use crate::core::error::AppResult;

/// Supported LLM providers.
///
/// This enum is used in configuration and model metadata to select the
/// appropriate concrete service implementation at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKind {
    Ollama,
    Runpod,
}

/// Generation response with token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    /// The generated text content
    pub content: String,
    /// Token usage information (if available from provider)
    pub token_usage: Option<TokenUsage>,
}

/// Token usage information from LLM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Input tokens consumed
    pub input_tokens: u32,
    /// Output tokens generated
    pub output_tokens: u32,
    /// Total tokens used
    pub total_tokens: u32,
}

/// Minimal provider-agnostic interface used by the command layer.
///
/// Concrete implementations live under `crate::llm::services::*`.
pub trait LlmService: Send + Sync {
    /// Execute a prompt and return the raw model output.
    fn generate<'a>(
        &'a self,
        prompt: &'a str,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<String>> + Send + 'a>>;

    /// Execute a prompt and return the model output with token usage information.
    fn generate_with_usage<'a>(
        &'a self,
        prompt: &'a str,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = AppResult<GenerationResponse>> + Send + 'a>,
    >;

    /// Health check or lightweight connectivity test.
    fn test_connection<'a>(
        &'a self,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<bool>> + Send + 'a>>;

    /// Check whether the internal config matches the provided one (used for reuse).
    fn config_matches(&self, other: &crate::models::provider::LlmConfig) -> bool;
}
