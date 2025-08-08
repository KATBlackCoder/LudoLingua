// This file contains all #[tauri::command] wrappers.
// It's the only file that contains the tauri::command macro.

use log::debug;

use crate::commands::engine;
use crate::commands::provider;
use crate::commands::languages;
use crate::commands::translation;
use crate::models::engine::{EngineInfo, GameDataFile};
use crate::models::language::Language;
use crate::models::provider::{LlmConfig, ModelInfo};
use crate::models::translation::TextUnit;

/// Load a project from a selected directory
#[tauri::command]
pub async fn load_project(
    project_path: String,
    source_language: Language,
    target_language: Language,
) -> Result<crate::models::engine::EngineInfo, String> {
    debug!("Command: load_project - {}", project_path);
    engine::load_project(project_path, source_language, target_language).await
}

/// Extract text from a project
#[tauri::command]
pub async fn extract_text(
    project_info: crate::models::engine::EngineInfo,
) -> Result<Vec<TextUnit>, String> {
    debug!("Command: extract_text - {}", project_info.name);
    engine::extract_text(project_info).await
}

/// Extract all game data files from a project
#[tauri::command]
pub async fn extract_game_data_files(
    project_info: crate::models::engine::EngineInfo,
) -> Result<Vec<GameDataFile>, String> {
    debug!("Command: extract_game_data_files - {}", project_info.name);
    engine::extract_game_data_files(project_info).await
}

/// Inject text units back into project files
#[tauri::command]
pub async fn inject_text_units(
    project_info: crate::models::engine::EngineInfo,
    text_units: Vec<TextUnit>,
) -> Result<(), String> {
    debug!("Command: inject_text_units - {} units", text_units.len());
    engine::inject_text_units(project_info, text_units).await
}

/// Translate a single text unit
#[tauri::command]
pub async fn translate_text_unit(
    text_unit: TextUnit,
    config: LlmConfig,
    engine_info: EngineInfo,
) -> Result<TextUnit, String> {
    debug!("Command: translate_text_unit - {}", text_unit.id);
    translation::translate_text_unit(text_unit, config, engine_info)
        .await
        .map_err(|e| e.to_string())
}

/// Test LLM connection
#[tauri::command]
pub async fn test_llm_connection(config: LlmConfig) -> Result<bool, String> {
    debug!("Command: test_llm_connection");
    provider::test_llm_connection(config).await
}

#[tauri::command]
pub async fn get_ollama_models() -> Result<Vec<ModelInfo>, String> {
    debug!("Command: get_ollama_models");
    provider::get_ollama_models().await
}

/// Get enabled languages from the bundled language catalog
#[tauri::command]
pub fn get_languages() -> Result<Vec<Language>, String> {
    debug!("Command: get_languages");
    languages::get_languages()
}
