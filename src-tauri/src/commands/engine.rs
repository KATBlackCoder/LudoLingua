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
                // Wolf RPG: extract from existing dump folder (user created with WolfTL externally)
                match engine.extract_text_units(&project_info) {
                    Ok(text_units) => {
                        info!(
                            "Successfully extracted {} text units (WolfRpg)",
                            text_units.len()
                        );
                        // Wrap as a single pseudo-file for UI grouping
                        let file = GameDataFile {
                            name: "WolfRPG Dump".into(),
                            path: "dump".into(),
                            text_units,
                            text_unit_count: 0,
                        };
                        Ok(vec![file])
                    }
                    Err(e) => {
                        error!("Failed to extract text units: {}", e);
                        Err(format!("Failed to extract text units: {}", e))
                    }
                }
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
pub async fn load_subset_with_manifest(
    project_info: EngineInfo,
) -> Result<Option<Vec<TextUnit>>, String> {
    #[derive(Deserialize)]
    struct ManifestUnit {
        id: String,
        #[allow(dead_code)]
        prompt_type: Option<String>,
        #[allow(dead_code)]
        source_text: Option<String>,
        translated_text: String,
    }
    #[derive(Deserialize)]
    struct ManifestFile {
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

    // Build id -> translated_text map
    use std::collections::HashMap;
    let mut id_to_text: HashMap<String, String> = HashMap::with_capacity(manifest.units.len());
    for u in manifest.units {
        id_to_text.insert(u.id, u.translated_text);
    }

    // Prefer structured file extraction when available
    let filtered_units: Vec<TextUnit> = match extract_game_data_files(project_info.clone()).await {
        Ok(files) => {
            // Normalize manifest file paths to use forward slashes to match our relative paths
            let file_set: std::collections::HashSet<String> = manifest
                .files
                .into_iter()
                .map(|mut s| {
                    if cfg!(windows) {
                        s = s.replace('\\', "/");
                    }
                    s
                })
                .collect();
            let mut keep: Vec<TextUnit> = Vec::new();
            for f in files.into_iter() {
                let rel = f.path.replace('\\', "/");
                if file_set.contains(&rel) {
                    for unit in f.text_units.into_iter() {
                        keep.push(unit);
                    }
                }
            }
            keep
        }
        Err(_) => extract_text(project_info.clone()).await.unwrap_or_default(),
    };

    // Merge translated_text from manifest
    let mut merged: Vec<TextUnit> = Vec::with_capacity(filtered_units.len());
    for mut u in filtered_units.into_iter() {
        if let Some(t) = id_to_text.get(&u.id) {
            u.translated_text = t.clone();
            u.status = crate::models::translation::TranslationStatus::MachineTranslated;
        }
        merged.push(u);
    }

    Ok(Some(merged))
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
