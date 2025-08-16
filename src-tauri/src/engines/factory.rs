use std::path::{Path, PathBuf};

use crate::core::engine::Engine;
use crate::core::error::{AppError, AppResult};
use crate::engines::rpg_maker_mv::engine::RpgMakerMvEngine;
use crate::engines::rpg_maker_mz::engine::RpgMakerMzEngine;
use crate::engines::wolf_rpg::engine::WolfRpgEngine;
use crate::models::engine::EngineInfo;
use crate::models::engine::{EngineCriteria, EngineType};
use crate::models::translation::TextUnit;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

/// Factory function to get the appropriate engine implementation based on project path.
///
/// This function analyzes the given project directory and determines which game engine
/// implementation should be used to handle the project files.
///
/// # Arguments
///
/// * `project_path` - Path to the root directory of the game project
///
/// # Returns
///
/// * `AppResult<Box<dyn Engine>>` - A boxed trait object implementing the Engine trait
///
/// # Errors
///
/// Returns an error if:
/// * The project path doesn't exist
/// * The project path is not a directory
/// * The project type cannot be determined
pub fn get_engine(project_path: &Path) -> AppResult<Box<dyn Engine>> {
    // Check if the path exists and is a directory
    if !project_path.exists() {
        return Err(AppError::FileSystem(format!(
            "Project path does not exist: {}",
            project_path.display()
        )));
    }

    if !project_path.is_dir() {
        return Err(AppError::FileSystem(format!(
            "Project path is not a directory: {}",
            project_path.display()
        )));
    }

    // Detect the engine type based on project structure
    let engine_type = detect_engine_type(project_path)?;

    // Return the appropriate engine implementation based on the detected type
    match engine_type {
        EngineType::RpgMakerMv => Ok(Box::new(RpgMakerMvEngine::new()) as Box<dyn Engine>),
        EngineType::RpgMakerMz => Ok(Box::new(RpgMakerMzEngine::new()) as Box<dyn Engine>),
        EngineType::WolfRpg => Ok(Box::new(WolfRpgEngine::new()) as Box<dyn Engine>),
        EngineType::Unknown => Err(AppError::Other(format!(
            "Could not determine engine type for project at: {}",
            project_path.display()
        ))),
    }
}

/// Detects the engine type based on the project directory structure.
///
/// This function checks the project against criteria for each supported engine type
/// and returns the matching engine type.
///
/// # Arguments
///
/// * `project_path` - Path to the root directory of the game project
///
/// # Returns
///
/// * `AppResult<EngineType>` - The detected engine type
fn detect_engine_type(project_path: &Path) -> AppResult<EngineType> {
    // Check for RPG Maker MV
    if matches_criteria(project_path, &RpgMakerMvEngine::get_detection_criteria())? {
        return Ok(EngineType::RpgMakerMv);
    }

    // Check for RPG Maker MZ
    if matches_criteria(project_path, &RpgMakerMzEngine::get_detection_criteria())? {
        return Ok(EngineType::RpgMakerMz);
    }

    // Check for Wolf RPG - looks for dump folder
    if WolfRpgEngine::is_wolf_rpg_project(project_path) {
        return Ok(EngineType::WolfRpg);
    }

    // If no engine type was detected, return Unknown
    Ok(EngineType::Unknown)
}

/// Checks if a project directory matches the given engine criteria.
///
/// # Arguments
///
/// * `project_path` - Path to the root directory of the game project
/// * `criteria` - The criteria to check against
///
/// # Returns
///
/// * `AppResult<bool>` - True if the project matches the criteria, false otherwise
fn matches_criteria(project_path: &Path, criteria: &EngineCriteria) -> AppResult<bool> {
    use log::{debug, info};

    debug!(
        "Checking project at {} against criteria",
        project_path.display()
    );

    // Check required files
    for file in &criteria.required_files {
        let file_path = project_path.join(file);
        debug!("Checking required file: {}", file_path.display());
        if !file_path.exists() || !file_path.is_file() {
            debug!("Required file not found: {}", file_path.display());
            return Ok(false);
        }
    }

    // Check required folders
    for folder in &criteria.required_folders {
        let folder_path = project_path.join(folder);
        debug!("Checking required folder: {}", folder_path.display());
        if !folder_path.exists() || !folder_path.is_dir() {
            debug!("Required folder not found: {}", folder_path.display());
            return Ok(false);
        }
    }

    // Check extra files (lenient): if any are declared, log whether at least one exists
    if !criteria.extra_files.is_empty() {
        let mut found_any = false;
        for file in &criteria.extra_files {
            let file_path = project_path.join(file);
            debug!("Checking extra file: {}", file_path.display());
            if file_path.exists() && file_path.is_file() {
                debug!("Found extra file: {}", file_path.display());
                found_any = true;
                break;
            }
        }
        if !found_any {
            info!("No extra files found; proceeding based on required files/folders");
        }
    }

    debug!("Project matches criteria");
    Ok(true)
}

/// Engine-agnostic minimal export that copies data subtree and detection artifacts, then injects translations
pub fn export_translated_subset_via_factory(
    project_info: &EngineInfo,
    text_units: &[TextUnit],
    destination_root: &Path,
) -> AppResult<PathBuf> {
    use log::info;
    use std::fs;

    let src_root = &project_info.path;
    if !src_root.exists() {
        return Err(AppError::FileSystem(format!(
            "Source path does not exist: {}",
            src_root.display()
        )));
    }

    if !destination_root.exists() {
        fs::create_dir_all(destination_root).map_err(|e| {
            AppError::FileSystem(format!(
                "Failed to create destination root {}: {}",
                destination_root.display(),
                e
            ))
        })?;
    }

    // Create target folder name under destination_root (fixed name with collision suffix)
    let base_name = "ludolingua-translated-data";
    let mut dest_project_dir = destination_root.join(base_name);
    let mut counter = 1usize;
    while dest_project_dir.exists() {
        dest_project_dir = destination_root.join(format!("{}-{}", base_name, counter));
        counter += 1;
    }

    fs::create_dir_all(&dest_project_dir).map_err(|e| {
        AppError::FileSystem(format!(
            "Failed to create export directory {}: {}",
            dest_project_dir.display(),
            e
        ))
    })?;

    // Prepare list of exported files for manifest
    let mut exported_files: Vec<String> = Vec::new();

    // 1) Copy engine-declared export data roots (mandatory)
    let candidate_data_dirs: Vec<PathBuf> = project_info
        .detection_criteria
        .export_data_roots
        .iter()
        .map(PathBuf::from)
        .collect();
    let mut copied_any_data = false;
    for data_rel in candidate_data_dirs.iter() {
        let src_data = src_root.join(data_rel);
        if src_data.exists() {
            let dst_data = dest_project_dir.join(data_rel);
            copy_dir_recursive_collect(
                &src_data,
                &dst_data,
                &mut exported_files,
                &dest_project_dir,
            )?;
            copied_any_data = true;
        }
    }
    if !copied_any_data {
        info!("No known data directory found (www/data or data). Skipping data copy.");
    }

    // 2) Copy detection artifacts from criteria
    for folder in &project_info.detection_criteria.required_folders {
        let dst_folder = dest_project_dir.join(folder);
        if !dst_folder.exists() {
            fs::create_dir_all(&dst_folder).map_err(|e| {
                AppError::FileSystem(format!(
                    "Failed to create required folder {}: {}",
                    dst_folder.display(),
                    e
                ))
            })?;
        }
    }

    for rel in &project_info.detection_criteria.required_files {
        let src_path = src_root.join(rel);
        let dst_path = dest_project_dir.join(rel);
        if src_path.exists() {
            copy_file_create_parent(&src_path, &dst_path)?;
            exported_files.push(rel.to_string());
        }
    }

    for rel in &project_info.detection_criteria.extra_files {
        let src_path = src_root.join(rel);
        let dst_path = dest_project_dir.join(rel);
        if src_path.exists() {
            copy_file_create_parent(&src_path, &dst_path)?;
            exported_files.push(rel.to_string());
        }
    }

    // Re-point EngineInfo and inject using the chosen engine
    let mut new_info = project_info.clone();
    new_info.path = dest_project_dir.clone();

    let engine = super::factory::get_engine(&new_info.path)?;
    engine
        .inject_text_units(&new_info, text_units)
        .map_err(|e| AppError::Other(e.to_string()))?;

    // 4) Write manifest file
    #[derive(Serialize)]
    struct AppMeta<'a> {
        name: &'a str,
        version: &'a str,
    }
    #[derive(Serialize)]
    struct EngineMeta<'a> {
        #[serde(rename = "type")]
        r#type: &'a str,
        export_data_roots: &'a [String],
    }
    #[derive(Serialize)]
    struct ProjectMeta<'a> {
        name: &'a str,
        source_lang: &'a str,
        target_lang: &'a str,
    }
    #[derive(Serialize)]
    struct TranslationStats {
        total_units: usize,
        translated_units: usize,
        completion_rate: f32,
    }
    #[derive(Serialize)]
    struct UnitEntry<'a> {
        id: &'a str,
        prompt_type: &'a str,
        source_text: &'a str,
        translated_text: &'a str,
    }
    #[derive(Serialize)]
    struct Manifest<'a> {
        schema: u32,
        app: AppMeta<'a>,
        exported_at_unix: u64,
        engine: EngineMeta<'a>,
        project: ProjectMeta<'a>,
        translation_stats: TranslationStats,
        files: &'a [String],
        units: Vec<UnitEntry<'a>>,
    }

    let app = AppMeta {
        name: "LudoLingua",
        version: env!("CARGO_PKG_VERSION"),
    };
    let engine_meta = EngineMeta {
        r#type: match project_info.engine_type {
            EngineType::RpgMakerMv => "RpgMakerMv",
            EngineType::RpgMakerMz => "RpgMakerMz",
            EngineType::WolfRpg => "WolfRpg",
            EngineType::Unknown => "Unknown",
        },
        export_data_roots: &project_info.detection_criteria.export_data_roots,
    };
    let project_meta = ProjectMeta {
        name: &project_info.name,
        source_lang: &project_info.source_language.id,
        target_lang: &project_info.target_language.id,
    };
    let exported_at_unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    // Calculate translation statistics
    let total_units = text_units.len();
    let translated_units = text_units.iter().filter(|u| !u.translated_text.trim().is_empty()).count();
    let completion_rate = if total_units > 0 { 
        (translated_units as f32 / total_units as f32) * 100.0 
    } else { 
        0.0 
    };
    
    let translation_stats = TranslationStats {
        total_units,
        translated_units,
        completion_rate,
    };
    
    let units_vec: Vec<UnitEntry> = text_units
        .iter()
        .map(|u| UnitEntry {
            id: &u.id,
            prompt_type: match u.prompt_type {
                crate::models::translation::PromptType::Character => "Character",
                crate::models::translation::PromptType::State => "State",
                crate::models::translation::PromptType::System => "System",
                crate::models::translation::PromptType::Dialogue => "Dialogue",
                crate::models::translation::PromptType::Equipment => "Equipment",
                crate::models::translation::PromptType::Skill => "Skill",
                crate::models::translation::PromptType::Class => "Class",
                crate::models::translation::PromptType::Other => "Other",
            },
            source_text: &u.source_text,
            translated_text: &u.translated_text,
        })
        .collect();
    let manifest = Manifest {
        schema: 1,
        app,
        exported_at_unix,
        engine: engine_meta,
        project: project_meta,
        translation_stats,
        files: &exported_files,
        units: units_vec,
    };
    let manifest_path = dest_project_dir.join("ludolingua.json");
    let json =
        serde_json::to_string_pretty(&manifest).map_err(|e| AppError::Other(e.to_string()))?;
    fs::write(&manifest_path, json).map_err(|e| {
        AppError::FileSystem(format!(
            "Failed to write {}: {}",
            manifest_path.display(),
            e
        ))
    })?;

    Ok(dest_project_dir)
}



fn copy_file_create_parent(src: &Path, dst: &Path) -> AppResult<()> {
    use std::fs;
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            AppError::FileSystem(format!(
                "Failed to create parent dir {}: {}",
                parent.display(),
                e
            ))
        })?;
    }
    fs::copy(src, dst).map_err(|e| {
        AppError::FileSystem(format!(
            "Failed to copy {} to {}: {}",
            src.display(),
            dst.display(),
            e
        ))
    })?;
    Ok(())
}

fn copy_dir_recursive_collect(
    src: &Path,
    dst: &Path,
    exported_files: &mut Vec<String>,
    dest_root: &Path,
) -> AppResult<()> {
    use std::fs;
    if !dst.exists() {
        fs::create_dir_all(dst).map_err(|e| {
            AppError::FileSystem(format!(
                "Failed to create destination directory {}: {}",
                dst.display(),
                e
            ))
        })?;
    }

    for entry in fs::read_dir(src).map_err(|e| {
        AppError::FileSystem(format!("Failed to read directory {}: {}", src.display(), e))
    })? {
        let entry = entry.map_err(|e| {
            AppError::FileSystem(format!("Failed to read entry in {}: {}", src.display(), e))
        })?;
        let file_type = entry.file_type().map_err(|e| {
            AppError::FileSystem(format!(
                "Failed to get file type for {}: {}",
                entry.path().display(),
                e
            ))
        })?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive_collect(&src_path, &dst_path, exported_files, dest_root)?;
        } else if file_type.is_file() {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    AppError::FileSystem(format!(
                        "Failed to create parent for {}: {}",
                        dst_path.display(),
                        e
                    ))
                })?;
            }
            fs::copy(&src_path, &dst_path).map_err(|e| {
                AppError::FileSystem(format!(
                    "Failed to copy {} to {}: {}",
                    src_path.display(),
                    dst_path.display(),
                    e
                ))
            })?;
            // record relative path from dest_root, normalize to forward slashes for cross-platform matching
            if let Ok(rel) = dst_path.strip_prefix(dest_root) {
                let mut s = rel.to_string_lossy().to_string();
                if cfg!(windows) {
                    s = s.replace('\\', "/");
                }
                exported_files.push(s);
            }
        }
    }
    Ok(())
}
