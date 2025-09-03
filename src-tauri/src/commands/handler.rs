//! Tauri command handlers.
//!
//! This file contains all `#[tauri::command]` wrappers and is the only
//! place where the macro is used (by convention). Each function forwards
//! to the corresponding pure logic in the sibling command modules.

use log::debug;

// ============================================================================
// IMPORTS - Organized by category
// ============================================================================

// Standard library and external crates
use tauri::State;

// Internal command modules
use crate::commands::{engine, glossary as glossary_cmd, languages, provider, translation};

// Database types
use crate::db::{glossary::model::{GlossaryQuery, GlossaryTerm}, ManagedGlossaryState, state::ManagedTranslationState};

// Core types
use crate::llm::state::LlmState;
use crate::models::{engine::{EngineInfo, GameDataFile}, language::Language, provider::{LlmConfig, ModelInfo}, translation::TextUnit};
use crate::utils::token_estimation::ProjectTokenEstimate;

// ============================================================================
// PROJECT MANAGEMENT COMMANDS
// ============================================================================

/// Load a project from a selected directory
#[tauri::command]
pub async fn load_project(
    project_path: String,
    source_language: Language,
    target_language: Language,
) -> Result<EngineInfo, String> {
    debug!("Command: load_project - {}", project_path);
    engine::load_project(project_path, source_language, target_language).await
}

/// Extract text from a project
#[tauri::command]
pub async fn extract_text(project_info: EngineInfo) -> Result<Vec<TextUnit>, String> {
    debug!("Command: extract_text - {}", project_info.name);
    engine::extract_text(project_info, None).await
}

/// Extract text from a project with database merge
#[tauri::command]
pub async fn extract_text_with_merge(
    project_info: EngineInfo,
    db: State<'_, ManagedTranslationState>,
) -> Result<Vec<TextUnit>, String> {
    debug!("Command: extract_text_with_merge - {}", project_info.name);
    engine::extract_text(project_info, Some(&db)).await
}

/// Extract all game data files from a project
#[tauri::command]
pub async fn extract_game_data_files(project_info: EngineInfo) -> Result<Vec<GameDataFile>, String> {
    debug!("Command: extract_game_data_files - {}", project_info.name);
    engine::extract_game_data_files(project_info).await
}

/// Load existing project translations from database
#[tauri::command]
pub async fn load_project_translations(
    project_info: EngineInfo,
    db: State<'_, ManagedTranslationState>,
) -> Result<Vec<TextUnit>, String> {
    debug!("Command: load_project_translations - {}", project_info.name);
    engine::load_project_translations(project_info, &db).await
}

// ============================================================================
// EXPORT COMMANDS
// ============================================================================

/// Export translation data using database-driven approach
#[tauri::command]
pub async fn export_translated_subset(
    project_info: EngineInfo,
    db: State<'_, ManagedTranslationState>,
    destination_root: String,
) -> Result<String, String> {
    debug!("Command: export_translated_subset - {} to {}", project_info.name, destination_root);
    engine::export_translated_subset(project_info, &db, destination_root).await
}

// ============================================================================
// TRANSLATION & LLM COMMANDS
// ============================================================================

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

    // Use manifest hash from engine info for project identification
    let manifest_hash = engine_info.manifest_hash.clone();

    translation::translate_text_unit(state, glossary, db, text_unit, config, engine_info, manifest_hash)
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// LLM PROVIDER COMMANDS
// ============================================================================

/// Test LLM connection
#[tauri::command]
pub async fn test_llm_connection(config: LlmConfig) -> Result<bool, String> {
    debug!("Command: test_llm_connection");
    provider::test_llm_connection(config).await
}

/// Get Ollama models
#[tauri::command]
pub async fn get_ollama_models() -> Result<Vec<ModelInfo>, String> {
    debug!("Command: get_ollama_models");
    provider::get_ollama_models().await
}

/// Get provider models by name (e.g., "ollama", "openai")
#[tauri::command]
pub fn get_provider_models(provider: String) -> Result<Vec<ModelInfo>, String> {
    debug!("Command: get_provider_models - {}", provider);
    provider::get_models(provider)
}

// ============================================================================
// UTILITY COMMANDS
// ============================================================================

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

// ============================================================================
// GLOSSARY COMMANDS
// ============================================================================

/// Get all glossary terms matching query
#[tauri::command]
pub async fn glossary_list_terms(
    glossary: State<'_, ManagedGlossaryState>,
    q: GlossaryQuery,
) -> Result<Vec<GlossaryTerm>, String> {
    debug!("Command: glossary_list_terms");
    glossary_cmd::list_terms(&glossary, q).await.map_err(|e| e.to_string())
}

/// Create or update a glossary term
#[tauri::command]
pub async fn glossary_upsert_term(
    glossary: State<'_, ManagedGlossaryState>,
    term: GlossaryTerm,
) -> Result<i64, String> {
    debug!("Command: glossary_upsert_term");
    glossary_cmd::upsert_term(&glossary, term).await.map_err(|e| e.to_string())
}

/// Delete a glossary term by ID
#[tauri::command]
pub async fn glossary_delete_term(
    glossary: State<'_, ManagedGlossaryState>,
    id: i64,
) -> Result<(), String> {
    debug!("Command: glossary_delete_term");
    glossary_cmd::delete_term(&glossary, id).await.map_err(|e| e.to_string())
}

/// Export glossary terms as JSON
#[tauri::command]
pub async fn glossary_export_terms(
    glossary: State<'_, ManagedGlossaryState>,
    q: GlossaryQuery,
) -> Result<String, String> {
    debug!("Command: glossary_export_terms");
    glossary_cmd::export_terms(&glossary, q).await.map_err(|e| e.to_string())
}

/// Import glossary terms from JSON
#[tauri::command]
pub async fn glossary_import_terms(
    glossary: State<'_, ManagedGlossaryState>,
    json: String,
) -> Result<usize, String> {
    debug!("Command: glossary_import_terms");
    glossary_cmd::import_terms(&glossary, json).await.map_err(|e| e.to_string())
}
