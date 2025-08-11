//! Tauri command handlers.
//!
//! This file contains all `#[tauri::command]` wrappers and is the only
//! place where the macro is used (by convention). Each function forwards
//! to the corresponding pure logic in the sibling command modules.

use log::debug;

use crate::commands::engine;
use crate::commands::provider;
use crate::commands::languages;
use crate::commands::translation;
use crate::commands::glossary as glossary_cmd;
use crate::glossaries::{GlossaryQuery, GlossaryState};
use crate::glossaries::model::GlossaryTerm;
use crate::llm::state::LlmState;
use tauri::State;
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

/// Load subset via manifest if present
#[tauri::command]
pub async fn load_subset_with_manifest(
    project_info: crate::models::engine::EngineInfo,
) -> Result<Option<Vec<TextUnit>>, String> {
    debug!("Command: load_subset_with_manifest - {}", project_info.name);
    crate::commands::engine::load_subset_with_manifest(project_info).await
}

// removed export_translated_copy

/// Export only the data subtree and detection artifacts, then inject into that copy
#[tauri::command]
pub async fn export_translated_subset(
    project_info: crate::models::engine::EngineInfo,
    text_units: Vec<TextUnit>,
    destination_root: String,
) -> Result<String, String> {
    debug!(
        "Command: export_translated_subset - {} units to {}",
        text_units.len(), destination_root
    );
    engine::export_translated_subset(project_info, text_units, destination_root).await
}

/// Translate a single text unit
#[tauri::command]
pub async fn translate_text_unit(
    state: State<'_, LlmState>,
    glossary: State<'_, GlossaryState>,
    text_unit: TextUnit,
    config: LlmConfig,
    engine_info: EngineInfo,
) -> Result<TextUnit, String> {
    debug!("Command: translate_text_unit - {}", text_unit.id);
    translation::translate_text_unit(state, glossary, text_unit, config, engine_info)
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

/// Glossary: list terms
#[tauri::command]
pub async fn glossary_list_terms(
    glossary: State<'_, GlossaryState>,
    q: GlossaryQuery,
) -> Result<Vec<GlossaryTerm>, String> {
    debug!("Command: glossary_list_terms");
    glossary_cmd::list_terms(&glossary, q).await.map_err(|e| e.to_string())
}

/// Glossary: upsert term
#[tauri::command]
pub async fn glossary_upsert_term(
    glossary: State<'_, GlossaryState>,
    term: GlossaryTerm,
) -> Result<i64, String> {
    debug!("Command: glossary_upsert_term");
    glossary_cmd::upsert_term(&glossary, term).await.map_err(|e| e.to_string())
}

/// Glossary: delete term
#[tauri::command]
pub async fn glossary_delete_term(
    glossary: State<'_, GlossaryState>,
    id: i64,
) -> Result<(), String> {
    debug!("Command: glossary_delete_term");
    glossary_cmd::delete_term(&glossary, id).await.map_err(|e| e.to_string())
}
