use crate::core::error::AppResult;
use crate::core::provider::{LlmService, ProviderKind};
use crate::llm::services::ollama::OllamaService;
use crate::llm::services::runpod::RunPodService;
use crate::models::provider::LlmConfig;

/// Create a concrete LLM service based on the provider kind found in the
/// model metadata. Currently supports Ollama and RunPod.
pub fn create_service(config: LlmConfig) -> AppResult<Box<dyn LlmService>> {
    // Debug: Log the provider name being processed
    log::debug!("Creating service for provider: '{}' (lowercase: '{}')",
                config.model.provider, config.model.provider.to_lowercase());

    // Only support implemented providers
    let provider = match config.model.provider.to_lowercase().as_str() {
        "ollama" => ProviderKind::Ollama,
        "runpod" => ProviderKind::Runpod,
        _ => ProviderKind::Ollama, // Default to Ollama for unknown providers
    };

    let service: AppResult<Box<dyn LlmService>> = match provider {
        ProviderKind::Ollama => {
            log::debug!("Creating OllamaService");
            Ok(Box::new(OllamaService::new(config)?))
        },
        ProviderKind::Runpod => {
            log::debug!("Creating RunPodService");
            Ok(Box::new(RunPodService::new(config)?))
        },
    };

    log::debug!("Service created successfully for provider: {:?}", provider);
    service
}

