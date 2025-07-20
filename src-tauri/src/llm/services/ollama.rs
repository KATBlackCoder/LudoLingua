use async_trait::async_trait;
use log::{debug, error, info, warn};
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fs;
use std::path::Path;

use crate::core::error::{AppError, AppResult};
use crate::core::provider::LlmProvider;
use crate::models::engine::EngineInfo;
use crate::models::provider::{LlmConfig, ModelInfo};
use crate::models::translation::{PromptType, TextUnit};

/// JSON configuration structure for Ollama models
#[derive(Debug, Deserialize, Serialize)]
struct OllamaModelsConfig {
    models: Vec<ModelInfo>,
}

/// Ollama LLM provider implementation using ollama-rs crate
#[derive(Debug)]
pub struct OllamaProvider {
    config: LlmConfig,
    client: Ollama,
}

impl OllamaProvider {
    /// Create a new Ollama provider with the given configuration
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
                info!(
                    "Successfully loaded {} Ollama models from JSON configuration",
                    config.models.len()
                );
                config.models
            }
            Err(e) => {
                warn!(
                    "Failed to parse Ollama models JSON configuration: {}. Using fallback models.",
                    e
                );
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
    pub fn default_model() -> String {
        // Try to get the first model from JSON configuration, fallback to mistral
        Self::get_available_models()
            .first()
            .map(|m| m.model_name.clone())
            .unwrap_or_else(|| "mistral:latest".to_string())
    }

    /// Validate if a model name is supported by this provider
    pub fn is_model_supported(model_name: &str) -> bool {
        Self::get_available_models()
            .iter()
            .any(|m| m.model_name == model_name)
    }

    /// Get the display name for the current model
    pub fn get_model_display_name(&self) -> String {
        Self::get_available_models()
            .iter()
            .find(|m| m.model_name == self.config.model.model_name)
            .map(|m| m.display_name.clone())
            .unwrap_or_else(|| self.config.model.display_name.clone())
    }

    /// Build a translation prompt based on the text unit, engine info, and glossary terms
    fn build_translation_prompt(
        &self,
        text_unit: &TextUnit,
        engine_info: &EngineInfo,
        glossary_terms: Option<&[(String, String)]>,
    ) -> String {
        // Load basic template
        let basic_template = match self.load_prompt_template("prompts/basic.txt") {
            Ok(template) => template,
            Err(e) => {
                error!("Failed to load basic template: {}", e);
                return self.build_fallback_prompt(text_unit, engine_info, glossary_terms);
            }
        };

        // Load specific template based on prompt type
        let specific_template = match text_unit.prompt_type {
            PromptType::Name => "prompts/character.txt",
            PromptType::Description => "prompts/description.txt",
            PromptType::Dialogue => "prompts/dialogue.txt",
            PromptType::Item => "prompts/item.txt",
            PromptType::Skill => "prompts/skill.txt",
            PromptType::Other => "prompts/other.txt",
        };

        let specific_content = match self.load_prompt_template(specific_template) {
            Ok(content) => content,
            Err(e) => {
                error!(
                    "Failed to load specific template {}: {}",
                    specific_template, e
                );
                String::new()
            }
        };

        // Combine templates
        let mut template = basic_template;
        template.push_str("\n\n");
        template.push_str(&specific_content);

        // Build glossary section
        let glossary_section = if let Some(terms) = glossary_terms {
            if !terms.is_empty() {
                let mut section = String::from("Glossary terms to use:\n");
                for (source, target) in terms {
                    section.push_str(&format!("- {} → {}\n", source, target));
                }
                section
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Replace template variables
        template
            .replace(
                "{source_language}",
                &engine_info.source_language.native_name,
            )
            .replace(
                "{target_language}",
                &engine_info.target_language.native_name,
            )
            .replace("{context}", "RPG Maker game content")
            .replace("{glossary_section}", &glossary_section)
            .replace("{text}", &text_unit.source_text)
    }

    /// Load a prompt template from the filesystem
    fn load_prompt_template(&self, template_path: &str) -> AppResult<String> {
        let path = Path::new(template_path);
        fs::read_to_string(path).map_err(|e| {
            AppError::FileSystem(format!(
                "Failed to read prompt template {}: {}",
                template_path, e
            ))
        })
    }

    /// Build a fallback prompt when template loading fails
    fn build_fallback_prompt(
        &self,
        text_unit: &TextUnit,
        engine_info: &EngineInfo,
        glossary_terms: Option<&[(String, String)]>,
    ) -> String {
        let mut prompt = String::from(
            "You are a professional translator specializing in game localization.\n\n",
        );

        prompt.push_str(&format!(
            "Translate the following text from {} to {}:\n\n",
            engine_info.source_language.native_name, engine_info.target_language.native_name
        ));

        // Add glossary terms if provided
        if let Some(terms) = glossary_terms {
            if !terms.is_empty() {
                prompt.push_str("Use these glossary terms for consistency:\n");
                for (source, target) in terms {
                    prompt.push_str(&format!("- {} → {}\n", source, target));
                }
                prompt.push_str("\n");
            }
        }

        prompt.push_str(&format!(
            "Text to translate: {}\n\nTranslation:",
            text_unit.source_text
        ));
        prompt
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn test_connection(&self) -> AppResult<bool> {
        debug!("Testing connection to Ollama");

        match self.client.list_local_models().await {
            Ok(_) => {
                info!("Successfully connected to Ollama");
                Ok(true)
            }
            Err(e) => {
                error!("Failed to connect to Ollama: {}", e);
                Ok(false)
            }
        }
    }

    async fn translate(
        &self,
        text_unit: &TextUnit,
        engine_info: &EngineInfo,
        glossary_terms: Option<&[(String, String)]>,
    ) -> AppResult<String> {
        debug!(
            "Translating text with Ollama: {} -> {}",
            engine_info.source_language.id, engine_info.target_language.id
        );

        // Build the translation prompt
        let prompt = self.build_translation_prompt(text_unit, engine_info, glossary_terms);

        // Create the generation request
        let request = GenerationRequest::new(self.config.model.model_name.clone(), prompt);

        // Generate the translation
        match self.client.generate(request).await {
            Ok(response) => {
                let translated_text = response.response.trim().to_string();
                info!("Successfully translated text using Ollama");
                Ok(translated_text)
            }
            Err(e) => {
                error!("Failed to translate text with Ollama: {}", e);
                Err(AppError::Llm(format!("Ollama translation failed: {}", e)))
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
