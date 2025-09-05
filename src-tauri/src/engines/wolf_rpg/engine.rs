use std::any::Any;
use std::path::{Path, PathBuf};

use crate::core::engine::Engine;
use crate::core::error::{AppError, AppResult};
use crate::models::engine::{EngineCriteria, EngineInfo, EngineType};
use crate::models::language::Language;
use crate::models::translation::TextUnit;

pub struct WolfRpgEngine {
    detection_criteria: EngineCriteria,
}

impl WolfRpgEngine {
    pub fn new() -> Self {
        Self {
            detection_criteria: Self::get_detection_criteria(),
        }
    }

    /// A Wolf RPG project with manually created dump folder.
    /// User handles .wolf decryption and WolfTL dump creation externally.
    pub fn get_detection_criteria() -> EngineCriteria {
        EngineCriteria {
            required_files: vec![],
            required_folders: vec!["dump".to_string()], // Require dump folder
            extra_files: vec![],
            export_data_roots: vec!["dump".to_string()],
        }
    }

    /// Check if this directory contains a Wolf RPG project with dump folder
    pub fn is_wolf_rpg_project(path: &Path) -> bool {
        // Check for dump folder (created by user with WolfTL)
        path.join("dump").exists()
    }





    fn dump_dir(&self, project_path: &Path) -> PathBuf {
        // User creates dump folder with WolfTL externally
        project_path.join("dump")
    }
    
    /// Extract text units from MPS directory using selective processing
    fn extract_from_mps_directory(
        &self,
        out: &mut Vec<TextUnit>,
        mps_dir: &Path,
        project_path: &Path,
    ) -> AppResult<()> {
        for entry in walkdir::WalkDir::new(mps_dir)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.path().is_file() {
                continue;
            }
            if entry.path().extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            
            // Read and parse JSON file
            let raw = std::fs::read_to_string(entry.path()).map_err(|e| {
                AppError::FileSystem(format!(
                    "Failed to read {}: {}",
                    entry.path().display(),
                    e
                ))
            })?;
            
            let json: serde_json::Value = match serde_json::from_str(&raw) {
                Ok(v) => v,
                Err(_) => {
                    log::debug!("Skipping invalid JSON: {}", entry.path().display());
                    continue;
                }
            };
            
            // Get relative path for ID generation
            let rel_path = entry
                .path()
                .strip_prefix(project_path)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();
            
            // Use selective MPS extraction instead of generic JSON walker
            let mut mps_units = super::files::mps::extract_text_units_from_mps(&json, &rel_path);
            out.append(&mut mps_units);
        }
        Ok(())
    }
    
    /// Inject text units into MPS directory using selective processing
    fn inject_into_mps_directory(
        &self,
        text_unit_map: &std::collections::HashMap<String, &TextUnit>,
        mps_dir: &Path,
        project_path: &Path,
    ) -> AppResult<()> {
        for entry in walkdir::WalkDir::new(mps_dir)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.path().is_file() {
                continue;
            }
            if entry.path().extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            
            // Read and parse JSON file
            let raw = std::fs::read_to_string(entry.path()).map_err(|e| {
                AppError::FileSystem(format!(
                    "Failed to read {}: {}",
                    entry.path().display(),
                    e
                ))
            })?;
            
            let mut json: serde_json::Value = match serde_json::from_str(&raw) {
                Ok(v) => v,
                Err(_) => {
                    log::debug!("Skipping invalid JSON: {}", entry.path().display());
                    continue;
                }
            };
            
            // Get relative path for ID matching
            let rel_path = entry
                .path()
                .strip_prefix(project_path)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();
            
            // Use selective MPS injection
            super::files::mps::inject_text_units_into_mps(&mut json, text_unit_map, &rel_path);
            
            // Write the updated JSON back to file
            let updated_content = serde_json::to_string_pretty(&json)
                .map_err(|e| AppError::Parsing(format!("Failed to serialize JSON: {}", e)))?;
            
            std::fs::write(entry.path(), updated_content).map_err(|e| {
                AppError::FileSystem(format!(
                    "Failed to write {}: {}",
                    entry.path().display(),
                    e
                ))
            })?;
        }
        Ok(())
    }

    /// Extract text units from DB directory (CDataBase.json, DataBase.json, SysDatabase.json)
    fn extract_from_db_directory(
        &self,
        out: &mut Vec<TextUnit>,
        db_dir: &Path,
        project_path: &Path,
    ) -> AppResult<()> {
        // Process specific database files: CDataBase.json, DataBase.json, SysDatabase.json
        let db_files = ["CDataBase.json", "DataBase.json", "SysDatabase.json"];
        
        for db_file in &db_files {
            let db_path = db_dir.join(db_file);
            if !db_path.exists() {
                log::debug!("Database file not found: {}", db_path.display());
                continue;
            }
            
            // Read and parse JSON file
            let raw = std::fs::read_to_string(&db_path).map_err(|e| {
                AppError::FileSystem(format!(
                    "Failed to read {}: {}",
                    db_path.display(),
                    e
                ))
            })?;
            
            let json: serde_json::Value = match serde_json::from_str(&raw) {
                Ok(v) => v,
                Err(e) => {
                    log::debug!("Skipping invalid JSON {}: {}", db_path.display(), e);
                    continue;
                }
            };
            
            // Get relative path for ID generation
            let rel_path = db_path
                .strip_prefix(project_path)
                .unwrap_or(&db_path)
                .to_string_lossy()
                .to_string();
            
            // Use selective DB extraction
            let mut db_units = super::files::db::extract_text_units_from_db(&json, &rel_path);
            out.append(&mut db_units);
        }
        
        Ok(())
    }
    
    /// Inject text units into DB directory (CDataBase.json, DataBase.json, SysDatabase.json)
    fn inject_into_db_directory(
        &self,
        text_unit_map: &std::collections::HashMap<String, &TextUnit>,
        db_dir: &Path,
        project_path: &Path,
    ) -> AppResult<()> {
        // Process specific database files: CDataBase.json, DataBase.json, SysDatabase.json
        let db_files = ["CDataBase.json", "DataBase.json", "SysDatabase.json"];
        
        for db_file in &db_files {
            let db_path = db_dir.join(db_file);
            if !db_path.exists() {
                log::debug!("Database file not found: {}", db_path.display());
                continue;
            }
            
            // Read and parse JSON file
            let raw = std::fs::read_to_string(&db_path).map_err(|e| {
                AppError::FileSystem(format!(
                    "Failed to read {}: {}",
                    db_path.display(),
                    e
                ))
            })?;
            
            let mut json: serde_json::Value = match serde_json::from_str(&raw) {
                Ok(v) => v,
                Err(e) => {
                    log::debug!("Skipping invalid JSON {}: {}", db_path.display(), e);
                    continue;
                }
            };
            
            // Get relative path for ID matching
            let rel_path = db_path
                .strip_prefix(project_path)
                .unwrap_or(&db_path)
                .to_string_lossy()
                .to_string();
            
            // Use selective DB injection
            super::files::db::inject_text_units_into_db(&mut json, text_unit_map, &rel_path);
            
            // Write the updated JSON back to file
            let updated_content = serde_json::to_string_pretty(&json)
                .map_err(|e| AppError::Parsing(format!("Failed to serialize JSON: {}", e)))?;
            
            std::fs::write(&db_path, updated_content).map_err(|e| {
                AppError::FileSystem(format!(
                    "Failed to write {}: {}",
                    db_path.display(),
                    e
                ))
            })?;
        }
        
        Ok(())
    }


}

impl Engine for WolfRpgEngine {
    fn load_project_info(
        &self,
        path: &Path,
        source_language: Language,
        target_language: Language,
    ) -> AppResult<EngineInfo> {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("WolfRPG Project").to_string();
        Ok(EngineInfo {
            name,
            path: path.to_path_buf(),
            engine_type: EngineType::WolfRpg,
            source_language,
            target_language,
            version: None,
            detection_criteria: self.detection_criteria.clone(),
            manifest_hash: None,
        })
    }

    fn get_detection_criteria(&self) -> EngineCriteria {
        self.detection_criteria.clone()
    }

    fn extract_text_units(&self, project_info: &EngineInfo) -> AppResult<Vec<TextUnit>> {
        let project_path = &project_info.path;
        let dump_dir = self.dump_dir(project_path);
        
        // Parse dump JSONs → TextUnit (user created dump with WolfTL externally)
        if !dump_dir.exists() {
            return Err(AppError::FileSystem(format!(
                "WolfTL dump directory not found at {}. Please create dump folder using WolfTL first.",
                dump_dir.display()
            )));
        }

        let mut out: Vec<TextUnit> = Vec::new();
        
        // Focus on MPS files first (Wolf RPG's main event/map script files)
        let mps_dir = dump_dir.join("mps");
        if mps_dir.exists() {
            self.extract_from_mps_directory(&mut out, &mps_dir, project_path)?;
        }
        
        // Process database files (CDataBase.json, DataBase.json, SysDatabase.json)
        let db_dir = dump_dir.join("db");
        if db_dir.exists() {
            self.extract_from_db_directory(&mut out, &db_dir, project_path)?;
        }
        
        // TODO: Add common/ extraction later once MPS and DB are working well
        
        Ok(out)
    }

    fn inject_text_units(
        &self,
        project_info: &EngineInfo,
        text_units: &[TextUnit],
    ) -> AppResult<()> {
        let project_path = &project_info.path;
        let dump_dir = self.dump_dir(project_path);
        if !dump_dir.exists() {
            return Err(AppError::FileSystem(
                "WolfTL dump not found; please create dump folder using WolfTL first".into(),
            ));
        }

        // Create a map for quick text unit lookup
        let text_unit_map: std::collections::HashMap<String, &TextUnit> = text_units
            .iter()
            .map(|unit| (unit.id.clone(), unit))
            .collect();

        // Focus on MPS files first (matching the extraction logic)
        let mps_dir = dump_dir.join("mps");
        if mps_dir.exists() {
            self.inject_into_mps_directory(&text_unit_map, &mps_dir, project_path)?;
        }
        
        // Process database files (matching the extraction logic)
        let db_dir = dump_dir.join("db");
        if db_dir.exists() {
            self.inject_into_db_directory(&text_unit_map, &db_dir, project_path)?;
        }
        
        // TODO: Add common/ injection later once MPS and DB are working well
        
        Ok(())
    }

    fn reconstruct_text_unit_id(&self, field_type: &str, source_text: &str, translated_text: &str) -> AppResult<TextUnit> {
        // Wolf RPG uses JSON-pointer style IDs and colon-separated field types
        // Examples:
        // MPS files: field_type = "command_101:dump/mps/Map001.json:events[0].pages[0].list[0]"
        // DB files: field_type = "Database value (DataBase.json)" or "Database entry name (SysDatabase.json)"

        // Parse the field_type to determine the correct ID format and prompt type
        let prompt_type = determine_prompt_type_from_field_type(field_type);

        // Generate the ID based on field_type format
        let id = if field_type.starts_with("command_") {
            // MPS file format: command_CODE:file_path:event_indices
            // Reconstruct as: wolf_json:file_path#event_indices.stringArgs[0]
            // Note: We need to determine the arg_idx, for now assume 0
            let parts: Vec<&str> = field_type.split(':').collect();
            if parts.len() >= 3 {
                let file_path = parts[1];
                let event_path = parts[2];
                format!("wolf_json:{}#{}.stringArgs[0]", file_path, event_path)
            } else {
                return Err(AppError::Other(format!("Invalid Wolf RPG MPS field_type format: {}", field_type)));
            }
        } else if field_type.contains("Database") {
            // DB file format: "Database value (FileName.json)" or "Database entry name (FileName.json)"
            // Reconstruct as: file_path:types[0]:data[0]:field[0]:value/name
            // Note: We need to determine the exact indices, for now use placeholders
            let file_name = if field_type.contains("DataBase.json") {
                "dump/db/DataBase.json"
            } else if field_type.contains("SysDatabase.json") {
                "dump/db/SysDatabase.json"
            } else if field_type.contains("CDataBase.json") {
                "dump/db/CDataBase.json"
            } else {
                return Err(AppError::Other(format!("Unknown Wolf RPG database file in field_type: {}", field_type)));
            };

            // For database entries, we need the actual indices from the original extraction
            // For now, create a generic format that matches the extraction pattern
            format!("{}:types[0]:data[0]:data[0]:value", file_name)
        } else {
            return Err(AppError::Other(format!("Unknown Wolf RPG field_type format: {}", field_type)));
        };

        Ok(TextUnit {
            id,
            source_text: source_text.to_string(),
            translated_text: translated_text.to_string(),
            status: crate::models::translation::TranslationStatus::MachineTranslated,
            field_type: field_type.to_string(),
            prompt_type,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Determine the appropriate prompt type based on Wolf RPG field_type
fn determine_prompt_type_from_field_type(field_type: &str) -> crate::models::translation::PromptType {
    if field_type.starts_with("command_101") {
        // Message command - dialogue
        crate::models::translation::PromptType::Dialogue
    } else if field_type.starts_with("command_") {
        // Other commands (210, 150, 122) - could be dialogue or other
        crate::models::translation::PromptType::Other
    } else if field_type.contains("Database") {
        // Database entries - typically character/item/skill names
        crate::models::translation::PromptType::Character
    } else {
        // Default fallback
        crate::models::translation::PromptType::Other
    }
}
