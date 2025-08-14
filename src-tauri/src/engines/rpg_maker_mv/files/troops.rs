use crate::core::error::{AppError, AppResult};
use crate::models::engine::GameDataFile;
use crate::models::translation::{PromptType, TextUnit};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::common::{
    extract_text_from_file_with_objects, extract_text_units_for_object,
    extract_text_units_from_event_commands, inject_text_units_for_object,
    inject_text_units_into_event_commands, inject_translations_into_file_with_objects,
    EventCommand as CommonEventCommand,
};

/// Represents a single event command in RPG Maker MV Troops.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCommand {
    /// Command code (101 = Show Text, 401 = Message, 108 = Comment, etc.)
    #[serde(default)]
    pub code: i32,

    /// Indentation level
    #[serde(default)]
    pub indent: i32,

    /// Command parameters (array of values)
    #[serde(default)]
    pub parameters: Vec<serde_json::Value>,
}

impl From<EventCommand> for CommonEventCommand {
    fn from(event_command: EventCommand) -> Self {
        CommonEventCommand {
            code: event_command.code,
            indent: event_command.indent,
            parameters: event_command.parameters,
        }
    }
}

impl From<CommonEventCommand> for EventCommand {
    fn from(common_command: CommonEventCommand) -> Self {
        EventCommand {
            code: common_command.code,
            indent: common_command.indent,
            parameters: common_command.parameters,
        }
    }
}

/// Represents a single page in a troop from RPG Maker MV Troops.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TroopPage {
    /// Page conditions (not translatable)
    #[serde(default)]
    pub conditions: HashMap<String, serde_json::Value>,

    /// List of event commands
    #[serde(default)]
    pub list: Vec<EventCommand>,

    /// Page span (not translatable)
    #[serde(default)]
    pub span: i32,

    /// Additional fields that might be present in the JSON
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

/// Represents a single troop from RPG Maker MV Troops.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Troop {
    /// Troop ID
    #[serde(default)]
    pub id: i32,

    /// Troop name
    #[serde(default)]
    pub name: String,

    /// List of troop members (not translatable)
    #[serde(default)]
    pub members: Vec<HashMap<String, serde_json::Value>>,

    /// List of troop pages
    #[serde(default)]
    pub pages: Vec<TroopPage>,

    /// Additional fields that might be present in the JSON
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

/// Extracts translatable text from Troops.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Troops.json file
///
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Troops.json
    let parse_troops = |content: &str| -> AppResult<Vec<Option<Troop>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Troops.json: {}", e)))
    };

    // Extract function for each troop
    let extract_troop_units = |troop: &Troop, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip empty troops (id 0 or empty name)
        if troop.id == 0 || troop.name.is_empty() {
            return Vec::new();
        }

        let mut text_units = Vec::new();

        // Extract troop name
        if !troop.name.is_empty() {
            text_units.extend(extract_text_units_for_object(
                "troop",
                troop.id,
                file_path,
                index,
                vec![("name", &troop.name, PromptType::Character)],
            ));
        }

        // Extract text from all pages' event commands
        for (page_index, page) in troop.pages.iter().enumerate() {
            let common_commands: Vec<CommonEventCommand> =
                page.list.iter().map(|cmd| cmd.clone().into()).collect();
            text_units.extend(extract_text_units_from_event_commands(
                &format!("troop_{}_page_{}", troop.id, page_index),
                troop.id,
                &common_commands,
                file_path,
            ));
        }

        text_units
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Troops.json",
        parse_troops,
        extract_troop_units,
    )
}

/// Injects translated text back into Troops.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Troops.json file
/// * `text_units` - Vector of translated text units
///
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    // Parse function for Troops.json
    let parse_troops = |content: &str| -> AppResult<Vec<Option<Troop>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Troops.json: {}", e)))
    };

    // Update function for each troop
    let update_troop = |troop: &mut Troop, text_unit_map: &HashMap<String, &TextUnit>| {
        // Update troop name
        inject_text_units_for_object(
            "troop",
            troop.id,
            text_unit_map,
            vec![("name", &mut troop.name)],
        );

        // Update text in all pages' event commands
        for (page_index, page) in troop.pages.iter_mut().enumerate() {
            let mut common_commands: Vec<CommonEventCommand> =
                page.list.iter().map(|cmd| cmd.clone().into()).collect();
            inject_text_units_into_event_commands(
                &format!("troop_{}_page_{}", troop.id, page_index),
                troop.id,
                &mut common_commands,
                text_unit_map,
            );
            // Convert back to EventCommand
            page.list = common_commands.into_iter().map(|cmd| cmd.into()).collect();
        }
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Troops.json",
        text_units,
        parse_troops,
        update_troop,
    )
}
