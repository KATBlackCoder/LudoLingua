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

/// Represents a weapon in RPG Maker MV.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Weapon {
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

/// Extracts text units from a Weapons.json file and organizes them into a GameDataFile.
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Weapons.json
    let parse_weapons = |content: &str| -> AppResult<Vec<Option<Weapon>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Weapons.json: {}", e)))
    };

    // Extract function for each weapon
    let extract_weapon_units = |weapon: &Weapon, index: usize, file_path: &str| -> Vec<TextUnit> {
        extract_text_units_for_object(
            "weapon",
            weapon.id,
            file_path,
            index,
            vec![
                ("name", &weapon.name, PromptType::Equipment),
                ("description", &weapon.description, PromptType::Equipment),
            ],
        )
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Weapons.json",
        parse_weapons,
        extract_weapon_units,
    )
}

/// Injects translated text units back into the Weapons.json file.
///
/// # Arguments
///
/// * `project_path` - The path to the project directory
/// * `file_path` - The path to the Weapons.json file, relative to the project directory
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
    // Parse function for Weapons.json
    let parse_weapons = |content: &str| -> AppResult<Vec<Option<Weapon>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Weapons.json: {}", e)))
    };

    // Update function for each weapon
    let update_weapon = |weapon: &mut Weapon, text_unit_map: &HashMap<String, &TextUnit>| {
        inject_text_units_for_object(
            "weapon",
            weapon.id,
            text_unit_map,
            vec![
                ("name", &mut weapon.name),
                ("description", &mut weapon.description),
                ("note", &mut weapon.note),
            ],
        );
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Weapons.json",
        text_units,
        parse_weapons,
        update_weapon,
    )
}
