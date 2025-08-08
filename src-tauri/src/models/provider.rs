use serde::{Deserialize, Serialize};

/// Model information containing display name and actual model name for connections
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelInfo {
    /// User-friendly display name for the model
    pub display_name: String,
    /// Actual model name used for API calls
    pub model_name: String,
}

/// Simplified LLM configuration (Ollama-only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// The model to use
    pub model: ModelInfo,
    /// Base URL for the API endpoint (optional)
    pub base_url: Option<String>,
    /// Temperature for text generation (0.0 to 1.0)
    pub temperature: f32,
    /// Maximum number of tokens to generate
    pub max_tokens: u32,
}
