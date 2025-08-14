use crate::models::language::Language;

/// Return enabled languages from the embedded JSON catalog.
pub fn get_languages() -> Result<Vec<Language>, String> {
    let data = include_str!("../../models/language.json");
    let entries: Vec<Language> = match serde_json::from_str(data) {
        Ok(v) => v,
        Err(e) => return Err(format!("Invalid language.json: {}", e)),
    };

    let enabled: Vec<Language> = entries.into_iter().filter(|e| e.enabled).collect();

    Ok(enabled)
}
