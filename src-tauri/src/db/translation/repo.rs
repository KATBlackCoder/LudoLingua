use sqlx::{self, Row, Arguments};
use crate::core::error::{AppError, AppResult};
use crate::db::state::ManagedTranslationState;
use super::model::{TextUnitRecord, TextUnitQuery, BulkOperationResult};

/// Find text units matching the query criteria
pub async fn find_units(
    state: &ManagedTranslationState,
    query: &TextUnitQuery,
) -> AppResult<Vec<TextUnitRecord>> {
    let pool = state.pool().await;

    let mut sql = String::from(
        r#"SELECT id, project_path, file_path, field_type, source_text, translated_text,
                  status, prompt_type, source_lang, target_lang, manifest_hash,
                  created_at, updated_at
           FROM text_units WHERE 1=1"#
    );
    let mut args = sqlx::sqlite::SqliteArguments::default();

    if let Some(project_path) = &query.project_path {
        sql.push_str(" AND project_path = ?");
        args.add(project_path).map_err(|e| AppError::Database(e.to_string()))?;
    }

    if let Some(file_path) = &query.file_path {
        sql.push_str(" AND file_path = ?");
        args.add(file_path).map_err(|e| AppError::Database(e.to_string()))?;
    }

    if let Some(status) = &query.status {
        sql.push_str(" AND status = ?");
        args.add(status).map_err(|e| AppError::Database(e.to_string()))?;
    }

    if let Some(manifest_hash) = &query.manifest_hash {
        sql.push_str(" AND manifest_hash = ?");
        args.add(manifest_hash).map_err(|e| AppError::Database(e.to_string()))?;
    }

    sql.push_str(" ORDER BY updated_at DESC");

    if let Some(limit) = query.limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    if let Some(offset) = query.offset {
        sql.push_str(&format!(" OFFSET {}", offset));
    }

    let rows = sqlx::query_with(&sql, args)
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let units = rows
        .into_iter()
        .map(|row| TextUnitRecord {
            id: Some(row.get::<i64, _>("id")),
            project_path: row.get::<String, _>("project_path"),
            file_path: row.get::<String, _>("file_path"),
            field_type: row.get::<String, _>("field_type"),
            source_text: row.get::<String, _>("source_text"),
            translated_text: row.get::<Option<String>, _>("translated_text"),
            status: row.get::<String, _>("status"),
            prompt_type: row.get::<String, _>("prompt_type"),
            source_lang: row.get::<String, _>("source_lang"),
            target_lang: row.get::<String, _>("target_lang"),
            manifest_hash: row.get::<Option<String>, _>("manifest_hash"),
            created_at: row.get::<Option<String>, _>("created_at"),
            updated_at: row.get::<Option<String>, _>("updated_at"),
        })
        .collect();

    Ok(units)
}

/// Save a single text unit (insert or update)
pub async fn upsert_unit(
    state: &ManagedTranslationState,
    unit: &TextUnitRecord,
) -> AppResult<i64> {
    let pool = state.pool().await;

    if let Some(id) = unit.id {
        // Update existing record
        sqlx::query(
            r#"UPDATE text_units
               SET translated_text = ?, status = ?, prompt_type = ?, updated_at = CURRENT_TIMESTAMP
               WHERE id = ?"#
        )
        .bind(&unit.translated_text)
        .bind(&unit.status)
        .bind(&unit.prompt_type)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(id)
    } else {
        // Insert new record
        let result = sqlx::query(
            r#"INSERT INTO text_units
               (project_path, file_path, field_type, source_text, translated_text,
                status, prompt_type, source_lang, target_lang, manifest_hash)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&unit.project_path)
        .bind(&unit.file_path)
        .bind(&unit.field_type)
        .bind(&unit.source_text)
        .bind(&unit.translated_text)
        .bind(&unit.status)
        .bind(&unit.prompt_type)
        .bind(&unit.source_lang)
        .bind(&unit.target_lang)
        .bind(&unit.manifest_hash)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.last_insert_rowid())
    }
}

/// Bulk insert or update multiple text units
pub async fn bulk_upsert_units(
    state: &ManagedTranslationState,
    units: &[TextUnitRecord],
) -> AppResult<BulkOperationResult> {
    let pool = state.pool().await;
    let mut inserted = 0i64;
    let mut updated = 0i64;
    let mut errors = Vec::new();

    // Use a transaction for atomicity
    let mut tx = pool.begin().await
        .map_err(|e| AppError::Database(e.to_string()))?;

    for unit in units {
        match upsert_unit_in_transaction(&mut tx, unit).await {
            Ok(_id) => {
                if unit.id.is_some() {
                    updated += 1;
                } else {
                    inserted += 1;
                }
            }
            Err(e) => {
                errors.push(format!("Failed to save unit {}: {}", unit.source_text, e));
            }
        }
    }

    if errors.is_empty() {
        tx.commit().await
            .map_err(|e| AppError::Database(e.to_string()))?;
    } else {
        tx.rollback().await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }

    Ok(BulkOperationResult {
        inserted,
        updated,
        errors,
    })
}

/// Helper function for bulk operations within a transaction
async fn upsert_unit_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    unit: &TextUnitRecord,
) -> AppResult<i64> {
    if let Some(id) = unit.id {
        // Update existing record
        sqlx::query(
            r#"UPDATE text_units
               SET translated_text = ?, status = ?, prompt_type = ?, updated_at = CURRENT_TIMESTAMP
               WHERE id = ?"#
        )
        .bind(&unit.translated_text)
        .bind(&unit.status)
        .bind(&unit.prompt_type)
        .bind(id)
        .execute(&mut **tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(id)
    } else {
        // Insert new record
        let result = sqlx::query(
            r#"INSERT INTO text_units
               (project_path, file_path, field_type, source_text, translated_text,
                status, prompt_type, source_lang, target_lang, manifest_hash)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&unit.project_path)
        .bind(&unit.file_path)
        .bind(&unit.field_type)
        .bind(&unit.source_text)
        .bind(&unit.translated_text)
        .bind(&unit.status)
        .bind(&unit.prompt_type)
        .bind(&unit.source_lang)
        .bind(&unit.target_lang)
        .bind(&unit.manifest_hash)
        .execute(&mut **tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.last_insert_rowid())
    }
}

/// Delete a text unit by ID
pub async fn delete_unit(
    state: &ManagedTranslationState,
    id: i64,
) -> AppResult<()> {
    let pool = state.pool().await;

    sqlx::query("DELETE FROM text_units WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

/// Find units by project and status (useful for resuming work)
pub async fn find_units_by_project_and_status(
    state: &ManagedTranslationState,
    project_path: &str,
    status: Option<&str>,
) -> AppResult<Vec<TextUnitRecord>> {
    let query = TextUnitQuery {
        project_path: Some(project_path.to_string()),
        status: status.map(|s| s.to_string()),
        ..Default::default()
    };

    find_units(state, &query).await
}

/// Mark a unit as being translated (for concurrency control)
pub async fn mark_unit_as_translating(
    state: &ManagedTranslationState,
    id: i64,
) -> AppResult<()> {
    let pool = state.pool().await;

    sqlx::query(
        r#"UPDATE text_units
           SET status = 'Translating', updated_at = CURRENT_TIMESTAMP
           WHERE id = ? AND status != 'Translating'"#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

/// Get statistics for a project
pub async fn get_project_stats(
    state: &ManagedTranslationState,
    project_path: &str,
) -> AppResult<serde_json::Value> {
    let pool = state.pool().await;

    let stats = sqlx::query(
        r#"SELECT
            COUNT(*) as total_units,
            COUNT(CASE WHEN status = 'MachineTranslated' THEN 1 END) as machine_translated,
            COUNT(CASE WHEN status = 'HumanReviewed' THEN 1 END) as human_reviewed,
            COUNT(CASE WHEN translated_text IS NOT NULL AND translated_text != '' THEN 1 END) as has_translation
           FROM text_units WHERE project_path = ?"#
    )
    .bind(project_path)
    .fetch_one(&pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(serde_json::json!({
        "total_units": stats.get::<i64, _>("total_units"),
        "machine_translated": stats.get::<i64, _>("machine_translated"),
        "human_reviewed": stats.get::<i64, _>("human_reviewed"),
        "has_translation": stats.get::<i64, _>("has_translation")
    }))
}
