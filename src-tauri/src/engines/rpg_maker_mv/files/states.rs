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

/// Represents a single status effect from RPG Maker MV States.json
/// This struct only includes translatable fields from the status effect.
/// Non-translatable fields (timing, icons, traits, etc.) are ignored.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct State {
    /// Status effect ID
    #[serde(default)]
    pub id: i32,

    /// Status effect name
    #[serde(default)]
    pub name: String,

    /// Status effect description
    #[serde(default)]
    pub description: String,

    /// Battle message when status is applied to target
    #[serde(default)]
    pub message1: String,

    /// Battle message when status is applied by user
    #[serde(default)]
    pub message2: String,

    /// Battle message when status is active
    #[serde(default)]
    pub message3: String,

    /// Battle message when status is removed
    #[serde(default)]
    pub message4: String,

    /// Developer notes
    #[serde(default)]
    pub note: String,

    /// Additional fields that might be present in the JSON
    /// These are ignored during translation but preserved during serialization
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

/// Extracts translatable text from States.json
/// 
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the States.json file
/// 
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for States.json
    let parse_states = |content: &str| -> AppResult<Vec<Option<State>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse States.json: {}", e)))
    };

    // Extract function for each state
    let extract_state_units = |state: &State, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip empty states (id 0 or empty name)
        if state.id == 0 || state.name.is_empty() {
            return Vec::new();
        }

        extract_text_units_for_object(
            "state",
            state.id,
            file_path,
            index,
            vec![
                ("name", &state.name, PromptType::State),
                ("description", &state.description, PromptType::State),
                ("message1", &state.message1, PromptType::State),
                ("message2", &state.message2, PromptType::State),
                ("message3", &state.message3, PromptType::State),
                ("message4", &state.message4, PromptType::State),
            ],
        )
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "States.json",
        parse_states,
        extract_state_units,
    )
}

/// Injects translated text back into States.json
/// 
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the States.json file
/// * `text_units` - Vector of translated text units
/// 
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(project_path: &Path, file_path: &str, text_units: &[&TextUnit]) -> AppResult<()> {
    // Parse function for States.json
    let parse_states = |content: &str| -> AppResult<Vec<Option<State>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse States.json: {}", e)))
    };

    // Update function for each state
    let update_state = |state: &mut State, text_unit_map: &HashMap<String, &TextUnit>| {
        inject_text_units_for_object(
            "state",
            state.id,
            text_unit_map,
            vec![
                ("name", &mut state.name),
                ("description", &mut state.description),
                ("message1", &mut state.message1),
                ("message2", &mut state.message2),
                ("message3", &mut state.message3),
                ("message4", &mut state.message4),
            ],
        );
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "States.json",
        text_units,
        parse_states,
        update_state,
    )
}
