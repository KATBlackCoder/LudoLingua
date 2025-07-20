use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::core::error::{AppError, AppResult};
use crate::models::engine::GameDataFile;
use crate::models::translation::{PromptType, TextUnit, TranslationStatus};
use super::common::extract_text_units_for_object;

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
    let full_path = project_path.join(file_path);
    debug!("Extracting text from Items.json at: {}", full_path.display());

    if !full_path.exists() {
        return Err(AppError::FileSystem(format!(
            "Items.json not found at {}",
            full_path.display()
        )));
    }

    let content = fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read Items.json: {}", e)))?;

    let items: Vec<Option<Item>> = serde_json::from_str(&content)
        .map_err(|e| AppError::Parsing(format!("Failed to parse Items.json: {}", e)))?;

    let mut text_units = Vec::new();

    for (index, item_option) in items.iter().enumerate() {
        if let Some(item) = item_option {
            debug!("Processing item: {} (ID: {})", item.name, item.id);
            let units = extract_text_units_for_object(
                "item",
                item.id,
                file_path,
                index,
                vec![
                    ("name", &item.name, PromptType::Name),
                    ("description", &item.description, PromptType::Description),
                    ("note", &item.note, PromptType::Description),
                ],
            );
            text_units.extend(units);
        }
    }

    let text_unit_count = text_units.len() as u32;
    info!("Extracted {} text units from Items.json", text_unit_count);

    let file_name = Path::new(file_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Items")
        .to_string();

    Ok(GameDataFile {
        name: file_name,
        path: file_path.to_string(),
        text_units,
        text_unit_count,
    })
} 