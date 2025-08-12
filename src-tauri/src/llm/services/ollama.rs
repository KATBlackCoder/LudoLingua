//use log::debug;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::models::ModelOptions;
use ollama_rs::Ollama;
use serde::{Deserialize, Serialize};

use crate::core::error::{AppError, AppResult};
use crate::models::engine::EngineInfo;
use crate::models::provider::{LlmConfig, ModelInfo};
use crate::models::translation::TextUnit;
use crate::utils::prompts::builder::PromptBuilder;

/// JSON configuration structure for Ollama models
#[derive(Debug, Deserialize, Serialize)]
struct OllamaModelsConfig {
    models: Vec<ModelInfo>,
}

/// Ollama LLM service implementation using ollama-rs crate
#[derive(Debug)]
pub struct OllamaService {
    config: LlmConfig,
    client: Ollama,
}

impl OllamaService {
    /// Create a new Ollama service with the given configuration
    pub fn new(config: LlmConfig) -> AppResult<Self> {
        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| "http://localhost:11434".to_string());

        // Parse the base URL to extract host and port
        let url = base_url.replace("http://", "").replace("https://", "");
        let parts: Vec<&str> = url.split(':').collect();
        let host = parts[0].to_string();
        let port = if parts.len() > 1 {
            parts[1].parse::<u16>().unwrap_or(11434)
        } else {
            11434
        };

        let client = Ollama::new(format!("http://{}", host), port);

        Ok(Self { config, client })
    }

    /// Get the list of available models for Ollama from JSON configuration
    pub fn get_available_models() -> Vec<ModelInfo> {
        // Load embedded JSON configuration
        let json_content = include_str!("../../../models/ollama.json");

        match serde_json::from_str::<OllamaModelsConfig>(json_content) {
            Ok(config) => {
                // info!(
                //     "Successfully loaded {} Ollama models from JSON configuration",
                //     config.models.len()
                // );
                config.models
            }
            Err(_e) => {
                // warn!(
                //     "Failed to parse Ollama models JSON configuration: {}. Using fallback models.",
                //     _e
                // );
                Self::get_fallback_models()
            }
        }
    }

    /// Get fallback models when JSON configuration fails to load
    fn get_fallback_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                display_name: "Mistral 7B".to_string(),
                model_name: "mistral:latest".to_string(),
            },
            ModelInfo {
                display_name: "Llama 3.1".to_string(),
                model_name: "llama3.1".to_string(),
            },
        ]
    }

    /// Get default model configuration for Ollama
    #[allow(dead_code)]
    pub fn default_model() -> String {
        // Try to get the first model from JSON configuration, fallback to mistral
        Self::get_available_models()
            .first()
            .map(|m| m.model_name.clone())
            .unwrap_or_else(|| "mistral:latest".to_string())
    }

    /// Validate if a model name is supported by this provider
    #[allow(dead_code)]
    pub fn is_model_supported(model_name: &str) -> bool {
        Self::get_available_models()
            .iter()
            .any(|m| m.model_name == model_name)
    }

    /// Get the display name for the current model
    #[allow(dead_code)]
    pub fn get_model_display_name(&self) -> String {
        Self::get_available_models()
            .iter()
            .find(|m| m.model_name == self.config.model.model_name)
            .map(|m| m.display_name.clone())
            .unwrap_or_else(|| self.config.model.display_name.clone())
    }

    /// Low-level generate call: takes a fully-built prompt and returns the raw model string
    pub async fn generate(&self, prompt: &str) -> AppResult<String> {
        // Attach model options from config (temperature, max tokens)
        let options = ModelOptions::default()
            .temperature(self.config.temperature)
            .num_predict(self.config.max_tokens as i32);

        let request = GenerationRequest::new(
            self.config.model.model_name.clone(),
            prompt.to_string(),
        )
        .options(options);
        match self.client.generate(request).await {
            Ok(response) => Ok(response.response.trim().to_string()),
            Err(e) => Err(AppError::Llm(format!("Ollama generation failed: {}", e))),
        }
    }


    pub async fn test_connection(&self) -> AppResult<bool> {
        // debug!("Testing connection to Ollama");

        match self.client.list_local_models().await {
            Ok(_) => {
                // info!("Successfully connected to Ollama");
                Ok(true)
            }
            Err(_e) => {
                // error!("Failed to connect to Ollama: {}", e);
                Ok(false)
            }
        }
    }

    /// Legacy helper kept temporarily for compatibility. Prefer building the prompt
    /// in the caller and using `generate`.
    pub async fn translate(&self, text_unit: &TextUnit, engine_info: &EngineInfo) -> AppResult<String> {
        let prompt = PromptBuilder::build_translation_prompt(text_unit, engine_info).await;
        self.generate(&prompt).await
    }

    /// Check if the internal config matches another config
    pub fn config_matches(&self, other: &LlmConfig) -> bool {
        self.config.model == other.model && self.config.base_url == other.base_url
            && (self.config.temperature - other.temperature).abs() < f32::EPSILON
            && self.config.max_tokens == other.max_tokens
    }
}
