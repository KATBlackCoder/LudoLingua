use std::any::Any;
use std::path::Path;

use crate::core::error::AppResult;
use crate::models::engine::{EngineCriteria, EngineInfo};
use crate::models::language::Language;
use crate::models::translation::TextUnit;
use crate::utils::text::pipeline::{RawTextUnit, TextProcessor};

/// Core trait that all game engine implementations must implement.
/// This defines the contract for interacting with different types of game projects.
pub trait Engine: Send + Sync {
    /// Load basic information about a project from the given path
    fn load_project_info(
        &self,
        path: &Path,
        source_language: Language,
        target_language: Language,
    ) -> AppResult<EngineInfo>;

    /// Get the detection criteria for the engine
    fn get_detection_criteria(&self) -> EngineCriteria;

    /// Extract all translatable text units from a project
    /// 
    /// This method has a default implementation that automatically processes
    /// raw text units through the unified text processing pipeline.
    fn extract_text_units(&self, project_info: &EngineInfo) -> AppResult<Vec<TextUnit>> {
        // Get raw text units from engine-specific implementation
        let raw_units = self.extract_raw_text_units(project_info)?;
        
        // Process through unified text processing pipeline
        let target_language = &project_info.target_language.id;
        Ok(TextProcessor::process_for_extraction(raw_units, target_language))
    }

    /// Inject translated text units back into the project files
    /// 
    /// This method has a default implementation that automatically processes
    /// text units through the unified text processing pipeline before injection.
    fn inject_text_units(
        &self,
        project_info: &EngineInfo,
        text_units: &[TextUnit],
    ) -> AppResult<()> {
        // Process through unified text processing pipeline
        let raw_units = TextProcessor::process_injection_pipeline(text_units);
        
        // Inject raw text units using engine-specific implementation
        self.inject_raw_text_units(project_info, &raw_units)
    }

    /// Reconstruct a TextUnit ID from a field_type string for export purposes
    /// This allows each engine to handle its own ID format during export
    fn reconstruct_text_unit_id(
        &self,
        field_type: &str,
        source_text: &str,
        translated_text: &str,
    ) -> AppResult<TextUnit>;

    /// Returns self as Any for downcasting to specific engine implementations
    fn as_any(&self) -> &dyn Any;

    /// Extract raw text units from project files (file I/O only, no text processing)
    /// 
    /// This method must be implemented by each engine to handle the specific
    /// file format and structure of that engine's project files.
    /// 
    /// The returned RawTextUnits contain text exactly as it appears in the files,
    /// without any formatting code replacement or validation.
    fn extract_raw_text_units(&self, project_info: &EngineInfo) -> AppResult<Vec<RawTextUnit>>;

    /// Inject raw text units back into project files (file I/O only, no text processing)
    /// 
    /// This method must be implemented by each engine to handle the specific
    /// file format and structure of that engine's project files.
    /// 
    /// The RawTextUnits contain text with formatting codes already restored,
    /// ready to be written directly to the files.
    fn inject_raw_text_units(
        &self,
        project_info: &EngineInfo,
        raw_units: &[RawTextUnit],
    ) -> AppResult<()>;
}
