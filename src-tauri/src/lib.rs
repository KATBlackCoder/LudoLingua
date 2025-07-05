mod commands;
mod core;
mod models;
mod engines;

use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::handler::load_project,
            commands::handler::extract_text,
            commands::handler::extract_game_data_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
