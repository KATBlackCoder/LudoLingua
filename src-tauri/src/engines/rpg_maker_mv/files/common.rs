use crate::core::error::{AppError, AppResult};
use crate::models::engine::GameDataFile;
use crate::models::translation::{PromptType, TextUnit, TranslationStatus};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Utility to generate TextUnits for RPG Maker objects (actor, item, etc.)
///
/// - object_type: e.g. "actor", "item"
/// - object_id: the numeric id of the object
/// - file_path: the file path string
/// - index: index in the file array
/// - fields: Vec of (field_name, value, PromptType)
///
/// Returns Vec<TextUnit> for all non-empty, non-technical values.
pub fn extract_text_units_for_object(
    object_type: &str,
    object_id: i32,
    file_path: &str,
    index: usize,
    fields: Vec<(&str, &str, PromptType)>,
) -> Vec<TextUnit> {
    let mut units = Vec::new();
    for (field, value, prompt_type) in fields {
        // Skip empty values and technical content
        if !value.is_empty() && !is_technical_content(value) {
            units.push(TextUnit {
                id: format!("{}_{}_{}", object_type, object_id, field),
                source_text: value.to_string(),
                translated_text: String::new(),
                field_type: format!("{}:{}:{}", field, file_path, index),
                status: TranslationStatus::NotTranslated,
                prompt_type,
            });
        }
    }
    units
}



/// Common function to extract text from RPG Maker JSON files with object iteration.
///
/// This function handles the common pattern of:
/// 1. Parsing JSON into Vec<Option<T>>
/// 2. Iterating through objects
/// 3. Extracting text units for each object
///
/// # Arguments
///
/// * `project_path` - Path to the project root
/// * `file_path` - Relative path to the JSON file
/// * `file_name` - Human-readable name for the file (e.g., "Actors.json")
/// * `parse_function` - Function to parse JSON content into objects
/// * `extract_function` - Function to extract text units from each object
///
/// # Returns
///
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text_from_file_with_objects<T, P, E>(
    project_path: &Path,
    file_path: &str,
    file_name: &str,
    parse_function: P,
    extract_function: E,
) -> AppResult<GameDataFile>
where
    P: Fn(&str) -> AppResult<Vec<Option<T>>>,
    E: Fn(&T, usize, &str) -> Vec<TextUnit>,
{
    let full_path = project_path.join(file_path);
    log::debug!("Extracting text from {} at: {}", file_name, full_path.display());

    // Check if the file exists
    if !full_path.exists() {
        return Err(AppError::FileSystem(format!(
            "{} not found at {}",
            file_name, full_path.display()
        )));
    }

    // Read the JSON file
    let content = fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read {}: {}", file_name, e)))?;

    // Parse the JSON content
    let objects: Vec<Option<T>> = parse_function(&content)?;

    let mut text_units = Vec::new();

    // Process each object
    for (index, object_option) in objects.iter().enumerate() {
        if let Some(object) = object_option {
            let units = extract_function(object, index, file_path);
            text_units.extend(units);
        }
    }

    let text_unit_count = text_units.len() as u32;
    log::info!("Extracted {} text units from {}", text_unit_count, file_name);

    let file_stem = Path::new(file_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or(file_name)
        .to_string();

    Ok(GameDataFile {
        name: file_stem,
        path: file_path.to_string(),
        text_units,
        text_unit_count,
    })
}



/// Common function to inject translations into RPG Maker JSON files with object iteration.
///
/// This function handles the common pattern of:
/// 1. Parsing JSON into Vec<Option<T>>
/// 2. Creating a HashMap for text unit lookup
/// 3. Updating each object with translated text
/// 4. Serializing back to JSON
///
/// # Arguments
///
/// * `project_path` - Path to the project root
/// * `file_path` - Relative path to the JSON file
/// * `file_name` - Human-readable name for the file (e.g., "Actors.json")
/// * `text_units` - Vector of translated text units to inject
/// * `parse_function` - Function to parse JSON content into objects
/// * `update_function` - Function to update each object with translations
///
/// # Returns
///
/// * `AppResult<()>` - Success or error
pub fn inject_translations_into_file_with_objects<T, P, U>(
    project_path: &Path,
    file_path: &str,
    file_name: &str,
    text_units: &[&TextUnit],
    parse_function: P,
    update_function: U,
) -> AppResult<()>
where
    P: Fn(&str) -> AppResult<Vec<Option<T>>>,
    U: Fn(&mut T, &HashMap<String, &TextUnit>),
    T: serde::Serialize,
{
    let full_path = project_path.join(file_path);
    log::debug!("Injecting translations into {} at: {}", file_name, full_path.display());

    // Check if the file exists
    if !full_path.exists() {
        return Err(AppError::FileSystem(format!(
            "{} not found at {}",
            file_name, full_path.display()
        )));
    }

    // Read the current JSON file
    let content = fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read {}: {}", file_name, e)))?;

    // Parse the JSON content
    let mut objects: Vec<Option<T>> = parse_function(&content)?;

    // Create a map of text units for quick lookup
    let text_unit_map: HashMap<String, &TextUnit> = text_units
        .iter()
        .map(|unit| (unit.id.clone(), *unit))
        .collect();

    // Update each object with translated text
    for object_option in objects.iter_mut() {
        if let Some(object) = object_option {
            update_function(object, &text_unit_map);
        }
    }

    // Serialize the updated objects back to JSON
    let updated_content = serde_json::to_string_pretty(&objects)
        .map_err(|e| AppError::Parsing(format!("Failed to serialize {}: {}", file_name, e)))?;

    // Write the updated content back to the file
    fs::write(&full_path, updated_content)
        .map_err(|e| AppError::FileSystem(format!("Failed to write {}: {}", file_name, e)))?;

    log::info!(
        "Successfully injected {} translations into {}",
        text_units.len(),
        file_name
    );
    Ok(())
}

/// Utility to inject translated text back into RPG Maker objects
///
/// - object_type: e.g. "actor", "item"
/// - object_id: the numeric id of the object
/// - text_units: HashMap of text units keyed by ID
/// - fields: Vec of (field_name, field_reference) to update
///
/// Updates the object fields with translated text if available.
pub fn inject_text_units_for_object(
    object_type: &str,
    object_id: i32,
    text_units: &HashMap<String, &TextUnit>,
    fields: Vec<(&str, &mut String)>,
) {
    for (field_name, field_ref) in fields {
        let unit_id = format!("{}_{}_{}", object_type, object_id, field_name);
        if let Some(unit) = text_units.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *field_ref = unit.translated_text.clone();
            }
        }
    }
}

/// Represents a single event command for common event processing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventCommand {
    /// Command code (101 = Show Text, 401 = Message, 108 = Comment, etc.)
    pub code: i32,
    /// Indentation level
    pub indent: i32,
    /// Command parameters (array of values)
    pub parameters: Vec<serde_json::Value>,
}

/// Extracts text units from event commands
///
/// # Arguments
/// * `object_type` - Type of object (e.g., "common_event", "troop")
/// * `object_id` - ID of the object
/// * `commands` - Vector of event commands to process
/// * `_file_path` - File path for context (unused but kept for consistency)
///
/// # Returns
/// * `Vec<TextUnit>` - Vector of extracted text units
pub fn extract_text_units_from_event_commands(
    object_type: &str,
    object_id: i32,
    commands: &[EventCommand],
    _file_path: &str,
) -> Vec<TextUnit> {
    let mut text_units = Vec::new();

    for (command_index, command) in commands.iter().enumerate() {
        match command.code {
            101 => {
                // Show Text - Message window attributes
                // Parameters: [0] = window type, [1] = position, [2] = background, [3] = position type
                // No translatable text in this command
            }
            401 => {
                // Show Text - Message content
                if let Some(text_param) = command.parameters.get(0) {
                    if let Some(text) = text_param.as_str() {
                        if !text.is_empty() && !is_technical_content(text) {
                            text_units.push(TextUnit {
                                id: format!("{}_{}_message_{}", object_type, object_id, command_index),
                                source_text: text.to_string(),
                                translated_text: String::new(),
                                field_type: "message".to_string(),
                                status: TranslationStatus::NotTranslated,
                                prompt_type: PromptType::Dialogue,
                            });
                        }
                    }
                }
            }
            // 108 => {
            //     // Comment - Developer comments, not player-facing text
            //     // Skipping these as they are internal documentation
            // }
            // Add more specific command codes here as needed
            // 356 => { /* Change Enemy Name */ }
            // 357 => { /* Change Enemy HP */ }
            // etc.
            _ => {
                // Skip all other command codes - they don't contain translatable text
                // This is much safer than trying to guess what might be translatable
            }
        }
    }

    text_units
}

/// Checks if content is technical and shouldn't be translated
///
/// # Arguments
/// * `content` - The content to check
///
/// # Returns
/// * `bool` - True if the content is technical and should be skipped
fn is_technical_content(content: &str) -> bool {
    let content = content.trim();
    
    // Skip empty or whitespace-only content
    if content.is_empty() {
        return true;
    }
    
    // Skip pure formatting codes (like "\\n[2]" alone)
    if content == "\\n[1]" || content == "\\n[2]" || content == "\\n[3]" || content == "\\n[4]" || content == "\\n[5]" {
        return true;
    }
    
    // Skip file names and extensions (images, sounds, etc.)
    // But be careful not to skip RPG Maker formatting codes like "\\n[2]"
    if content.contains('.') || content.contains('/') || 
       (content.contains('\\') && !content.contains("\\n[")) {
        return true;
    }
    
    // Skip JavaScript code and expressions
    if content.contains("user.") || content.contains("use.") || content.contains("&&") || content.contains("==") {
        return true;
    }
    
    // Skip technical markers
    if content == "終わり" || content == "==" || content.starts_with("==") {
        return true;
    }
    
    // Skip sound effect names (usually English words)
    if content.chars().all(|c| c.is_ascii_alphabetic()) && content.len() <= 20 {
        return true;
    }
    
    // Skip numeric-only content
    if content.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }
    
    // Skip very short content that's likely technical
    // But allow Japanese/Chinese characters even if short
    if content.len() <= 3 {
        // If it contains non-ASCII characters or Japanese punctuation, it might be translatable
        if content.chars().any(|c| c.is_alphabetic() && !c.is_ascii()) ||
           content.contains('・') || content.contains('…') || content.contains('。') {
            return false;
        }
        return true;
    }
    
    // If content contains Japanese characters or other translatable text, allow it
    // even if it also contains formatting codes
    if content.chars().any(|c| c.is_alphabetic() && !c.is_ascii()) {
        return false;
    }
    
    // If content contains common Japanese punctuation or quotes, allow it
    if content.contains('「') || content.contains('」') || content.contains('、') || 
       content.contains('。') || content.contains('・') || content.contains('…') {
        return false;
    }
    
    false
}



/// Injects translated text back into event commands
///
/// # Arguments
/// * `object_type` - Type of object (e.g., "common_event", "troop")
/// * `object_id` - ID of the object
/// * `commands` - Vector of event commands to update
/// * `text_unit_map` - HashMap of text units for lookup
///
/// Updates the command parameters with translated text if available.
pub fn inject_text_units_into_event_commands(
    object_type: &str,
    object_id: i32,
    commands: &mut [EventCommand],
    text_unit_map: &HashMap<String, &TextUnit>,
) {
    for (command_index, command) in commands.iter_mut().enumerate() {
        match command.code {
            401 => {
                // Show Text - Message content
                if let Some(text_param) = command.parameters.get_mut(0) {
                    if text_param.as_str().is_some() {
                        let unit_id = format!("{}_{}_message_{}", object_type, object_id, command_index);
                        if let Some(text_unit) = text_unit_map.get(&unit_id) {
                            *text_param = serde_json::Value::String(text_unit.translated_text.clone());
                        }
                    }
                }
            }
            // 108 => {
            //     // Comment - Developer comments, not player-facing text
            //     // Skipping these as they are internal documentation
            // }
            // Add more specific command codes here as needed
            // 356 => { /* Change Enemy Name */ }
            // 357 => { /* Change Enemy HP */ }
            // etc.
            _ => {
                // Skip all other command codes - they don't contain translatable text
            }
        }
    }
}
