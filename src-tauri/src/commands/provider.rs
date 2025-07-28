use crate::llm::factory::LlmFactory;
use crate::models::provider::{LlmConfig, ModelInfo, Provider};
use log::{error, info};
use serde_json;

/// Test the LLM connection for a given config
pub async fn test_llm_connection(config: LlmConfig) -> Result<bool, String> {
    let provider = match LlmFactory::create_provider(config) {
        Ok(p) => p,
        Err(e) => return Err(format!("Failed to create provider: {}", e)),
    };
    provider.test_connection().await.map_err(|e| e.to_string())
}

/// Loads available models for a given provider (e.g., "ollama").
/// Returns a Vec<ModelInfo> or an error string.
pub async fn get_available_models(provider: String) -> Result<Vec<ModelInfo>, String> {
    info!("Loading available models for provider: {}", provider);

    // Convert string to Provider enum using serde_json for case-insensitive matching
    let provider_enum: Provider = match serde_json::from_str(&format!("\"{}\"", provider)) {
        Ok(p) => p,
        Err(_) => {
            error!("Unknown provider: {}", provider);
            return Err(format!("Unknown provider: {}", provider));
        }
    };

    Ok(LlmFactory::get_available_models(provider_enum))
}

/// Returns a list of all supported provider names as strings.
pub async fn get_available_providers() -> Result<Vec<String>, String> {
    Ok(LlmFactory::get_available_providers()
        .into_iter()
        .map(|p| format!("{:?}", p).to_lowercase())
        .collect())
}
