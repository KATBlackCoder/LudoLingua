use log::{debug, info, warn};
use serde_json::Value;
use std::any::Any;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::engine::Engine;
use crate::core::error::{AppError, AppResult};
use crate::engines::rpg_maker_mv::files::actors;
use crate::models::engine::{EngineCriteria, EngineInfo, EngineType, GameDataFile};
use crate::models::language::Language;
use crate::models::translation::TextUnit;

/// Implementation of the Engine trait for RPG Maker MV games.
///
/// This struct handles the specifics of working with RPG Maker MV game files,
/// including parsing the JSON data files and extracting translatable text.
pub struct RpgMakerMvEngine {
    /// The base criteria used to detect RPG Maker MV projects
    detection_criteria: EngineCriteria,
}

impl RpgMakerMvEngine {
    /// Creates a new instance of the RPG Maker MV engine.
    ///
    /// # Returns
    ///
    /// * `RpgMakerMvEngine` - A new instance of the engine
    pub fn new() -> Self {
        Self {
            detection_criteria: Self::get_detection_criteria(),
        }
    }

    /// Returns the criteria used to detect RPG Maker MV projects.
    ///
    /// # Returns
    ///
    /// * `EngineCriteria` - The detection criteria for RPG Maker MV
    pub fn get_detection_criteria() -> EngineCriteria {
        // We'll be more flexible with detection to handle different project structures
        EngineCriteria {
            // Either of these structures is common for RPG Maker MV
            required_files: vec!["www/js/rpg_core.js".to_string()],
            required_folders: vec![
                "www".to_string(), // Most common structure
            ],
            optional_files: Some(vec![
                "js/rpg_core.js".to_string(), // Alternative structure
            ]),
        }
    }

    /// Reads and parses the package.json file to extract project metadata.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the project directory
    ///
    /// # Returns
    ///
    /// * `AppResult<(String, Option<String>)>` - A tuple containing the project name and version
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The package.json file doesn't exist
    /// * The file cannot be read
    /// * The file contains invalid JSON
    fn read_package_json(&self, path: &Path) -> AppResult<(String, Option<String>)> {
        let package_json_path = path.join("package.json");

        // Check if the file exists
        if !package_json_path.exists() {
            return Err(AppError::FileSystem(format!(
                "package.json not found at {}",
                package_json_path.display()
            )));
        }

        // Read the file content
        let content = fs::read_to_string(&package_json_path)
            .map_err(|e| AppError::FileSystem(format!("Failed to read package.json: {}", e)))?;

        // Parse the JSON
        let json: Value = serde_json::from_str(&content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse package.json: {}", e)))?;

        // Extract the name and version
        let name = json["name"]
            .as_str()
            .unwrap_or("Unknown Project")
            .to_string();

        let version = json["version"].as_str().map(|s| s.to_string());

        Ok((name, version))
    }

    /// Extracts text units from all supported game data files.
    ///
    /// # Arguments
    ///
    /// * `project_info` - Information about the project
    ///
    /// # Returns
    ///
    /// * `AppResult<Vec<GameDataFile>>` - A vector of GameDataFile objects
    pub fn extract_game_data_files(
        &self,
        project_info: &EngineInfo,
    ) -> AppResult<Vec<GameDataFile>> {
        debug!(
            "Extracting game data files from RPG Maker MV project: {}",
            project_info.name
        );
        let mut game_data_files = Vec::new();

        // For Phase 1, we're only implementing Actors.json parsing
        // Check for different possible locations of Actors.json
        let possible_paths = ["www/data/Actors.json"];

        let mut actors_found = false;

        for path in possible_paths.iter() {
            let file_path = project_info.path.join(path);
            if file_path.exists() {
                debug!("Found Actors.json at: {}", file_path.display());

                // Extract text from Actors.json
                match actors::extract_text(&project_info.path, path) {
                    Ok(game_data_file) => {
                        info!(
                            "Extracted {} text units from {}",
                            game_data_file.text_unit_count, path
                        );

                        // Add the game data file to our collection
                        game_data_files.push(game_data_file);

                        actors_found = true;
                        break; // We found and processed Actors.json, no need to check other paths
                    }
                    Err(e) => {
                        warn!("Failed to extract text from {}: {}", path, e);
                        // Continue trying other paths
                    }
                }
            }
        }

        if !actors_found {
            warn!("Could not find Actors.json in any expected location");
        }

        Ok(game_data_files)
    }

    /// Injects translated text units back into all supported game data files.
    ///
    /// # Arguments
    ///
    /// * `project_info` - Information about the project
    /// * `text_units` - Vector of translated text units to inject
    ///
    /// # Returns
    ///
    /// * `AppResult<()>` - Success or error
    pub fn inject_game_data_files(
        &self,
        project_info: &EngineInfo,
        text_units: &[TextUnit],
    ) -> AppResult<()> {
        debug!(
            "Injecting translations into RPG Maker MV project: {}",
            project_info.name
        );

        // Filter text units by file type to inject only relevant ones
        let actor_units: Vec<&TextUnit> = text_units
            .iter()
            .filter(|unit| unit.id.starts_with("actor_"))
            .collect();

        if !actor_units.is_empty() {
            // Check for different possible locations of Actors.json
            let possible_paths = ["www/data/Actors.json"];

            let mut actors_injected = false;

            for path in possible_paths.iter() {
                let file_path = project_info.path.join(path);
                if file_path.exists() {
                    debug!("Injecting into Actors.json at: {}", file_path.display());

                    // Inject translations into Actors.json (pass slice instead of cloning)
                    match actors::inject_translations(&project_info.path, path, &actor_units) {
                        Ok(()) => {
                            info!(
                                "Successfully injected {} actor translations into {}",
                                actor_units.len(),
                                path
                            );
                            actors_injected = true;
                            break; // We successfully injected, no need to check other paths
                        }
                        Err(e) => {
                            warn!("Failed to inject translations into {}: {}", path, e);
                            // Continue trying other paths
                        }
                    }
                }
            }

            if !actors_injected && !actor_units.is_empty() {
                warn!("Could not inject actor translations - Actors.json not found in any expected location");
            }
        }

        info!("Translation injection completed");
        Ok(())
    }
}

impl Engine for RpgMakerMvEngine {
    fn load_project_info(
        &self,
        path: &Path,
        source_language: Language,
        target_language: Language,
    ) -> AppResult<EngineInfo> {
        use log::info;

        // Try to read the package.json file to get project metadata
        let (name, version) = match self.read_package_json(path) {
            Ok(result) => result,
            Err(e) => {
                // If package.json doesn't exist or can't be read, use the folder name as project name
                info!(
                    "Couldn't read package.json: {}. Using folder name instead.",
                    e
                );
                let folder_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Unknown Project")
                    .to_string();

                (folder_name, None)
            }
        };

        // Create and return the EngineInfo
        Ok(EngineInfo {
            name,
            path: path.to_path_buf(),
            engine_type: EngineType::RpgMakerMv,
            source_language,
            target_language,
            version,
            detection_criteria: self.detection_criteria.clone(),
        })
    }

    fn get_detection_criteria(&self) -> EngineCriteria {
        self.detection_criteria.clone()
    }

    fn extract_text_units(&self, project_info: &EngineInfo) -> AppResult<Vec<TextUnit>> {
        debug!(
            "Extracting text units from RPG Maker MV project: {}",
            project_info.name
        );

        // Extract game data files
        let game_data_files = self.extract_game_data_files(project_info)?;

        // Flatten all text units into a single list
        let mut all_text_units = Vec::new();
        for file in &game_data_files {
            all_text_units.extend(file.text_units.clone());
        }

        info!("Total extracted text units: {}", all_text_units.len());
        Ok(all_text_units)
    }

    fn inject_text_units(&self, project_info: &EngineInfo, text_units: &[TextUnit]) -> AppResult<()> {
        debug!("Injecting text units for RPG Maker MV project: {}", project_info.name);
        
        // Use our inject_game_data_files method to inject translations back to the game files
        self.inject_game_data_files(project_info, text_units)?;
        
        info!("Successfully injected {} text units into project files", text_units.len());
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
