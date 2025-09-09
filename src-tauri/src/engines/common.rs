use log::{debug, info, warn};
use std::path::Path;

use crate::core::error::AppResult;
use crate::models::engine::{EngineInfo, GameDataFile};
use crate::models::translation::TextUnit;

/// Common helper function to extract text from a specific file type.
///
/// This function provides a reusable way to extract text from any file type,
/// eliminating repetitive code patterns across different engine implementations.
///
/// # Arguments
///
/// * `project_info` - Information about the project
/// * `file_paths` - Array of possible file paths to check
/// * `extract_function` - Function to extract text from the file
/// * `file_type_name` - Name of the file type for logging
///
/// # Returns
///
/// * `AppResult<Vec<GameDataFile>>` - Vector of extracted game data files
pub fn extract_file_type_text<F>(
    project_info: &EngineInfo,
    file_paths: &[&str],
    extract_function: F,
    file_type_name: &str,
) -> AppResult<Vec<GameDataFile>>
where
    F: Fn(&Path, &str) -> AppResult<GameDataFile>,
{
    let mut game_data_files = Vec::new();
    let mut file_found = false;

    for path in file_paths.iter() {
        let file_path = project_info.path.join(path);
        if file_path.exists() {
            debug!("Found {} at: {}", file_type_name, file_path.display());

            match extract_function(&project_info.path, path) {
                Ok(game_data_file) => {
                    info!(
                        "Extracted {} text units from {}",
                        game_data_file.text_unit_count, path
                    );

                    game_data_files.push(game_data_file);
                    file_found = true;
                    break; // Found and processed, no need to check other paths
                }
                Err(e) => {
                    warn!("Failed to extract text from {}: {}", path, e);
                    // Continue trying other paths
                }
            }
        }
    }

    if !file_found {
        warn!("Could not find {} in any expected location", file_type_name);
    }

    Ok(game_data_files)
}

/// Common helper function to inject translations into a specific file type.
///
/// This function provides a reusable way to inject translations into any file type,
/// eliminating repetitive code patterns across different engine implementations.
///
/// # Arguments
///
/// * `project_info` - Information about the project
/// * `text_units` - Vector of translated text units to inject
/// * `file_type_prefix` - The prefix to filter text units (e.g., "actor_", "item_")
/// * `file_paths` - Array of possible file paths to check
/// * `inject_function` - Function to call for injection
/// * `file_type_name` - Human-readable name for logging (e.g., "actor", "item")
///
/// # Returns
///
/// * `AppResult<()>` - Success or error
pub fn inject_file_type_translations<F>(
    project_info: &EngineInfo,
    text_units: &[TextUnit],
    file_type_prefix: &str,
    file_paths: &[&str],
    inject_function: F,
    file_type_name: &str,
) -> AppResult<()>
where
    F: Fn(&Path, &str, &[&TextUnit]) -> AppResult<()>,
{
    // Filter text units by file type to inject only relevant ones
    let filtered_units: Vec<&TextUnit> = text_units
        .iter()
        .filter(|unit| unit.id.starts_with(file_type_prefix))
        .collect();

    if filtered_units.is_empty() {
        return Ok(()); // No units to inject for this file type
    }

    let mut injected = false;

    for path in file_paths.iter() {
        let file_path = project_info.path.join(path);
        if file_path.exists() {
            debug!(
                "Injecting into {} at: {}",
                file_type_name,
                file_path.display()
            );

            // Inject translations using the provided function
            match inject_function(&project_info.path, path, &filtered_units) {
                Ok(()) => {
                    info!(
                        "Successfully injected {} {} translations into {}",
                        filtered_units.len(),
                        file_type_name,
                        path
                    );
                    injected = true;
                    break; // We successfully injected, no need to check other paths
                }
                Err(e) => {
                    warn!("Failed to inject translations into {}: {}", path, e);
                    // Continue trying other paths
                }
            }
        }
    }

    if !injected && !filtered_units.is_empty() {
        warn!(
            "Could not inject {} translations - {} not found in any expected location",
            file_type_name, file_type_name
        );
    }

    Ok(())
}

