use crate::models::translation::{PromptType, TextUnit, TranslationStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// RPG Maker MZ System.json structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    #[serde(rename = "gameTitle")]
    pub game_title: String,
    #[serde(rename = "currencyUnit")]
    pub currency_unit: String,
    #[serde(rename = "armorTypes")]
    pub armor_types: Vec<String>,
    #[serde(rename = "equipTypes")]
    pub equip_types: Vec<String>,
    #[serde(rename = "skillTypes")]
    pub skill_types: Vec<String>,
    // Non-translatable fields (commented out for clarity)
    // #[serde(rename = "weaponTypes")]
    // pub weapon_types: Vec<String>,
    // pub elements: Vec<String>,
    // pub switches: Vec<String>,
    // pub variables: Vec<String>,
    pub terms: Terms,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

/// RPG Maker MZ Terms structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terms {
    pub basic: Vec<String>,
    pub commands: Vec<String>,
    pub params: Vec<String>,
    pub messages: HashMap<String, String>,
}

/// Extract translatable text from RPG Maker MZ System.json
pub fn extract_text(project_path: &std::path::Path, relative_path: &str) -> Result<crate::models::engine::GameDataFile, crate::core::error::AppError> {
    let file_path = project_path.join(relative_path);
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| crate::core::error::AppError::Other(format!("Failed to read System.json: {}", e)))?;
    let system: System = serde_json::from_str(&content)
        .map_err(|e| crate::core::error::AppError::Other(format!("Failed to parse System.json: {}", e)))?;
    
    let mut text_units = Vec::new();
    
    // Extract game title
    if !system.game_title.is_empty() {
        text_units.push(TextUnit {
            id: "system_game_title".to_string(),
            source_text: system.game_title.clone(),
            translated_text: String::new(),
            field_type: "gameTitle:data/System.json:0".to_string(),
            status: TranslationStatus::NotTranslated,
            prompt_type: PromptType::Character,
        });
    }
    
    // Extract currency unit
    if !system.currency_unit.is_empty() {
        text_units.push(TextUnit {
            id: "system_currency_unit".to_string(),
            source_text: system.currency_unit.clone(),
            translated_text: String::new(),
            field_type: "currencyUnit:data/System.json:0".to_string(),
            status: TranslationStatus::NotTranslated,
            prompt_type: PromptType::System,
        });
    }
    
    // Extract armor types
    for (index, armor_type) in system.armor_types.iter().enumerate() {
        if !armor_type.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_armor_type_{}", index),
                source_text: armor_type.clone(),
                translated_text: String::new(),
                field_type: format!("armorTypes[{}]:data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::Equipment,
            });
        }
    }
    
    // Extract equip types
    for (index, equip_type) in system.equip_types.iter().enumerate() {
        if !equip_type.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_equip_type_{}", index),
                source_text: equip_type.clone(),
                translated_text: String::new(),
                field_type: format!("equipTypes[{}]:data/System.json:0", index),
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
                source_text: skill_type.clone(),
                translated_text: String::new(),
                field_type: format!("skillTypes[{}]:data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::Skill,
            });
        }
    }
    
    // Non-translatable fields (commented out)
    // // Extract elements
    // for (index, element) in system.elements.iter().enumerate() {
    //     if !element.is_empty() {
    //         text_units.push(TextUnit {
    //             id: format!("system_element_{}", index),
    //             source_text: element.clone(),
    //             translated_text: String::new(),
    //             field_type: format!("elements[{}]:data/System.json:0", index),
    //             status: TranslationStatus::NotTranslated,
    //             prompt_type: PromptType::System,
    //         });
    //     }
    // }
    
    // // Extract switches
    // for (index, switch_name) in system.switches.iter().enumerate() {
    //     if !switch_name.is_empty() {
    //         text_units.push(TextUnit {
    //             id: format!("system_switch_{}", index),
    //             source_text: switch_name.clone(),
    //             translated_text: String::new(),
    //             field_type: format!("switches[{}]:data/System.json:0", index),
    //             status: TranslationStatus::NotTranslated,
    //             prompt_type: PromptType::System,
    //         });
    //     }
    // }
    
    // // Extract variables
    // for (index, variable_name) in system.variables.iter().enumerate() {
    //     if !variable_name.is_empty() {
    //         text_units.push(TextUnit {
    //             id: format!("system_variable_{}", index),
    //             source_text: variable_name.clone(),
    //             translated_text: String::new(),
    //             field_type: format!("variables[{}]:data/System.json:0", index),
    //             status: TranslationStatus::NotTranslated,
    //             prompt_type: PromptType::System,
    //         });
    //     }
    // }
    
    // Extract basic terms
    for (index, term) in system.terms.basic.iter().enumerate() {
        if !term.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_basic_term_{}", index),
                source_text: term.clone(),
                translated_text: String::new(),
                field_type: format!("terms.basic[{}]:data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::System,
            });
        }
    }
    
    // Extract command terms
    for (index, term) in system.terms.commands.iter().enumerate() {
        if !term.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_command_term_{}", index),
                source_text: term.clone(),
                translated_text: String::new(),
                field_type: format!("terms.commands[{}]:data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::System,
            });
        }
    }
    
    // Extract param terms
    for (index, term) in system.terms.params.iter().enumerate() {
        if !term.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_param_term_{}", index),
                source_text: term.clone(),
                translated_text: String::new(),
                field_type: format!("terms.params[{}]:data/System.json:0", index),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::System,
            });
        }
    }
    
    // Extract message terms
    for (key, message) in &system.terms.messages {
        if !message.is_empty() {
            text_units.push(TextUnit {
                id: format!("system_message_{}", key),
                source_text: message.clone(),
                translated_text: String::new(),
                field_type: format!("terms.messages.{}:data/System.json:0", key),
                status: TranslationStatus::NotTranslated,
                prompt_type: PromptType::System,
            });
        }
    }
    
    Ok(crate::models::engine::GameDataFile {
        name: "System".to_string(),
        path: relative_path.to_string(),
        text_unit_count: text_units.len() as u32,
        text_units,
    })
}

/// Inject translations into RPG Maker MZ System.json
pub fn inject_translations(
    project_path: &std::path::Path,
    relative_path: &str,
    text_units: &[&TextUnit],
) -> Result<(), crate::core::error::AppError> {
    let file_path = project_path.join(relative_path);
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| crate::core::error::AppError::Other(format!("Failed to read System.json: {}", e)))?;
    let mut system: System = serde_json::from_str(&content)
        .map_err(|e| crate::core::error::AppError::Other(format!("Failed to parse System.json: {}", e)))?;
    
    // Create a map for quick lookup
    let text_units_map: std::collections::HashMap<String, &TextUnit> = 
        text_units.iter().map(|unit| (unit.id.clone(), *unit)).collect();
    
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
            system.currency_unit = unit.translated_text.clone();
        }
    }
    
    // Update armor types
    for (index, armor_type) in system.armor_types.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_armor_type_{}", index)) {
            if !unit.translated_text.is_empty() {
                *armor_type = unit.translated_text.clone();
            }
        }
    }
    
    // Update equip types
    for (index, equip_type) in system.equip_types.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_equip_type_{}", index)) {
            if !unit.translated_text.is_empty() {
                *equip_type = unit.translated_text.clone();
            }
        }
    }
    
    // Update skill types
    for (index, skill_type) in system.skill_types.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_skill_type_{}", index)) {
            if !unit.translated_text.is_empty() {
                *skill_type = unit.translated_text.clone();
            }
        }
    }
    
    // Non-translatable fields (commented out)
    // // Update elements
    // for (index, element) in system.elements.iter_mut().enumerate() {
    //     if let Some(unit) = text_units_map.get(&format!("system_element_{}", index)) {
    //         if !unit.translated_text.is_empty() {
    //             *element = unit.translated_text.clone();
    //         }
    //     }
    // }
    
    // // Update switches
    // for (index, switch_name) in system.switches.iter_mut().enumerate() {
    //     if let Some(unit) = text_units_map.get(&format!("system_switch_{}", index)) {
    //         if !unit.translated_text.is_empty() {
    //             *switch_name = unit.translated_text.clone();
    //         }
    //     }
    // }
    
    // // Update variables
    // for (index, variable_name) in system.variables.iter_mut().enumerate() {
    //     if let Some(unit) = text_units_map.get(&format!("system_variable_{}", index)) {
    //         if !unit.translated_text.is_empty() {
    //             *variable_name = unit.translated_text.clone();
    //         }
    //     }
    // }
    
    // Update basic terms
    for (index, term) in system.terms.basic.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_basic_term_{}", index)) {
            if !unit.translated_text.is_empty() {
                *term = unit.translated_text.clone();
            }
        }
    }
    
    // Update command terms
    for (index, term) in system.terms.commands.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_command_term_{}", index)) {
            if !unit.translated_text.is_empty() {
                *term = unit.translated_text.clone();
            }
        }
    }
    
    // Update param terms
    for (index, term) in system.terms.params.iter_mut().enumerate() {
        if let Some(unit) = text_units_map.get(&format!("system_param_term_{}", index)) {
            if !unit.translated_text.is_empty() {
                *term = unit.translated_text.clone();
            }
        }
    }
    
    // Update message terms
    for (key, message) in system.terms.messages.iter_mut() {
        if let Some(unit) = text_units_map.get(&format!("system_message_{}", key)) {
            if !unit.translated_text.is_empty() {
                *message = unit.translated_text.clone();
            }
        }
    }
    
    // Write back to file
    let updated_content = serde_json::to_string_pretty(&system)
        .map_err(|e| crate::core::error::AppError::Other(format!("Failed to serialize System.json: {}", e)))?;
    std::fs::write(&file_path, updated_content)
        .map_err(|e| crate::core::error::AppError::Other(format!("Failed to write System.json: {}", e)))?;
    
    log::info!("Successfully injected translations into System.json");
    Ok(())
}

