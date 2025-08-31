use log::{error, info, warn};
use std::path::Path;

use crate::db::translation::manifest::create_or_load_project_manifest;
use crate::db::state::ManagedTranslationState;
use crate::db::translation::repo;
use crate::engines::factory::{export_translated_subset_via_factory, get_engine};
use crate::engines::rpg_maker_mv::engine::RpgMakerMvEngine;
use crate::engines::rpg_maker_mz::engine::RpgMakerMzEngine;
use crate::engines::wolf_rpg::engine::WolfRpgEngine;
use crate::models::engine::{EngineInfo, GameDataFile};
use crate::models::language::Language;
use crate::models::translation::{TextUnit, TranslationStatus};
// removed unused: PathBuf, SystemTime, UNIX_EPOCH

/// Loads a project from the specified path.
///
/// This command is called from the frontend when the user selects a project folder.
/// It uses the engine factory to determine the appropriate engine and then loads
/// the project information. It also creates or loads the .ludolingua.json manifest
/// for project identification and translation persistence.
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
                Ok(mut project_info) => {
                    info!("Successfully loaded project: {}", project_info.name);

                    // Create or load the project manifest
                    match create_or_load_project_manifest(&project_info) {
                        Ok(manifest) => {
                            info!("Project manifest ready: {}", manifest.project_id);
                            // Store manifest hash in project_info for use by other commands
                            project_info.manifest_hash = Some(manifest.project_id.clone());
                            Ok(project_info)
                        }
                        Err(e) => {
                            warn!("Failed to create/load manifest: {}", e);
                            // Don't fail the entire load if manifest creation fails
                            // Just log the warning and continue
                            Ok(project_info)
                        }
                    }
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

/// Extracts translatable text units from a project and merges with existing translations.
///
/// This command is called from the frontend after a project is loaded.
/// It uses the appropriate engine to extract all translatable text from the project files,
/// then merges the results with any existing translations stored in the database.
///
/// # Arguments
///
/// * `project_info` - The project information object
/// * `db_state` - Database state for loading existing translations
///
/// # Returns
///
/// * `Result<Vec<TextUnit>, String>` - The merged text units or an error message
pub async fn extract_text(project_info: EngineInfo, db_state: Option<&ManagedTranslationState>) -> Result<Vec<TextUnit>, String> {
    info!("Extracting text from project: {}", project_info.name);

    // Get the appropriate engine for this project
    let engine_result = get_engine(&project_info.path);

    match engine_result {
        Ok(engine) => {
            // Use the engine to extract text units
            match engine.extract_text_units(&project_info) {
                Ok(extracted_units) => {
                    info!("Successfully extracted {} text units from files", extracted_units.len());

                    // If we have database access and a manifest hash, merge with existing translations
                    if let (Some(db), Some(manifest_hash)) = (db_state, &project_info.manifest_hash) {
                        match load_existing_translations_from_db(db, manifest_hash, project_info.path.to_string_lossy().as_ref()).await {
                            Ok(existing_units) => {
                                if !existing_units.is_empty() {
                                    info!("Found {} existing translations in database", existing_units.len());

                                    // Merge extracted units with existing translations
                                    let merged_units = merge_text_units(extracted_units, existing_units);
                                    info!("Merged result: {} total units", merged_units.len());
                                    return Ok(merged_units);
                                } else {
                                    info!("No existing translations found in database");
                                }
                            }
                            Err(e) => {
                                warn!("Failed to load existing translations from database: {}", e);
                                // Continue with just extracted units
                            }
                        }
                    } else if project_info.manifest_hash.is_none() {
                        info!("No manifest hash available, skipping database merge");
                    }

                    Ok(extracted_units)
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

/// Legacy version of extract_text for backward compatibility
/// This version doesn't merge with database translations
pub async fn extract_text_legacy(project_info: EngineInfo) -> Result<Vec<TextUnit>, String> {
    extract_text(project_info, None).await
}



/// Merge extracted text units with existing translations from the database
/// Preserves existing translations for unchanged source text
fn merge_text_units(extracted_units: Vec<TextUnit>, existing_units: Vec<TextUnit>) -> Vec<TextUnit> {
    use std::collections::HashMap;

    // Create a map of existing translations by ID for quick lookup
    let mut existing_map: HashMap<String, TextUnit> = HashMap::new();
    for unit in existing_units {
        existing_map.insert(unit.id.clone(), unit);
    }

    let mut merged_units = Vec::new();
    let mut preserved_count = 0;
    let mut new_count = 0;

    for extracted_unit in extracted_units {
        if let Some(existing_unit) = existing_map.remove(&extracted_unit.id) {
            // Unit exists in both - check if source text changed
            if extracted_unit.source_text == existing_unit.source_text {
                // Source text unchanged - preserve existing translation
                merged_units.push(existing_unit);
                preserved_count += 1;
            } else {
                // Source text changed - use extracted unit but keep translation if it exists
                let mut merged_unit = extracted_unit;
                if !existing_unit.translated_text.is_empty() {
                    merged_unit.translated_text = existing_unit.translated_text;
                    merged_unit.status = TranslationStatus::MachineTranslated;
                }
                merged_units.push(merged_unit);
                new_count += 1;
            }
        } else {
            // New unit not in database
            merged_units.push(extracted_unit);
            new_count += 1;
        }
    }

    info!("Merge summary: preserved {} existing translations, added {} new units", preserved_count, new_count);
    merged_units
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

/// Load existing project translations from the database using the manifest system
/// This replaces the old ludolingua.json system with the new .ludolingua.json manifest
pub async fn load_project_translations(
    project_info: EngineInfo,
    db_state: &ManagedTranslationState,
) -> Result<Vec<TextUnit>, String> {
    let manifest_hash = project_info.manifest_hash
        .as_ref()
        .ok_or_else(|| "No manifest hash available in project info".to_string())?;

    info!("Loading translations for project: {} (manifest: {})",
          project_info.name, manifest_hash);

    // Load existing translations from database
    load_existing_translations_from_db(
        db_state,
        manifest_hash,
        project_info.path.to_string_lossy().as_ref()
    ).await
}

/// Load existing translations from database for a project (helper function)
/// This is used by both load_project_translations and extract_text merge logic
async fn load_existing_translations_from_db(
    db_state: &ManagedTranslationState,
    manifest_hash: &str,
    project_path: &str,
) -> Result<Vec<TextUnit>, String> {
    use crate::db::translation::model::TextUnitQuery;
    use crate::models::translation::{TranslationStatus, PromptType};

    let query = TextUnitQuery {
        project_path: Some(project_path.to_string()),
        manifest_hash: Some(manifest_hash.to_string()),
        ..Default::default()
    };

    match repo::find_units(db_state, &query).await {
        Ok(records) => {
            let mut text_units = Vec::new();
            for record in records {
                // Convert TextUnitRecord back to TextUnit
                let text_unit = TextUnit {
                    id: record.id.map(|id| id.to_string()).unwrap_or_else(|| format!("{}/{}", record.file_path, record.field_type)),
                    source_text: record.source_text,
                    translated_text: record.translated_text.unwrap_or_default(),
                    field_type: record.field_type,
                    status: match record.status.as_str() {
                        "MachineTranslated" => TranslationStatus::MachineTranslated,
                        "HumanReviewed" => TranslationStatus::HumanReviewed,
                        "NotTranslated" => TranslationStatus::NotTranslated,
                        "Ignored" => TranslationStatus::Ignored,
                        _ => TranslationStatus::NotTranslated,
                    },
                    prompt_type: match record.prompt_type.as_str() {
                        "Dialogue" => PromptType::Dialogue,
                        "Character" => PromptType::Character,
                        "Class" => PromptType::Class,
                        "Skill" => PromptType::Skill,
                        "State" => PromptType::State,
                        "Equipment" => PromptType::Equipment,
                        "System" => PromptType::System,
                        "Other" => PromptType::Other,
                        _ => PromptType::Other,
                    },
                };
                text_units.push(text_unit);
            }
            Ok(text_units)
        }
        Err(e) => Err(format!("Failed to load existing translations: {}", e)),
    }
}

/// Legacy function for backward compatibility with old ludolingua.json format
/// This will be removed in a future version
#[deprecated(note = "Use load_project_translations with the new .ludolingua.json manifest system")]
pub async fn load_subset_with_manifest(
    project_info: EngineInfo,
) -> Result<Option<Vec<TextUnit>>, String> {
    warn!("load_subset_with_manifest is deprecated. Use load_project_translations instead.");

    // Try to load from old ludolingua.json format for backward compatibility
    let old_manifest_path = project_info.path.join("ludolingua.json");
    if !old_manifest_path.exists() {
        return Ok(None);
    }

    warn!("Found old ludolingua.json file. Consider migrating to the new .ludolingua.json manifest system.");
    // For now, return None to indicate no translations loaded via old system
    // This encourages migration to the new system
    Ok(None)
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
