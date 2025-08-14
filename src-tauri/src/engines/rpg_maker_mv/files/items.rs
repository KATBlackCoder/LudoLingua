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

/// Represents an item in RPG Maker MV.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub note: String,
    #[serde(flatten)]
    pub extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

/// Extracts text units from an Items.json file and organizes them into a GameDataFile.
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Items.json
    let parse_items = |content: &str| -> AppResult<Vec<Option<Item>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Items.json: {}", e)))
    };

    // Extract function for each item
    let extract_item_units = |item: &Item, index: usize, file_path: &str| -> Vec<TextUnit> {
        extract_text_units_for_object(
            "item",
            item.id,
            file_path,
            index,
            vec![
                ("name", &item.name, PromptType::Equipment),
                ("description", &item.description, PromptType::Equipment),
            ],
        )
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Items.json",
        parse_items,
        extract_item_units,
    )
}

/// Injects translated text units back into the Items.json file.
///
/// # Arguments
///
/// * `project_path` - The path to the project directory
/// * `file_path` - The path to the Items.json file, relative to the project directory
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
    // Parse function for Items.json
    let parse_items = |content: &str| -> AppResult<Vec<Option<Item>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Items.json: {}", e)))
    };

    // Update function for each item
    let update_item = |item: &mut Item, text_unit_map: &HashMap<String, &TextUnit>| {
        inject_text_units_for_object(
            "item",
            item.id,
            text_unit_map,
            vec![
                ("name", &mut item.name),
                ("description", &mut item.description),
            ],
        );
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Items.json",
        text_units,
        parse_items,
        update_item,
    )
}
