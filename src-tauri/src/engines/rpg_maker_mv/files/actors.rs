use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use log::{info, debug, error};

use crate::core::error::{AppError, AppResult};
use crate::models::translation::{TextUnit, TranslationStatus, PromptType};
use crate::models::engine::GameDataFile;

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
    let full_path = project_path.join(file_path);
    debug!("Extracting text from Actors.json at: {}", full_path.display());
    
    // Check if the file exists
    if !full_path.exists() {
        return Err(AppError::FileSystem(format!(
            "Actors.json not found at {}",
            full_path.display()
        )));
    }
    
    // Read the file content
    let content = fs::read_to_string(&full_path).map_err(|e| {
        AppError::FileSystem(format!(
            "Failed to read Actors.json: {}",
            e
        ))
    })?;
    
    // Parse the JSON - Actors.json is an array of actors, with the first element being null
    let actors: Vec<Option<Actor>> = serde_json::from_str(&content).map_err(|e| {
        AppError::Parsing(format!(
            "Failed to parse Actors.json: {}",
            e
        ))
    })?;
    
    let mut text_units = Vec::new();
    
    // Process each actor
    for (index, actor_option) in actors.iter().enumerate() {
        if let Some(actor) = actor_option {
            debug!("Processing actor: {} (ID: {})", actor.name, actor.id);
            
            // Extract name
            if !actor.name.is_empty() {
                text_units.push(TextUnit {
                    id: format!("actor_{}_name", actor.id),
                    source_text: actor.name.clone(),
                    translated_text: String::new(),
                    field_type: format!("name:{}:{}", file_path, index),
                    status: TranslationStatus::NotTranslated,
                    prompt_type: PromptType::Name,
                });
            }
            
            // Extract nickname
            if !actor.nickname.is_empty() {
                text_units.push(TextUnit {
                    id: format!("actor_{}_nickname", actor.id),
                    source_text: actor.nickname.clone(),
                    translated_text: String::new(),
                    field_type: format!("nickname:{}:{}", file_path, index),
                    status: TranslationStatus::NotTranslated,
                    prompt_type: PromptType::Name,
                });
            }
            
            // Extract profile
            if !actor.profile.is_empty() {
                text_units.push(TextUnit {
                    id: format!("actor_{}_profile", actor.id),
                    source_text: actor.profile.clone(),
                    translated_text: String::new(),
                    field_type: format!("profile:{}:{}", file_path, index),
                    status: TranslationStatus::NotTranslated,
                    prompt_type: PromptType::Description,
                });
            }
            
            // Extract note
            if !actor.note.is_empty() {
                text_units.push(TextUnit {
                    id: format!("actor_{}_note", actor.id),
                    source_text: actor.note.clone(),
                    translated_text: String::new(),
                    field_type: format!("note:{}:{}", file_path, index),
                    status: TranslationStatus::NotTranslated,
                    prompt_type: PromptType::Description,
                });
            }
        }
    }
    
    let text_unit_count = text_units.len() as u32;
    info!("Extracted {} text units from Actors.json", text_unit_count);
    
    // Get the file name without the path
    let file_name = Path::new(file_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Actors")
        .to_string();
    
    // Create and return the GameDataFile
    Ok(GameDataFile {
        name: file_name,
        path: file_path.to_string(),
        text_units,
        text_unit_count,
    })
} 