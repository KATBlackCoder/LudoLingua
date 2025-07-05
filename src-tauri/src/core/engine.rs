use std::path::Path;
use std::any::Any;

use crate::core::error::AppResult;
use crate::models::engine::EngineInfo;
use crate::models::translation::TextUnit;

/// Core trait that all game engine implementations must implement.
/// This defines the contract for interacting with different types of game projects.
pub trait Engine: Send + Sync {
    /// Load basic information about a project from the given path
    fn load_project_info(&self, path: &Path) -> AppResult<EngineInfo>;
    
    /// Extract all translatable text units from a project
    fn extract_text_units(&self, project_info: &EngineInfo) -> AppResult<Vec<TextUnit>>;
    
    /// Save translated text units back to the project files
    fn save_text_units(&self, project_info: &EngineInfo, text_units: &[TextUnit]) -> AppResult<()>;
    
    /// Returns self as Any for downcasting to specific engine implementations
    fn as_any(&self) -> &dyn Any;
} 