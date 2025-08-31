use log::{debug, error, info};

use crate::core::error::AppResult;
use crate::core::provider::GenerationResponse;
use crate::db::glossary::GlossaryQuery;
use crate::db::ManagedGlossaryState;
use crate::db::state::ManagedTranslationState;
use crate::llm::state::LlmState;
use crate::models::engine::EngineInfo;
use crate::models::provider::LlmConfig;
use crate::models::translation::{PromptType, TextUnit, TranslationStatus};
use crate::utils::prompts::builder::PromptBuilder;
use crate::utils::token_estimation::ActualTokenUsage;
use tauri::State;
use tokio::time::{sleep, timeout, Duration};

/// Response for text unit translation including token usage
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TranslationResult {
    /// The translated text unit
    pub text_unit: TextUnit,
    /// Actual token usage for this translation (if available)
    pub token_usage: Option<ActualTokenUsage>,
}

/// Translate a single text unit using the configured LLM
pub async fn translate_text_unit(
    state: State<'_, LlmState>,
    glossary: State<'_, ManagedGlossaryState>,
    db: State<'_, ManagedTranslationState>,
    text_unit: TextUnit,
    config: LlmConfig,
    engine_info: EngineInfo,
    manifest_hash: Option<String>,
) -> AppResult<TranslationResult> {
    debug!("Translating text unit: {}", text_unit.id);

    // Ensure shared service and apply lightweight rate limiting
    state.ensure_service(&config).await?;
    let _permit = state.limiter.acquire().await.unwrap();
    
    // Add small delay for cloud providers to prevent rate limits
    let provider = config.model.provider.to_lowercase();
    match provider.as_str() {
        "groq" => sleep(Duration::from_millis(500)).await,      // Groq: 500ms delay (increased for strict rate limits)
        "openai" => sleep(Duration::from_millis(200)).await,    // OpenAI: 200ms delay
        "openrouter" => sleep(Duration::from_millis(150)).await, // OpenRouter: 150ms delay
        _ => sleep(Duration::from_millis(500)).await,          // Default: 500ms delay for remote Ollama and other providers
    }
    // Build prompt at the command layer to keep service focused on generation
    // Try to fetch glossary terms filtered by prompt type
    let categories: Vec<String> = match text_unit.prompt_type {
        PromptType::Dialogue | PromptType::Character => {
            vec!["Characters".into(), "Essential Terms".into()]
        }
        PromptType::State | PromptType::Skill => vec![
            "Status Effects".into(),
            "Mechanics".into(),
            "Essential Terms".into(),
        ],
        PromptType::Equipment => vec!["Mechanics".into(), "Essential Terms".into()],
        PromptType::System | PromptType::Class | PromptType::Other => {
            vec!["Mechanics".into(), "Essential Terms".into()]
        }
    };

    let q = GlossaryQuery {
        source_lang: engine_info.source_language.id.clone(),
        target_lang: engine_info.target_language.id.clone(),
        categories,
        prompt_types: Vec::new(),
        project_scope: Some(engine_info.path.to_string_lossy().to_string()),
        limit: Some(200),
        only_enabled: true,
    };

    let terms = match crate::db::glossary::repo::find_terms(&glossary, &q).await {
        Ok(v) => v,
        Err(_) => Vec::new(),
    };

    let prompt = if terms.is_empty() {
        PromptBuilder::build_translation_prompt(&text_unit, &engine_info).await
    } else {
        PromptBuilder::build_translation_prompt_with_terms(&text_unit, &engine_info, &terms).await
    };
    let generation_result = translate_with_retry_and_usage(&*state, &prompt).await?;

    // Clean the model output to remove thinking process and extract only translation
    let cleaned_content = clean_model_output(&generation_result.content);

    // Create updated text unit
    let mut updated_unit = text_unit;
    updated_unit.translated_text = cleaned_content;
    updated_unit.status = TranslationStatus::MachineTranslated;

    // Create token usage record if available
    let token_usage = generation_result.token_usage.map(|usage| ActualTokenUsage {
        input_tokens: usage.input_tokens,
        output_tokens: usage.output_tokens,
        total_tokens: usage.total_tokens,
        text_unit_id: updated_unit.id.clone(),
        model_name: config.model.model_name.clone(),
    });

    // Save translation to database immediately
    // Extract project path from text unit ID (format: "project_path/file_path/field_type")
    let project_path = engine_info.path.to_string_lossy().to_string();
    let file_path = if let Some(_id_parts) = updated_unit.id.split('/').next() {
        // For now, use a simple file path extraction - this could be improved
        format!("{}/data", project_path)
    } else {
        format!("{}/data", project_path)
    };

    let text_unit_record = crate::db::translation::model::TextUnitRecord::from_text_unit(
        &updated_unit,
        &project_path,
        &file_path,
        manifest_hash.as_deref(),
    );

    // Save to database with error handling
    match crate::db::translation::repo::upsert_unit(&db, &text_unit_record).await {
        Ok(_) => {
            info!("Translation saved to database for unit: {}", updated_unit.id);
        }
        Err(e) => {
            // Log error but don't fail the translation - data consistency is more important than DB save
            error!("Failed to save translation to database: {}", e);
        }
    }

    info!("Translation completed for unit: {}", updated_unit.id);
    Ok(TranslationResult {
        text_unit: updated_unit,
        token_usage,
    })
}



/// Execute a single prompt with timeout and retry/backoff using the shared service, returning token usage.
/// Optimized for remote Ollama servers (RunPod, Vast.ai) with enhanced network latency handling.
async fn translate_with_retry_and_usage(state: &LlmState, prompt: &str) -> AppResult<GenerationResponse> {
    const REQ_TIMEOUT: Duration = Duration::from_secs(120); // Increased for remote servers
    const RETRIES: usize = 5; // Increased retries for better network resilience

    let mut last_err: Option<crate::core::error::AppError> = None;
    for attempt in 0..RETRIES {
        let fut = async {
            let guard = state.service.lock().await;
            let svc = guard.as_ref().expect("LLM service must be initialized");
            svc.generate_with_usage(prompt).await
        };
        match timeout(REQ_TIMEOUT, fut).await {
            Ok(Ok(response)) => return Ok(response),
            Ok(Err(e)) => {
                // Fatal provider errors should not be retried
                let is_fatal_quota = match &e {
                    crate::core::error::AppError::Llm(msg) => {
                        let m = msg.to_ascii_lowercase();
                        m.contains("insufficient_quota") || m.contains("exceeded your current quota")
                    }
                    _ => false,
                };
                if is_fatal_quota {
                    return Err(e);
                }
                
                // Handle rate limits with longer backoff
                let is_rate_limit = match &e {
                    crate::core::error::AppError::Llm(msg) => {
                        let m = msg.to_ascii_lowercase();
                        m.contains("429") || m.contains("too many requests") || m.contains("rate limit")
                    }
                    _ => false,
                };
                if is_rate_limit {
                    // Longer backoff for rate limits: 2s, 4s, 8s, 16s, 32s
                    let backoff_ms = 2000 * (1 << attempt);
                    debug!("Rate limit hit, backing off for {}ms", backoff_ms);
                    sleep(Duration::from_millis(backoff_ms)).await;
                    continue;
                }

                // Enhanced network error detection for remote servers (RunPod, Vast.ai)
                let is_network_error = match &e {
                    crate::core::error::AppError::Llm(msg) => {
                        let m = msg.to_ascii_lowercase();
                        m.contains("connection") || m.contains("timeout") ||
                        m.contains("network") || m.contains("unreachable") ||
                        m.contains("dns") || m.contains("resolve") ||
                        m.contains("connection refused") || m.contains("connection reset") ||
                        m.contains("connection aborted") || m.contains("no route to host") ||
                        m.contains("network is unreachable") || m.contains("temporary failure") ||
                        m.contains("server error") || m.contains("bad gateway") ||
                        m.contains("service unavailable") || m.contains("gateway timeout")
                    }
                    _ => false,
                };
                if is_network_error {
                    // Exponential backoff for network issues: 1s, 2s, 4s, 8s, 16s
                    let backoff_ms = 1000 * (1 << attempt);
                    debug!("Network error detected, backing off for {}ms (attempt {})", backoff_ms, attempt + 1);
                    sleep(Duration::from_millis(backoff_ms)).await;
                    continue;
                }

                last_err = Some(e);
            }
            Err(_to) => {
                last_err = Some(crate::core::error::AppError::Llm("request timeout - remote server may be busy".into()));
            }
        }
        // simple backoff: 200ms, 400ms, 600ms
        sleep(Duration::from_millis(200 * (attempt as u64 + 1))).await;
    }

    Err(last_err.unwrap_or_else(|| crate::core::error::AppError::Llm("max retries exceeded - check network connection to remote server".into())))
}

/// Clean model output to remove thinking process and extract only the translation
fn clean_model_output(content: &str) -> String {
    let content = content.trim();
    
    // First, try to extract content between <<<INPUT_START>>> and <<<INPUT_END>>>
    if let Some(start_idx) = content.find("<<<INPUT_START>>>") {
        if let Some(end_idx) = content.find("<<<INPUT_END>>>") {
            let start = start_idx + "<<<INPUT_START>>>".len();
            if start < end_idx {
                return content[start..end_idx].trim().to_string();
            }
        }
    }
    
    // Remove everything between <think> and </think> tags (including the tags)
    let mut cleaned = content.to_string();
    while let Some(start_idx) = cleaned.find("<think>") {
        if let Some(end_idx) = cleaned.find("</think>") {
            if start_idx < end_idx {
                // Remove the entire <think>...</think> block
                cleaned.replace_range(start_idx..end_idx + 8, "");
            } else {
                // Malformed tags, just remove the opening tag
                cleaned.replace_range(start_idx..start_idx + 7, "");
            }
        } else {
            // No closing tag found, just remove the opening tag
            cleaned.replace_range(start_idx..start_idx + 7, "");
        }
    }
    
    // Clean up any remaining whitespace and newlines
    cleaned = cleaned
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();
    
    // If the cleaned content is empty, return the original content
    if cleaned.is_empty() {
        content.to_string()
    } else {
        cleaned
    }
}
