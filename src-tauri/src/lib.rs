//! Tauri application entry point and builder configuration.
//!
//! Registers managed state, plugins, and exposes the Tauri command handler
//! that bridges the Nuxt frontend and the Rust backend.
//! The `LlmState` is provided here to share connections and apply lightweight
//! rate limiting across translation requests.

mod commands;
mod core;
mod db;
mod engines;
mod llm;
mod models;
mod utils;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .manage(crate::llm::state::LlmState::new(1)) // Single request at a time to prevent rate limits
        .setup(|app| {
            // Resolve per-OS app data directory and create the DB there
            let app_data_dir = app
                .handle()
                .path()
                .app_data_dir()
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            std::fs::create_dir_all(&app_data_dir)?;
            let db_path = app_data_dir.join("ludolingua.db");

            let db_state = crate::db::DbState::new(db_path.clone());
            let glossary_state = crate::db::ManagedGlossaryState::new(db_path.clone());
            let translation_state = crate::db::state::ManagedTranslationState::new(db_path.clone());
            app.handle().manage(db_state);
            app.handle().manage(glossary_state);
            app.handle().manage(translation_state);
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::handler::load_project,
            commands::handler::extract_text,
            commands::handler::extract_text_with_merge,
            commands::handler::extract_game_data_files,
            commands::handler::export_translated_subset,
            commands::handler::load_project_translations,
            commands::handler::get_available_projects,
            commands::handler::delete_project,
            commands::handler::translate_text_unit,
            commands::handler::test_llm_connection,
            commands::handler::get_ollama_models,
            commands::handler::get_provider_models,
            commands::handler::get_languages,
            commands::handler::glossary_list_terms,
            commands::handler::glossary_upsert_term,
            commands::handler::glossary_delete_term,
            commands::handler::glossary_export_terms,
            commands::handler::glossary_import_terms,
            // Translation management commands
            commands::handler::list_translations_cmd,
            commands::handler::get_translation_cmd,
            commands::handler::update_translation_cmd,
            commands::handler::delete_translation_cmd,
            commands::handler::bulk_delete_translations_cmd,
            commands::handler::get_translation_stats_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
