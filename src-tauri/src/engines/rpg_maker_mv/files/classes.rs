use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::common::{
    extract_text_from_file_with_objects, extract_text_units_for_object,
    inject_text_units_for_object, inject_translations_into_file_with_objects,
};
use crate::core::error::{AppError, AppResult};
use crate::models::engine::GameDataFile;
use crate::models::translation::{PromptType, TextUnit};

/// Represents a character class in RPG Maker MV.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Class {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub note: String,
    #[serde(flatten)]
    pub extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

/// Extracts text units from a Classes.json file and organizes them into a GameDataFile.
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Classes.json
    let parse_classes = |content: &str| -> AppResult<Vec<Option<Class>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Classes.json: {}", e)))
    };

    // Extract function for each class
    let extract_class_units = |class: &Class, index: usize, file_path: &str| -> Vec<TextUnit> {
        extract_text_units_for_object(
            "class",
            class.id,
            file_path,
            index,
            vec![("name", &class.name, PromptType::Class)],
        )
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Classes.json",
        parse_classes,
        extract_class_units,
    )
}

/// Injects translated text units back into the Classes.json file.
///
/// # Arguments
///
/// * `project_path` - The path to the project directory
/// * `file_path` - The path to the Classes.json file, relative to the project directory
/// * `text_units` - Slice of translated text units to inject
///
/// # Returns
///
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    // Parse function for Classes.json
    let parse_classes = |content: &str| -> AppResult<Vec<Option<Class>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Classes.json: {}", e)))
    };

    // Update function for each class
    let update_class = |class: &mut Class, text_unit_map: &HashMap<String, &TextUnit>| {
        inject_text_units_for_object(
            "class",
            class.id,
            text_unit_map,
            vec![("name", &mut class.name), ("note", &mut class.note)],
        );
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Classes.json",
        text_units,
        parse_classes,
        update_class,
    )
}
