//! Updater logic for handling AppImage renaming and custom update logic

use std::path::Path;
use std::process::Command;

pub async fn rename_appimage_with_version(
    current_path: String,
    new_version: String,
) -> Result<String, String> {
    // Check platform and handle accordingly
    let current_platform = tauri_plugin_os::platform();
    
    match current_platform {
        "linux" => rename_linux_appimage(current_path, new_version).await,
        "windows" => rename_windows_executable(current_path, new_version).await,
        _ => Err(format!("File renaming not supported on platform: {}", current_platform)),
    }
}

async fn rename_linux_appimage(
    current_path: String,
    new_version: String,
) -> Result<String, String> {
    // Auto-detect current AppImage path if not provided
    let current_path = if current_path.is_empty() || current_path == "auto-detect" {
        // Try to get the current executable path
        std::env::current_exe()
            .map_err(|e| format!("Failed to get current executable path: {}", e))?
            .into_os_string()
            .into_string()
            .map_err(|e| format!("Invalid executable path: {:?}", e))?
    } else {
        current_path
    };
    
    let current_path = Path::new(&current_path);
    
    // Get the directory and filename without extension
    let parent_dir = current_path.parent()
        .ok_or("Invalid AppImage path")?;
    let filename = current_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid filename")?;
    
    // Create new filename with version
    let new_filename = format!("{}-v{}.AppImage", filename, new_version);
    let _new_path = parent_dir.join(&new_filename);
    let old_backup_path = parent_dir.join(format!("{}.AppImage.old", filename));
    
    // Rename current AppImage to .old backup
    if current_path.exists() {
        std::fs::rename(current_path, &old_backup_path)
            .map_err(|e| format!("Failed to rename current AppImage to backup: {}", e))?;
        
        log::info!("Renamed current AppImage to: {:?}", old_backup_path);
    }
    
    // Find the newly installed AppImage (usually in the same directory)
    // The updater typically installs to the same location
    let temp_new_path = parent_dir.join(format!("{}-v{}.AppImage", filename, new_version));
    
    // If the new file doesn't exist with version, try without version first
    let final_new_path = if temp_new_path.exists() {
        temp_new_path
    } else {
        // Look for any new AppImage file that might have been created
        let entries = std::fs::read_dir(parent_dir)
            .map_err(|e| format!("Failed to read directory: {}", e))?;
        
        let mut latest_appimage = None;
        let mut latest_time = 0;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("AppImage") {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let modified_time = modified.duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        
                        if modified_time > latest_time {
                            latest_time = modified_time;
                            latest_appimage = Some(path);
                        }
                    }
                }
            }
        }
        
        latest_appimage.ok_or("No new AppImage found after update")?
    };
    
    // Rename the new AppImage to the standard name
    let final_path = parent_dir.join(format!("{}.AppImage", filename));
    std::fs::rename(&final_new_path, &final_path)
        .map_err(|e| format!("Failed to rename new AppImage: {}", e))?;
    
    // Make it executable
    #[cfg(unix)]
    {
        Command::new("chmod")
            .arg("+x")
            .arg(&final_path)
            .output()
            .map_err(|e| format!("Failed to make AppImage executable: {}", e))?;
    }
    
    log::info!("Successfully renamed AppImage to: {:?}", final_path);
    
    Ok(final_path.to_string_lossy().to_string())
}

async fn rename_windows_executable(
    current_path: String,
    new_version: String,
) -> Result<String, String> {
    // Auto-detect current executable path if not provided
    let current_path = if current_path.is_empty() || current_path == "auto-detect" {
        std::env::current_exe()
            .map_err(|e| format!("Failed to get current executable path: {}", e))?
            .into_os_string()
            .into_string()
            .map_err(|e| format!("Invalid executable path: {:?}", e))?
    } else {
        current_path
    };
    
    let current_path = Path::new(&current_path);
    
    // Get the directory and filename without extension
    let parent_dir = current_path.parent()
        .ok_or("Invalid executable path")?;
    let filename = current_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid filename")?;
    
    // Create new filename with version (Windows typically uses .exe)
    let new_filename = format!("{}-v{}.exe", filename, new_version);
    let old_backup_path = parent_dir.join(format!("{}.exe.old", filename));
    
    // Rename current executable to .old backup
    if current_path.exists() {
        std::fs::rename(current_path, &old_backup_path)
            .map_err(|e| format!("Failed to rename current executable to backup: {}", e))?;
        
        log::info!("Renamed current executable to: {:?}", old_backup_path);
    }
    
    // Find the newly installed executable (usually in the same directory)
    let temp_new_path = parent_dir.join(&new_filename);
    
    // If the new file doesn't exist with version, try without version first
    let final_new_path = if temp_new_path.exists() {
        temp_new_path
    } else {
        // Look for any new executable file that might have been created
        let entries = std::fs::read_dir(parent_dir)
            .map_err(|e| format!("Failed to read directory: {}", e))?;
        
        let mut latest_executable = None;
        let mut latest_time = 0;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            // Look for .exe files
            if path.extension().and_then(|s| s.to_str()) == Some("exe") {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let modified_time = modified.duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        
                        if modified_time > latest_time {
                            latest_time = modified_time;
                            latest_executable = Some(path);
                        }
                    }
                }
            }
        }
        
        latest_executable.ok_or("No new executable found after update")?
    };
    
    // Rename the new executable to the standard name
    let final_path = parent_dir.join(format!("{}.exe", filename));
    std::fs::rename(&final_new_path, &final_path)
        .map_err(|e| format!("Failed to rename new executable: {}", e))?;
    
    log::info!("Successfully renamed Windows executable to: {:?}", final_path);
    
    Ok(final_path.to_string_lossy().to_string())
}

pub async fn cleanup_old_executable_backups() -> Result<String, String> {
    // This command can be used to clean up old .old backup files
    // For now, we'll keep them as they provide rollback capability
    Ok("Backup files preserved for rollback capability".to_string())
}
