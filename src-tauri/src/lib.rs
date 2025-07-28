mod commands;
mod core;
mod engines;
mod llm;
mod models;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::handler::load_project,
            commands::handler::extract_text,
            commands::handler::extract_game_data_files,
            commands::handler::inject_text_units,
            commands::handler::translate_text_unit,
            commands::handler::test_llm_connection,
            commands::handler::get_available_providers,
            commands::handler::get_available_models
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
