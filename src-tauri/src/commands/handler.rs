//! Tauri command handlers.
//!
//! This file contains all `#[tauri::command]` wrappers and is the only
//! place where the macro is used (by convention). Each function forwards
//! to the corresponding pure logic in the sibling command modules.

use log::debug;

use crate::commands::engine;
use crate::commands::glossary as glossary_cmd;
use crate::commands::languages;
use crate::commands::provider;
use crate::commands::translation;
use crate::db::glossary::model::GlossaryTerm;
use crate::db::glossary::model::GlossaryQuery;
use crate::db::ManagedGlossaryState;
use crate::db::state::ManagedTranslationState;
use crate::llm::state::LlmState;
use crate::models::engine::{EngineInfo, GameDataFile};
use crate::models::language::Language;
use crate::models::provider::{LlmConfig, ModelInfo};
use crate::models::translation::TextUnit;
use crate::utils::token_estimation::ProjectTokenEstimate;
use tauri::State;

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
        text_units.len(),
        destination_root
    );
    engine::export_translated_subset(project_info, text_units, destination_root).await
}

/// Translate a single text unit
#[tauri::command]
pub async fn translate_text_unit(
    state: State<'_, LlmState>,
    glossary: State<'_, ManagedGlossaryState>,
    db: State<'_, ManagedTranslationState>,
    text_unit: TextUnit,
    config: LlmConfig,
    engine_info: EngineInfo,
) -> Result<translation::TranslationResult, String> {
    debug!("Command: translate_text_unit - {}", text_unit.id);

    // Generate manifest hash for project identification
    // TODO: Use actual manifest system when implemented
    let manifest_hash = Some(format!("project_{}",
        engine_info.path.to_string_lossy().to_string().replace("/", "_").replace("\\", "_")));

    translation::translate_text_unit(state, glossary, db, text_unit, config, engine_info, manifest_hash)
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

/// General models loader by provider name (e.g., "ollama", "openai")
#[tauri::command]
pub fn get_provider_models(provider: String) -> Result<Vec<ModelInfo>, String> {
    debug!("Command: get_provider_models - {}", provider);
    provider::get_models(provider)
}

/// Get enabled languages from the bundled language catalog
#[tauri::command]
pub fn get_languages() -> Result<Vec<Language>, String> {
    debug!("Command: get_languages");
    languages::get_languages()
}

/// Estimate token usage for project translation
#[tauri::command]
pub async fn estimate_project_tokens(
    text_units: Vec<TextUnit>,
    engine_info: EngineInfo,
    config: LlmConfig,
) -> Result<ProjectTokenEstimate, String> {
    debug!("Command: estimate_project_tokens - {} units", text_units.len());
    crate::utils::token_estimation::estimate_project_tokens(&text_units, &engine_info, &config)
        .await
        .map_err(|e| e.to_string())
}

/// Glossary: list terms
#[tauri::command]
pub async fn glossary_list_terms(
    glossary: State<'_, ManagedGlossaryState>,
    q: GlossaryQuery,
) -> Result<Vec<GlossaryTerm>, String> {
    debug!("Command: glossary_list_terms");
    glossary_cmd::list_terms(&glossary, q)
        .await
        .map_err(|e| e.to_string())
}

/// Glossary: upsert term
#[tauri::command]
pub async fn glossary_upsert_term(
    glossary: State<'_, ManagedGlossaryState>,
    term: GlossaryTerm,
) -> Result<i64, String> {
    debug!("Command: glossary_upsert_term");
    glossary_cmd::upsert_term(&glossary, term)
        .await
        .map_err(|e| e.to_string())
}

/// Glossary: delete term
#[tauri::command]
pub async fn glossary_delete_term(
    glossary: State<'_, ManagedGlossaryState>,
    id: i64,
) -> Result<(), String> {
    debug!("Command: glossary_delete_term");
    glossary_cmd::delete_term(&glossary, id)
        .await
        .map_err(|e| e.to_string())
}

/// Glossary: export terms (JSON)
#[tauri::command]
pub async fn glossary_export_terms(
    glossary: State<'_, ManagedGlossaryState>,
    q: GlossaryQuery,
) -> Result<String, String> {
    debug!("Command: glossary_export_terms");
    glossary_cmd::export_terms(&glossary, q)
        .await
        .map_err(|e| e.to_string())
}

/// Glossary: import terms from JSON
#[tauri::command]
pub async fn glossary_import_terms(
    glossary: State<'_, ManagedGlossaryState>,
    json: String,
) -> Result<usize, String> {
    debug!("Command: glossary_import_terms");
    glossary_cmd::import_terms(&glossary, json)
        .await
        .map_err(|e| e.to_string())
}
