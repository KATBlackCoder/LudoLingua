use crate::models::translation::TextUnit;
use crate::utils::text::formatting_optimized::OptimizedTextFormatter;
use crate::utils::text::llm_output::clean_llm_output;
use crate::utils::text::validation::ContentValidator;

/// Raw text unit for file I/O operations
///
/// This struct represents text units as they exist in game files,
/// before any text processing is applied.
#[derive(Debug, Clone)]
pub struct RawTextUnit {
    pub id: String,
    pub source_text: String,
    pub field_type: String,
    pub prompt_type: crate::models::translation::PromptType,
}

/// Unified TextProcessor with complete processing pipeline
///
/// This struct provides the complete text processing pipeline that all engines
/// use automatically through the Engine trait default implementations.
pub struct TextProcessor;

impl TextProcessor {
    /// Process raw text units for extraction (Filter → UNIFIED FORMAT → Encode pipeline)
    ///
    /// This method takes raw text units from file I/O and processes them through
    /// the unified text processing pipeline to create TextUnits ready for translation.
    pub fn process_for_extraction(
        raw_units: Vec<RawTextUnit>,
        target_language: &str,
    ) -> Vec<TextUnit> {
        raw_units
            .into_iter()
            .filter(|raw_unit| ContentValidator::validate_text(&raw_unit.source_text))
            .map(|raw_unit| {
                let clean_text = OptimizedTextFormatter::prepare_for_translation(&raw_unit.source_text);
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

    /// Enhanced LLM response cleaning and extraction
    ///
    /// This method cleans LLM output to remove thinking blocks, input/output tags,
    /// and other artifacts while preserving legitimate game content.
    pub fn clean_llm_output(content: &str) -> String {
        clean_llm_output(content)
    }

    /// Process text units for injection (UNIFIED RESTORE → Decode pipeline)
    ///
    /// This method takes translated TextUnits and processes them through the
    /// unified text processing pipeline to restore formatting codes and prepare
    /// them for file I/O operations.
    pub fn process_for_injection(text_units: &[TextUnit]) -> Vec<RawTextUnit> {
        text_units
            .iter()
            .map(|unit| {
                let restored_text = OptimizedTextFormatter::restore_after_translation(&unit.translated_text);

                RawTextUnit {
                    id: unit.id.clone(),
                    source_text: restored_text,
                    field_type: unit.field_type.clone(),
                    prompt_type: unit.prompt_type.clone(),
                }
            })
            .collect()
    }

    /// Complete processing pipeline for translation workflow
    ///
    /// This method provides the complete pipeline from raw text to translated text
    /// with all formatting preserved and LLM output cleaned.
    pub fn process_translation_pipeline(
        raw_units: Vec<RawTextUnit>,
        target_language: &str,
    ) -> Vec<TextUnit> {
        // Step 1: Process for extraction (filter, format, encode)
        let mut text_units = Self::process_for_extraction(raw_units, target_language);

        // Step 2: Clean any existing translated text (in case of re-processing)
        for unit in &mut text_units {
            if !unit.translated_text.is_empty() {
                unit.translated_text = Self::clean_llm_output(&unit.translated_text);
            }
        }

        text_units
    }

    /// Process translated text units back to raw format for file injection
    ///
    /// This method takes translated TextUnits and converts them back to RawTextUnits
    /// with all formatting codes restored, ready for file I/O operations.
    pub fn process_injection_pipeline(text_units: &[TextUnit]) -> Vec<RawTextUnit> {
        // Step 1: Clean LLM output from translated text
        let cleaned_units: Vec<TextUnit> = text_units
            .iter()
            .map(|unit| {
                let mut cleaned_unit = unit.clone();
                if !cleaned_unit.translated_text.is_empty() {
                    cleaned_unit.translated_text =
                        Self::clean_llm_output(&cleaned_unit.translated_text);
                }
                cleaned_unit
            })
            .collect();

        // Step 2: Process for injection (restore, decode)
        Self::process_for_injection(&cleaned_units)
    }
}
