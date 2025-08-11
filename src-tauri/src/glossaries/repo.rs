use sqlx::{self, Row, Arguments};

use crate::core::error::AppResult;
use crate::glossaries::{GlossaryQuery, GlossaryState};
use crate::glossaries::model::GlossaryTerm;

pub async fn find_terms(state: &GlossaryState, q: &GlossaryQuery) -> AppResult<Vec<GlossaryTerm>> {
    let pool = state.pool().await;

    // Build dynamic query
    let mut sql = String::from(
        "SELECT id, category, source_lang, target_lang, input, output, enabled \
         FROM glossary_terms WHERE 1=1",
    );
    let mut args = sqlx::sqlite::SqliteArguments::default();

    if q.only_enabled {
        sql.push_str(" AND enabled = 1");
    }
    // Allow "all languages" by skipping filters when source/target are empty or special marker
    let is_all = |s: &str| s.is_empty() || s == "*" || s == "__ALL__";
    if !is_all(&q.source_lang) {
        sql.push_str(" AND source_lang = ?");
        let _ = args.add(q.source_lang.as_str());
    }
    if !is_all(&q.target_lang) {
        sql.push_str(" AND target_lang = ?");
        let _ = args.add(q.target_lang.as_str());
    }

    if !q.categories.is_empty() {
        sql.push_str(" AND category IN (");
        sql.push_str(&vec!["?"; q.categories.len()].join(","));
        sql.push_str(")");
        for c in &q.categories {
            let _ = args.add(c.as_str());
        }
    }

    // project_scope removed

    sql.push_str(" ORDER BY enabled DESC, id ASC");
    if let Some(limit) = q.limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    let rows = sqlx::query_with(&sql, args).fetch_all(&pool).await
        .map_err(|e| crate::core::error::AppError::Database(e.to_string()))?;
    let terms = rows
        .into_iter()
        .map(|r| GlossaryTerm {
            id: r.get::<i64, _>("id"),
            category: r.get::<String, _>("category"),
            source_lang: r.get::<String, _>("source_lang"),
            target_lang: r.get::<String, _>("target_lang"),
            input: r.get::<String, _>("input"),
            output: r.get::<String, _>("output"),
            enabled: r.get::<i64, _>("enabled") != 0,
        })
        .collect();
    Ok(terms)
}

pub async fn upsert_term(state: &GlossaryState, term: GlossaryTerm) -> AppResult<i64> {
    let pool = state.pool().await;
    let id = if term.id > 0 {
        sqlx::query(
            r#"UPDATE glossary_terms
               SET category = ?, source_lang = ?, target_lang = ?, input = ?, output = ?, enabled = ?
               WHERE id = ?"#,
        )
        .bind(&term.category)
        .bind(&term.source_lang)
        .bind(&term.target_lang)
        .bind(&term.input)
        .bind(&term.output)
        .bind(term.enabled as i64)
        .bind(term.id)
        .execute(&pool)
        .await
        .map_err(|e| crate::core::error::AppError::Database(e.to_string()))?;
        term.id
    } else {
        let res = sqlx::query(
            r#"INSERT INTO glossary_terms (category, source_lang, target_lang, input, output, enabled)
               VALUES (?, ?, ?, ?, ?, ?)
               ON CONFLICT(source_lang, target_lang, category, input)
               DO UPDATE SET
                 output = excluded.output,
                 enabled = excluded.enabled
            "#,
        )
        .bind(&term.category)
        .bind(&term.source_lang)
        .bind(&term.target_lang)
        .bind(&term.input)
        .bind(&term.output)
        .bind(term.enabled as i64)
        .execute(&pool)
        .await
        .map_err(|e| crate::core::error::AppError::Database(e.to_string()))?;
        res.last_insert_rowid()
    };
    Ok(id)
}

pub async fn delete_term(state: &GlossaryState, id: i64) -> AppResult<()> {
    let pool = state.pool().await;
    sqlx::query("DELETE FROM glossary_terms WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| crate::core::error::AppError::Database(e.to_string()))?;
    Ok(())
}


