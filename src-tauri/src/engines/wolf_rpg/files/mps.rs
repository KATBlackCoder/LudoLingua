use crate::models::translation::{PromptType, TextUnit, TranslationStatus};
use crate::engines::wolf_rpg::files::regex::{
    is_translatable_wolf_text, wolf_replace_placeholders_for_translation
};
use serde_json::Value;
use std::collections::HashMap;

/// Extract text units from Wolf RPG MPS (Map Script) files
/// Based on actual Wolf RPG JSON structure with events, pages, and command lists
pub fn extract_text_units_from_mps(
    mps_data: &Value,
    file_path: &str,
) -> Vec<TextUnit> {
    let mut text_units = Vec::new();
    
    if let Some(obj) = mps_data.as_object() {
        // Wolf RPG MPS files have an "events" array
        if let Some(events) = obj.get("events").and_then(|v| v.as_array()) {
            for (event_idx, event) in events.iter().enumerate() {
                extract_from_wolf_event(&mut text_units, event, file_path, event_idx);
            }
        }
    }
    
    text_units
}

/// Extract text from a single Wolf RPG event
/// Only extracts from command codes, not event names or descriptions
fn extract_from_wolf_event(
    text_units: &mut Vec<TextUnit>,
    event: &Value,
    file_path: &str,
    event_idx: usize,
) {
    if let Some(event_obj) = event.as_object() {
        // Skip event names - user says not to translate them
        
        // Extract from event pages (where the actual commands are)
        if let Some(pages) = event_obj.get("pages").and_then(|v| v.as_array()) {
            for (page_idx, page) in pages.iter().enumerate() {
                extract_from_wolf_page(text_units, page, file_path, event_idx, page_idx);
            }
        }
    }
}

/// Extract text from a Wolf RPG event page
/// Pages contain a "list" array of commands with codes, like RPG Maker event commands
fn extract_from_wolf_page(
    text_units: &mut Vec<TextUnit>,
    page: &Value,
    file_path: &str,
    event_idx: usize,
    page_idx: usize,
) {
    if let Some(page_obj) = page.as_object() {
        // Extract from command list (similar to RPG Maker event commands)
        if let Some(commands) = page_obj.get("list").and_then(|v| v.as_array()) {
            for (cmd_idx, command) in commands.iter().enumerate() {
                extract_from_wolf_command(text_units, command, file_path, event_idx, page_idx, cmd_idx);
            }
        }
    }
}

/// Extract text from Wolf RPG commands based on specific command codes
/// Only processes known translatable command codes: 101, 210, 150, 122
fn extract_from_wolf_command(
    text_units: &mut Vec<TextUnit>,
    command: &Value,
    file_path: &str,
    event_idx: usize,
    page_idx: usize,
    cmd_idx: usize,
) {
    if let Some(cmd_obj) = command.as_object() {
        // Get command code (like RPG Maker's command codes)
        let code = cmd_obj.get("code").and_then(|v| v.as_i64()).unwrap_or(0);
        
        // Extract based on specific translatable command codes only
        match code {
            101 => {
                // Message - extract all text from stringArgs
                extract_command_strings(text_units, cmd_obj, file_path, event_idx, page_idx, cmd_idx, code, PromptType::Dialogue, false);
            }
            210 => {
                // CommonEvent - extract text but skip files (.mp3, .png, etc)
                extract_command_strings(text_units, cmd_obj, file_path, event_idx, page_idx, cmd_idx, code, PromptType::Dialogue, true);
            }
            150 => {
                // Picture - extract text but skip files, handle Wolf codes like \\E\\c[2], \n
                extract_command_strings(text_units, cmd_obj, file_path, event_idx, page_idx, cmd_idx, code, PromptType::Dialogue, true);
            }
            122 => {
                // SetString - extract text only if not empty
                extract_command_strings(text_units, cmd_obj, file_path, event_idx, page_idx, cmd_idx, code, PromptType::Other, true);
            }
            _ => {
                // Skip all other command codes - they don't contain translatable text
            }
        }
    }
}

/// Helper function to extract strings from command stringArgs
fn extract_command_strings(
    text_units: &mut Vec<TextUnit>,
    cmd_obj: &serde_json::Map<String, Value>,
    file_path: &str,
    event_idx: usize,
    page_idx: usize,
    cmd_idx: usize,
    code: i64,
    prompt_type: PromptType,
    skip_files: bool,
) {
    if let Some(string_args) = cmd_obj.get("stringArgs").and_then(|v| v.as_array()) {
        for (arg_idx, arg) in string_args.iter().enumerate() {
            if let Some(arg_text) = arg.as_str() {
                // Skip empty strings for SetString (122) and others
                if arg_text.trim().is_empty() {
                    continue;
                }
                
                // Use enhanced filtering that skips files if requested
                if skip_files {
                    if !is_translatable_wolf_text(arg_text) {
                        continue;
                    }
                } else {
                    // For Message (101), we're less strict - just check it's not obviously technical
                    if !is_text_worth_translating(arg_text) {
                        continue;
                    }
                }
                
                let processed_text = wolf_replace_placeholders_for_translation(arg_text);
                let normalized_path = file_path.replace('\\', "/");
                text_units.push(TextUnit {
                    id: format!(
                        "wolf_json:{}#events[{}].pages[{}].list[{}].stringArgs[{}]",
                        normalized_path, event_idx, page_idx, cmd_idx, arg_idx
                    ),
                    source_text: processed_text,
                    translated_text: String::new(),
                    field_type: format!(
                        "command_{}:{}:events[{}].pages[{}].list[{}]",
                        code, file_path, event_idx, page_idx, cmd_idx
                    ),
                    status: TranslationStatus::NotTranslated,
                    prompt_type,
                });
            }
        }
    }
}

/// Basic check for text worth translating (less strict than is_translatable_wolf_text)
fn is_text_worth_translating(text: &str) -> bool {
    let text = text.trim();
    // Skip empty or very short strings
    text.len() >= 2 && 
    // Must contain some alphabetic characters (any language)
    text.chars().any(|c| c.is_alphabetic())
}





/// Inject translated text back into Wolf RPG MPS structures
pub fn inject_text_units_into_mps(
    mps_data: &mut Value,
    text_units: &HashMap<String, &TextUnit>,
    file_path: &str,
) {
    if let Some(obj) = mps_data.as_object_mut() {
        // Wolf RPG MPS files have an "events" array
        if let Some(events) = obj.get_mut("events").and_then(|v| v.as_array_mut()) {
            for (event_idx, event) in events.iter_mut().enumerate() {
                inject_into_wolf_event(event, text_units, file_path, event_idx);
            }
        }
    }
}

/// Inject translations into a single Wolf RPG event
fn inject_into_wolf_event(
    event: &mut Value,
    text_units: &HashMap<String, &TextUnit>,
    file_path: &str,
    event_idx: usize,
) {
    if let Some(event_obj) = event.as_object_mut() {
        // Skip event name injection - user says not to translate names
        
        // Inject into event pages (where the actual commands are)
        if let Some(pages) = event_obj.get_mut("pages").and_then(|v| v.as_array_mut()) {
            for (page_idx, page) in pages.iter_mut().enumerate() {
                inject_into_wolf_page(page, text_units, file_path, event_idx, page_idx);
            }
        }
    }
}

/// Inject translations into a Wolf RPG event page
fn inject_into_wolf_page(
    page: &mut Value,
    text_units: &HashMap<String, &TextUnit>,
    file_path: &str,
    event_idx: usize,
    page_idx: usize,
) {
    if let Some(page_obj) = page.as_object_mut() {
        // Inject into command list
        if let Some(commands) = page_obj.get_mut("list").and_then(|v| v.as_array_mut()) {
            for (cmd_idx, command) in commands.iter_mut().enumerate() {
                inject_into_wolf_command(command, text_units, file_path, event_idx, page_idx, cmd_idx);
            }
        }
    }
}

/// Inject translations into Wolf RPG commands
fn inject_into_wolf_command(
    command: &mut Value,
    text_units: &HashMap<String, &TextUnit>,
    file_path: &str,
    event_idx: usize,
    page_idx: usize,
    cmd_idx: usize,
) {
    if let Some(cmd_obj) = command.as_object_mut() {
        // Inject into stringArgs if present
        if let Some(string_args) = cmd_obj.get_mut("stringArgs").and_then(|v| v.as_array_mut()) {
            for (arg_idx, arg) in string_args.iter_mut().enumerate() {
                let normalized_path = file_path.replace('\\', "/");
                let unit_id = format!(
                    "wolf_json:{}#events[{}].pages[{}].list[{}].stringArgs[{}]",
                    normalized_path, event_idx, page_idx, cmd_idx, arg_idx
                );
                if let Some(text_unit) = text_units.get(&unit_id) {
                    if !text_unit.translated_text.is_empty() {
                        let restored_text = crate::engines::wolf_rpg::files::regex::wolf_restore_placeholders_after_translation(&text_unit.translated_text);
                        *arg = Value::String(restored_text);
                    }
                }
            }
        }
    }
}
