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
            "Unknown engine type - cannot create engine".to_string()
        )),
    }
}

/// Extract game data files using engine-specific logic.
/// This function handles the engine dispatch automatically based on engine type.
pub fn extract_game_data_files(project_info: &crate::models::engine::EngineInfo) -> AppResult<Vec<crate::models::engine::GameDataFile>> {
    match project_info.engine_type {
        EngineType::RpgMakerMv => {
            let engine = RpgMakerMvEngine::new();
            engine.extract_game_data_files(project_info)
        }
        EngineType::RpgMakerMz => {
            let engine = RpgMakerMzEngine::new();
            engine.extract_game_data_files(project_info)
        }
        EngineType::WolfRpg => {
            Err(AppError::Other(
                "Wolf RPG does not support structured game data file extraction".to_string()
            ))
        }
        EngineType::Unknown => {
            Err(AppError::Other(
                "Unknown engine type - cannot extract game data files".to_string()
            ))
        }
    }
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





