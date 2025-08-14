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

/// Represents a skill in RPG Maker MV.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Skill {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub message1: String,
    #[serde(default)]
    pub message2: String,
    #[serde(default)]
    pub note: String,
    #[serde(flatten)]
    pub extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

/// Extracts text units from Skills.json file.
///
/// # Arguments
///
/// * `project_path` - Path to the project root
/// * `file_path` - Relative path to the Skills.json file
///
/// # Returns
///
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Skills.json
    let parse_skills = |content: &str| -> AppResult<Vec<Option<Skill>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Skills.json: {}", e)))
    };

    // Extract function for each skill
    let extract_skill_units = |skill: &Skill, index: usize, file_path: &str| -> Vec<TextUnit> {
        extract_text_units_for_object(
            "skill",
            skill.id,
            file_path,
            index,
            vec![
                ("name", &skill.name, PromptType::Skill),
                ("description", &skill.description, PromptType::Skill),
                ("message1", &skill.message1, PromptType::Skill),
                ("message2", &skill.message2, PromptType::Skill),
            ],
        )
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Skills.json",
        parse_skills,
        extract_skill_units,
    )
}

/// Injects translated text units back into Skills.json file.
///
/// # Arguments
///
/// * `project_path` - Path to the project root
/// * `file_path` - Relative path to the Skills.json file
/// * `text_units` - Vector of translated text units to inject
///
/// # Returns
///
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    // Parse function for Skills.json
    let parse_skills = |content: &str| -> AppResult<Vec<Option<Skill>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Skills.json: {}", e)))
    };

    // Update function for each skill
    let update_skill = |skill: &mut Skill, text_unit_map: &HashMap<String, &TextUnit>| {
        inject_text_units_for_object(
            "skill",
            skill.id,
            text_unit_map,
            vec![
                ("name", &mut skill.name),
                ("description", &mut skill.description),
                ("message1", &mut skill.message1),
                ("message2", &mut skill.message2),
            ],
        );
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Skills.json",
        text_units,
        parse_skills,
        update_skill,
    )
}
