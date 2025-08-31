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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
