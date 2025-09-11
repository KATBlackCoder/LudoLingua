use log::{info, warn};
use std::path::Path;

use crate::db::translation::manifest::create_or_load_project_manifest;
use crate::db::state::ManagedTranslationState;
use crate::db::translation::repo;
use crate::engines::factory::{get_engine, extract_game_data_files as factory_extract_game_data_files};
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
    let engine = get_engine(path).map_err(|e| format!("Failed to get engine: {}", e))?;

    // Load the project info
    let mut project_info = engine.load_project_info(path, source_language, target_language)
        .map_err(|e| format!("Failed to load project info: {}", e))?;
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
                    Ok(project_info)
        }
    }
}

/// Extracts translatable text units from a project with smart loading logic.
///
/// This command implements intelligent project loading:
/// - If NO manifest exists: Extract translatable texts from files (fresh project)
/// - If manifest EXISTS: Load from database with smart status-based routing
///
/// Smart status routing:
/// - NotTranslated units → should go to TranslationRaw.vue
/// - MachineTranslated/HumanReviewed units → should go to TranslationResult.vue
///
/// # Arguments
///
/// * `project_info` - The project information object
/// * `db_state` - Database state for loading existing translations
///
/// # Returns
///
/// * `Result<Vec<TextUnit>, String>` - The text units or an error message
pub async fn extract_text(project_info: EngineInfo, db_state: Option<&ManagedTranslationState>) -> Result<Vec<TextUnit>, String> {
    info!("Smart loading text from project: {}", project_info.name);

    // Check if this is a fresh project (no manifest) or existing project (manifest exists)
    let manifest_path = project_info.path.join(".ludolingua.json");
    let has_manifest = manifest_path.exists();

    if !has_manifest {
        // FRESH PROJECT: No manifest exists, extract from files
        info!("No manifest found - extracting text from files for fresh project");
        return extract_text_from_files(project_info, db_state).await;
    }

    // EXISTING PROJECT: Manifest exists, load from database with smart routing
    info!("Manifest found - loading translations from database with smart routing");
    return load_text_with_smart_routing(project_info, db_state).await;
}

/// Extract text units from files for fresh projects (no manifest)
/// This function now saves ALL extracted units to database for complete persistence
async fn extract_text_from_files(project_info: EngineInfo, db_state: Option<&ManagedTranslationState>) -> Result<Vec<TextUnit>, String> {
    // Get the appropriate engine for this project
    let engine = get_engine(&project_info.path)
        .map_err(|e| format!("Failed to get engine: {}", e))?;

    // Extract text units from files
    let extracted_units = engine.extract_text_units(&project_info)
        .map_err(|e| format!("Failed to extract text units: {}", e))?;
                    info!("Successfully extracted {} text units from files", extracted_units.len());

    // Save ALL extracted units to database for complete persistence
    if let Some(db) = db_state {
        info!("Saving all {} extracted text units to database for persistence", extracted_units.len());

        // Convert TextUnits to TextUnitRecords with proper metadata
        let records: Vec<crate::db::translation::model::TextUnitRecord> = extracted_units.iter()
            .map(|unit| {
                let project_path = project_info.path.to_string_lossy().to_string();
                let file_path = format!("{}/data", project_path); // Default file path for extracted units

                crate::db::translation::model::TextUnitRecord::from_text_unit(
                    unit,
                    &project_path,
                    &file_path,
                    project_info.manifest_hash.as_deref(),
                )
            })
            .collect();

        // Bulk save to database
        match crate::db::translation::repo::bulk_upsert_units(db, &records).await {
            Ok(result) => {
                info!("Database persistence: {} inserted, {} updated, {} errors",
                      result.inserted, result.updated, result.errors.len());
                if !result.errors.is_empty() {
                    warn!("Database save errors: {:?}", result.errors);
                }
            }
            Err(e) => {
                warn!("Failed to save extracted units to database: {}", e);
                // Don't fail the extraction if database save fails - user can still work
            }
        }
    } else {
        warn!("No database state available - extracted units will not be persisted");
    }

                    // Update manifest with total text units count
                    if let Some(manifest_hash) = &project_info.manifest_hash {
                        if let Err(e) = update_manifest_with_total_units(&project_info.path, manifest_hash, extracted_units.len() as i64).await {
                            warn!("Failed to update manifest with total text units: {}", e);
                        }
                    }

                    Ok(extracted_units)
                }

/// Load text units from database with smart status-based routing
/// Now loads ALL units (including NotTranslated) for complete project state restoration
async fn load_text_with_smart_routing(project_info: EngineInfo, db_state: Option<&ManagedTranslationState>) -> Result<Vec<TextUnit>, String> {
    let db = db_state.ok_or_else(|| "Database state required for smart loading".to_string())?;
    let manifest_hash = project_info.manifest_hash
        .as_ref()
        .ok_or_else(|| "Manifest hash required for smart loading".to_string())?;

    // Load ALL text units from database (including NotTranslated ones)
    let query = crate::db::translation::model::TextUnitQuery {
        project_path: Some(project_info.path.to_string_lossy().to_string()),
        manifest_hash: Some(manifest_hash.to_string()),
        ..Default::default()
    };

    let records = repo::find_units(db, &query).await
        .map_err(|e| format!("Failed to load all text units from database: {}", e))?;

    if records.is_empty() {
        info!("No text units found in database - falling back to file extraction");
        return extract_text_from_files(project_info, Some(db)).await;
    }

    // Convert records to TextUnits
    let db_units: Vec<TextUnit> = records.into_iter()
        .map(|record| record.to_text_unit())
        .collect();

    info!("Loaded {} total text units from database (all statuses)", db_units.len());

    // Apply smart status-based routing - this now handles ALL units properly
    let routed_units = apply_smart_status_routing(db_units);
    info!("Smart routing applied: {} units ready for frontend", routed_units.len());

    Ok(routed_units)
}

/// Apply smart status-based routing to database-loaded units
/// This prepares units for the frontend by ensuring all units are included
/// The frontend will handle routing based on unit status:
/// - NotTranslated → TranslationRaw.vue
/// - MachineTranslated/HumanReviewed → TranslationResult.vue
fn apply_smart_status_routing(units: Vec<TextUnit>) -> Vec<TextUnit> {
    let mut not_translated = 0;
    let mut machine_translated = 0;
    let mut human_reviewed = 0;
    let mut ignored = 0;

    for unit in &units {
        match unit.status {
            TranslationStatus::NotTranslated => not_translated += 1,
            TranslationStatus::MachineTranslated => machine_translated += 1,
            TranslationStatus::HumanReviewed => human_reviewed += 1,
            TranslationStatus::Ignored => ignored += 1,
        }
    }

    info!("Smart routing summary: {} not translated, {} machine translated, {} human reviewed, {} ignored",
          not_translated, machine_translated, human_reviewed, ignored);

    // Return ALL units - frontend will handle routing based on status
    // This enables 100% smart loading capacity
    units
}

// REMOVED: extract_text_legacy - deprecated function removed



// REMOVED: merge_text_units - replaced with smart routing approach

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

    // Extract game data files using factory-managed dispatch (no downcasting needed!)
    let game_data_files = factory_extract_game_data_files(&project_info)
        .map_err(|e| format!("Failed to extract game data files: {}", e))?;

    info!("Successfully extracted {} game data files", game_data_files.len());
                        Ok(game_data_files)
}

// in-place inject removed; prefer minimal export flow via export_translated_subset

/// Load translations from database for a project using manifest hash
/// Unified function that replaces both load_project_translations and load_existing_translations_from_db
pub async fn load_translations_from_database(
    db_state: &ManagedTranslationState,
    manifest_hash: &str,
    project_path: &str,
) -> Result<Vec<TextUnit>, String> {
    use crate::db::translation::model::TextUnitQuery;
    use crate::models::translation::{TranslationStatus, PromptType};

    info!("Loading translations from database (manifest: {}, project: {})", manifest_hash, project_path);

    let query = TextUnitQuery {
        project_path: Some(project_path.to_string()),
        manifest_hash: Some(manifest_hash.to_string()),
        ..Default::default()
    };

    let records = repo::find_units(db_state, &query).await
        .map_err(|e| format!("Failed to load existing translations: {}", e))?;

    let text_units = records.into_iter().map(|record| {
        TextUnit {
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
        }
    }).collect::<Vec<_>>();

    info!("Loaded {} translations from database", text_units.len());
    Ok(text_units)
}

/// Legacy wrapper for backward compatibility - delegates to unified function
pub async fn load_project_translations(
    project_info: EngineInfo,
    db_state: &ManagedTranslationState,
) -> Result<Vec<TextUnit>, String> {
    let manifest_hash = project_info.manifest_hash
        .as_ref()
        .ok_or_else(|| "No manifest hash available in project info".to_string())?;

    load_translations_from_database(
        db_state,
        manifest_hash,
        project_info.path.to_string_lossy().as_ref()
    ).await
}

// REMOVED: load_subset_with_manifest - deprecated function removed




/// Export translation data using database-driven approach
/// Delegates to factory function for engine-agnostic export
pub async fn export_translated_subset(
    project_info: EngineInfo,
    db: &crate::db::state::ManagedTranslationState,
    destination_root: String,
) -> Result<String, String> {
    // Delegate to factory function for clean engine dispatch
    crate::engines::factory::export_translated_subset(&project_info, db, &destination_root)
        .await
        .map_err(|e| e.to_string())
}

// removed copy_file_create_parent; logic centralized in factory

/// Helper function to update the manifest with total text units count
async fn update_manifest_with_total_units(
    project_path: &Path,
    manifest_hash: &str,
    total_units: i64,
) -> Result<(), String> {
    match crate::db::translation::manifest::ProjectManifest::load_from_project(project_path) {
        Ok(Some(mut manifest)) => {
            if manifest.project_id == manifest_hash {
                manifest.update_total_text_units(total_units);
                manifest.save_to_project(project_path)
                    .map_err(|e| format!("Failed to save manifest: {}", e))?;
                info!("Updated manifest with total text units: {}", total_units);
            }
            Ok(())
        }
        Ok(_none) => {
            warn!("Manifest not found for project: {}", project_path.display());
            Ok(())
        }
        Err(e) => {
            warn!("Failed to load manifest: {}", e);
            Ok(())
        }
    }
}

/// Helper function to update the manifest with translated text units count
pub async fn update_manifest_with_translated_units(
    project_path: &Path,
    manifest_hash: &str,
    translated_units: i64,
) -> Result<(), String> {
    match crate::db::translation::manifest::ProjectManifest::load_from_project(project_path) {
        Ok(Some(mut manifest)) => {
            if manifest.project_id == manifest_hash {
                manifest.update_translated_text_units(translated_units);
                manifest.save_to_project(project_path)
                    .map_err(|e| format!("Failed to save manifest: {}", e))?;
            } else {
                warn!("Manifest hash mismatch: expected {}, got {}", manifest_hash, manifest.project_id);
            }
            Ok(())
        }
        Ok(_none) => {
            warn!("Manifest not found for project: {}", project_path.display());
            Ok(())
        }
        Err(e) => {
            warn!("Failed to load manifest: {}", e);
            Ok(())
        }
    }
}

#[derive(serde::Serialize)]
pub struct ProjectInfo {
    pub name: String,
    pub path: String,
    pub hash: String,
}

/// Get all available projects from the database
pub async fn get_available_projects(
    state: &ManagedTranslationState,
) -> Result<Vec<ProjectInfo>, String> {
    match repo::get_all_project_manifests(state).await {
        Ok(projects) => {
            let project_list = projects
                .into_iter()
                .map(|(manifest_hash, project_path)| ProjectInfo {
                    name: project_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown Project")
                        .to_string(),
                    path: project_path.to_string_lossy().to_string(),
                    hash: manifest_hash,
                })
                .collect();
            Ok(project_list)
        }
        Err(e) => Err(format!("Failed to get projects: {}", e)),
    }
}

/// Delete a project and all its associated translations
pub async fn delete_project(
    state: &ManagedTranslationState,
    project_hash: String,
) -> Result<(), String> {
    info!("Deleting project with hash: {}", project_hash);
    
    // Delete all translations for this project
    match repo::delete_all_translations_for_project(state, &project_hash).await {
        Ok(deleted_count) => {
            info!("Deleted {} translations for project {}", deleted_count, project_hash);
        }
        Err(e) => {
            warn!("Error deleting translations for project {}: {}", project_hash, e);
            return Err(format!("Failed to delete project translations: {}", e));
        }
    }
    
    // Note: We don't delete the .ludolingua.json manifest file from the file system
    // as that would require knowing the exact project path and could be risky
    // The project can be re-imported if needed and will get a new hash
    
    info!("Successfully deleted project: {}", project_hash);
    Ok(())
}
