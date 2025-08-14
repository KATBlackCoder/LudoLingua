use super::common::{
    extract_text_from_file_with_objects, extract_text_units_for_object,
    inject_text_units_for_object, inject_translations_into_file_with_objects,
};
use crate::core::error::{AppError, AppResult};
use crate::models::engine::GameDataFile;
use crate::models::translation::{PromptType, TextUnit};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Represents a map information entry from MapInfos.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapInfo {
    /// Map ID (can be null for empty entries)
    pub id: Option<i32>,
    /// Whether the map is expanded in the editor
    pub expanded: Option<bool>,
    /// Map name (translatable)
    pub name: String,
    /// Display order
    pub order: Option<i32>,
    /// Parent map ID
    pub parent_id: Option<i32>,
    /// Scroll X position
    pub scroll_x: Option<f64>,
    /// Scroll Y position
    pub scroll_y: Option<f64>,
}

/// Extracts text units from a MapInfos.json file and organizes them into a GameDataFile.
///
/// # Arguments
///
/// * `project_path` - The path to the project directory
/// * `file_path` - The path to the MapInfos.json file, relative to the project directory
///
/// # Returns
///
/// * `AppResult<GameDataFile>` - A GameDataFile containing all extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for MapInfos.json
    let parse_map_infos = |content: &str| -> AppResult<Vec<Option<MapInfo>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse MapInfos.json: {}", e)))
    };

    // Extract function for each map info
    let extract_map_info_units =
        |map_info: &MapInfo, index: usize, file_path: &str| -> Vec<TextUnit> {
            extract_text_units_for_object(
                "map_info",
                map_info.id.unwrap(),
                file_path,
                index,
                vec![("name", &map_info.name, PromptType::Character)],
            )
        };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "MapInfos.json",
        parse_map_infos,
        extract_map_info_units,
    )
}

/// Injects translated text units back into the MapInfos.json file.
///
/// # Arguments
///
/// * `project_path` - The path to the project directory
/// * `file_path` - The path to the MapInfos.json file, relative to the project directory
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
    // Parse function for MapInfos.json
    let parse_map_infos = |content: &str| -> AppResult<Vec<Option<MapInfo>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse MapInfos.json: {}", e)))
    };

    // Update function for each map info
    let update_map_info = |map_info: &mut MapInfo, text_unit_map: &HashMap<String, &TextUnit>| {
        inject_text_units_for_object(
            "map_info",
            map_info.id.unwrap(),
            text_unit_map,
            vec![("name", &mut map_info.name)],
        );
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "MapInfos.json",
        text_units,
        parse_map_infos,
        update_map_info,
    )
}
