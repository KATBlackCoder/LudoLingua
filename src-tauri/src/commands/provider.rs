use crate::llm::services::ollama::OllamaService;
use crate::models::provider::{LlmConfig, ModelInfo};
use log::info;

/// Test the LLM connection for a given config
pub async fn test_llm_connection(config: LlmConfig) -> Result<bool, String> {
    let service = match OllamaService::new(config) {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to create Ollama service: {}", e)),
    };
    service.test_connection().await.map_err(|e| e.to_string())
}

/// Loads available models for Ollama
pub async fn get_ollama_models() -> Result<Vec<ModelInfo>, String> {
    info!("Loading available models for Ollama");
    Ok(OllamaService::get_available_models())
}
