use crate::core::error::{AppError, AppResult};
use crate::core::provider::LlmProvider;
use crate::llm::services::ollama::OllamaProvider;
use crate::models::provider::{LlmConfig, ModelInfo, Provider};
use std::sync::Arc;

/// Factory for creating LLM providers
pub struct LlmFactory;

impl LlmFactory {
    /// Create an LLM provider based on the configuration
    pub fn create_provider(config: LlmConfig) -> AppResult<Arc<dyn LlmProvider>> {
        match config.provider {
            Provider::Ollama => {
                let provider = OllamaProvider::new(config)?;
                Ok(Arc::new(provider))
            }
            _ => Err(AppError::Llm(format!(
                "Unsupported LLM provider: {:?}",
                config.provider
            ))),
        }
    }

    /// Get list of supported providers
    pub fn get_available_providers() -> Vec<Provider> {
        vec![Provider::Ollama]
    }

    /// Get available models for a given provider (must be update and use in commands/provider.rs)
    pub fn get_available_models(provider: Provider) -> Vec<ModelInfo> {
        match provider {
            Provider::Ollama => OllamaProvider::get_available_models(),
            //_ => vec![], // Add more providers here as needed
        }
    }
}
