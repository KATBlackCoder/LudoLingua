use crate::core::error::AppResult;
use crate::models::engine::EngineInfo;
use crate::models::translation::{PromptType, TextUnit};
use crate::models::provider::LlmConfig;
use crate::utils::prompts::builder::PromptBuilder;
use serde::{Deserialize, Serialize};

/// Token estimation for different LLM providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenEstimate {
    /// Input tokens (prompt + source text + glossary)
    pub input_tokens: u32,
    /// Estimated output tokens (translated text)
    pub output_tokens: u32,
    /// Total tokens (input + output)
    pub total_tokens: u32,
}

/// Token estimation for the entire project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTokenEstimate {
    /// Total input tokens for all text units
    pub total_input_tokens: u32,
    /// Total estimated output tokens for all text units
    pub total_output_tokens: u32,
    /// Grand total tokens
    pub total_tokens: u32,
    /// Number of text units to translate
    pub text_units_count: u32,
    /// Breakdown by prompt type
    pub by_prompt_type: Vec<PromptTypeEstimate>,
}

/// Token estimation breakdown by prompt type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTypeEstimate {
    pub prompt_type: PromptType,
    pub count: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

/// Simple token estimation using character count approximation
/// This is a rough estimate that accounts for language-specific tokenization patterns
pub fn estimate_tokens_from_text(text: &str) -> u32 {
    let char_count = text.chars().count() as f32;
    
    // Detect language composition and adjust ratio
    let japanese_chars = text.chars().filter(|c| {
        // Hiragana, Katakana, and CJK Unified Ideographs
        (*c >= '\u{3040}' && *c <= '\u{309F}') ||  // Hiragana
        (*c >= '\u{30A0}' && *c <= '\u{30FF}') ||  // Katakana
        (*c >= '\u{4E00}' && *c <= '\u{9FAF}')     // CJK Ideographs
    }).count() as f32;
    
    let japanese_ratio = japanese_chars / char_count;
    
    // More realistic character-to-token ratios based on actual tokenizer behavior
    let chars_per_token = if japanese_ratio > 0.5 {
        // Mostly Japanese: more realistic ratio for Llama tokenizer
        1.8
    } else if japanese_ratio > 0.1 {
        // Mixed content: compromise ratio
        2.5
    } else {
        // Mostly English/Latin: standard ratio
        3.5
    };
    
    (char_count / chars_per_token).ceil() as u32
}

/// Provider-specific calibration factors for token estimation
/// These factors account for different tokenizer behaviors and API overhead
pub fn get_provider_calibration_factor(provider: &str) -> f64 {
    match provider.to_lowercase().as_str() {
        "ollama" | "groq" => 1.6,   // Llama tokenizer - adjusted down from 1.85
        "openai" => 1.4,            // GPT tokenizer - more efficient
        "anthropic" => 1.6,         // Claude tokenizer - middle ground
        _ => 1.5,                   // Default fallback
    }
}

/// Estimate tokens for a single text unit with its prompt
pub async fn estimate_text_unit_tokens(
    text_unit: &TextUnit,
    engine_info: &EngineInfo,
    config: &LlmConfig,
) -> AppResult<TokenEstimate> {
    // Build the actual prompt that would be sent to the LLM
    let prompt = PromptBuilder::build_translation_prompt(text_unit, engine_info).await;
    
    // Estimate input tokens (prompt + source text)
    let base_input_tokens = estimate_tokens_from_text(&prompt);
    
    // Add overhead for JSON formatting, system messages, and special tokens
    // This accounts for the actual API request structure
    let prompt_overhead = 150; // Base overhead for JSON + system messages
    let raw_input_tokens = base_input_tokens + prompt_overhead;
    
    // Apply provider-specific calibration
    let provider = config.model.provider.as_str();
    let calibration_factor = get_provider_calibration_factor(provider);
    let input_tokens = apply_calibration(raw_input_tokens, calibration_factor);
    
    // Estimate output tokens based on source text length
    // Typically, translations are similar length or slightly longer
    let source_length = text_unit.source_text.chars().count() as f32;
    let output_tokens = ((source_length * 1.2) / 4.0).ceil() as u32; // 20% longer than source
    
    let total_tokens = input_tokens + output_tokens;
    
    Ok(TokenEstimate {
        input_tokens,
        output_tokens,
        total_tokens,
    })
}

/// Estimate tokens for all untranslated text units in a project
pub async fn estimate_project_tokens(
    text_units: &[TextUnit],
    engine_info: &EngineInfo,
    config: &LlmConfig,
) -> AppResult<ProjectTokenEstimate> {
    let mut total_input = 0u32;
    let mut total_output = 0u32;
    let mut by_type: std::collections::HashMap<PromptType, (u32, u32, u32)> = std::collections::HashMap::new();
    
    let untranslated_units: Vec<&TextUnit> = text_units
        .iter()
        .filter(|unit| unit.status == crate::models::translation::TranslationStatus::NotTranslated)
        .collect();
    
    for unit in &untranslated_units {
        let estimate = estimate_text_unit_tokens(unit, engine_info, config).await?;
        
        total_input += estimate.input_tokens;
        total_output += estimate.output_tokens;
        
        // Accumulate by prompt type
        let entry = by_type.entry(unit.prompt_type.clone()).or_insert((0, 0, 0));
        entry.0 += 1; // count
        entry.1 += estimate.input_tokens;
        entry.2 += estimate.output_tokens;
    }
    
    let by_prompt_type = by_type
        .into_iter()
        .map(|(prompt_type, (count, input, output))| PromptTypeEstimate {
            prompt_type,
            count,
            input_tokens: input,
            output_tokens: output,
            total_tokens: input + output,
        })
        .collect();
    
    Ok(ProjectTokenEstimate {
        total_input_tokens: total_input,
        total_output_tokens: total_output,
        total_tokens: total_input + total_output,
        text_units_count: untranslated_units.len() as u32,
        by_prompt_type,
    })
}

/// Actual token usage from completed translation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualTokenUsage {
    /// Input tokens used (from LLM response)
    pub input_tokens: u32,
    /// Output tokens used (from LLM response)
    pub output_tokens: u32,
    /// Total tokens used
    pub total_tokens: u32,
    /// Text unit ID this usage is for
    pub text_unit_id: String,
    /// Model used for this translation
    pub model_name: String,
}

/// Aggregated actual token usage for multiple translations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectActualUsage {
    /// Total input tokens used across all translations
    pub total_input_tokens: u32,
    /// Total output tokens used across all translations
    pub total_output_tokens: u32,
    /// Total tokens used
    pub total_tokens: u32,
    /// Number of completed translations
    pub completed_translations: u32,
    /// Individual translation usages
    pub individual_usages: Vec<ActualTokenUsage>,
    /// Usage breakdown by prompt type
    pub by_prompt_type: Vec<PromptTypeActualUsage>,
}

/// Actual token usage breakdown by prompt type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTypeActualUsage {
    pub prompt_type: PromptType,
    pub count: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

/// Calculate actual cost based on real token usage and model pricing
pub fn calculate_actual_cost(usage: &ProjectActualUsage, pricing: &crate::models::provider::TokenPricing) -> f64 {
    let input_cost = (usage.total_input_tokens as f64 / 1000.0) * pricing.input_price_per_1k;
    let output_cost = (usage.total_output_tokens as f64 / 1000.0) * pricing.output_price_per_1k;
    input_cost + output_cost
}

/// Calculate calibration factor based on estimated vs actual usage
/// This can be used to improve future estimates
pub fn calculate_calibration_factor(estimated_total: u32, actual_total: u32) -> f64 {
    if estimated_total == 0 {
        return 1.0;
    }
    actual_total as f64 / estimated_total as f64
}

/// Apply calibration factor to improve token estimation
pub fn apply_calibration(base_estimate: u32, calibration_factor: f64) -> u32 {
    ((base_estimate as f64) * calibration_factor).ceil() as u32
}
