use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::ChatMessage,
    LLMProvider,
};

use crate::core::error::{AppError, AppResult};
use crate::core::provider::{GenerationResponse, LlmService, TokenUsage};
use crate::models::provider::LlmConfig;

/// Groq LLM service implementation using graniet/llm crate
pub struct GroqService {
    config: LlmConfig,
    client: Box<dyn LLMProvider>,
}

impl GroqService {
    pub fn new(config: LlmConfig) -> AppResult<Self> {
        let api_key = config
            .api_key
            .clone()
            .ok_or_else(|| AppError::Llm("Groq api_key is required".into()))?;
        
        let mut builder = LLMBuilder::new()
            .backend(LLMBackend::Groq)
            .api_key(api_key)
            .model(&config.model.model_name)
            .temperature(config.temperature)
            .max_tokens(config.max_tokens)
            .stream(false);

        // Groq uses a fixed base URL, but we can override if needed
        if let Some(url) = &config.base_url {
            let trimmed = url.trim();
            if !trimmed.is_empty() {
                builder = builder.base_url(trimmed.trim_end_matches('/').to_string());
            }
        }

        let client = builder
            .build()
            .map_err(|e| AppError::Llm(format!("Failed to create Groq client: {}", e)))?;

        Ok(Self { config, client })
    }

    async fn do_generate(&self, prompt: &str) -> AppResult<String> {
        let messages = vec![ChatMessage::user().content(prompt).build()];
        match self.client.chat(&messages).await {
            Ok(response) => Ok(format!("{}", response)),
            Err(e) => Err(AppError::Llm(format!("Groq generation failed: {}", e))),
        }
    }

    fn do_config_matches(&self, other: &LlmConfig) -> bool {
        self.config.model == other.model
            && self.config.api_key == other.api_key
            && (self.config.temperature - other.temperature).abs() < f32::EPSILON
            && self.config.max_tokens == other.max_tokens
    }
}

impl LlmService for GroqService {
    fn generate<'a>(&'a self, prompt: &'a str) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<String>> + Send + 'a>> {
        Box::pin(async move { self.do_generate(prompt).await })
    }

    fn generate_with_usage<'a>(&'a self, prompt: &'a str) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<GenerationResponse>> + Send + 'a>> {
        Box::pin(async move { 
            // Groq via graniet/llm crate doesn't provide token usage, so we fall back to text-only generation
            let content = self.do_generate(prompt).await?;
            Ok(GenerationResponse {
                content,
                token_usage: None, // graniet/llm doesn't expose usage stats for Groq
            })
        })
    }

    fn test_connection<'a>(&'a self) -> core::pin::Pin<Box<dyn core::future::Future<Output = AppResult<bool>> + Send + 'a>> {
        Box::pin(async move {
            log::debug!("Groq test_connection: calling do_generate('ping')");
            match self.do_generate("ping").await {
                Ok(resp) => {
                    log::debug!("Groq test_connection: do_generate succeeded with response: {}", resp);
                    Ok(true)
                }
                Err(AppError::Llm(e)) => {
                    log::debug!("Groq test_connection: do_generate failed with LLM error: {}", e);
                    let el = e.to_ascii_lowercase();
                    if el.contains("insufficient_quota") || el.contains("exceeded your current quota") {
                        log::debug!("Groq test_connection: insufficient quota detected, returning false");
                        return Ok(false);
                    }
                    if el.contains("429") {
                        log::debug!("Groq test_connection: 429 rate limit detected, treating as connected");
                        return Ok(true); // rate limit but reachable
                    }
                    if el.contains("http status: 200") || el.contains("200 ok") {
                        log::debug!("Groq test_connection: 200 OK parse error detected, treating as connected");
                        return Ok(true);
                    }
                    log::debug!("Groq test_connection: other LLM error, returning false");
                    Ok(false)
                }
                Err(other) => {
                    log::debug!("Groq test_connection: non-LLM error: {:?}, returning false", other);
                    Ok(false)
                }
            }
        })
    }

    fn config_matches(&self, other: &LlmConfig) -> bool {
        self.do_config_matches(other)
    }
}
