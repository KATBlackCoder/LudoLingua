use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::HashMap;
use std::fs;
use crate::core::error::{AppError, AppResult};
use crate::models::translation::{TextUnit, PromptType};
use crate::models::engine::GameDataFile;
use super::common::{extract_text_from_file_with_objects, extract_text_units_for_object, inject_text_units_for_object, extract_text_units_from_event_commands, inject_text_units_into_event_commands, EventCommand};

/// Extract map ID from file path (e.g., "Map004.json" -> "004")
/// 
/// # Arguments
/// * `file_path` - Relative path to the MapXXX.json file
/// 
/// # Returns
/// * `&str` - The map ID as a string slice
pub fn extract_map_id(file_path: &str) -> &str {
    file_path
        .split('/')
        .last()
        .unwrap_or("")
        .strip_prefix("Map")
        .and_then(|s| s.strip_suffix(".json"))
        .unwrap_or("unknown")
}

/// Represents a single page in a map event from RPG Maker MV MapXXX.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventPage {
    /// Page conditions (not translatable)
    #[serde(default)]
    pub conditions: HashMap<String, serde_json::Value>,

    /// List of event commands
    #[serde(default)]
    pub list: Vec<EventCommand>,

    /// Page image (not translatable)
    #[serde(default)]
    pub image: HashMap<String, serde_json::Value>,

    /// Page move type (not translatable)
    #[serde(default)]
    pub move_type: i32,

    /// Page move speed (not translatable)
    #[serde(default)]
    pub move_speed: i32,

    /// Page move frequency (not translatable)
    #[serde(default)]
    pub move_frequency: i32,

    /// Page move route (not translatable)
    #[serde(default)]
    pub move_route: Vec<HashMap<String, serde_json::Value>>,

    /// Page walk animation (not translatable)
    #[serde(default)]
    pub walk_anime: bool,

    /// Page step animation (not translatable)
    #[serde(default)]
    pub step_anime: bool,

    /// Page direction fix (not translatable)
    #[serde(default)]
    pub direction_fix: bool,

    /// Page through (not translatable)
    #[serde(default)]
    pub through: bool,

    /// Page priority (not translatable)
    #[serde(default)]
    pub priority_type: i32,

    /// Page trigger (not translatable)
    #[serde(default)]
    pub trigger: i32,

    /// Additional fields that might be present in the JSON
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

/// Represents a single event in a map from RPG Maker MV MapXXX.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MapEvent {
    /// Event ID
    #[serde(default)]
    pub id: i32,

    /// Event name
    #[serde(default)]
    pub name: String,

    /// Event note
    #[serde(default)]
    pub note: String,

    /// Event pages
    #[serde(default)]
    pub pages: Vec<EventPage>,

    /// Additional fields that might be present in the JSON
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

/// Represents a map from RPG Maker MV MapXXX.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Map {
    /// Map events
    #[serde(default)]
    pub events: Vec<Option<MapEvent>>,

    /// Map width (not translatable)
    #[serde(default)]
    pub width: i32,

    /// Map height (not translatable)
    #[serde(default)]
    pub height: i32,

    /// Map scroll type (not translatable)
    #[serde(default, rename = "scrollType")]
    pub scroll_type: i32,

    /// Map autoplay BGM (not translatable)
    #[serde(default, rename = "autoplayBgm")]
    pub autoplay_bgm: bool,

    /// Map BGM (not translatable)
    #[serde(default)]
    pub bgm: HashMap<String, serde_json::Value>,

    /// Map autoplay BGS (not translatable)
    #[serde(default, rename = "autoplayBgs")]
    pub autoplay_bgs: bool,

    /// Map BGS (not translatable)
    #[serde(default)]
    pub bgs: HashMap<String, serde_json::Value>,

    /// Map disable dashing (not translatable)
    #[serde(default, rename = "disableDashing")]
    pub disable_dashing: bool,

    /// Map encounter list (not translatable)
    #[serde(default, rename = "encounterList")]
    pub encounter_list: Vec<HashMap<String, serde_json::Value>>,

    /// Map encounter step (not translatable)
    #[serde(default, rename = "encounterStep")]
    pub encounter_step: i32,

    /// Map parallax name (not translatable)
    #[serde(default, rename = "parallaxName")]
    pub parallax_name: String,

    /// Map parallax loop x (not translatable)
    #[serde(default, rename = "parallaxLoopX")]
    pub parallax_loop_x: bool,

    /// Map parallax loop y (not translatable)
    #[serde(default, rename = "parallaxLoopY")]
    pub parallax_loop_y: bool,

    /// Map parallax sx (not translatable)
    #[serde(default, rename = "parallaxSx")]
    pub parallax_sx: i32,

    /// Map parallax sy (not translatable)
    #[serde(default, rename = "parallaxSy")]
    pub parallax_sy: i32,

    /// Map data (not translatable)
    #[serde(default)]
    pub data: Vec<i32>,

    /// Additional fields that might be present in the JSON
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

/// Discover all MapXXX.json files in the project
/// 
/// # Arguments
/// * `project_path` - Path to the project directory
/// 
/// # Returns
/// * `AppResult<Vec<String>>` - Vector of discovered map file paths
pub fn discover_map_files(project_path: &Path) -> AppResult<Vec<String>> {
    use std::fs;
    
    let data_path = project_path.join("www/data");
    if !data_path.exists() {
        return Ok(Vec::new());
    }

    let mut map_files = Vec::new();
    
    match fs::read_dir(&data_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    if let Some(name) = file_name.to_str() {
                        if name.starts_with("Map") && name.ends_with(".json") {
                            let relative_path = format!("www/data/{}", name);
                            map_files.push(relative_path);
                        }
                    }
                }
            }
        }
        Err(e) => {
            return Err(AppError::FileSystem(format!("Failed to read data directory: {}", e)));
        }
    }

    // Sort files to ensure consistent order
    map_files.sort();
    Ok(map_files)
}

/// Extract text units from a single MapXXX.json file
/// 
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the MapXXX.json file
/// 
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Extract map ID from file path
    let map_id = extract_map_id(file_path);

    // Parse function for MapXXX.json
    let parse_map = |content: &str| -> AppResult<Vec<Option<Map>>> {
        let map: Map = serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse {}: {}", file_path, e)))?;
        Ok(vec![Some(map)])
    };

    // Extract function for the map
    let extract_map_units = |map: &Map, _index: usize, file_path: &str| -> Vec<TextUnit> {
        let mut text_units = Vec::new();
        
        // Extract text from events
        for event_option in &map.events {
            if let Some(event) = event_option {
                // Extract event name and messages
                text_units.extend(extract_text_units_for_object(
                    &format!("map_{}_event", map_id),
                    event.id,
                    file_path,
                    event.id as usize,
                    vec![
                        ("name", &event.name, PromptType::Character),
                    ],
                ));

                // Extract text from event pages using common function
                for (_page_index, page) in event.pages.iter().enumerate() {
                    text_units.extend(extract_text_units_from_event_commands(
                        &format!("map_{}_event", map_id),
                        event.id,
                        &page.list,
                        file_path,
                    ));
                }
            }
        }
        
        text_units
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        file_path,
        parse_map,
        extract_map_units,
    )
}

/// Inject translations back into a MapXXX.json file
/// 
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the MapXXX.json file
/// * `text_units` - Vector of translated text units to inject
/// 
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    log::info!("Starting injection for {} with {} text units", file_path, text_units.len());
    
    // Extract map ID from file path
    let map_id = extract_map_id(file_path);
    log::info!("Extracted map_id: {}", map_id);

    let full_path = project_path.join(file_path);
    log::debug!("Injecting translations into {} at: {}", file_path, full_path.display());

    // Check if the file exists
    if !full_path.exists() {
        return Err(AppError::FileSystem(format!(
            "{} not found at {}",
            file_path, full_path.display()
        )));
    }

    // Read the current JSON file
    let content = fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read {}: {}", file_path, e)))?;

    // Parse the JSON content directly as a Map (not wrapped in Vec<Option<>>)
    let mut map: Map = serde_json::from_str(&content)
        .map_err(|e| AppError::Parsing(format!("Failed to parse {}: {}", file_path, e)))?;

    // Create a map of text units for quick lookup
    let text_unit_map: HashMap<String, &TextUnit> = text_units
        .iter()
        .map(|unit| (unit.id.clone(), *unit))
        .collect();

    // Update the map with translated text
    log::info!("Updating map with {} events", map.events.len());
    
    // Update events
    for event_option in &mut map.events {
        if let Some(event) = event_option {
            log::info!("Processing event {} with name: '{}'", event.id, event.name);
            
            // Update event name
            inject_text_units_for_object(
                &format!("map_{}_event", map_id),
                event.id,
                &text_unit_map,
                vec![
                    ("name", &mut event.name),
                ],
            );

            // Update event pages using common function
            for (page_index, page) in event.pages.iter_mut().enumerate() {
                log::info!("Processing page {} with {} commands", page_index, page.list.len());
                inject_text_units_into_event_commands(
                    &format!("map_{}_event", map_id),
                    event.id,
                    &mut page.list,
                    &text_unit_map,
                );
            }
        }
    }

    // Serialize the updated map back to JSON (direct object, not wrapped in array)
    let updated_content = serde_json::to_string_pretty(&map)
        .map_err(|e| AppError::Parsing(format!("Failed to serialize {}: {}", file_path, e)))?;

    // Write the updated content back to the file
    fs::write(&full_path, updated_content)
        .map_err(|e| AppError::FileSystem(format!("Failed to write {}: {}", file_path, e)))?;

    log::info!(
        "Successfully injected {} translations into {}",
        text_units.len(),
        file_path
    );
    Ok(())
}
