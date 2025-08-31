use log::{debug, info, warn};
use std::path::Path;

use crate::core::error::{AppError, AppResult};
use crate::models::engine::EngineType;
use crate::models::engine::{EngineInfo, GameDataFile};
use crate::models::translation::TextUnit;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

/// Common helper function to extract text from a specific file type.
///
/// This function provides a reusable way to extract text from any file type,
/// eliminating repetitive code patterns across different engine implementations.
///
/// # Arguments
///
/// * `project_info` - Information about the project
/// * `file_paths` - Array of possible file paths to check
/// * `extract_function` - Function to extract text from the file
/// * `file_type_name` - Name of the file type for logging
///
/// # Returns
///
/// * `AppResult<Vec<GameDataFile>>` - Vector of extracted game data files
pub fn extract_file_type_text<F>(
    project_info: &EngineInfo,
    file_paths: &[&str],
    extract_function: F,
    file_type_name: &str,
) -> AppResult<Vec<GameDataFile>>
where
    F: Fn(&Path, &str) -> AppResult<GameDataFile>,
{
    let mut game_data_files = Vec::new();
    let mut file_found = false;

    for path in file_paths.iter() {
        let file_path = project_info.path.join(path);
        if file_path.exists() {
            debug!("Found {} at: {}", file_type_name, file_path.display());

            match extract_function(&project_info.path, path) {
                Ok(game_data_file) => {
                    info!(
                        "Extracted {} text units from {}",
                        game_data_file.text_unit_count, path
                    );

                    game_data_files.push(game_data_file);
                    file_found = true;
                    break; // Found and processed, no need to check other paths
                }
                Err(e) => {
                    warn!("Failed to extract text from {}: {}", path, e);
                    // Continue trying other paths
                }
            }
        }
    }

    if !file_found {
        warn!("Could not find {} in any expected location", file_type_name);
    }

    Ok(game_data_files)
}

/// Common helper function to inject translations into a specific file type.
///
/// This function provides a reusable way to inject translations into any file type,
/// eliminating repetitive code patterns across different engine implementations.
///
/// # Arguments
///
/// * `project_info` - Information about the project
/// * `text_units` - Vector of translated text units to inject
/// * `file_type_prefix` - The prefix to filter text units (e.g., "actor_", "item_")
/// * `file_paths` - Array of possible file paths to check
/// * `inject_function` - Function to call for injection
/// * `file_type_name` - Human-readable name for logging (e.g., "actor", "item")
///
/// # Returns
///
/// * `AppResult<()>` - Success or error
pub fn inject_file_type_translations<F>(
    project_info: &EngineInfo,
    text_units: &[TextUnit],
    file_type_prefix: &str,
    file_paths: &[&str],
    inject_function: F,
    file_type_name: &str,
) -> AppResult<()>
where
    F: Fn(&Path, &str, &[&TextUnit]) -> AppResult<()>,
{
    // Filter text units by file type to inject only relevant ones
    let filtered_units: Vec<&TextUnit> = text_units
        .iter()
        .filter(|unit| unit.id.starts_with(file_type_prefix))
        .collect();

    if filtered_units.is_empty() {
        return Ok(()); // No units to inject for this file type
    }

    let mut injected = false;

    for path in file_paths.iter() {
        let file_path = project_info.path.join(path);
        if file_path.exists() {
            debug!(
                "Injecting into {} at: {}",
                file_type_name,
                file_path.display()
            );

            // Inject translations using the provided function
            match inject_function(&project_info.path, path, &filtered_units) {
                Ok(()) => {
                    info!(
                        "Successfully injected {} {} translations into {}",
                        filtered_units.len(),
                        file_type_name,
                        path
                    );
                    injected = true;
                    break; // We successfully injected, no need to check other paths
                }
                Err(e) => {
                    warn!("Failed to inject translations into {}: {}", path, e);
                    // Continue trying other paths
                }
            }
        }
    }

    if !injected && !filtered_units.is_empty() {
        warn!(
            "Could not inject {} translations - {} not found in any expected location",
            file_type_name, file_type_name
        );
    }

    Ok(())
}

/// Recursively collect string values from a JSON document into `TextUnit`s.
///
/// The generated id format is: `wolf_json:{rel_path}#{json_pointer}` where `rel_path`
/// is a forward-slash relative path including a `dump/` prefix (e.g. `dump/Maps/Map001_1.json`)
/// and `json_pointer` follows RFC6901 (e.g. `/events/0/name`).
pub fn collect_text_units_from_json(
    out: &mut Vec<TextUnit>,
    value: &Value,
    _engine_type: &EngineType,
    rel_path_within_workdir: String,
) {
    // Avoid unused warning for now; heuristics can use engine_type later
    let _ = _engine_type;

    fn esc(segment: &str) -> String {
        segment.replace('~', "~0").replace('/', "~1")
    }

    fn walk(out: &mut Vec<TextUnit>, v: &Value, ptr: &str, id_prefix: &str, is_wolf: bool) {
        match v {
            Value::String(s) => {
                // Apply Wolf-specific filtering and placeholder encoding when enabled
                #[allow(unused_mut)]
                let mut should_keep = true;
                if is_wolf {
                    #[cfg(target_os = "windows")]
                    {
                        should_keep =
                            crate::engines::wolf_rpg::files::regex::is_translatable_wolf_text(s);
                    }
                }
                if should_keep {
                    let id = format!("wolf_json:{}#{}", id_prefix, ptr);
                    #[allow(unused_mut)]
                    let mut source = s.clone();
                    if is_wolf {
                        #[cfg(target_os = "windows")]
                        {
                            source = crate::engines::wolf_rpg::files::regex::wolf_replace_placeholders_for_translation(&source);
                        }
                    }
                    out.push(TextUnit {
                        id,
                        source_text: source,
                        translated_text: String::new(),
                        field_type: "text".to_string(),
                        status: crate::models::translation::TranslationStatus::NotTranslated,
                        prompt_type: crate::models::translation::PromptType::Other,
                    });
                }
            }
            Value::Array(arr) => {
                for (i, item) in arr.iter().enumerate() {
                    let next = format!("{}/{}", ptr, i);
                    walk(out, item, &next, id_prefix, is_wolf);
                }
            }
            Value::Object(map) => {
                for (k, item) in map.iter() {
                    let next = format!("{}/{}", ptr, esc(k));
                    walk(out, item, &next, id_prefix, is_wolf);
                }
            }
            _ => {}
        }
    }

    // Normalize path separators to forward slashes for stable ids on Windows
    let mut rel = rel_path_within_workdir.replace('\\', "/");
    // ensure it starts with "dump/" for consistency
    if !rel.starts_with("dump/") {
        rel = format!("dump/{}", rel.trim_start_matches("./"));
    }
    let is_wolf = matches!(_engine_type, EngineType::WolfRpg);
    walk(out, value, "", &rel, is_wolf);
}

/// Apply translated text units back into a JSON dump directory.
///
/// Expects ids generated by `collect_text_units_from_json`.
pub fn apply_text_units_to_json_dir(dump_dir: &Path, text_units: &[TextUnit]) -> AppResult<()> {
    // Build lookup: (rel_with_dump_prefix + '#' + pointer) -> translated_text
    let mut id_to_text: HashMap<String, &str> = HashMap::new();
    for u in text_units.iter() {
        if !u.translated_text.is_empty() && u.id.starts_with("wolf_json:") {
            let rest = &u.id[9..];
            id_to_text.insert(rest.to_string(), u.translated_text.as_str());
        }
    }

    fn update_value(v: &mut Value, ptr: &str, id_prefix: &str, map: &HashMap<String, &str>) {
        match v {
            Value::String(s) => {
                let key = format!("{}#{}", id_prefix, ptr);
                if let Some(new_text) = map.get(&key) {
                    *s = (*new_text).to_string();
                }
            }
            Value::Array(arr) => {
                for (i, item) in arr.iter_mut().enumerate() {
                    let next = format!("{}/{}", ptr, i);
                    update_value(item, &next, id_prefix, map);
                }
            }
            Value::Object(map_obj) => {
                for (k, item) in map_obj.iter_mut() {
                    // reverse-escape not needed for pointer composition here
                    let seg = k.replace('~', "~0").replace('/', "~1");
                    let next = format!("{}/{}", ptr, seg);
                    update_value(item, &next, id_prefix, map);
                }
            }
            _ => {}
        }
    }

    for entry in walkdir::WalkDir::new(dump_dir)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(path).map_err(|e| {
            AppError::FileSystem(format!("Failed to read {}: {}", path.display(), e))
        })?;
        let mut json: Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Build id_prefix = "dump/<rel>"
        let rel_from_dump = path.strip_prefix(dump_dir).unwrap_or(path);
        let mut id_prefix = format!("dump/{}", rel_from_dump.to_string_lossy());
        if cfg!(windows) {
            id_prefix = id_prefix.replace('\\', "/");
        }

        update_value(&mut json, "", &id_prefix, &id_to_text);

        let out = serde_json::to_string_pretty(&json).map_err(|e| {
            AppError::Other(format!("Failed to serialize {}: {}", path.display(), e))
        })?;
        fs::write(path, out).map_err(|e| {
            AppError::FileSystem(format!("Failed to write {}: {}", path.display(), e))
        })?;
    }
    Ok(())
}
