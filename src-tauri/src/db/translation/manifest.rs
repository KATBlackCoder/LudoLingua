use crate::core::error::{AppError, AppResult};
use crate::models::engine::{EngineCriteria, EngineInfo, EngineType};
use log::info;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// Project manifest stored as .ludolingua.json in project root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectManifest {
    /// Schema version for future compatibility
    pub schema_version: u32,
    /// Unique project identifier based on path and engine
    pub project_id: String,
    /// Absolute path to the project
    pub project_path: String,
    /// Game engine type
    pub engine_type: String,
    /// Engine version if detected
    pub engine_version: Option<String>,
    /// Source language code
    pub source_language: String,
    /// Target language code
    pub target_language: String,
    /// Engine detection criteria
    pub detection_criteria: EngineDetectionCriteria,
    /// When the manifest was created
    pub created_at: String,
    /// When the project was last accessed
    pub last_accessed: String,
    /// Total number of text units that need translation
    pub total_text_units: Option<i64>,
    /// Number of text units already translated
    pub translated_text_units: Option<i64>,
}

/// Serializable version of EngineCriteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineDetectionCriteria {
    pub required_files: Vec<String>,
    pub required_folders: Vec<String>,
    pub extra_files: Vec<String>,
    pub export_data_roots: Vec<String>,
}

impl From<&EngineCriteria> for EngineDetectionCriteria {
    fn from(criteria: &EngineCriteria) -> Self {
        Self {
            required_files: criteria.required_files.clone(),
            required_folders: criteria.required_folders.clone(),
            extra_files: criteria.extra_files.clone(),
            export_data_roots: criteria.export_data_roots.clone(),
        }
    }
}

impl ProjectManifest {
    /// Generate a unique project ID based on path and engine info
    pub fn generate_project_id(project_path: &Path, engine_info: &EngineInfo) -> String {
        let mut hasher = Sha256::new();
        hasher.update(project_path.to_string_lossy().as_bytes());
        hasher.update(
            match engine_info.engine_type {
                EngineType::RpgMakerMv => "RpgMakerMv",
                EngineType::RpgMakerMz => "RpgMakerMz",
                EngineType::WolfRpg => "WolfRpg",
                EngineType::Unknown => "Unknown",
            }
            .as_bytes(),
        );
        if let Some(version) = &engine_info.version {
            hasher.update(version.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    /// Create a new manifest from engine info
    pub fn from_engine_info(engine_info: &EngineInfo) -> Self {
        let project_path = engine_info.path.to_string_lossy().to_string();
        let project_id = Self::generate_project_id(&engine_info.path, engine_info);
        let now = chrono::Utc::now().to_rfc3339();

        Self {
            schema_version: 1,
            project_id,
            project_path,
            engine_type: match engine_info.engine_type {
                EngineType::RpgMakerMv => "RpgMakerMv".to_string(),
                EngineType::RpgMakerMz => "RpgMakerMz".to_string(),
                EngineType::WolfRpg => "WolfRpg".to_string(),
                EngineType::Unknown => "Unknown".to_string(),
            },
            engine_version: engine_info.version.clone(),
            source_language: engine_info.source_language.id.clone(),
            target_language: engine_info.target_language.id.clone(),
            detection_criteria: (&engine_info.detection_criteria).into(),
            created_at: now.clone(),
            last_accessed: now,
            total_text_units: None, // Will be updated when text units are extracted
            translated_text_units: None, // Will be updated when translations are saved
        }
    }

    /// Update the last accessed timestamp
    pub fn update_last_accessed(&mut self) {
        self.last_accessed = chrono::Utc::now().to_rfc3339();
    }

    /// Update the total number of text units
    pub fn update_total_text_units(&mut self, total: i64) {
        self.total_text_units = Some(total);
    }

    /// Update the number of translated text units
    pub fn update_translated_text_units(&mut self, translated: i64) {
        self.translated_text_units = Some(translated);
    }

    /// Update both total and translated counts
    pub fn update_translation_stats(&mut self, total: i64, translated: i64) {
        self.total_text_units = Some(total);
        self.translated_text_units = Some(translated);
    }

    /// Get the manifest file path for a project
    pub fn get_manifest_path(project_path: &Path) -> PathBuf {
        project_path.join(".ludolingua.json")
    }

    /// Save manifest to project root
    pub fn save_to_project(&self, project_path: &Path) -> AppResult<()> {
        let manifest_path = Self::get_manifest_path(project_path);
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| AppError::Other(format!("Failed to serialize manifest: {}", e)))?;

        std::fs::write(&manifest_path, json).map_err(|e| {
            AppError::Other(format!(
                "Failed to write manifest to {}: {}",
                manifest_path.display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Load manifest from project root
    pub fn load_from_project(project_path: &Path) -> AppResult<Option<Self>> {
        let manifest_path = Self::get_manifest_path(project_path);

        if !manifest_path.exists() {
            return Ok(None);
        }

        let json = std::fs::read_to_string(&manifest_path).map_err(|e| {
            AppError::Other(format!(
                "Failed to read manifest from {}: {}",
                manifest_path.display(),
                e
            ))
        })?;

        let mut manifest: Self = serde_json::from_str(&json).map_err(|e| {
            AppError::Other(format!(
                "Failed to parse manifest from {}: {}",
                manifest_path.display(),
                e
            ))
        })?;

        // Update last accessed time
        manifest.update_last_accessed();
        manifest.save_to_project(project_path)?;

        Ok(Some(manifest))
    }

    /// Check if manifest matches current engine info
    pub fn matches_engine_info(&self, engine_info: &EngineInfo) -> bool {
        self.project_path == engine_info.path.to_string_lossy()
            && self.engine_type
                == match engine_info.engine_type {
                    EngineType::RpgMakerMv => "RpgMakerMv",
                    EngineType::RpgMakerMz => "RpgMakerMz",
                    EngineType::WolfRpg => "WolfRpg",
                    EngineType::Unknown => "Unknown",
                }
            && self.source_language == engine_info.source_language.id
            && self.target_language == engine_info.target_language.id
    }
}

/// Create or load project manifest
pub fn create_or_load_project_manifest(engine_info: &EngineInfo) -> AppResult<ProjectManifest> {
    match ProjectManifest::load_from_project(&engine_info.path)? {
        Some(mut manifest) => {
            // Check if manifest matches current engine info
            if manifest.matches_engine_info(engine_info) {
                // Update last accessed time
                manifest.update_last_accessed();
                manifest.save_to_project(&engine_info.path)?;
                Ok(manifest)
            } else {
                // Recreate manifest if it doesn't match, but preserve translation statistics
                info!("Manifest doesn't match current project, recreating...");
                let mut new_manifest = ProjectManifest::from_engine_info(engine_info);
                // Preserve existing translation statistics
                new_manifest.total_text_units = manifest.total_text_units;
                new_manifest.translated_text_units = manifest.translated_text_units;
                new_manifest.save_to_project(&engine_info.path)?;
                Ok(new_manifest)
            }
        }
        _none => {
            // Create new manifest
            let manifest = ProjectManifest::from_engine_info(engine_info);
            manifest.save_to_project(&engine_info.path)?;
            Ok(manifest)
        }
    }
}

/// Check if project has a valid manifest
pub fn project_has_manifest(project_path: &Path) -> bool {
    ProjectManifest::get_manifest_path(project_path).exists()
}
