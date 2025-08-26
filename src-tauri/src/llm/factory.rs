use crate::core::error::AppResult;
use crate::core::provider::{LlmService, ProviderKind};
use crate::llm::services::ollama::OllamaService;
use crate::llm::services::openai::OpenAiService;
use crate::llm::services::openrouter::OpenRouterService;
use crate::llm::services::runpod::RunPodService;
use crate::llm::services::groq::GroqService;
use crate::models::provider::LlmConfig;

/// Create a concrete LLM service based on the provider kind found in the
/// model metadata. For now only Ollama is implemented; others will be added
/// by introducing `openai.rs`, `deepl.rs`, etc. under `llm/services/`.
pub fn create_service(config: LlmConfig) -> AppResult<Box<dyn LlmService>> {
    // Debug: Log the provider name being processed
    log::debug!("Creating service for provider: '{}' (lowercase: '{}')", 
                config.model.provider, config.model.provider.to_lowercase());
    
    // Prefer provider from config.model if present; default to Ollama for now
    let provider = match config.model.provider.to_lowercase().as_str() {
        "ollama" => ProviderKind::Ollama,
        "openai" => ProviderKind::Openai,
        "openrouter" => ProviderKind::Openrouter,
        "runpod" => ProviderKind::Runpod,
        "groq" => ProviderKind::Groq,
        "deepl" => ProviderKind::Deepl,
        "anthropic" => ProviderKind::Anthropic,
        "google" => ProviderKind::Google,
        "mistral" => ProviderKind::Mistral,
        "cohere" => ProviderKind::Cohere,
        _ => ProviderKind::Ollama,
    };

    let service: AppResult<Box<dyn LlmService>> = match provider {
        ProviderKind::Ollama => {
            log::debug!("Creating OllamaService");
            Ok(Box::new(OllamaService::new(config)?))
        },
        ProviderKind::Openai => {
            log::debug!("Creating OpenAiService");
            Ok(Box::new(OpenAiService::new(config)?))
        },
        ProviderKind::Openrouter => {
            log::debug!("Creating OpenRouterService");
            Ok(Box::new(OpenRouterService::new(config)?))
        },
        ProviderKind::Runpod => {
            log::debug!("Creating RunPodService");
            Ok(Box::new(RunPodService::new(config)?))
        },
        ProviderKind::Groq => {
            log::debug!("Creating GroqService");
            Ok(Box::new(GroqService::new(config)?))
        },
        ProviderKind::Deepl
        | ProviderKind::Anthropic
        | ProviderKind::Google
        | ProviderKind::Mistral
        | ProviderKind::Cohere => Err(crate::core::error::AppError::Llm(
            "Provider not yet implemented".into(),
        )),
    };
    
    log::debug!("Service created successfully for provider: {:?}", provider);
    service
}

