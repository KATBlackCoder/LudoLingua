use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::ChatMessage,
    LLMProvider,
};

use crate::core::error::{AppError, AppResult};
use crate::core::provider::{GenerationResponse, LlmService, TokenUsage};
use crate::models::provider::LlmConfig;

/// OpenAI LLM service implementation using graniet/llm crate
pub struct OpenAiService {
    config: LlmConfig,
    client: Box<dyn LLMProvider>,
}

impl OpenAiService {
    pub fn new(config: LlmConfig) -> AppResult<Self> {
        let api_key = config
            .api_key
            .clone()
            .ok_or_else(|| AppError::Llm("OpenAI api_key is required".into()))?;
        let mut builder = LLMBuilder::new()
            .backend(LLMBackend::OpenAI)
            .api_key(api_key)
            .model(&config.model.model_name)
            .temperature(config.temperature)
            .max_tokens(config.max_tokens)
            .stream(false);

        if let Some(url) = &config.base_url {
            let trimmed = url.trim();
            if !trimmed.is_empty() {
                // Different handling for OpenRouter vs OpenAI
                let normalized = if trimmed.contains("openrouter.ai") {
                    // For OpenRouter, ensure we have the complete /api/v1 path
                    // The graniet/llm crate will append /chat/completions to this
                    let base = trimmed.trim_end_matches('/');
                    if base.ends_with("/api/v1") {
                        base.to_string() // Keep complete path
                    } else if base.ends_with("/api") {
                        format!("{}/v1", base) // Add /v1
                    } else {
                        format!("{}/api/v1", base) // Add complete /api/v1
                    }
                } else {
                    // For OpenAI, strip /v1 as the client will add it back
                    let mut normalized = trimmed.trim_end_matches('/').to_string();
                    if normalized.ends_with("/v1") {
                        normalized = normalized[..normalized.len()-3].trim_end_matches('/').to_string();
                    }
                    normalized
                };
                builder = builder.base_url(normalized);
            }
        }

        let client = builder
            .build()
            .map_err(|e| AppError::Llm(format!("Failed to create OpenAI client: {}", e)))?;

        Ok(Self { config, client })
    }
    async fn do_generate(&self, prompt: &str) -> AppResult<String> {
        let messages = vec![ChatMessage::user().content(prompt).build()];
        match self.client.chat(&messages).await {
            Ok(response) => Ok(format!("{}", response)),
            Err(e) => Err(AppError::Llm(format!("OpenAI generation failed: {}", e))),
        }
    }

    // do_test_connection removed; test_connection uses do_generate for Send-safe future

    fn do_config_matches(&self, other: &LlmConfig) -> bool {
        self.config.model == other.model
            && self.config.api_key == other.api_key
            && (self.config.temperature - other.temperature).abs() < f32::EPSILON
            && self.config.max_tokens == other.max_tokens
    }
}

impl LlmService for OpenAiService {
    fn generate<'a>(&'a self, prompt: &'a str) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<String>> + Send + 'a>> {
        Box::pin(async move { self.do_generate(prompt).await })
    }

    fn generate_with_usage<'a>(&'a self, prompt: &'a str) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<GenerationResponse>> + Send + 'a>> {
        Box::pin(async move { 
            // OpenAI via graniet/llm crate doesn't provide token usage, so we fall back to text-only generation
            let content = self.do_generate(prompt).await?;
            Ok(GenerationResponse {
                content,
                token_usage: None, // graniet/llm doesn't expose usage stats
            })
        })
    }

    fn test_connection<'a>(&'a self) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<bool>> + Send + 'a>> {
        // Avoid non-Send ChatResponse by delegating to do_generate (returns String)
        Box::pin(async move {
            log::debug!("OpenAI test_connection: calling do_generate('ping')");
            match self.do_generate("ping").await {
                Ok(resp) => {
                    log::debug!("OpenAI test_connection: do_generate succeeded with response: {}", resp);
                    Ok(true)
                }
                Err(AppError::Llm(e)) => {
                    log::debug!("OpenAI test_connection: do_generate failed with LLM error: {}", e);
                    let el = e.to_ascii_lowercase();
                    if el.contains("insufficient_quota") || el.contains("exceeded your current quota") {
                        log::debug!("OpenAI test_connection: insufficient quota detected, returning false");
                        return Ok(false);
                    }
                    if el.contains("429") {
                        log::debug!("OpenAI test_connection: 429 rate limit detected, treating as connected");
                        return Ok(true); // rate limit but reachable
                    }
                    // Some gateways (e.g., OpenRouter) can return HTTP 200 with a body parse error; treat as reachable
                    if el.contains("http status: 200") || el.contains("200 ok") {
                        log::debug!("OpenAI test_connection: 200 OK parse error detected, treating as connected");
                        return Ok(true);
                    }
                    log::debug!("OpenAI test_connection: other LLM error, returning false");
                    Ok(false)
                }
                Err(other) => {
                    log::debug!("OpenAI test_connection: non-LLM error: {:?}, returning false", other);
                    Ok(false)
                }
            }
        })
    }

    fn config_matches(&self, other: &LlmConfig) -> bool {
        self.do_config_matches(other)
    }
}
