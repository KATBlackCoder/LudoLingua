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
    EventCommand as CommonEventCommand
};

/// Represents a single event command in RPG Maker MV CommonEvents.json
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

/// Represents a single common event in RPG Maker MV CommonEvents.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommonEvent {
    /// Event ID
    #[serde(default)]
    pub id: i32,

    /// Event name
    #[serde(default)]
    pub name: String,

    /// Switch ID that triggers this event
    #[serde(default, rename = "switchId")]
    pub switch_id: i32,

    /// Trigger type (0 = None, 1 = Autorun, 2 = Parallel)
    #[serde(default)]
    pub trigger: i32,

    /// List of event commands
    #[serde(default)]
    pub list: Vec<EventCommand>,

    /// Additional fields that might be present in the JSON
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

/// Extracts translatable text from CommonEvents.json
/// 
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the CommonEvents.json file
/// 
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for CommonEvents.json
    let parse_common_events = |content: &str| -> AppResult<Vec<Option<CommonEvent>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse CommonEvents.json: {}", e)))
    };

    // Extract function for each common event
    let extract_common_event_units = |common_event: &CommonEvent, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip empty events (id 0 or empty name)
        if common_event.id == 0 || common_event.name.is_empty() {
            return Vec::new();
        }

        let mut text_units = Vec::new();

        // Extract event name
        if !common_event.name.is_empty() {
            text_units.extend(extract_text_units_for_object(
                "common_event",
                common_event.id,
                file_path,
                index,
                vec![("name", &common_event.name, PromptType::Character)],
            ));
        }

        // Extract text from event commands using common helper
        let common_commands: Vec<CommonEventCommand> = common_event.list.iter()
            .map(|cmd| cmd.clone().into())
            .collect();
        text_units.extend(extract_text_units_from_event_commands(
            "common_event",
            common_event.id,
            &common_commands,
            file_path,
        ));

        text_units
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "CommonEvents.json",
        parse_common_events,
        extract_common_event_units,
    )
}

/// Injects translated text back into CommonEvents.json
/// 
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the CommonEvents.json file
/// * `text_units` - Vector of translated text units
/// 
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(project_path: &Path, file_path: &str, text_units: &[&TextUnit]) -> AppResult<()> {
    // Parse function for CommonEvents.json
    let parse_common_events = |content: &str| -> AppResult<Vec<Option<CommonEvent>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse CommonEvents.json: {}", e)))
    };

    // Update function for each common event
    let update_common_event = |common_event: &mut CommonEvent, text_unit_map: &HashMap<String, &TextUnit>| {
        // Update event name
        inject_text_units_for_object(
            "common_event",
            common_event.id,
            text_unit_map,
            vec![("name", &mut common_event.name)],
        );

                // Update text in event commands using common helper
        let mut common_commands: Vec<CommonEventCommand> = common_event.list.iter()
            .map(|cmd| cmd.clone().into())
            .collect();
        inject_text_units_into_event_commands(
            "common_event",
            common_event.id,
            &mut common_commands,
            text_unit_map,
        );
        // Convert back to EventCommand
        common_event.list = common_commands.into_iter()
            .map(|cmd| cmd.into())
            .collect();
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "CommonEvents.json",
        text_units,
        parse_common_events,
        update_common_event,
    )
}
