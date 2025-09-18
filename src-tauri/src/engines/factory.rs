use std::path::Path;

use crate::core::engine::Engine;
use crate::core::error::{AppError, AppResult};
use crate::engines::rpg_maker_mv::engine::RpgMakerMvEngine;
use crate::engines::rpg_maker_mz::engine::RpgMakerMzEngine;
use crate::engines::wolf_rpg::engine::WolfRpgEngine;
use crate::models::engine::{EngineCriteria, EngineType};

/// Factory function to get the appropriate engine implementation based on project path.
///
/// This function analyzes the given project directory and determines which game engine
/// implementation should be used to handle the project files.
///
/// # Arguments
///
/// * `project_path` - Path to the root directory of the game project
///
/// # Returns
///
/// * `AppResult<Box<dyn Engine>>` - A boxed trait object implementing the Engine trait
///
/// # Errors
///
/// Returns an error if:
/// * The project path doesn't exist
/// * The project path is not a directory
/// * The project type cannot be determined
pub fn get_engine(project_path: &Path) -> AppResult<Box<dyn Engine>> {
    // Check if the path exists and is a directory
    if !project_path.exists() {
        return Err(AppError::FileSystem(format!(
            "Project path does not exist: {}",
            project_path.display()
        )));
    }

    if !project_path.is_dir() {
        return Err(AppError::FileSystem(format!(
            "Project path is not a directory: {}",
            project_path.display()
        )));
    }

    // Detect the engine type based on project structure
    let engine_type = detect_engine_type(project_path)?;

    // Create the engine using the detected type
    create_engine_from_type(engine_type)
}

/// Factory function to get the appropriate engine implementation based on engine type.
///
/// This function creates the appropriate engine implementation for a known engine type.
/// This is more efficient than `get_engine()` when you already know the engine type.
///
/// # Arguments
///
/// * `engine_type` - The known engine type
///
/// # Returns
///
/// * `AppResult<Box<dyn Engine>>` - A boxed trait object implementing the Engine trait
///
/// # Errors
///
/// Returns an error if:
/// * The engine type is Unknown or unsupported
pub fn get_engine_from_type(engine_type: EngineType) -> AppResult<Box<dyn Engine>> {
    create_engine_from_type(engine_type)
}

/// Private helper function to create an engine from a known engine type.
/// This contains the common logic shared between get_engine and get_engine_from_type.
fn create_engine_from_type(engine_type: EngineType) -> AppResult<Box<dyn Engine>> {
    match engine_type {
        EngineType::RpgMakerMv => Ok(Box::new(RpgMakerMvEngine::new()) as Box<dyn Engine>),
        EngineType::RpgMakerMz => Ok(Box::new(RpgMakerMzEngine::new()) as Box<dyn Engine>),
        EngineType::WolfRpg => Ok(Box::new(WolfRpgEngine::new()) as Box<dyn Engine>),
        EngineType::Unknown => Err(AppError::Other(
            "Unknown engine type - cannot create engine".to_string(),
        )),
    }
}

/// Extract game data files using engine-specific logic.
/// This function handles the engine dispatch automatically based on engine type.
pub fn extract_game_data_files(
    project_info: &crate::models::engine::EngineInfo,
) -> AppResult<Vec<crate::models::engine::GameDataFile>> {
    match project_info.engine_type {
        EngineType::RpgMakerMv => {
            let engine = RpgMakerMvEngine::new();
            engine.extract_game_data_files(project_info)
        }
        EngineType::RpgMakerMz => {
            let engine = RpgMakerMzEngine::new();
            engine.extract_game_data_files(project_info)
        }
        EngineType::WolfRpg => Err(AppError::Other(
            "Wolf RPG does not support structured game data file extraction".to_string(),
        )),
        EngineType::Unknown => Err(AppError::Other(
            "Unknown engine type - cannot extract game data files".to_string(),
        )),
    }
}

/// Export translated text units using engine-specific logic.
/// This function handles the engine dispatch automatically based on engine type.
pub async fn export_translated_subset(
    project_info: &crate::models::engine::EngineInfo,
    db: &crate::db::state::ManagedTranslationState,
    destination_root: &str,
) -> AppResult<String> {
    use log::info;

    info!("Starting export to: {}", destination_root);

    // Get manifest hash from project path
    let manifest = crate::db::translation::manifest::create_or_load_project_manifest(project_info)
        .map_err(|e| AppError::Other(format!("Failed to load manifest: {}", e)))?;

    let manifest_hash = manifest.project_id.clone();

    // Find all translated units for this project
    let translated_records =
        crate::db::translation::repo::find_translated_units_for_export(db, &manifest_hash)
            .await
            .map_err(|e| AppError::Other(format!("Failed to query translated units: {}", e)))?;

    if translated_records.is_empty() {
        return Err(AppError::Other(
            "No translated units found for export".to_string(),
        ));
    }

    info!(
        "Found {} translated units for export",
        translated_records.len()
    );

    // Get engine for this project type
    let engine = create_engine_from_type(project_info.engine_type.clone())?;

    // Convert database records to TextUnit for engine injection
    let text_units: Vec<crate::models::translation::TextUnit> = translated_records
        .into_iter()
        .filter_map(|record| {
            record.translated_text.as_ref().and_then(|translated| {
                if translated.is_empty() {
                    None
                } else {
                    // Use engine's own ID reconstruction method
                    match engine.reconstruct_text_unit_id(
                        &record.field_type,
                        &record.source_text,
                        translated,
                    ) {
                        Ok(mut text_unit) => {
                            // Set the correct status from database record
                            text_unit.status = match record.status.as_str() {
                                "MachineTranslated" => {
                                    crate::models::translation::TranslationStatus::MachineTranslated
                                }
                                "HumanReviewed" => {
                                    crate::models::translation::TranslationStatus::HumanReviewed
                                }
                                _ => crate::models::translation::TranslationStatus::NotTranslated,
                            };
                            Some(text_unit)
                        }
                        Err(e) => {
                            log::warn!("Failed to reconstruct ID for {}: {}", record.field_type, e);
                            None
                        }
                    }
                }
            })
        })
        .collect();

    info!("Converted {} units for injection", text_units.len());

    // Create destination directory if it doesn't exist
    std::fs::create_dir_all(destination_root).map_err(|e| {
        AppError::FileSystem(format!("Failed to create destination directory: {}", e))
    })?;

    // Copy project files to destination (only the files needed for injection)
    let criteria = engine.get_detection_criteria();
    for required_file in &criteria.required_files {
        let src_path = project_info.path.join(required_file);
        let dest_path = std::path::Path::new(destination_root).join(required_file);

        if src_path.exists() {
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    AppError::FileSystem(format!("Failed to create parent directory: {}", e))
                })?;
            }
            std::fs::copy(&src_path, &dest_path).map_err(|e| {
                AppError::FileSystem(format!("Failed to copy {}: {}", required_file, e))
            })?;
        }
    }

    // Copy export data roots (the actual game data files to be translated)
    for data_root in &criteria.export_data_roots {
        let src_path = project_info.path.join(data_root);
        let dest_path = std::path::Path::new(destination_root).join(data_root);

        if src_path.exists() {
            copy_dir_recursive(&src_path, &dest_path).map_err(|e| {
                AppError::FileSystem(format!(
                    "Failed to copy data directory {}: {}",
                    data_root, e
                ))
            })?;
        }
    }

    // Create new EngineInfo for the destination path
    let mut dest_engine_info = project_info.clone();
    dest_engine_info.path = std::path::Path::new(destination_root).to_path_buf();

    // Inject translations into the copied files
    engine
        .inject_text_units(&dest_engine_info, &text_units)
        .map_err(|e| AppError::Other(format!("Failed to inject translations: {}", e)))?;

    info!(
        "Successfully exported {} translations to {}",
        text_units.len(),
        destination_root
    );

    Ok(destination_root.to_string())
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &std::path::Path, dest: &std::path::Path) -> Result<(), std::io::Error> {
    use std::fs;

    if src.is_dir() {
        fs::create_dir_all(dest)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dest_path = dest.join(entry.file_name());
            if src_path.is_dir() {
                copy_dir_recursive(&src_path, &dest_path)?;
            } else {
                fs::copy(&src_path, &dest_path)?;
            }
        }
    }
    Ok(())
}

/// Detects the engine type based on the project directory structure.
///
/// This function checks the project against criteria for each supported engine type
/// and returns the matching engine type.
///
/// # Arguments
///
/// * `project_path` - Path to the root directory of the game project
///
/// # Returns
///
/// * `AppResult<EngineType>` - The detected engine type
fn detect_engine_type(project_path: &Path) -> AppResult<EngineType> {
    // Check for RPG Maker MV
    if matches_criteria(project_path, &RpgMakerMvEngine::get_detection_criteria())? {
        return Ok(EngineType::RpgMakerMv);
    }

    // Check for RPG Maker MZ
    if matches_criteria(project_path, &RpgMakerMzEngine::get_detection_criteria())? {
        return Ok(EngineType::RpgMakerMz);
    }

    // Check for Wolf RPG - looks for dump folder
    if WolfRpgEngine::is_wolf_rpg_project(project_path) {
        return Ok(EngineType::WolfRpg);
    }

    // If no engine type was detected, return Unknown
    Ok(EngineType::Unknown)
}

/// Checks if a project directory matches the given engine criteria.
///
/// # Arguments
///
/// * `project_path` - Path to the root directory of the game project
/// * `criteria` - The criteria to check against
///
/// # Returns
///
/// * `AppResult<bool>` - True if the project matches the criteria, false otherwise
fn matches_criteria(project_path: &Path, criteria: &EngineCriteria) -> AppResult<bool> {
    use log::{debug, info};

    debug!(
        "Checking project at {} against criteria",
        project_path.display()
    );

    // Check required files
    for file in &criteria.required_files {
        let file_path = project_path.join(file);
        debug!("Checking required file: {}", file_path.display());
        if !file_path.exists() || !file_path.is_file() {
            debug!("Required file not found: {}", file_path.display());
            return Ok(false);
        }
    }

    // Check required folders
    for folder in &criteria.required_folders {
        let folder_path = project_path.join(folder);
        debug!("Checking required folder: {}", folder_path.display());
        if !folder_path.exists() || !folder_path.is_dir() {
            debug!("Required folder not found: {}", folder_path.display());
            return Ok(false);
        }
    }

    // Check extra files (lenient): if any are declared, log whether at least one exists
    if !criteria.extra_files.is_empty() {
        let mut found_any = false;
        for file in &criteria.extra_files {
            let file_path = project_path.join(file);
            debug!("Checking extra file: {}", file_path.display());
            if file_path.exists() && file_path.is_file() {
                debug!("Found extra file: {}", file_path.display());
                found_any = true;
                break;
            }
        }
        if !found_any {
            info!("No extra files found; proceeding based on required files/folders");
        }
    }

    debug!("Project matches criteria");
    Ok(true)
}
