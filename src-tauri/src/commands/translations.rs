use crate::core::error::AppResult;
use crate::db::state::ManagedTranslationState;
use crate::db::translation::model::{TextUnitQuery, TextUnitRecord};

/// List translations with optional filtering and pagination
pub async fn list_translations(
    state: &ManagedTranslationState,
    query: TextUnitQuery,
) -> AppResult<Vec<TextUnitRecord>> {
    crate::db::translation::repo::find_units(state, &query).await
}

/// Get a single translation by ID
pub async fn get_translation(
    state: &ManagedTranslationState,
    id: i64,
) -> AppResult<TextUnitRecord> {
    crate::db::translation::repo::find_unit_by_id(state, id).await
}

/// Update an existing translation
pub async fn update_translation(
    state: &ManagedTranslationState,
    id: i64,
    translated_text: String,
    status: Option<String>,
) -> AppResult<bool> {
    // First get the existing record
    let mut record = crate::db::translation::repo::find_unit_by_id(state, id).await?;

    // Update the fields
    record.translated_text = Some(translated_text);
    if let Some(new_status) = status {
        record.status = new_status;
    }

    // Upsert back to database
    let _updated_id = crate::db::translation::repo::upsert_unit(state, &record).await?;
    Ok(true)
}

/// Delete a single translation by ID
pub async fn delete_translation(state: &ManagedTranslationState, id: i64) -> AppResult<bool> {
    crate::db::translation::repo::delete_unit(state, id).await?;
    Ok(true)
}

/// Bulk delete multiple translations
pub async fn bulk_delete_translations(
    state: &ManagedTranslationState,
    ids: Vec<i64>,
) -> AppResult<i64> {
    crate::db::translation::repo::bulk_delete_units(state, ids).await
}

/// Get translation statistics
pub async fn get_translation_stats(
    state: &ManagedTranslationState,
    project_path: Option<String>,
) -> AppResult<serde_json::Value> {
    if let Some(path) = project_path {
        let stats = crate::db::translation::repo::get_project_stats(state, &path).await?;
        serde_json::to_value(stats).map_err(|e| crate::core::error::AppError::Other(e.to_string()))
    } else {
        let stats = crate::db::translation::repo::get_overall_stats(state).await?;
        serde_json::to_value(stats).map_err(|e| crate::core::error::AppError::Other(e.to_string()))
    }
}
