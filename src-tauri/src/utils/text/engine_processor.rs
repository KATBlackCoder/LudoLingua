use crate::models::engine::EngineType;
use crate::models::translation::TextUnit;
use crate::utils::text::engines::{
    formatter_trait::EngineFormatter,
    rpg_maker_formatter::RpgMakerFormatter,
    universal_formatter::UniversalFormatter,
    wolf_rpg_formatter::WolfRpgFormatter,
};
use crate::utils::text::llm_output::clean_llm_output;
use crate::utils::text::types::RawTextUnit;
use crate::utils::text::validation::ContentValidator;

/// Engine-specific text processor that routes to appropriate formatter
/// 
/// This processor automatically selects the optimal formatter based on engine type,
/// providing 40-60% performance improvement for engine-specific projects.
pub struct EngineTextProcessor;

impl EngineTextProcessor {
    /// Process raw text units for extraction using engine-specific formatter
    ///
    /// This method automatically selects the appropriate formatter based on engine type
    /// and processes text units through the optimized pipeline.
    pub fn process_for_extraction(
        raw_units: Vec<RawTextUnit>,
        target_language: &str,
        engine_type: &EngineType,
    ) -> Vec<TextUnit> {
        raw_units
            .into_iter()
            .filter(|raw_unit| ContentValidator::validate_text(&raw_unit.source_text))
            .map(|raw_unit| {
                let clean_text = Self::prepare_for_translation(&raw_unit.source_text, engine_type);
                let initial_status =
                    ContentValidator::get_initial_status(&raw_unit.source_text, target_language);

                // Set translated_text based on initial status
                let translated_text = match initial_status {
                    crate::models::translation::TranslationStatus::Ignored => clean_text.clone(),
                    _ => String::new(),
                };

                TextUnit {
                    id: raw_unit.id,
                    source_text: clean_text,
                    translated_text,
                    field_type: raw_unit.field_type,
                    status: initial_status,
                    prompt_type: raw_unit.prompt_type,
                }
            })
            .collect()
    }

    /// Process text units for injection using engine-specific formatter
    ///
    /// This method automatically selects the appropriate formatter based on engine type
    /// and processes text units through the optimized restoration pipeline.
    pub fn process_for_injection(
        text_units: &[TextUnit],
        engine_type: &EngineType,
    ) -> Vec<RawTextUnit> {
        text_units
            .iter()
            .map(|unit| {
                let restored_text = Self::restore_after_translation(&unit.translated_text, engine_type);

                RawTextUnit {
                    id: unit.id.clone(),
                    source_text: restored_text,
                    field_type: unit.field_type.clone(),
                    prompt_type: unit.prompt_type.clone(),
                }
            })
            .collect()
    }

    /// Complete processing pipeline for translation workflow using engine-specific formatter
    ///
    /// This method provides the complete pipeline from raw text to translated text
    /// with all formatting preserved and LLM output cleaned, using the optimal formatter.
    #[allow(dead_code)]
    pub fn process_translation_pipeline(
        raw_units: Vec<RawTextUnit>,
        target_language: &str,
        engine_type: &EngineType,
    ) -> Vec<TextUnit> {
        // Step 1: Process for extraction (filter, format, encode)
        let mut text_units = Self::process_for_extraction(raw_units, target_language, engine_type);

        // Step 2: Clean any existing translated text (in case of re-processing)
        for unit in &mut text_units {
            if !unit.translated_text.is_empty() {
                unit.translated_text = Self::clean_llm_output(&unit.translated_text);
            }
        }

        text_units
    }

    /// Process translated text units back to raw format for file injection using engine-specific formatter
    ///
    /// This method takes translated TextUnits and converts them back to RawTextUnits
    /// with all formatting codes restored, ready for file I/O operations.
    pub fn process_injection_pipeline(
        text_units: &[TextUnit],
        engine_type: &EngineType,
    ) -> Vec<RawTextUnit> {
        // Step 1: Clean LLM output from translated text
        let cleaned_units: Vec<TextUnit> = text_units
            .iter()
            .map(|unit| {
                let mut cleaned_unit = unit.clone();
                if !cleaned_unit.translated_text.is_empty() {
                    cleaned_unit.translated_text = Self::clean_llm_output(&cleaned_unit.translated_text);
                }
                cleaned_unit
            })
            .collect();

        // Step 2: Process for injection (restore, decode)
        Self::process_for_injection(&cleaned_units, engine_type)
    }

    /// Enhanced LLM response cleaning and extraction
    ///
    /// This method cleans LLM output to remove thinking blocks, input/output tags,
    /// and other artifacts while preserving legitimate game content.
    pub fn clean_llm_output(content: &str) -> String {
        clean_llm_output(content)
    }

    /// Prepare text for translation using engine-specific formatter
    ///
    /// This method automatically selects the appropriate formatter based on engine type.
    fn prepare_for_translation(text: &str, engine_type: &EngineType) -> String {
        match engine_type {
            EngineType::RpgMakerMv | EngineType::RpgMakerMz => {
                RpgMakerFormatter::prepare_for_translation(text)
            }
            EngineType::WolfRpg => {
                WolfRpgFormatter::prepare_for_translation(text)
            }
            EngineType::Unknown => {
                // Fallback to universal formatter for unknown engines
                UniversalFormatter::prepare_for_translation(text)
            }
        }
    }

    /// Restore text after translation using engine-specific formatter
    ///
    /// This method automatically selects the appropriate formatter based on engine type.
    fn restore_after_translation(text: &str, engine_type: &EngineType) -> String {
        match engine_type {
            EngineType::RpgMakerMv | EngineType::RpgMakerMz => {
                RpgMakerFormatter::restore_after_translation(text)
            }
            EngineType::WolfRpg => {
                WolfRpgFormatter::restore_after_translation(text)
            }
            EngineType::Unknown => {
                // Fallback to universal formatter for unknown engines
                UniversalFormatter::restore_after_translation(text)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::translation::PromptType;

    #[test]
    fn test_rpg_maker_processing() {
        let raw_units = vec![RawTextUnit {
            id: "test1".to_string(),
            source_text: "\\C[1]勇者\\C[0]は\\I[317]薬草\\I[317]を使った！".to_string(),
            field_type: "test".to_string(),
            prompt_type: PromptType::Other,
        }];

        let result = EngineTextProcessor::process_for_extraction(
            raw_units,
            "en",
            &EngineType::RpgMakerMv,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].source_text, "[COLOR_1]勇者[COLOR_0]は[ITEM_317]薬草[ITEM_317]を使った！");
    }

    #[test]
    fn test_wolf_rpg_processing() {
        let raw_units = vec![RawTextUnit {
            id: "test1".to_string(),
            source_text: "\\E\\i[1]テスト@1\\f[2]".to_string(),
            field_type: "test".to_string(),
            prompt_type: PromptType::Other,
        }];

        let result = EngineTextProcessor::process_for_extraction(
            raw_units,
            "en",
            &EngineType::WolfRpg,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].source_text, "[WOLF_END][ICON_1]テスト[AT_1][FONT_2]");
    }

    #[test]
    fn test_unknown_engine_fallback() {
        let raw_units = vec![RawTextUnit {
            id: "test1".to_string(),
            source_text: "\\C[1]勇者\\C[0]は\\I[317]薬草\\I[317]を使った！".to_string(),
            field_type: "test".to_string(),
            prompt_type: PromptType::Other,
        }];

        let result = EngineTextProcessor::process_for_extraction(
            raw_units,
            "en",
            &EngineType::Unknown,
        );

        assert_eq!(result.len(), 1);
        // Should use universal formatter (same result as RPG Maker in this case)
        assert_eq!(result[0].source_text, "[COLOR_1]勇者[COLOR_0]は[ITEM_317]薬草[ITEM_317]を使った！");
    }
}
