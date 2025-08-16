use log::{error, info};
use std::path::Path;

use crate::engines::factory::{export_translated_subset_via_factory, get_engine};
use crate::engines::rpg_maker_mv::engine::RpgMakerMvEngine;
use crate::engines::rpg_maker_mz::engine::RpgMakerMzEngine;
use crate::engines::wolf_rpg::engine::WolfRpgEngine;
use crate::models::engine::{EngineInfo, GameDataFile};
use crate::models::language::Language;
use crate::models::translation::TextUnit;
// removed unused: PathBuf, SystemTime, UNIX_EPOCH
use serde::Deserialize;

/// Loads a project from the specified path.
///
/// This command is called from the frontend when the user selects a project folder.
/// It uses the engine factory to determine the appropriate engine and then loads
/// the project information.
///
/// # Arguments
///
/// * `project_path` - The path to the project directory
///
/// # Returns
///
/// * `Result<EngineInfo, String>` - The project information or an error message
pub async fn load_project(
    project_path: String,
    source_language: Language,
    target_language: Language,
) -> Result<EngineInfo, String> {
    info!("Loading project from path: {}", project_path);

    // Convert the string path to a Path
    let path = Path::new(&project_path);

    // Get the appropriate engine for this project
    let engine_result = get_engine(path);

    match engine_result {
        Ok(engine) => {
            // Use the engine to load the project info
            match engine.load_project_info(path, source_language, target_language) {
                Ok(project_info) => {
                    info!("Successfully loaded project: {}", project_info.name);
                    Ok(project_info)
                }
                Err(e) => {
                    error!("Failed to load project info: {}", e);
                    Err(format!("Failed to load project info: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Failed to get engine: {}", e);
            Err(format!("Failed to get engine: {}", e))
        }
    }
}

/// Extracts translatable text units from a project.
///
/// This command is called from the frontend after a project is loaded.
/// It uses the appropriate engine to extract all translatable text from the project files.
///
/// # Arguments
///
/// * `project_info` - The project information object
///
/// # Returns
///
/// * `Result<Vec<TextUnit>, String>` - The extracted text units or an error message
pub async fn extract_text(project_info: EngineInfo) -> Result<Vec<TextUnit>, String> {
    info!("Extracting text from project: {}", project_info.name);

    // Get the appropriate engine for this project
    let engine_result = get_engine(&project_info.path);

    match engine_result {
        Ok(engine) => {
            // Use the engine to extract text units
            match engine.extract_text_units(&project_info) {
                Ok(text_units) => {
                    info!("Successfully extracted {} text units", text_units.len());
                    Ok(text_units)
                }
                Err(e) => {
                    error!("Failed to extract text units: {}", e);
                    Err(format!("Failed to extract text units: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Failed to get engine: {}", e);
            Err(format!("Failed to get engine: {}", e))
        }
    }
}

/// Extracts game data files from a project.
///
/// This command is called from the frontend after a project is loaded.
/// It returns the structured game data files with their text units.
///
/// # Arguments
///
/// * `project_info` - The project information object
///
/// # Returns
///
/// * `Result<Vec<GameDataFile>, String>` - The extracted game data files or an error message
pub async fn extract_game_data_files(
    project_info: EngineInfo,
) -> Result<Vec<GameDataFile>, String> {
    info!(
        "Extracting game data files from project: {}",
        project_info.name
    );

    // Get the appropriate engine for this project
    let engine_result = get_engine(&project_info.path);

    match engine_result {
        Ok(engine) => {
            // Dispatch to the specific engine implementation (MV or MZ)
            if let Some(mv_engine) = engine.as_any().downcast_ref::<RpgMakerMvEngine>() {
                match mv_engine.extract_game_data_files(&project_info) {
                    Ok(game_data_files) => {
                        info!(
                            "Successfully extracted {} game data files",
                            game_data_files.len()
                        );
                        Ok(game_data_files)
                    }
                    Err(e) => {
                        error!("Failed to extract game data files: {}", e);
                        Err(format!("Failed to extract game data files: {}", e))
                    }
                }
            } else if let Some(mz_engine) = engine.as_any().downcast_ref::<RpgMakerMzEngine>() {
                match mz_engine.extract_game_data_files(&project_info) {
                    Ok(game_data_files) => {
                        info!(
                            "Successfully extracted {} game data files",
                            game_data_files.len()
                        );
                        Ok(game_data_files)
                    }
                    Err(e) => {
                        error!("Failed to extract game data files: {}", e);
                        Err(format!("Failed to extract game data files: {}", e))
                    }
                }
            } else if let Some(_wolf) = engine.as_any().downcast_ref::<WolfRpgEngine>() {
                // Wolf RPG doesn't support structured game data files like RPG Maker
                // Force fallback to extract_text during manifest merging
                error!("Wolf RPG does not support structured game data file extraction");
                Err("Wolf RPG does not support structured game data file extraction".to_string())
            } else {
                error!("Engine does not support extracting game data files");
                Err("Engine does not support extracting game data files".to_string())
            }
        }
        Err(e) => {
            error!("Failed to get engine: {}", e);
            Err(format!("Failed to get engine: {}", e))
        }
    }
}

// in-place inject removed; prefer minimal export flow via export_translated_subset

/// If a ludolingua.json manifest exists at project root, load and merge translated units
/// Returns Some(units) when manifest is found and parsed; otherwise None
/// When manifest exists, bypasses regex filtering to avoid losing previously translated text
pub async fn load_subset_with_manifest(
    project_info: EngineInfo,
) -> Result<Option<Vec<TextUnit>>, String> {
    #[derive(Deserialize)]
    struct ManifestUnit {
        id: String,
        #[allow(dead_code)]
        prompt_type: Option<String>,
        source_text: Option<String>,
        translated_text: String,
    }
    #[derive(Deserialize)]
    struct ManifestFile {
        #[allow(dead_code)]
        files: Vec<String>,
        #[allow(dead_code)]
        engine: serde_json::Value,
        #[allow(dead_code)]
        project: serde_json::Value,
        units: Vec<ManifestUnit>,
    }

    let manifest_path = project_info.path.join("ludolingua.json");
    if !manifest_path.exists() {
        return Ok(None);
    }

    let raw = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read {}: {}", manifest_path.display(), e))?;
    let manifest: ManifestFile = serde_json::from_str(&raw)
        .map_err(|e| format!("Failed to parse {}: {}", manifest_path.display(), e))?;

    // Build id -> (source_text, translated_text) map
    use std::collections::HashMap;
    let mut id_to_manifest: HashMap<String, (Option<String>, String)> = HashMap::with_capacity(manifest.units.len());
    for u in manifest.units {
        id_to_manifest.insert(u.id, (u.source_text, u.translated_text));
    }

    // Manifest-first approach: Create units directly from manifest, then fill gaps with extraction
    log::info!("Creating TextUnits directly from manifest to preserve all translated content");
    
    // Create base units from manifest 
    let mut manifest_units: Vec<TextUnit> = id_to_manifest.iter().map(|(id, (source_opt, translated))| {
        use crate::models::translation::{PromptType, TranslationStatus};
        TextUnit {
            id: id.clone(),
            source_text: source_opt.clone().unwrap_or_else(|| translated.clone()),
            translated_text: translated.clone(),
            field_type: "Manifest entry".to_string(),
            status: TranslationStatus::MachineTranslated,
            prompt_type: PromptType::Other,
        }
    }).collect();
    
    log::info!("Created {} units from manifest", manifest_units.len());
    
    // Extract current units to find any new ones not in manifest
    let current_units = match extract_text(project_info.clone()).await {
        Ok(units) => {
            log::info!("Extracted {} current units to check for new content", units.len());
            units
        }
        Err(_) => {
            log::warn!("Failed to extract current units, using manifest only");
            vec![]
        }
    };
    
    // Add any new units that aren't in the manifest
    let mut new_units_count = 0;
    for unit in current_units {
        if !id_to_manifest.contains_key(&unit.id) {
            manifest_units.push(unit);
            new_units_count += 1;
        }
    }
    
    if new_units_count > 0 {
        log::info!("Added {} new units not found in manifest", new_units_count);
    }
    
    Ok(Some(manifest_units))
}



/// Export only the translatable data subtree and detection artifacts, then inject into that copy.
pub async fn export_translated_subset(
    project_info: EngineInfo,
    text_units: Vec<TextUnit>,
    destination_root: String,
) -> Result<String, String> {
    // Fully delegate to the factory to keep this engine-agnostic
    let dest_root = Path::new(&destination_root);
    let exported = export_translated_subset_via_factory(&project_info, &text_units, dest_root)
        .map_err(|e| e.to_string())?;
    Ok(exported.display().to_string())
}

// removed copy_file_create_parent; logic centralized in factory
