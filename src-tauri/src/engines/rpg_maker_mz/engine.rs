use std::any::Any;
use std::path::Path;

use crate::core::engine::Engine;
use crate::core::error::AppResult;
use crate::engines::common;
use crate::engines::rpg_maker_mv::files::actors as mz_actors;
use crate::engines::rpg_maker_mv::files::armors as mz_armors;
use crate::engines::rpg_maker_mv::files::classes as mz_classes;
use crate::engines::rpg_maker_mv::files::common_events as mz_common_events;
use crate::engines::rpg_maker_mv::files::enemies as mz_enemies;
use crate::engines::rpg_maker_mv::files::items as mz_items;
use crate::engines::rpg_maker_mv::files::maps as mz_maps;
use crate::engines::rpg_maker_mv::files::maps_infos as mz_maps_infos;
use crate::engines::rpg_maker_mv::files::skills as mz_skills;
use crate::engines::rpg_maker_mv::files::states as mz_states;
use crate::engines::rpg_maker_mv::files::system as mz_system;
use crate::engines::rpg_maker_mv::files::troops as mz_troops;
use crate::engines::rpg_maker_mv::files::weapons as mz_weapons;
use crate::models::engine::{EngineCriteria, EngineInfo, EngineType, GameDataFile};
use crate::models::language::Language;
use crate::models::translation::TextUnit;

/// RPG Maker MZ engine (MVP: Actors.json only)
pub struct RpgMakerMzEngine {
    detection_criteria: EngineCriteria,
}

impl RpgMakerMzEngine {
    pub fn new() -> Self {
        Self {
            detection_criteria: Self::get_detection_criteria(),
        }
    }

    pub fn get_detection_criteria() -> EngineCriteria {
        EngineCriteria {
            required_files: vec!["js/rmmz_core.js".to_string()],
            required_folders: vec!["data".to_string()],
            extra_files: vec![],
            export_data_roots: vec!["data".to_string()],
        }
    }

    /// Extract core MZ files (reuse MV modules; MZ uses data/ root)
    pub fn extract_game_data_files(
        &self,
        project_info: &EngineInfo,
    ) -> AppResult<Vec<GameDataFile>> {
        let mut files = Vec::new();

        let actors_paths = ["data/Actors.json"];
        let actors_files = common::extract_file_type_text(
            project_info,
            &actors_paths,
            mz_actors::extract_text,
            "Actors.json",
        )?;
        files.extend(actors_files);

        let classes_paths = ["data/Classes.json"];
        let classes_files = common::extract_file_type_text(
            project_info,
            &classes_paths,
            mz_classes::extract_text,
            "Classes.json",
        )?;
        files.extend(classes_files);

        let items_paths = ["data/Items.json"];
        let items_files = common::extract_file_type_text(
            project_info,
            &items_paths,
            mz_items::extract_text,
            "Items.json",
        )?;
        files.extend(items_files);

        let skills_paths = ["data/Skills.json"];
        let skills_files = common::extract_file_type_text(
            project_info,
            &skills_paths,
            mz_skills::extract_text,
            "Skills.json",
        )?;
        files.extend(skills_files);

        let weapons_paths = ["data/Weapons.json"];
        let weapons_files = common::extract_file_type_text(
            project_info,
            &weapons_paths,
            mz_weapons::extract_text,
            "Weapons.json",
        )?;
        files.extend(weapons_files);

        let armors_paths = ["data/Armors.json"];
        let armors_files = common::extract_file_type_text(
            project_info,
            &armors_paths,
            mz_armors::extract_text,
            "Armors.json",
        )?;
        files.extend(armors_files);

        let system_paths = ["data/System.json"];
        let system_files = common::extract_file_type_text(
            project_info,
            &system_paths,
            mz_system::extract_text,
            "System.json",
        )?;
        files.extend(system_files);

        let states_paths = ["data/States.json"];
        let states_files = common::extract_file_type_text(
            project_info,
            &states_paths,
            mz_states::extract_text,
            "States.json",
        )?;
        files.extend(states_files);

        let enemies_paths = ["data/Enemies.json"];
        let enemies_files = common::extract_file_type_text(
            project_info,
            &enemies_paths,
            mz_enemies::extract_text,
            "Enemies.json",
        )?;
        files.extend(enemies_files);

        let common_events_paths = ["data/CommonEvents.json"];
        let common_events_files = common::extract_file_type_text(
            project_info,
            &common_events_paths,
            mz_common_events::extract_text,
            "CommonEvents.json",
        )?;
        files.extend(common_events_files);

        let troops_paths = ["data/Troops.json"];
        let troops_files = common::extract_file_type_text(
            project_info,
            &troops_paths,
            mz_troops::extract_text,
            "Troops.json",
        )?;
        files.extend(troops_files);

        let map_infos_paths = ["data/MapInfos.json"];
        let map_infos_files = common::extract_file_type_text(
            project_info,
            &map_infos_paths,
            mz_maps_infos::extract_text,
            "MapInfos.json",
        )?;
        files.extend(map_infos_files);

        // Discover and extract MapXXX.json files under data/
        {
            use std::fs;
            let data_dir = project_info.path.join("data");
            if data_dir.exists() {
                let mut map_names: Vec<String> = fs::read_dir(&data_dir)
                    .ok()
                    .into_iter()
                    .flatten()
                    .filter_map(|entry| entry.ok())
                    .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
                    .filter(|name| name.starts_with("Map") && name.ends_with(".json"))
                    .collect();
                map_names.sort();
                for name in map_names {
                    let rel = format!("data/{}", name);
                    if let Ok(map_file) = mz_maps::extract_text(&project_info.path, &rel) {
                        files.push(map_file);
                    }
                }
            }
        }

        Ok(files)
    }

    /// Inject core MZ files
    pub fn inject_game_data_files(
        &self,
        project_info: &EngineInfo,
        text_units: &[TextUnit],
    ) -> AppResult<()> {
        common::inject_file_type_translations(
            project_info,
            text_units,
            "actor_",
            &["data/Actors.json"],
            mz_actors::inject_translations,
            "actor",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "class_",
            &["data/Classes.json"],
            mz_classes::inject_translations,
            "class",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "item_",
            &["data/Items.json"],
            mz_items::inject_translations,
            "item",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "skill_",
            &["data/Skills.json"],
            mz_skills::inject_translations,
            "skill",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "weapon_",
            &["data/Weapons.json"],
            mz_weapons::inject_translations,
            "weapon",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "armor_",
            &["data/Armors.json"],
            mz_armors::inject_translations,
            "armor",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "system_",
            &["data/System.json"],
            mz_system::inject_translations,
            "system",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "state_",
            &["data/States.json"],
            mz_states::inject_translations,
            "state",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "enemy_",
            &["data/Enemies.json"],
            mz_enemies::inject_translations,
            "enemy",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "common_event_",
            &["data/CommonEvents.json"],
            mz_common_events::inject_translations,
            "common_event",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "troop_",
            &["data/Troops.json"],
            mz_troops::inject_translations,
            "troop",
        )?;

        common::inject_file_type_translations(
            project_info,
            text_units,
            "map_info_",
            &["data/MapInfos.json"],
            mz_maps_infos::inject_translations,
            "map_info",
        )?;

        // Inject map events into MapXXX.json files under data/
        {
            use std::fs;
            let data_dir = project_info.path.join("data");
            if data_dir.exists() {
                let mut map_names: Vec<String> = fs::read_dir(&data_dir)
                    .ok()
                    .into_iter()
                    .flatten()
                    .filter_map(|entry| entry.ok())
                    .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
                    .filter(|name| name.starts_with("Map") && name.ends_with(".json"))
                    .collect();
                map_names.sort();

                for name in map_names {
                    let rel = format!("data/{}", name);
                    let map_id = mz_maps::extract_map_id(&rel).to_string();
                    let map_units: Vec<&TextUnit> = text_units
                        .iter()
                        .filter(|u| u.id.starts_with(&format!("map_{}_event_", map_id)))
                        .collect();
                    if !map_units.is_empty() {
                        let _ = mz_maps::inject_translations(&project_info.path, &rel, &map_units);
                    }
                }
            }
        }

        Ok(())
    }
}

impl Engine for RpgMakerMzEngine {
    fn load_project_info(
        &self,
        path: &Path,
        source_language: Language,
        target_language: Language,
    ) -> AppResult<EngineInfo> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown Project")
            .to_string();

        Ok(EngineInfo {
            name,
            path: path.to_path_buf(),
            engine_type: EngineType::RpgMakerMz,
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
        let files = self.extract_game_data_files(project_info)?;
        let mut out = Vec::new();
        for f in files {
            out.extend(f.text_units);
        }
        Ok(out)
    }

    fn inject_text_units(
        &self,
        project_info: &EngineInfo,
        text_units: &[TextUnit],
    ) -> AppResult<()> {
        self.inject_game_data_files(project_info, text_units)
    }

    fn reconstruct_text_unit_id(
        &self,
        field_type: &str,
        source_text: &str,
        translated_text: &str,
    ) -> AppResult<TextUnit> {
        // Parse field_type format - handle both regular format and event command format
        let parts: Vec<&str> = field_type.split(':').collect();
        
        // Handle event command format: "message:file_path:object_id:command_index"
        if parts.len() == 4 && parts[0] == "message" {
            let file_path = parts[1];
            let object_id: i32 = parts[2].parse().map_err(|_| {
                crate::core::error::AppError::Other(format!("Invalid object_id in field_type: {}", field_type))
            })?;
            let command_index: i32 = parts[3].parse().map_err(|_| {
                crate::core::error::AppError::Other(format!(
                    "Invalid command_index in field_type: {}",
                    field_type
                ))
            })?;
            
            // Determine object type from file path for event commands
            let object_type = if file_path.contains("Map") && file_path.contains(".json") {
                // Handle MapXXX.json files - extract map ID and create proper object type
                let map_id = mz_maps::extract_map_id(file_path);
                format!("map_{}_event", map_id)
            } else if file_path.contains("CommonEvents.json") {
                "common_event".to_string()
            } else if file_path.contains("Troops.json") {
                "troop".to_string()
            } else {
                "event".to_string() // fallback
            };
            
            let reconstructed_id = format!("{}_{}_message_{}", object_type, object_id, command_index);
            
            return Ok(TextUnit {
                id: reconstructed_id,
                source_text: source_text.to_string(),
                translated_text: translated_text.to_string(),
                status: crate::models::translation::TranslationStatus::MachineTranslated,
                field_type: field_type.to_string(),
                prompt_type: crate::models::translation::PromptType::Dialogue,
            });
        }
        
        // Handle choice command format: "choice:file_path:object_id:command_index:choice_index"
        if parts.len() == 5 && parts[0] == "choice" {
            let file_path = parts[1];
            let object_id: i32 = parts[2].parse().map_err(|_| {
                crate::core::error::AppError::Other(format!("Invalid object_id in field_type: {}", field_type))
            })?;
            let command_index: i32 = parts[3].parse().map_err(|_| {
                crate::core::error::AppError::Other(format!(
                    "Invalid command_index in field_type: {}",
                    field_type
                ))
            })?;
            let choice_index: i32 = parts[4].parse().map_err(|_| {
                crate::core::error::AppError::Other(format!(
                    "Invalid choice_index in field_type: {}",
                    field_type
                ))
            })?;
            
            // Determine object type from file path for event commands
            let object_type = if file_path.contains("Map") && file_path.contains(".json") {
                // Handle MapXXX.json files - extract map ID and create proper object type
                let map_id = mz_maps::extract_map_id(file_path);
                format!("map_{}_event", map_id)
            } else if file_path.contains("CommonEvents.json") {
                "common_event".to_string()
            } else if file_path.contains("Troops.json") {
                "troop".to_string()
            } else {
                "event".to_string() // fallback
            };
            
            let reconstructed_id = format!(
                "{}_{}_choice_{}_{}",
                object_type, object_id, command_index, choice_index
            );
            
            return Ok(TextUnit {
                id: reconstructed_id,
                source_text: source_text.to_string(),
                translated_text: translated_text.to_string(),
                status: crate::models::translation::TranslationStatus::MachineTranslated,
                field_type: field_type.to_string(),
                prompt_type: crate::models::translation::PromptType::Dialogue,
            });
        }
        
        // Handle regular format: "field:file.json:index"
        if parts.len() < 3 {
            return Err(crate::core::error::AppError::Other(format!(
                "Invalid field_type format: {}",
                field_type
            )));
        }

        let field = parts[0];
        let file_path = parts[1];
        let index: i32 = parts[2].parse().map_err(|_| {
            crate::core::error::AppError::Other(format!(
                "Invalid index in field_type: {}",
                field_type
            ))
        })?;

        // Determine object type from file path (RPG Maker MZ specific)
        let object_type = if file_path.contains("Actors.json") {
            format!("actor")
        } else if file_path.contains("Items.json") {
            format!("item")
        } else if file_path.contains("Skills.json") {
            format!("skill")
        } else if file_path.contains("Weapons.json") {
            format!("weapon")
        } else if file_path.contains("Armors.json") {
            format!("armor")
        } else if file_path.contains("Classes.json") {
            format!("class")
        } else if file_path.contains("System.json") {
            format!("system")
        } else if file_path.contains("States.json") {
            format!("state")
        } else if file_path.contains("Enemies.json") {
            format!("enemy")
        } else if file_path.contains("CommonEvents.json") {
            format!("common_event")
        } else if file_path.contains("Troops.json") {
            format!("troop")
        } else if file_path.contains("MapInfos.json") {
            format!("map_info")
        } else if file_path.contains("Map") && file_path.contains(".json") {
            // Handle MapXXX.json files - extract map ID and create proper object type
            let map_id = mz_maps::extract_map_id(file_path);
            format!("map_{}_event", map_id)
        } else {
            format!("other")
        };

        // Special handling for System.json - it uses simple field-based IDs without index
        let reconstructed_id = if file_path.contains("System.json") {
            // System uses format: "system_{field_name}" (no index)
            format!("system_{}", field)
        } else {
            // All other files use format: "object_type_index_field"
            format!("{}_{}_{}", object_type, index, field)
        };

        Ok(TextUnit {
            id: reconstructed_id,
            source_text: source_text.to_string(),
            translated_text: translated_text.to_string(),
            status: crate::models::translation::TranslationStatus::MachineTranslated,
            field_type: field_type.to_string(),
            prompt_type: crate::models::translation::PromptType::Character,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
