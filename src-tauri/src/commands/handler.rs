// This file contains all #[tauri::command] wrappers.
// It's the only file that contains the tauri::command macro.

// Example command that can be called from the frontend
#[tauri::command]
pub fn hello(name: &str) -> String {
    format!("Hello, {}! This is a response from Rust!", name)
} 