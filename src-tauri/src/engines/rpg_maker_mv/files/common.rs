use crate::models::translation::{PromptType, TextUnit, TranslationStatus};
use std::collections::HashMap;

/// Utility to generate TextUnits for RPG Maker objects (actor, item, etc.)
///
/// - object_type: e.g. "actor", "item"
/// - object_id: the numeric id of the object
/// - file_path: the file path string
/// - index: index in the file array
/// - fields: Vec of (field_name, value, PromptType)
///
/// Returns Vec<TextUnit> for all non-empty values.
pub fn extract_text_units_for_object(
    object_type: &str,
    object_id: i32,
    file_path: &str,
    index: usize,
    fields: Vec<(&str, &str, PromptType)>,
) -> Vec<TextUnit> {
    let mut units = Vec::new();
    for (field, value, prompt_type) in fields {
        if !value.is_empty() {
            units.push(TextUnit {
                id: format!("{}_{}_{}", object_type, object_id, field),
                source_text: value.to_string(),
                translated_text: String::new(),
                field_type: format!("{}:{}:{}", field, file_path, index),
                status: TranslationStatus::NotTranslated,
                prompt_type,
            });
        }
    }
    units
}

/// Utility to inject translated text back into RPG Maker objects
///
/// - object_type: e.g. "actor", "item"
/// - object_id: the numeric id of the object
/// - text_units: HashMap of text units keyed by ID
/// - fields: Vec of (field_name, field_reference) to update
///
/// Updates the object fields with translated text if available.
pub fn inject_text_units_for_object(
    object_type: &str,
    object_id: i32,
    text_units: &HashMap<String, &TextUnit>,
    fields: Vec<(&str, &mut String)>,
) {
    for (field_name, field_ref) in fields {
        let unit_id = format!("{}_{}_{}", object_type, object_id, field_name);
        if let Some(unit) = text_units.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *field_ref = unit.translated_text.clone();
            }
        }
    }
}
