use serde::{Deserialize, Serialize};
use std::path::Path;

use super::common::{
    extract_text_from_file_with_objects, extract_text_units_for_object, 
    inject_text_units_for_object, inject_translations_into_file_with_objects
};
use crate::core::error::{AppError, AppResult};
use crate::models::engine::GameDataFile;
use crate::models::translation::{PromptType, TextUnit};
use std::collections::HashMap;

/// Represents an actor (character) in RPG Maker MV.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Actor {
    /// The ID of the actor
    #[serde(default)]
    pub id: i32,

    /// The name of the actor
    #[serde(default)]
    pub name: String,

    /// The nickname of the actor
    #[serde(default)]
    pub nickname: String,

    /// The class ID of the actor
    #[serde(default, rename = "classId")]
    pub class_id: i32,

    /// The initial level of the actor
    #[serde(default, rename = "initialLevel")]
    pub initial_level: i32,

    /// The maximum level of the actor
    #[serde(default, rename = "maxLevel")]
    pub max_level: i32,

    /// The character file name
    #[serde(default, rename = "characterName")]
    pub character_name: String,

    /// The character index
    #[serde(default, rename = "characterIndex")]
    pub character_index: i32,

    /// The face file name
    #[serde(default, rename = "faceName")]
    pub face_name: String,

    /// The face index
    #[serde(default, rename = "faceIndex")]
    pub face_index: i32,

    /// The battler file name
    #[serde(default, rename = "battlerName")]
    pub battler_name: String,

    /// The profile text of the actor
    #[serde(default)]
    pub profile: String,

    /// The note field which may contain additional text
    #[serde(default)]
    pub note: String,

    /// Equipment slots
    #[serde(default)]
    pub equips: Vec<i32>,

    /// Character traits
    #[serde(default)]
    pub traits: Vec<Trait>,

    /// Additional fields that might be present in the JSON
    #[serde(flatten)]
    pub extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

/// Represents a trait (characteristic) of an actor in RPG Maker MV.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Trait {
    /// The code identifying the trait type
    #[serde(default)]
    pub code: i32,

    /// The data ID for the trait
    #[serde(default, rename = "dataId")]
    pub data_id: i32,

    /// The value of the trait
    #[serde(default)]
    pub value: f64,
}

/// Extracts text units from an Actors.json file and organizes them into a GameDataFile.
///
/// # Arguments
///
/// * `project_path` - The path to the project directory
/// * `file_path` - The path to the Actors.json file, relative to the project directory
///
/// # Returns
///
/// * `AppResult<GameDataFile>` - A GameDataFile containing all extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Actors.json
    let parse_actors = |content: &str| -> AppResult<Vec<Option<Actor>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Actors.json: {}", e)))
    };

    // Extract function for each actor
    let extract_actor_units = |actor: &Actor, index: usize, file_path: &str| -> Vec<TextUnit> {
        extract_text_units_for_object(
            "actor",
            actor.id,
            file_path,
            index,
            vec![
                ("name", &actor.name, PromptType::Character),
                ("nickname", &actor.nickname, PromptType::Character),
                ("profile", &actor.profile, PromptType::Character),
            ],
        )
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Actors.json",
        parse_actors,
        extract_actor_units,
    )
}

/// Injects translated text units back into the Actors.json file.
///
/// # Arguments
///
/// * `project_path` - The path to the project directory
/// * `file_path` - The path to the Actors.json file, relative to the project directory
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
    // Parse function for Actors.json
    let parse_actors = |content: &str| -> AppResult<Vec<Option<Actor>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Actors.json: {}", e)))
    };

    // Update function for each actor
    let update_actor = |actor: &mut Actor, text_unit_map: &HashMap<String, &TextUnit>| {
        inject_text_units_for_object(
            "actor",
            actor.id,
            text_unit_map,
            vec![
                ("name", &mut actor.name),
                ("nickname", &mut actor.nickname),
                ("profile", &mut actor.profile),
            ],
        );
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Actors.json",
        text_units,
        parse_actors,
        update_actor,
    )
}
