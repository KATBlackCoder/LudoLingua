use std::collections::HashMap;

use tokio::sync::Mutex;

use crate::core::error::{AppError, AppResult};

pub struct GlossaryState {
    pub(crate) pool: Mutex<Option<sqlx::SqlitePool>>,
    // naive in-memory cache keyed by a composed string
    pub(crate) cache: Mutex<HashMap<String, std::sync::Arc<Vec<crate::glossaries::model::GlossaryTerm>>>>,
}

impl GlossaryState {
    pub fn new() -> Self {
        Self {
            pool: Mutex::new(None),
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn ensure_pool(&self) -> AppResult<()> {
        if self.pool.lock().await.is_none() {
            use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
            use sqlx::sqlite::SqlitePoolOptions;

            let opts = SqliteConnectOptions::new()
                .filename("ludolingua.db")
                .create_if_missing(true)
                .journal_mode(SqliteJournalMode::Wal)
                .synchronous(SqliteSynchronous::Normal);

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect_with(opts)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            // Busy timeout for writer contention
            sqlx::query("PRAGMA busy_timeout = 5000;")
                .execute(&pool)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            // Schema (idempotent)
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS glossary_terms (
                  id INTEGER PRIMARY KEY,
                  category TEXT NOT NULL,
                  source_lang TEXT NOT NULL,
                  target_lang TEXT NOT NULL,
                  input TEXT NOT NULL,
                  output TEXT NOT NULL,
                  enabled INTEGER NOT NULL DEFAULT 1,
                  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                );
                CREATE INDEX IF NOT EXISTS idx_gls_main
                  ON glossary_terms (enabled, source_lang, target_lang, category);
                "#,
            )
            .execute(&pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            // Backfill only `enabled` if missing
            let colnames: Vec<String> = sqlx::query_scalar(
                "SELECT name FROM pragma_table_info('glossary_terms')",
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
            if !colnames.into_iter().any(|n| n == "enabled") {
                sqlx::query("ALTER TABLE glossary_terms ADD COLUMN enabled INTEGER NOT NULL DEFAULT 1;")
                    .execute(&pool)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?;
            }

            *self.pool.lock().await = Some(pool);
        }
        Ok(())
    }

    pub async fn pool(&self) -> sqlx::SqlitePool {
        self.ensure_pool().await.expect("glossary db init");
        self.pool.lock().await.as_ref().unwrap().clone()
    }
}


