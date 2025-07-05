// This file contains all #[tauri::command] wrappers.
// It's the only file that contains the tauri::command macro.

use crate::models::engine::{EngineInfo, GameDataFile};
use crate::models::translation::TextUnit;
use crate::commands::engine;

/// Tauri command wrapper for loading a project
#[tauri::command]
pub async fn load_project(project_path: String) -> Result<EngineInfo, String> {
    engine::load_project(project_path).await
}

/// Tauri command wrapper for extracting text from a project
#[tauri::command]
pub async fn extract_text(project_info: EngineInfo) -> Result<Vec<TextUnit>, String> {
    engine::extract_text(project_info).await
}

/// Tauri command wrapper for extracting game data files from a project
#[tauri::command]
pub async fn extract_game_data_files(project_info: EngineInfo) -> Result<Vec<GameDataFile>, String> {
    engine::extract_game_data_files(project_info).await
}
