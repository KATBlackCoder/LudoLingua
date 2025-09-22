use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::ChatMessage,
    LLMProvider,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::{timeout, Duration};

use crate::core::error::{AppError, AppResult};
use crate::core::provider::{GenerationResponse, LlmService, TokenUsage};
use crate::models::provider::{LlmConfig, ModelInfo};

/// JSON configuration structure for Ollama models
#[derive(Debug, Deserialize, Serialize)]
struct OllamaModelsConfig {
    models: Vec<ModelInfo>,
}

/// Ollama API request for chat
#[derive(Debug, Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct OllamaChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
}

/// Ollama API response for chat
#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaChatResponseMessage,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
    #[serde(default)]
    #[allow(dead_code)]
    done: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaChatResponseMessage {
    #[allow(dead_code)]
    role: String,
    content: String,
}

/// Ollama LLM service implementation using llm crate
pub struct OllamaService {
    config: LlmConfig,
    client: Box<dyn LLMProvider>,
    http_client: Client,
}

impl OllamaService {
    /// Create a new Ollama service with the given configuration
    pub fn new(config: LlmConfig) -> AppResult<Self> {
        // Always use localhost for Ollama - no need for user configuration
        let base_url = "http://localhost:11434".to_string();

        // Create Ollama client using llm crate's builder pattern
        let client = LLMBuilder::new()
            .backend(LLMBackend::Ollama)
            .base_url(base_url.clone())
            .model(&config.model.model_name)
            .max_tokens(config.max_tokens)
            .temperature(config.temperature)
            .stream(false)
            .build()
            .map_err(|e| AppError::Llm(format!("Failed to create Ollama client: {}", e)))?;

        let http_client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| AppError::Llm(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            client,
            http_client,
        })
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
        use crate::models::provider::TokenPricing;

        vec![
            ModelInfo {
                display_name: "Mistral 7B".to_string(),
                model_name: "mistral:latest".to_string(),
                provider: "Ollama".to_string(),
                description: Some("A 7B parameter model trained by Mistral AI, good for general-purpose text generation and translation".to_string()),
                pricing: TokenPricing {
                    input_price_per_1k: 0.0,
                    output_price_per_1k: 0.0,
                    currency: "USD".to_string(),
                },
                context_window: Some(32768),
                enabled: true,
            },
            ModelInfo {
                display_name: "Llama 3.1".to_string(),
                model_name: "llama3.1".to_string(),
                provider: "Ollama".to_string(),
                description: Some("Meta's Llama 3.1 model with improved instruction following and reasoning capabilities".to_string()),
                pricing: TokenPricing {
                    input_price_per_1k: 0.0,
                    output_price_per_1k: 0.0,
                    currency: "USD".to_string(),
                },
                context_window: Some(131072),
                enabled: true,
            },
        ]
    }

    /// Low-level generate call: takes a fully-built prompt and returns the raw model string
    #[allow(dead_code)]
    pub async fn generate(&self, prompt: &str) -> AppResult<String> {
        // Create a simple user message for the prompt
        let messages = vec![ChatMessage::user().content(prompt).build()];

        match self.client.chat(&messages).await {
            Ok(response) => Ok(format!("{}", response)),
            Err(e) => Err(AppError::Llm(format!("Ollama generation failed: {}", e))),
        }
    }

    /// Generate text with usage information using direct Ollama API
    async fn do_generate_with_usage(&self, prompt: &str) -> AppResult<GenerationResponse> {
        // Always use localhost for Ollama - no need for user configuration
        let base_url = "http://localhost:11434".to_string();

        let request = OllamaChatRequest {
            model: self.config.model.model_name.clone(),
            messages: vec![OllamaChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            options: Some(OllamaOptions {
                temperature: Some(self.config.temperature),
                num_predict: Some(self.config.max_tokens),
            }),
            stream: false,
        };

        let response = timeout(
            Duration::from_secs(60),
            self.http_client
                .post(&format!("{}/api/chat", base_url))
                .header("Content-Type", "application/json")
                .json(&request)
                .send(),
        )
        .await
        .map_err(|_| AppError::Llm("Request timeout".into()))?
        .map_err(|e| AppError::Llm(format!("Request failed: {}", e)))?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(|e| AppError::Llm(format!("Failed to read response: {}", e)))?;

        log::debug!("Ollama HTTP status: {}", status);
        log::debug!("Ollama response: {}", response_text);

        if !status.is_success() {
            return Err(AppError::Llm(format!(
                "Ollama API error: HTTP {} - {}",
                status, response_text
            )));
        }

        let ollama_response: OllamaChatResponse =
            serde_json::from_str(&response_text).map_err(|e| {
                AppError::Llm(format!(
                    "Failed to parse Ollama response: {} - Response: {}",
                    e, response_text
                ))
            })?;

        let content = ollama_response.message.content;

        // Extract token usage if available
        let token_usage = match (
            ollama_response.prompt_eval_count,
            ollama_response.eval_count,
        ) {
            (Some(input), Some(output)) => Some(TokenUsage {
                input_tokens: input,
                output_tokens: output,
                total_tokens: input + output,
            }),
            _ => None,
        };

        Ok(GenerationResponse {
            content,
            token_usage,
        })
    }

    pub async fn test_connection(&self) -> AppResult<bool> {
        // Test connection with a simple chat message
        let messages = vec![ChatMessage::user().content("test").build()];

        match self.client.chat(&messages).await {
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

    /// Check if the internal config matches another config
    pub fn config_matches(&self, other: &LlmConfig) -> bool {
        self.config.model == other.model
            && (self.config.temperature - other.temperature).abs() < f32::EPSILON
            && self.config.max_tokens == other.max_tokens
            // base_url is always localhost for Ollama, so no need to compare
    }
}

impl LlmService for OllamaService {
    fn generate<'a>(
        &'a self,
        prompt: &'a str,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<String>> + Send + 'a>> {
        Box::pin(async move { self.generate(prompt).await })
    }

    fn generate_with_usage<'a>(
        &'a self,
        prompt: &'a str,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = AppResult<GenerationResponse>> + Send + 'a>,
    > {
        Box::pin(async move {
            // Try to get usage data directly from Ollama API
            self.do_generate_with_usage(prompt).await
        })
    }

    fn test_connection<'a>(
        &'a self,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<bool>> + Send + 'a>> {
        Box::pin(async move { self.test_connection().await })
    }

    fn config_matches(&self, other: &LlmConfig) -> bool {
        self.config_matches(other)
    }
}
