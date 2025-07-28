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
/// Returns Vec<TextUnit> for all non-empty values.
pub fn extract_text_units_for_object(
    object_type: &str,
    object_id: i32,
    file_path: &str,
    index: usize,
    fields: Vec<(&str, &str, PromptType)>,
) -> Vec<TextUnit> {
    let mut units = Vec::new();
    for (field, value, prompt_type) in fields {
        if !value.is_empty() {
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
