use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::core::error::{AppError, AppResult};
use crate::models::engine::GameDataFile;
use crate::models::translation::{PromptType, TextUnit};
use super::common::{
    extract_text_from_file_with_objects, extract_text_units_for_object, 
    inject_text_units_for_object, inject_translations_into_file_with_objects
};

/// Represents an armor in RPG Maker MV.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Armor {
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

/// Extracts text units from an Armors.json file and organizes them into a GameDataFile.
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Armors.json
    let parse_armors = |content: &str| -> AppResult<Vec<Option<Armor>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Armors.json: {}", e)))
    };

    // Extract function for each armor
    let extract_armor_units = |armor: &Armor, index: usize, file_path: &str| -> Vec<TextUnit> {
        extract_text_units_for_object(
            "armor",
            armor.id,
            file_path,
            index,
            vec![
                ("name", &armor.name, PromptType::Equipment),
                ("description", &armor.description, PromptType::Equipment),
            ],
        )
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Armors.json",
        parse_armors,
        extract_armor_units,
    )
}

/// Injects translated text units back into the Armors.json file.
///
/// # Arguments
///
/// * `project_path` - The path to the project directory
/// * `file_path` - The path to the Armors.json file, relative to the project directory
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
    // Parse function for Armors.json
    let parse_armors = |content: &str| -> AppResult<Vec<Option<Armor>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Armors.json: {}", e)))
    };

    // Update function for each armor
    let update_armor = |armor: &mut Armor, text_unit_map: &HashMap<String, &TextUnit>| {
        inject_text_units_for_object(
            "armor",
            armor.id,
            text_unit_map,
            vec![
                ("name", &mut armor.name),
                ("description", &mut armor.description),
            ],
        );
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Armors.json",
        text_units,
        parse_armors,
        update_armor,
    )
}
