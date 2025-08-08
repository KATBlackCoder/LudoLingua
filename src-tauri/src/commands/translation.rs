use log::{debug, info};

use crate::core::error::AppResult;
use crate::llm::services::ollama::OllamaService;
use crate::models::engine::EngineInfo;
use crate::models::provider::LlmConfig;
use crate::models::translation::{TextUnit, TranslationStatus};

/// Translate a single text unit using the configured LLM
pub async fn translate_text_unit(
    text_unit: TextUnit,
    config: LlmConfig,
    engine_info: EngineInfo,
) -> AppResult<TextUnit> {
    debug!("Translating text unit: {}", text_unit.id);

    // Create LLM service and perform translation
    let service = OllamaService::new(config)?;
    let translated_text = service.translate(&text_unit, &engine_info).await?;

    // Create updated text unit
    let mut updated_unit = text_unit;
    updated_unit.translated_text = translated_text;
    updated_unit.status = TranslationStatus::MachineTranslated;

    info!("Translation completed for unit: {}", updated_unit.id);
    Ok(updated_unit)
}
