use crate::models::language::Language;
use crate::models::translation::TextUnit;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Represents the type of game engine detected in a project.
///
/// This enum identifies the specific RPG Maker version or other engine
/// that was detected when analyzing a game project's files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineType {
    /// RPG Maker MV engine (JavaScript-based)
    RpgMakerMv,

    /// RPG Maker MZ engine (newer JavaScript-based version)
    RpgMakerMz,

    /// Engine could not be determined or is not supported
    Unknown,
}

/// Criteria used to detect a specific game engine type from filesystem analysis.
///
/// This struct contains the file and folder patterns that are used to identify
/// a specific game engine when scanning a project directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineCriteria {
    /// Required files that must exist for this engine type to be detected
    ///
    /// Example: ["package.json", "index.html"]
    pub required_files: Vec<String>,

    /// Required folders that must exist for this engine type to be detected
    ///
    /// Example: ["data", "img"]
    pub required_folders: Vec<String>,

    /// Extra files that may exist and help identify or run the engine.
    /// Also copied during minimal export if present.
    ///
    /// Example: ["js/rpg_core.js", "js/rpg_managers.js"]
    pub extra_files: Vec<String>,

    /// Engine-declared data roots to export for minimal translated copies
    /// Example: ["www/data"] for RPG Maker MV/MZ
    pub export_data_roots: Vec<String>,
}

/// Information about the detected game engine and the game project itself.
///
/// This struct stores metadata about a loaded game project, including its
/// detected engine type, location, and the criteria that were used to identify it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineInfo {
    /// Name of the game project
    pub name: String,

    /// Root filesystem path to the game project files
    pub path: PathBuf,

    /// Detected type of game engine
    pub engine_type: EngineType,

    /// Source language of the game project
    pub source_language: Language,

    /// Target language of the game project
    pub target_language: Language,

    /// Version of the game engine if it could be determined
    ///
    /// Example: Some("1.6.1")
    pub version: Option<String>,

    /// Criteria that were used to detect this engine type
    pub detection_criteria: EngineCriteria,
}

/// Represents a game data file containing text that needs translation.
///
/// This struct models a file from the game that contains translatable text,
/// along with the extracted text units and metadata about the file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataFile {
    /// Name or identifier of the file (e.g., "Actors", "Items", "Map001")
    pub name: String,

    /// Relative path to the file from the project root
    pub path: String,

    /// Collection of text units extracted from this file that need translation
    pub text_units: Vec<TextUnit>,

    /// Total number of text units in the file (for quick reference)
    pub text_unit_count: u32,
}
