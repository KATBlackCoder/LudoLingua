use crate::llm::factory::create_service;
use crate::llm::services::ollama::OllamaService;
use crate::models::provider::{LlmConfig, ModelInfo};
use log::info;
use serde::Deserialize;

/// Test the LLM connection for a given config
pub async fn test_llm_connection(config: LlmConfig) -> Result<bool, String> {
    log::debug!(
        "Testing LLM connection for provider: '{}', model: '{}'",
        config.model.provider,
        config.model.model_name
    );
    // Create provider-specific service via factory
    let service = create_service(config).map_err(|e| e.to_string())?;
    service.test_connection().await.map_err(|e| e.to_string())
}

/// Loads available models for Ollama
pub async fn get_ollama_models() -> Result<Vec<ModelInfo>, String> {
    info!("Loading available models for Ollama");
    Ok(OllamaService::get_available_models())
}

#[derive(Debug, Deserialize)]
struct ModelCatalog {
    models: Vec<ModelInfo>,
}

/// Load models for a provider from bundled JSON files (e.g., models/ollama.json)
pub fn get_models(provider: String) -> Result<Vec<ModelInfo>, String> {
    log::debug!("Getting models for provider: '{}'", provider);
    let key = provider.to_lowercase();
    log::debug!("Provider key (lowercase): '{}'", key);

    let json = match key.as_str() {
        // Add more providers as their JSON files are added under src-tauri/models/
        "ollama" => include_str!("../../models/ollama.json"),
        "runpod" => include_str!("../../models/runpod.json"), // RunPod has its own model catalog
        _ => return Err(format!("unknown provider: {}", provider)),
    };

    let parsed: ModelCatalog = serde_json::from_str(json)
        .map_err(|e| format!("failed to parse models for {}: {}", provider, e))?;

    log::debug!("Parsed {} models from JSON", parsed.models.len());

    log::debug!(
        "Returning {} models for provider '{}'",
        parsed.models.len(),
        provider
    );
    Ok(parsed.models)
}
