use log::{debug, info};

use crate::core::error::AppResult;
use crate::llm::factory::LlmFactory;
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

    // Create LLM provider
    let provider = LlmFactory::create_provider(config)?;

    // Perform translation
    let translated_text = provider.translate(&text_unit, &engine_info).await?;

    // Create updated text unit
    let mut updated_unit = text_unit;
    updated_unit.translated_text = translated_text;
    updated_unit.status = TranslationStatus::MachineTranslated;

    info!("Translation completed for unit: {}", updated_unit.id);
    Ok(updated_unit)
}
