use std::path::Path;
use log::{info, error};

use crate::core::error::AppResult;
use crate::engines::factory::get_engine;
use crate::models::engine::{EngineInfo, GameDataFile};
use crate::models::translation::TextUnit;
use crate::engines::rpg_maker_mv::engine::RpgMakerMvEngine;

/// Loads a project from the specified path.
/// 
/// This command is called from the frontend when the user selects a project folder.
/// It uses the engine factory to determine the appropriate engine and then loads
/// the project information.
///
/// # Arguments
///
/// * `project_path` - The path to the project directory
///
/// # Returns
///
/// * `Result<EngineInfo, String>` - The project information or an error message
pub async fn load_project(project_path: String) -> Result<EngineInfo, String> {
    info!("Loading project from path: {}", project_path);
    
    // Convert the string path to a Path
    let path = Path::new(&project_path);
    
    // Get the appropriate engine for this project
    let engine_result = get_engine(path);
    
    match engine_result {
        Ok(engine) => {
            // Use the engine to load the project info
            match engine.load_project_info(path) {
                Ok(project_info) => {
                    info!("Successfully loaded project: {}", project_info.name);
                    Ok(project_info)
                },
                Err(e) => {
                    error!("Failed to load project info: {}", e);
                    Err(format!("Failed to load project info: {}", e))
                }
            }
        },
        Err(e) => {
            error!("Failed to get engine: {}", e);
            Err(format!("Failed to get engine: {}", e))
        }
    }
}

/// Extracts translatable text units from a project.
///
/// This command is called from the frontend after a project is loaded.
/// It uses the appropriate engine to extract all translatable text from the project files.
///
/// # Arguments
///
/// * `project_info` - The project information object
///
/// # Returns
///
/// * `Result<Vec<TextUnit>, String>` - The extracted text units or an error message
pub async fn extract_text(project_info: EngineInfo) -> Result<Vec<TextUnit>, String> {
    info!("Extracting text from project: {}", project_info.name);
    
    // Get the appropriate engine for this project
    let engine_result = get_engine(&project_info.path);
    
    match engine_result {
        Ok(engine) => {
            // Use the engine to extract text units
            match engine.extract_text_units(&project_info) {
                Ok(text_units) => {
                    info!("Successfully extracted {} text units", text_units.len());
                    Ok(text_units)
                },
                Err(e) => {
                    error!("Failed to extract text units: {}", e);
                    Err(format!("Failed to extract text units: {}", e))
                }
            }
        },
        Err(e) => {
            error!("Failed to get engine: {}", e);
            Err(format!("Failed to get engine: {}", e))
        }
    }
}

/// Extracts game data files from a project.
///
/// This command is called from the frontend after a project is loaded.
/// It returns the structured game data files with their text units.
///
/// # Arguments
///
/// * `project_info` - The project information object
///
/// # Returns
///
/// * `Result<Vec<GameDataFile>, String>` - The extracted game data files or an error message
pub async fn extract_game_data_files(project_info: EngineInfo) -> Result<Vec<GameDataFile>, String> {
    info!("Extracting game data files from project: {}", project_info.name);
    
    // Get the appropriate engine for this project
    let engine_result = get_engine(&project_info.path);
    
    match engine_result {
        Ok(engine) => {
            // Check if the engine is RPG Maker MV
            if let Some(mv_engine) = engine.as_any().downcast_ref::<RpgMakerMvEngine>() {
                match mv_engine.extract_game_data_files(&project_info) {
                    Ok(game_data_files) => {
                        info!("Successfully extracted {} game data files", game_data_files.len());
                        Ok(game_data_files)
                    },
                    Err(e) => {
                        error!("Failed to extract game data files: {}", e);
                        Err(format!("Failed to extract game data files: {}", e))
                    }
                }
            } else {
                error!("Engine does not support extracting game data files");
                Err("Engine does not support extracting game data files".to_string())
            }
        },
        Err(e) => {
            error!("Failed to get engine: {}", e);
            Err(format!("Failed to get engine: {}", e))
        }
    }
} 