//use log::{debug, info};
use serde_json::Value;
use std::any::Any;
use std::fs;
use std::path::Path;

use crate::core::engine::Engine;
use crate::core::error::{AppError, AppResult};
use crate::engines::common;
use crate::engines::rpg_maker_mv::files::{
    actors, armors, classes, common_events, enemies, items, maps, maps_infos, skills, states,
    system, troops, weapons,
};
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
            extra_files: vec![
                "js/rpg_core.js".to_string(), // Alternative structure
            ],
            export_data_roots: vec!["www/data".to_string()],
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
        // debug!(
        //     "Extracting game data files from RPG Maker MV project: {}",
        //     project_info.name
        // );
        let mut game_data_files = Vec::new();

        // Extract text from Actors.json
        let actors_paths = ["www/data/Actors.json"];
        let actors_files = common::extract_file_type_text(
            project_info,
            &actors_paths,
            actors::extract_text,
            "Actors.json",
        )?;
        game_data_files.extend(actors_files);

        // Extract text from Items.json
        let items_paths = ["www/data/Items.json"];
        let items_files = common::extract_file_type_text(
            project_info,
            &items_paths,
            items::extract_text,
            "Items.json",
        )?;
        game_data_files.extend(items_files);

        // Extract text from Skills.json
        let skills_paths = ["www/data/Skills.json"];
        let skills_files = common::extract_file_type_text(
            project_info,
            &skills_paths,
            skills::extract_text,
            "Skills.json",
        )?;
        game_data_files.extend(skills_files);

        // Extract text from Weapons.json
        let weapons_paths = ["www/data/Weapons.json"];
        let weapons_files = common::extract_file_type_text(
            project_info,
            &weapons_paths,
            weapons::extract_text,
            "Weapons.json",
        )?;
        game_data_files.extend(weapons_files);

        // Extract text from Armors.json
        let armors_paths = ["www/data/Armors.json"];
        let armors_files = common::extract_file_type_text(
            project_info,
            &armors_paths,
            armors::extract_text,
            "Armors.json",
        )?;
        game_data_files.extend(armors_files);

        // Extract text from Classes.json
        let classes_paths = ["www/data/Classes.json"];
        let classes_files = common::extract_file_type_text(
            project_info,
            &classes_paths,
            classes::extract_text,
            "Classes.json",
        )?;
        game_data_files.extend(classes_files);

        // Extract text from System.json
        let system_paths = ["www/data/System.json"];
        let system_files = common::extract_file_type_text(
            project_info,
            &system_paths,
            system::extract_text,
            "System.json",
        )?;
        game_data_files.extend(system_files);

        // Extract text from States.json
        let states_paths = ["www/data/States.json"];
        let states_files = common::extract_file_type_text(
            project_info,
            &states_paths,
            states::extract_text,
            "States.json",
        )?;
        game_data_files.extend(states_files);

        // Extract text from Enemies.json
        let enemies_paths = ["www/data/Enemies.json"];
        let enemies_files = common::extract_file_type_text(
            project_info,
            &enemies_paths,
            enemies::extract_text,
            "Enemies.json",
        )?;
        game_data_files.extend(enemies_files);

        // Extract text from CommonEvents.json
        let common_events_paths = ["www/data/CommonEvents.json"];
        let common_events_files = common::extract_file_type_text(
            project_info,
            &common_events_paths,
            common_events::extract_text,
            "CommonEvents.json",
        )?;
        game_data_files.extend(common_events_files);

        // Extract text from Troops.json
        let troops_paths = ["www/data/Troops.json"];
        let troops_files = common::extract_file_type_text(
            project_info,
            &troops_paths,
            troops::extract_text,
            "Troops.json",
        )?;
        game_data_files.extend(troops_files);

        // Extract text from MapInfos.json
        let map_infos_paths = ["www/data/MapInfos.json"];
        let map_infos_files = common::extract_file_type_text(
            project_info,
            &map_infos_paths,
            maps_infos::extract_text,
            "MapInfos.json",
        )?;
        game_data_files.extend(map_infos_files);

        // Extract text from MapXXX.json files (dynamic discovery)
        let map_files = maps::discover_map_files(&project_info.path)?;
        for map_file_path in &map_files {
            match maps::extract_text(&project_info.path, map_file_path) {
                Ok(map_file) => {
                    game_data_files.push(map_file);
                }
                Err(_e) => {
                    // log::warn!("Failed to extract text from {}: {}", map_file_path, e);
                    // Continue with other map files
                }
            }
        }

        // info!(
        //     "Extracted {} game data files from RPG Maker MV project",
        //     game_data_files.len()
        // );

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
        // debug!(
        //     "Injecting translations into RPG Maker MV project: {}",
        //     project_info.name
        // );

        // Inject actor translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "actor_",
            &["www/data/Actors.json"],
            actors::inject_translations,
            "actor",
        )?;

        // Inject item translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "item_",
            &["www/data/Items.json"],
            items::inject_translations,
            "item",
        )?;

        // Inject skill translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "skill_",
            &["www/data/Skills.json"],
            skills::inject_translations,
            "skill",
        )?;

        // Inject weapon translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "weapon_",
            &["www/data/Weapons.json"],
            weapons::inject_translations,
            "weapon",
        )?;

        // Inject armor translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "armor_",
            &["www/data/Armors.json"],
            armors::inject_translations,
            "armor",
        )?;

        // Inject class translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "class_",
            &["www/data/Classes.json"],
            classes::inject_translations,
            "class",
        )?;

        // Inject system translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "system_",
            &["www/data/System.json"],
            system::inject_translations,
            "system",
        )?;

        // Inject states translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "state_",
            &["www/data/States.json"],
            states::inject_translations,
            "state",
        )?;

        // Inject enemies translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "enemy_",
            &["www/data/Enemies.json"],
            enemies::inject_translations,
            "enemy",
        )?;

        // Inject common events translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "common_event_",
            &["www/data/CommonEvents.json"],
            common_events::inject_translations,
            "common_event",
        )?;

        // Inject troops translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "troop_",
            &["www/data/Troops.json"],
            troops::inject_translations,
            "troop",
        )?;

        // Inject map infos translations
        common::inject_file_type_translations(
            project_info,
            text_units,
            "map_info_",
            &["www/data/MapInfos.json"],
            maps_infos::inject_translations,
            "map_info",
        )?;

        // Inject map event translations (dynamic discovery)
        let map_files = maps::discover_map_files(&project_info.path)?;
        log::info!("Found {} map files to process", map_files.len());

        for map_file_path in &map_files {
            // Extract map ID from file path
            let map_id = maps::extract_map_id(map_file_path);
            log::info!(
                "Processing map file: {} with map_id: {}",
                map_file_path,
                map_id
            );

            // Filter text units for this specific map file
            let map_event_units: Vec<&TextUnit> = text_units
                .iter()
                .filter(|unit| unit.id.starts_with(&format!("map_{}_event_", map_id)))
                .collect();

            log::info!(
                "Found {} text units for map {}: {:?}",
                map_event_units.len(),
                map_id,
                map_event_units.iter().map(|u| &u.id).collect::<Vec<_>>()
            );

            if !map_event_units.is_empty() {
                match maps::inject_translations(&project_info.path, map_file_path, &map_event_units)
                {
                    Ok(_) => {
                        log::info!("Successfully injected translations into {}", map_file_path);
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to inject translations into {}: {}",
                            map_file_path,
                            e
                        );
                        // Continue with other map files
                    }
                }
            } else {
                log::info!("No text units found for map {}", map_id);
            }
        }

        // info!("Translation injection completed");
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
        // Try to read the package.json file to get project metadata
        let (name, version) = match self.read_package_json(path) {
            Ok(result) => result,
            Err(_e) => {
                // If package.json doesn't exist or can't be read, use the folder name as project name
                // info!(
                //     "Couldn't read package.json: {}. Using folder name instead.",
                //     e
                // );
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
            manifest_hash: None,
        })
    }

    fn get_detection_criteria(&self) -> EngineCriteria {
        self.detection_criteria.clone()
    }

    fn extract_text_units(&self, project_info: &EngineInfo) -> AppResult<Vec<TextUnit>> {
        // debug!(
        //     "Extracting text units from RPG Maker MV project: {}",
        //     project_info.name
        // );

        // Extract game data files
        let game_data_files = self.extract_game_data_files(project_info)?;

        // Flatten all text units into a single list
        let mut all_text_units = Vec::new();
        for file in &game_data_files {
            all_text_units.extend(file.text_units.clone());
        }

        // info!("Total extracted text units: {}", all_text_units.len());
        Ok(all_text_units)
    }

    fn inject_text_units(
        &self,
        project_info: &EngineInfo,
        text_units: &[TextUnit],
    ) -> AppResult<()> {
        // debug!(
        //     "Injecting text units for RPG Maker MV project: {}",
        //     project_info.name
        // );

        // Use our inject_game_data_files method to inject translations back to the game files
        self.inject_game_data_files(project_info, text_units)?;

        // info!(
        //     "Successfully injected {} text units into project files",
        //     text_units.len()
        // );
        Ok(())
    }

    fn reconstruct_text_unit_id(&self, field_type: &str, source_text: &str, translated_text: &str) -> AppResult<TextUnit> {
        // Parse field_type format: "field:file.json:index"
        let parts: Vec<&str> = field_type.split(':').collect();
        if parts.len() < 3 {
            return Err(AppError::Other(format!("Invalid field_type format: {}", field_type)));
        }

        let field = parts[0];
        let file_path = parts[1];
        let index: i32 = parts[2].parse().map_err(|_| AppError::Other(format!("Invalid index in field_type: {}", field_type)))?;

        // Determine object type from file path (RPG Maker MV specific)
        let object_type = if file_path.contains("Actors.json") {
            "actor"
        } else if file_path.contains("Items.json") {
            "item"
        } else if file_path.contains("Skills.json") {
            "skill"
        } else if file_path.contains("Weapons.json") {
            "weapon"
        } else if file_path.contains("Armors.json") {
            "armor"
        } else if file_path.contains("Classes.json") {
            "class"
        } else if file_path.contains("States.json") {
            "state"
        } else if file_path.contains("Enemies.json") {
            "enemy"
        } else if file_path.contains("Troops.json") {
            "troop"
        } else {
            "other"
        };

        let reconstructed_id = format!("{}_{}_{}", object_type, index, field);

        Ok(TextUnit {
            id: reconstructed_id,
            source_text: source_text.to_string(),
            translated_text: translated_text.to_string(),
            status: crate::models::translation::TranslationStatus::MachineTranslated, // Default for export
            field_type: field_type.to_string(),
            prompt_type: crate::models::translation::PromptType::Character, // Default, can be refined
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
