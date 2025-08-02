use crate::core::error::{AppError, AppResult};
use crate::models::engine::GameDataFile;
use crate::models::translation::{PromptType, TextUnit, TranslationStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::common::{
    extract_text_from_file_with_objects, extract_text_units_for_object, 
    inject_text_units_for_object, inject_translations_into_file_with_objects
};

/// Represents a single enemy from RPG Maker MV Enemies.json
/// This struct only includes translatable fields from the enemy.
/// Non-translatable fields (actions, traits, params, etc.) are ignored.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Enemy {
    /// Enemy ID
    #[serde(default)]
    pub id: i32,

    /// Enemy name
    #[serde(default)]
    pub name: String,

    /// Developer notes
    #[serde(default)]
    pub note: String,

    /// Additional fields that might be present in the JSON
    /// These are ignored during translation but preserved during serialization
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

/// Extracts translatable text from Enemies.json
/// 
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Enemies.json file
/// 
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Enemies.json
    let parse_enemies = |content: &str| -> AppResult<Vec<Option<Enemy>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Enemies.json: {}", e)))
    };

    // Extract function for each enemy
    let extract_enemy_units = |enemy: &Enemy, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip empty enemies (id 0 or empty name)
        if enemy.id == 0 || enemy.name.is_empty() {
            return Vec::new();
        }

        extract_text_units_for_object(
            "enemy",
            enemy.id,
            file_path,
            index,
            vec![
                ("name", &enemy.name, PromptType::Character),
            ],
        )
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Enemies.json",
        parse_enemies,
        extract_enemy_units,
    )
}

/// Injects translated text back into Enemies.json
/// 
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Enemies.json file
/// * `text_units` - Vector of translated text units
/// 
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(project_path: &Path, file_path: &str, text_units: &[&TextUnit]) -> AppResult<()> {
    // Parse function for Enemies.json
    let parse_enemies = |content: &str| -> AppResult<Vec<Option<Enemy>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Enemies.json: {}", e)))
    };

    // Update function for each enemy
    let update_enemy = |enemy: &mut Enemy, text_unit_map: &HashMap<String, &TextUnit>| {
        inject_text_units_for_object(
            "enemy",
            enemy.id,
            text_unit_map,
            vec![
                ("name", &mut enemy.name),
            ],
        );
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Enemies.json",
        text_units,
        parse_enemies,
        update_enemy,
    )
} 