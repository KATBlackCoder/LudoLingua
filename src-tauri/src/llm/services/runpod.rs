use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::{timeout, Duration};

use crate::core::error::{AppError, AppResult};
use crate::core::provider::{GenerationResponse, LlmService, TokenUsage};
use crate::models::provider::{LlmConfig, ModelInfo};

/// JSON configuration structure for RunPod models (uses Ollama models)
#[derive(Debug, Deserialize, Serialize)]
struct RunPodModelsConfig {
    models: Vec<ModelInfo>,
}

/// RunPod API request for chat (same as Ollama)
#[derive(Debug, Serialize)]
struct RunPodChatRequest {
    model: String,
    messages: Vec<RunPodChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<RunPodOptions>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct RunPodChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct RunPodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
}

/// RunPod API response for chat (same as Ollama)
#[derive(Debug, Deserialize)]
struct RunPodChatResponse {
    message: RunPodChatResponseMessage,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
    #[serde(default)]
    #[allow(dead_code)]
    done: bool,
}

#[derive(Debug, Deserialize)]
struct RunPodChatResponseMessage {
    #[allow(dead_code)]
    role: String,
    content: String,
}

/// RunPod LLM service implementation using RunPod's Ollama deployment
pub struct RunPodService {
    config: LlmConfig,
    http_client: Client,
}

impl RunPodService {
    /// Create a new RunPod service with the given configuration
    pub fn new(config: LlmConfig) -> AppResult<Self> {
        // Validate RunPod URL format
        Self::format_runpod_url(&config)?;

        let http_client = Client::builder()
            .timeout(Duration::from_secs(120)) // Longer timeout for cloud
            .build()
            .map_err(|e| AppError::Llm(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Convert RunPod pod ID to proper API endpoint URL
    fn format_runpod_url(config: &LlmConfig) -> AppResult<String> {
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| AppError::Llm("RunPod requires base_url with pod ID".into()))?;

        // Handle different RunPod URL formats
        let formatted_url = if base_url.contains("runpod.net") {
            // Already a full RunPod URL
            base_url.clone()
        } else if base_url.contains("-11434.proxy.runpod.net") {
            // Already formatted proxy URL
            format!("https://{}", base_url)
        } else {
            // Assume it's a pod ID and format accordingly
            let pod_id = base_url.trim();
            if pod_id.is_empty() {
                return Err(AppError::Llm("Empty RunPod pod ID".into()));
            }
            format!("https://{}-11434.proxy.runpod.net", pod_id)
        };

        // Ensure URL doesn't end with a slash to avoid double slashes
        Ok(formatted_url.trim_end_matches('/').to_string())
    }

    /// Get the list of available models for RunPod (uses Ollama models)
    pub fn get_available_models() -> Vec<ModelInfo> {
        // For RunPod, we use the same models as Ollama
        use crate::llm::services::ollama::OllamaService;
        OllamaService::get_available_models()
    }

    /// Generate text with usage information using direct RunPod API
    async fn do_generate_with_usage(&self, prompt: &str) -> AppResult<GenerationResponse> {
        log::debug!(
            "RunPod: Starting generation with prompt length: {}",
            prompt.len()
        );
        let base_url = Self::format_runpod_url(&self.config)?;
        log::debug!("RunPod: Using base URL: {}", base_url);

        let request = RunPodChatRequest {
            model: self.config.model.model_name.clone(),
            messages: vec![RunPodChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            options: Some(RunPodOptions {
                temperature: Some(self.config.temperature),
                num_predict: Some(self.config.max_tokens),
            }),
            stream: false,
        };

        log::debug!("RunPod: Making POST request to {}/api/chat", base_url);
        let response = timeout(
            Duration::from_secs(120),
            self.http_client
                .post(&format!("{}/api/chat", base_url))
                .header("Content-Type", "application/json")
                .json(&request)
                .send(),
        )
        .await
        .map_err(|_| AppError::Llm("Request timeout - RunPod server may be busy".into()))?
        .map_err(|e| AppError::Llm(format!("Request failed: {}", e)))?;

        // Get status before reading response body
        let status = response.status();
        log::debug!("RunPod HTTP status: {}", status);

        let response_text = response
            .text()
            .await
            .map_err(|e| AppError::Llm(format!("Failed to read response: {}", e)))?;

        log::debug!("RunPod response: {}", response_text);

        if !status.is_success() {
            return Err(AppError::Llm(format!(
                "RunPod API error: HTTP {} - {}",
                status, response_text
            )));
        }

        let runpod_response: RunPodChatResponse =
            serde_json::from_str(&response_text).map_err(|e| {
                AppError::Llm(format!(
                    "Failed to parse RunPod response: {} - Response: {}",
                    e, response_text
                ))
            })?;

        let content = runpod_response.message.content;

        // Extract token usage if available
        let token_usage = match (
            runpod_response.prompt_eval_count,
            runpod_response.eval_count,
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

    /// Test connection to RunPod service
    pub async fn do_test_connection(&self) -> AppResult<bool> {
        let base_url = Self::format_runpod_url(&self.config)?;

        // Test with a simple tags endpoint
        let response_result = timeout(
            Duration::from_secs(30),
            self.http_client
                .get(&format!("{}/api/tags", base_url))
                .send(),
        )
        .await;

        match response_result {
            Ok(Ok(response)) => {
                let status = response.status();
                if status.is_success() {
                    log::debug!("Successfully connected to RunPod service");
                    Ok(true)
                } else {
                    log::debug!("Failed to connect to RunPod: HTTP {}", status);
                    Ok(false)
                }
            }
            Ok(Err(e)) => {
                log::debug!("HTTP request failed: {}", e);
                Ok(false)
            }
            Err(_) => {
                log::debug!("Connection timeout to RunPod");
                Ok(false)
            }
        }
    }

    /// Check if the internal config matches another config
    pub fn config_matches(&self, other: &LlmConfig) -> bool {
        self.config.model == other.model
            && self.config.base_url == other.base_url
            && (self.config.temperature - other.temperature).abs() < f32::EPSILON
            && self.config.max_tokens == other.max_tokens
    }
}

impl LlmService for RunPodService {
    fn generate<'a>(
        &'a self,
        prompt: &'a str,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<String>> + Send + 'a>> {
        Box::pin(async move {
            let response = self.do_generate_with_usage(prompt).await?;
            Ok(response.content)
        })
    }

    fn generate_with_usage<'a>(
        &'a self,
        prompt: &'a str,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = AppResult<GenerationResponse>> + Send + 'a>,
    > {
        Box::pin(async move { self.do_generate_with_usage(prompt).await })
    }

    fn test_connection<'a>(
        &'a self,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<bool>> + Send + 'a>> {
        Box::pin(async move { self.do_test_connection().await })
    }

    fn config_matches(&self, other: &LlmConfig) -> bool {
        self.config_matches(other)
    }
}
