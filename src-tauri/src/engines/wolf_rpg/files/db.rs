// Text processing now handled by unified pipeline
use crate::models::translation::{PromptType, TextUnit, TranslationStatus};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

/// Extract text units from Wolf RPG database JSON files
/// Handles CDataBase.json, DataBase.json, and SysDatabase.json
pub fn extract_text_units_from_db(db_data: &Value, file_path: &str) -> Vec<TextUnit> {
    let mut text_units = Vec::new();

    if let Some(types) = db_data.get("types").and_then(|v| v.as_array()) {
        for (type_idx, type_obj) in types.iter().enumerate() {
            if let Some(data_array) = type_obj.get("data").and_then(|v| v.as_array()) {
                for (data_idx, data_obj) in data_array.iter().enumerate() {
                    extract_from_db_data_entry(
                        &mut text_units,
                        data_obj,
                        file_path,
                        type_idx,
                        data_idx,
                    );
                }
            }
        }
    }

    text_units
}

/// Extract text from a single data entry in the database
fn extract_from_db_data_entry(
    text_units: &mut Vec<TextUnit>,
    data_obj: &Value,
    file_path: &str,
    type_idx: usize,
    data_idx: usize,
) {
    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // For SysDatabase.json: extract types=>data=>name
    if file_name == "SysDatabase.json" {
        if let Some(name) = data_obj.get("name").and_then(|v| v.as_str()) {
            if is_translatable_sys_db_name(name, data_obj) {
                let processed_text = name.to_string(); // Raw text, no processing
                let normalized_path = file_path.replace('\\', "/");
                let text_unit = TextUnit {
                    id: format!(
                        "{}:types[{}]:data[{}]:name",
                        normalized_path, type_idx, data_idx
                    ),
                    source_text: processed_text,
                    translated_text: String::new(),
                    field_type: format!("Database entry name ({})", file_name),
                    status: TranslationStatus::NotTranslated,
                    prompt_type: PromptType::Other,
                };
                text_units.push(text_unit);
            }
        }
    }

    // For CDataBase.json and DataBase.json: extract types=>data=>data=>value
    if file_name == "CDataBase.json" || file_name == "DataBase.json" {
        if let Some(data_data_array) = data_obj.get("data").and_then(|v| v.as_array()) {
            for (data_data_idx, data_data_obj) in data_data_array.iter().enumerate() {
                if let Some(value) = data_data_obj.get("value").and_then(|v| v.as_str()) {
                    if is_translatable_db_value(value) {
                        let processed_text = value.to_string(); // Raw text, no processing
                        let normalized_path = file_path.replace('\\', "/");
                        let text_unit = TextUnit {
                            id: format!(
                                "{}:types[{}]:data[{}]:data[{}]:value",
                                normalized_path, type_idx, data_idx, data_data_idx
                            ),
                            source_text: processed_text,
                            translated_text: String::new(),
                            field_type: format!("Database value ({})", file_name),
                            status: TranslationStatus::NotTranslated,
                            prompt_type: PromptType::Other,
                        };
                        text_units.push(text_unit);
                    }
                }
            }
        }
    }
}

/// Check if a database value field is translatable
/// Rules: not empty, not numeric, no file extensions
fn is_translatable_db_value(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    // Skip if it's purely numeric
    if value
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
    {
        return false;
    }

    // Skip if it has file extensions (case insensitive)
    let file_extensions =
        Regex::new(r"\.(png|jpg|jpeg|gif|bmp|wav|mp3|ogg|txt|json|dat)$").unwrap();
    if file_extensions.is_match(&value.to_lowercase()) {
        return false;
    }

    // Text validation now handled by unified pipeline
    !value.trim().is_empty()
}

/// Check if a SysDatabase.json name field is translatable
/// Rules:
/// 1. If "data" array is empty, name is not translatable
/// 2. If name starts with "[読]", "○", or "×", it's not translatable
/// 3. Must pass general translatability checks
fn is_translatable_sys_db_name(name: &str, data_obj: &Value) -> bool {
    if name.is_empty() {
        return false;
    }

    // Rule 1: If data array is empty, name is not translatable
    if let Some(data_array) = data_obj.get("data").and_then(|v| v.as_array()) {
        if data_array.is_empty() {
            return false;
        }
    } else {
        // If data field doesn't exist or is not an array, consider it empty
        return false;
    }

    // Rule 2: Skip names starting with [読], ○, or ×
    if name.starts_with("[読]") || name.starts_with("○") || name.starts_with("×") {
        return false;
    }

    // Rule 3: Text validation now handled by unified pipeline
    !name.trim().is_empty()
}

/// Inject translated text back into Wolf RPG database JSON
pub fn inject_text_units_into_db(
    db_data: &mut Value,
    text_units: &HashMap<String, &TextUnit>,
    file_path: &str,
) {
    if let Some(types) = db_data.get_mut("types").and_then(|v| v.as_array_mut()) {
        for (type_idx, type_obj) in types.iter_mut().enumerate() {
            if let Some(data_array) = type_obj.get_mut("data").and_then(|v| v.as_array_mut()) {
                for (data_idx, data_obj) in data_array.iter_mut().enumerate() {
                    inject_into_db_data_entry(data_obj, text_units, file_path, type_idx, data_idx);
                }
            }
        }
    }
}

/// Inject translated text into a single database data entry
fn inject_into_db_data_entry(
    data_obj: &mut Value,
    text_units: &HashMap<String, &TextUnit>,
    file_path: &str,
    type_idx: usize,
    data_idx: usize,
) {
    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // For SysDatabase.json: inject into types=>data=>name
    if file_name == "SysDatabase.json" {
        let normalized_path = file_path.replace('\\', "/");
        let name_id = format!(
            "{}:types[{}]:data[{}]:name",
            normalized_path, type_idx, data_idx
        );
        if let Some(text_unit) = text_units.get(&name_id) {
            if !text_unit.translated_text.is_empty() {
                data_obj["name"] = Value::String(text_unit.translated_text.clone());
            }
        }
    }

    // For CDataBase.json and DataBase.json: inject into types=>data=>data=>value
    if file_name == "CDataBase.json" || file_name == "DataBase.json" {
        if let Some(data_data_array) = data_obj.get_mut("data").and_then(|v| v.as_array_mut()) {
            for (data_data_idx, data_data_obj) in data_data_array.iter_mut().enumerate() {
                let normalized_path = file_path.replace('\\', "/");
                let value_id = format!(
                    "{}:types[{}]:data[{}]:data[{}]:value",
                    normalized_path, type_idx, data_idx, data_data_idx
                );
                if let Some(text_unit) = text_units.get(&value_id) {
                    if !text_unit.translated_text.is_empty() {
                        data_data_obj["value"] = Value::String(text_unit.translated_text.clone());
                    }
                }
            }
        }
    }
}
