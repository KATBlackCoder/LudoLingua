use serde::{Deserialize, Serialize};
/// Pricing information for token usage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenPricing {
    /// USD per 1K input tokens
    pub input_price_per_1k: f64,
    /// USD per 1K output tokens
    pub output_price_per_1k: f64,
    /// Currency code (e.g., "USD")
    pub currency: String,
}

/// Model information with comprehensive details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelInfo {
    /// User-friendly display name for the model
    pub display_name: String,
    /// Actual model name used for API calls
    pub model_name: String,
    /// Provider (enum)
    pub provider: String,
    /// Brief description of the model's capabilities
    pub description: Option<String>,
    /// Token pricing information
    pub pricing: TokenPricing,
    /// Approximate context window size in tokens
    pub context_window: Option<u32>,
    /// Whether the model is available/enabled
    pub enabled: bool,
}

/// Simplified LLM configuration (Ollama-only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// The model to use
    pub model: ModelInfo,
    /// Base URL for the API endpoint (optional)
    pub base_url: Option<String>,
    /// API key for cloud providers (optional, not used by Ollama)
    pub api_key: Option<String>,
    /// Temperature for text generation (0.0 to 1.0)
    pub temperature: f32,
    /// Maximum number of tokens to generate
    pub max_tokens: u32,
}
