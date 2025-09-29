use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::core::error::{AppError, AppResult};
use crate::models::engine::GameDataFile;
use crate::models::translation::{PromptType, TextUnit, TranslationStatus};

/// Represents the RPG Maker MV System.json file structure.
/// This struct only includes translatable fields from the System.json file.
/// Non-translatable fields (audio, graphics, coordinates, etc.) are ignored.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct System {
    /// Game title
    #[serde(default, rename = "gameTitle")]
    pub game_title: String,

    /// Currency unit symbol
    #[serde(default, rename = "currencyUnit")]
    pub currency_unit: String,

    /// Armor type names
    #[serde(default, rename = "armorTypes")]
    pub armor_types: Vec<String>,

    /// Element names
    #[serde(default)]
    pub elements: Vec<String>,

    /// Equipment slot names
    #[serde(default, rename = "equipTypes")]
    pub equip_types: Vec<String>,

    /// Skill type names
    #[serde(default, rename = "skillTypes")]
    pub skill_types: Vec<String>,

    /// Weapon type names
    #[serde(default, rename = "weaponTypes")]
    pub weapon_types: Vec<String>,

    /// Switch names
    #[serde(default)]
    pub switches: Vec<String>,

    /// Variable names
    #[serde(default)]
    pub variables: Vec<String>,

    /// UI terms and messages
    #[serde(default)]
    pub terms: Terms,

    /// Additional fields that might be present in the JSON
    /// These are ignored during translation but preserved during serialization
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

/// Represents the terms section of System.json containing UI text.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Terms {
    /// Basic UI terms
    #[serde(default)]
    pub basic: Vec<Option<String>>,

    /// Command menu terms
    #[serde(default)]
    pub commands: Vec<Option<String>>,

    /// Parameter names
    #[serde(default)]
    pub params: Vec<Option<String>>,

    /// Battle and UI messages
    #[serde(default)]
    pub messages: HashMap<String, String>,
}

/// Extracts text units from a System.json file.
///
/// System.json has a unique structure compared to other RPG Maker files:
/// - It's a single object (not an array of objects)
/// - Contains multiple arrays of translatable text
/// - Has nested structures like the terms object
///
/// # Arguments
///
/// * `project_path` - Path to the project root
/// * `file_path` - Relative path to the System.json file
///
/// # Returns
///
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    let full_path = project_path.join(file_path);
    log::debug!(
        "Extracting text from System.json at: {}",
        full_path.display()
    );

    // Check if the file exists
    if !full_path.exists() {
        return Err(AppError::FileSystem(format!(
            "System.json not found at {}",
            full_path.display()
        )));
    }

    // Read the JSON file
    let content = fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read System.json: {}", e)))?;

    // Parse the JSON content
    let system: System = serde_json::from_str(&content)
        .map_err(|e| AppError::Parsing(format!("Failed to parse System.json: {}", e)))?;

    let mut text_units = Vec::new();

    // Extract game title
    if !system.game_title.is_empty() {
        text_units.push(TextUnit {
            id: "system_game_title".to_string(),
            source_text: system.game_title.clone(), // Raw text, no processing
            translated_text: String::new(),
            field_type: "gameTitle:www/data/System.json:0".to_string(),
            status: TranslationStatus::NotTranslated,
            prompt_type: PromptType::Character,
        });
    }

    // Extract currency unit
    if !system.currency_unit.is_empty() {
        text_units.push(TextUnit {
            id: "system_currency_unit".to_string(),
            source_text: system.currency_unit.clone(), // Raw text, no processing
            translated_text: String::new(),
            field_type: "currencyUnit:www/data/System.json:0".to_string(),
            status: TranslationStatus::NotTranslated,
            prompt_type: PromptType::System,
        });
    }

    // Extract armor types
    for (index, armor_type) in system.armor_types.iter().enumerate() {
        if !armor_type.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_armor_type_{}", index),
                source_text: armor_type.clone(), // Raw text, no processing
                translated_text: String::new(),
                field_type: format!("armorTypes[{}]:www/data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::Equipment,
            });
        }
    }

    // Extract elements
    for (index, element) in system.elements.iter().enumerate() {
        if !element.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_element_{}", index),
                source_text: element.clone(), // Raw text, no processing
                translated_text: String::new(),
                field_type: format!("elements[{}]:www/data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::System,
            });
        }
    }

    // Extract equipment types
    for (index, equip_type) in system.equip_types.iter().enumerate() {
        if !equip_type.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_equip_type_{}", index),
                source_text: equip_type.clone(), // Raw text, no processing
                translated_text: String::new(),
                field_type: format!("equipTypes[{}]:www/data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::Equipment,
            });
        }
    }

    // Extract skill types
    for (index, skill_type) in system.skill_types.iter().enumerate() {
        if !skill_type.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_skill_type_{}", index),
                source_text: skill_type.clone(), // Raw text, no processing
                translated_text: String::new(),
                field_type: format!("skillTypes[{}]:www/data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::Skill,
            });
        }
    }

    /* // Extract weapon types
    for (index, weapon_type) in system.weapon_types.iter().enumerate() {
        if !weapon_type.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_weapon_type_{}", index),
                source_text: weapon_type.clone(), // Fixed: use actual weapon_type instead of game_title
                translated_text: String::new(),
                field_type: format!("weaponTypes[{}]:www/data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::Equipment,
            });
        }
    }*/

    /* // Extract switches (filter out technical/empty ones)
    for (index, switch_name) in system.switches.iter().enumerate() {
        if !switch_name.is_empty() && !is_technical_switch_name(switch_name) {
            text_units.push(TextUnit {
                id: format!("system_switch_{}", index),
                source_text: switch_name.clone(), // Fixed: use actual switch_name
                translated_text: String::new(),
                field_type: format!("switches[{}]:www/data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::System,
            });
        }
    }*/

    /* // Extract variables (filter out technical/empty ones)
    for (index, variable_name) in system.variables.iter().enumerate() {
        if !variable_name.is_empty() && !is_technical_variable_name(variable_name) {
            text_units.push(TextUnit {
                id: format!("system_variable_{}", index),
                source_text: variable_name.clone(), // Fixed: use actual variable_name
                translated_text: String::new(),
                field_type: format!("variables[{}]:www/data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::System,
            });
        }
    }*/

    // Extract basic terms
    for (index, basic_term) in system.terms.basic.iter().enumerate() {
        if let Some(term) = basic_term {
            if !term.is_empty() {
                text_units.push(TextUnit {
                    id: format!("system_basic_term_{}", index),
                    source_text: term.clone(), // Raw text, no processing
                    translated_text: String::new(),
                    field_type: format!("terms.basic[{}]:www/data/System.json:0", index),
                    status: TranslationStatus::NotTranslated,
                    prompt_type: PromptType::System,
                });
            }
        }
    }

    // Extract command terms
    for (index, command_term) in system.terms.commands.iter().enumerate() {
        if let Some(term) = command_term {
            if !term.is_empty() {
                text_units.push(TextUnit {
                    id: format!("system_command_term_{}", index),
                    source_text: term.clone(), // Raw text, no processing
                    translated_text: String::new(),
                    field_type: format!("terms.commands[{}]:www/data/System.json:0", index),
                    status: TranslationStatus::NotTranslated,
                    prompt_type: PromptType::System,
                });
            }
        }
    }

    // Extract parameter terms
    for (index, param_term) in system.terms.params.iter().enumerate() {
        if let Some(term) = param_term {
            if !term.is_empty() {
                text_units.push(TextUnit {
                    id: format!("system_param_term_{}", index),
                    source_text: term.clone(), // Raw text, no processing
                    translated_text: String::new(),
                    field_type: format!("terms.params[{}]:www/data/System.json:0", index),
                    status: TranslationStatus::NotTranslated,
                    prompt_type: PromptType::System,
                });
            }
        }
    }

    // Extract message terms
    for (key, message) in system.terms.messages.iter() {
        if !message.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_message_{}", key),
                source_text: message.clone(), // Raw text, no processing
                translated_text: String::new(),
                field_type: format!("terms.messages.{}:www/data/System.json:0", key),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::System,
            });
        }
    }

    let text_unit_count = text_units.len() as u32;
    log::info!("Extracted {} text units from System.json", text_unit_count);

    Ok(GameDataFile {
        name: "System.json".to_string(),
        path: file_path.to_string(),
        text_units,
        text_unit_count: text_unit_count,
    })
}

/// Injects translated text back into a System.json file.
///
/// # Arguments
///
/// * `project_path` - Path to the project root
/// * `file_path` - Relative path to the System.json file
/// * `text_units` - Array of text units with translations
///
/// # Returns
///
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    let full_path = project_path.join(file_path);
    log::debug!(
        "Injecting translations into System.json at: {}",
        full_path.display()
    );

    // Check if the file exists
    if !full_path.exists() {
        return Err(AppError::FileSystem(format!(
            "System.json not found at {}",
            full_path.display()
        )));
    }

    // Read the JSON file
    let content = fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read System.json: {}", e)))?;

    // Parse the JSON content
    let mut system: System = serde_json::from_str(&content)
        .map_err(|e| AppError::Parsing(format!("Failed to parse System.json: {}", e)))?;

    // Create a HashMap for quick lookup of text units by ID
    let text_units_map: HashMap<String, &TextUnit> = text_units
        .iter()
        .map(|unit| (unit.id.clone(), *unit))
        .collect();

    // Update game title
    if let Some(unit) = text_units_map.get("system_game_title") {
        if !unit.translated_text.is_empty() {
            let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                                                         // Add LudoLingua signature to the translated title
            system.game_title = format!("{} - Translated by LudoLingua", restored);
        }
    }

    // Update currency unit
    if let Some(unit) = text_units_map.get("system_currency_unit") {
        if !unit.translated_text.is_empty() {
            let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
            system.currency_unit = restored;
        }
    }

    // Update armor types
    for (index, armor_type) in system.armor_types.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_armor_type_{}", index)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *armor_type = restored;
            }
        }
    }

    // Update elements
    for (index, element) in system.elements.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_element_{}", index)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *element = restored;
            }
        }
    }

    // Update equipment types
    for (index, equip_type) in system.equip_types.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_equip_type_{}", index)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *equip_type = restored;
            }
        }
    }

    // Update skill types
    for (index, skill_type) in system.skill_types.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_skill_type_{}", index)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *skill_type = restored;
            }
        }
    }

    // Update weapon types
    /*for (index, weapon_type) in system.weapon_types.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_weapon_type_{}", index)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *weapon_type = restored;
            }
        }
    }*/

    // Update switches
    /*for (index, switch_name) in system.switches.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_switch_{}", index)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *switch_name = restored;
            }
        }
    }*/

    // Update variables
    /*for (index, variable_name) in system.variables.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_variable_{}", index)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *variable_name = restored;
            }
        }
    }*/

    // Update basic terms
    for (index, basic_term) in system.terms.basic.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_basic_term_{}", index)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *basic_term = Some(restored);
            }
        }
    }

    // Update command terms
    for (index, command_term) in system.terms.commands.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_command_term_{}", index)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *command_term = Some(restored);
            }
        }
    }

    // Update parameter terms
    for (index, param_term) in system.terms.params.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_param_term_{}", index)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *param_term = Some(restored);
            }
        }
    }

    // Update message terms
    for (key, message) in system.terms.messages.iter_mut() {
        if let Some(unit) = text_units_map.get(&format!("system_message_{}", key)) {
            if !unit.translated_text.is_empty() {
                let restored = unit.translated_text.clone(); // Text processing handled by unified pipeline
                *message = restored;
            }
        }
    }

    // Write the updated JSON back to the file
    let updated_content = serde_json::to_string_pretty(&system)
        .map_err(|e| AppError::Parsing(format!("Failed to serialize System.json: {}", e)))?;

    fs::write(&full_path, updated_content)
        .map_err(|e| AppError::FileSystem(format!("Failed to write System.json: {}", e)))?;

    log::info!("Successfully injected translations into System.json");
    Ok(())
}
