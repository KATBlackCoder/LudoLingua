use std::any::Any;
use std::path::Path;

use crate::core::error::AppResult;
use crate::models::engine::{EngineCriteria, EngineInfo};
use crate::models::language::Language;
use crate::models::translation::TextUnit;

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
    fn extract_text_units(&self, project_info: &EngineInfo) -> AppResult<Vec<TextUnit>>;

    /// Inject translated text units back into the project files
    fn inject_text_units(
        &self,
        project_info: &EngineInfo,
        text_units: &[TextUnit],
    ) -> AppResult<()>;

    /// Reconstruct a TextUnit ID from a field_type string for export purposes
    /// This allows each engine to handle its own ID format during export
    fn reconstruct_text_unit_id(&self, field_type: &str, source_text: &str, translated_text: &str) -> AppResult<TextUnit>;

    /// Returns self as Any for downcasting to specific engine implementations
    fn as_any(&self) -> &dyn Any;
}
