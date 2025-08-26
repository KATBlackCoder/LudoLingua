use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::{timeout, Duration};

use crate::core::error::{AppError, AppResult};
use crate::core::provider::{GenerationResponse, LlmService, TokenUsage};
use crate::models::provider::LlmConfig;

const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// OpenRouter API request structures following https://openrouter.ai/docs/api-reference/overview
#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<OpenRouterMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Serialize)]
struct OpenRouterMessage {
    role: String,
    content: String,
}

/// OpenRouter API response structures
#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<OpenRouterChoice>,
    #[serde(default)]
    usage: Option<OpenRouterUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoice {
    message: OpenRouterResponseMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponseMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenRouterUsage {
    #[serde(default)]
    prompt_tokens: u32,
    #[serde(default)]
    completion_tokens: u32,
    #[serde(default)]
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct OpenRouterError {
    error: OpenRouterErrorDetails,
}

#[derive(Debug, Deserialize)]
struct OpenRouterErrorDetails {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
    code: Option<String>,
}

/// OpenRouter LLM service implementation using direct HTTP calls
pub struct OpenRouterService {
    config: LlmConfig,
    client: Client,
    api_key: String,
}

impl OpenRouterService {
    pub fn new(config: LlmConfig) -> AppResult<Self> {
        let api_key = config
            .api_key
            .clone()
            .ok_or_else(|| AppError::Llm("OpenRouter API key is required".into()))?;

        let client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(|e| AppError::Llm(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            client,
            api_key,
        })
    }

    async fn do_generate(&self, prompt: &str) -> AppResult<String> {
        let request = OpenRouterRequest {
            model: self.config.model.model_name.clone(),
            messages: vec![OpenRouterMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: Some(self.config.max_tokens),
            temperature: Some(self.config.temperature),
            stream: Some(false),
        };

        let response = timeout(
            REQUEST_TIMEOUT,
            self.client
                .post(&format!("{}/chat/completions", OPENROUTER_BASE_URL))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .header("HTTP-Referer", "https://ludolingua.app") // Optional: for OpenRouter analytics
                .header("X-Title", "LudoLingua") // Optional: for OpenRouter analytics
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

        log::debug!("OpenRouter HTTP status: {}", status);
        log::debug!("OpenRouter response: {}", response_text);

        if !status.is_success() {
            // Try to parse as OpenRouter error format
            if let Ok(error_response) = serde_json::from_str::<OpenRouterError>(&response_text) {
                return Err(AppError::Llm(format!(
                    "OpenRouter API error: {} ({})",
                    error_response.error.message,
                    status
                )));
            }
            return Err(AppError::Llm(format!(
                "OpenRouter API error: HTTP {} - {}",
                status, response_text
            )));
        }

        let openrouter_response: OpenRouterResponse =
            serde_json::from_str(&response_text).map_err(|e| {
                AppError::Llm(format!(
                    "Failed to parse OpenRouter response: {} - Response: {}",
                    e, response_text
                ))
            })?;

        if openrouter_response.choices.is_empty() {
            return Err(AppError::Llm("No choices in OpenRouter response".into()));
        }

        Ok(openrouter_response.choices[0].message.content.clone())
    }

    async fn do_generate_with_usage(&self, prompt: &str) -> AppResult<GenerationResponse> {
        let request = OpenRouterRequest {
            model: self.config.model.model_name.clone(),
            messages: vec![OpenRouterMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: Some(self.config.max_tokens),
            temperature: Some(self.config.temperature),
            stream: Some(false),
        };

        let response = timeout(
            REQUEST_TIMEOUT,
            self.client
                .post(&format!("{}/chat/completions", OPENROUTER_BASE_URL))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .header("HTTP-Referer", "https://ludolingua.app") // Optional: for OpenRouter analytics
                .header("X-Title", "LudoLingua") // Optional: for OpenRouter analytics
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

        log::debug!("OpenRouter HTTP status: {}", status);
        log::debug!("OpenRouter response: {}", response_text);

        if !status.is_success() {
            // Try to parse as OpenRouter error format
            if let Ok(error_response) = serde_json::from_str::<OpenRouterError>(&response_text) {
                return Err(AppError::Llm(format!(
                    "OpenRouter API error: {} ({})",
                    error_response.error.message,
                    status
                )));
            }
            return Err(AppError::Llm(format!(
                "OpenRouter API error: HTTP {} - {}",
                status, response_text
            )));
        }

        let openrouter_response: OpenRouterResponse =
            serde_json::from_str(&response_text).map_err(|e| {
                AppError::Llm(format!(
                    "Failed to parse OpenRouter response: {} - Response: {}",
                    e, response_text
                ))
            })?;

        if openrouter_response.choices.is_empty() {
            return Err(AppError::Llm("No choices in OpenRouter response".into()));
        }

        let content = openrouter_response.choices[0].message.content.clone();
        
        // Extract token usage if available
        let token_usage = openrouter_response.usage.map(|usage| TokenUsage {
            input_tokens: usage.prompt_tokens,
            output_tokens: usage.completion_tokens,
            total_tokens: usage.total_tokens,
        });

        Ok(GenerationResponse {
            content,
            token_usage,
        })
    }

    async fn do_test_connection(&self) -> AppResult<bool> {
        // Use a minimal test prompt to verify connectivity
        match self.do_generate("Hi").await {
            Ok(_) => Ok(true),
            Err(AppError::Llm(e)) => {
                let error_lower = e.to_lowercase();
                
                // Check for quota/billing issues (should return false)
                if error_lower.contains("insufficient_quota")
                    || error_lower.contains("exceeded your current quota")
                    || error_lower.contains("billing")
                {
                    log::debug!("OpenRouter test_connection: quota/billing issue detected");
                    return Ok(false);
                }

                // Check for rate limiting (should return true - connection works)
                if error_lower.contains("429") || error_lower.contains("rate limit") {
                    log::debug!("OpenRouter test_connection: rate limited but connection works");
                    return Ok(true);
                }

                // Authentication errors (should return false)
                if error_lower.contains("401") || error_lower.contains("unauthorized") {
                    log::debug!("OpenRouter test_connection: authentication failed");
                    return Ok(false);
                }

                // Other errors (connection failed)
                log::debug!("OpenRouter test_connection: other error: {}", e);
                Ok(false)
            }
            Err(_) => Ok(false),
        }
    }

    fn do_config_matches(&self, other: &LlmConfig) -> bool {
        self.config.model == other.model
            && self.config.api_key == other.api_key
            && (self.config.temperature - other.temperature).abs() < f32::EPSILON
            && self.config.max_tokens == other.max_tokens
    }
}

impl LlmService for OpenRouterService {
    fn generate<'a>(
        &'a self,
        prompt: &'a str,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<String>> + Send + 'a>> {
        Box::pin(async move { self.do_generate(prompt).await })
    }

    fn generate_with_usage<'a>(
        &'a self,
        prompt: &'a str,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<GenerationResponse>> + Send + 'a>> {
        Box::pin(async move { self.do_generate_with_usage(prompt).await })
    }

    fn test_connection<'a>(
        &'a self,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<bool>> + Send + 'a>> {
        Box::pin(async move { self.do_test_connection().await })
    }

    fn config_matches(&self, other: &LlmConfig) -> bool {
        self.do_config_matches(other)
    }
}
