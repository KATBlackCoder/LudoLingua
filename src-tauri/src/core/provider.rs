use async_trait::async_trait;
use std::any::Any;

use crate::core::error::AppResult;
use crate::models::engine::EngineInfo;
use crate::models::translation::TextUnit;

/// Core trait that all LLM provider implementations must implement.
/// This defines the contract for interacting with different LLM services.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Test connection to the LLM provider
    async fn test_connection(&self) -> AppResult<bool>;

    /// Translate text using this provider
    ///
    /// # Arguments
    ///
    /// * `text_unit` - The text unit to translate
    /// * `engine_info` - The engine info containing language context
    ///
    /// # Returns
    ///
    /// * `AppResult<String>` - The translated text
    async fn translate(&self, text_unit: &TextUnit, engine_info: &EngineInfo) -> AppResult<String>;

    /// Returns self as Any for downcasting to specific provider implementations
    fn as_any(&self) -> &dyn Any;

    // ---
    // Static/associated contract (not enforced by Rust, but required for all providers):
    //
    // /// Get available models for this provider
    // fn get_available_models() -> Vec<crate::models::provider::ModelInfo>;
    //
    // /// Get available providers (if this is a factory/registry)
    // fn get_available_providers() -> Vec<crate::models::provider::Provider>;
    // ---
}
