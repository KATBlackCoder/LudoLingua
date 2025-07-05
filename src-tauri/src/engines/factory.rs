use std::path::Path;
use std::fs;

use crate::core::engine::Engine;
use crate::core::error::{AppError, AppResult};
use crate::engines::rpg_maker_mv::engine::RpgMakerMvEngine;
use crate::models::engine::{EngineType, EngineCriteria};

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
    
    // Return the appropriate engine implementation based on the detected type
    match engine_type {
        EngineType::RpgMakerMv => Ok(Box::new(RpgMakerMvEngine::new()) as Box<dyn Engine>),
        EngineType::RpgMakerMz => {
            // For now, use the MV engine for MZ projects too
            // In the future, we'll implement a dedicated MZ engine
            Ok(Box::new(RpgMakerMvEngine::new()) as Box<dyn Engine>)
        },
        EngineType::Unknown => {
            Err(AppError::Other(format!(
                "Could not determine engine type for project at: {}",
                project_path.display()
            )))
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
    
    // Future: Add checks for other engine types here
    // Example:
    // if matches_criteria(project_path, &RpgMakerMzEngine::get_detection_criteria())? {
    //     return Ok(EngineType::RpgMakerMz);
    // }

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
    use log::{info, debug};
    
    debug!("Checking project at {} against criteria", project_path.display());
    
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

    // Check optional files if specified
    if let Some(optional_files) = &criteria.optional_files {
        // If there are optional files, at least one must exist
        if !optional_files.is_empty() {
            let mut found_optional = false;
            
            for file in optional_files {
                let file_path = project_path.join(file);
                debug!("Checking optional file: {}", file_path.display());
                if file_path.exists() && file_path.is_file() {
                    debug!("Found optional file: {}", file_path.display());
                    found_optional = true;
                    break;
                }
            }
            
            if !found_optional {
                debug!("No optional files found");
                // We'll make this more lenient - don't fail if optional files aren't found
                // Just log it but still return true if required files/folders are present
                info!("No optional files found, but continuing with detection");
            }
        }
    }

    debug!("Project matches criteria");
    Ok(true)
} 