use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported LLM providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Ollama,
    //OpenAI,
    //Anthropic,
    //Groq,
    //LocalAI,
}

/// Model information containing display name and actual model name for provider connections
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelInfo {
    /// User-friendly display name for the model
    pub display_name: String,
    /// Actual model name used for API calls to the provider
    pub model_name: String,
}

/// Configuration for LLM providers - flexible for different service types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// The provider type
    pub provider: Provider,
    /// The model name to use
    pub model: ModelInfo,
    /// Base URL for the API endpoint (optional, service-specific)
    pub base_url: Option<String>,
    /// API key if required (optional, service-specific)
    pub api_key: Option<String>,
    /// Temperature for text generation (0.0 to 1.0)
    pub temperature: f32,
    /// Maximum number of tokens to generate
    pub max_tokens: u32,
    /// Additional service-specific configuration options
    pub extra_config: HashMap<String, String>,
}
