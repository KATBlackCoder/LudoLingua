use log::{debug, info};

use crate::core::error::AppResult;
use crate::llm::state::LlmState;
use crate::models::engine::EngineInfo;
use crate::models::provider::LlmConfig;
use crate::models::translation::{TextUnit, TranslationStatus};
use crate::utils::prompts::builder::PromptBuilder;
use tauri::State;
use tokio::time::{timeout, sleep, Duration};

/// Translate a single text unit using the configured LLM
pub async fn translate_text_unit(
    state: State<'_, LlmState>,
    text_unit: TextUnit,
    config: LlmConfig,
    engine_info: EngineInfo,
) -> AppResult<TextUnit> {
    debug!("Translating text unit: {}", text_unit.id);

    // Ensure shared service and apply lightweight rate limiting
    state.ensure_service(&config).await?;
    let _permit = state.limiter.acquire().await.unwrap();
    // Build prompt at the command layer to keep service focused on generation
    let prompt = PromptBuilder::build_translation_prompt(&text_unit, &engine_info).await;
    let translated_text = translate_with_retry(&state, &prompt).await?;

    // Create updated text unit
    let mut updated_unit = text_unit;
    updated_unit.translated_text = translated_text;
    updated_unit.status = TranslationStatus::MachineTranslated;

    info!("Translation completed for unit: {}", updated_unit.id);
    Ok(updated_unit)
}

/// Execute a single prompt with timeout and retry/backoff using the shared service.
async fn translate_with_retry(state: &LlmState, prompt: &str) -> AppResult<String> {
    const REQ_TIMEOUT: Duration = Duration::from_secs(90);
    const RETRIES: usize = 3;

    let mut last_err: Option<crate::core::error::AppError> = None;
    for attempt in 0..RETRIES {
        let fut = async {
            let guard = state.service.lock().await;
            let svc = guard.as_ref().expect("OllamaService must be initialized");
            svc.generate(prompt).await
        };
        match timeout(REQ_TIMEOUT, fut).await {
            Ok(Ok(text)) => return Ok(text),
            Ok(Err(e)) => {
                last_err = Some(e);
            }
            Err(_to) => {
                last_err = Some(crate::core::error::AppError::Llm("request timeout".into()));
            }
        }
        // simple backoff: 200ms, 400ms, 600ms
        sleep(Duration::from_millis(200 * (attempt as u64 + 1))).await;
    }

    Err(last_err.unwrap_or_else(|| crate::core::error::AppError::Llm("unknown LLM error".into())))
}
