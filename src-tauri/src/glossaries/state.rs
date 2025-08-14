use std::collections::HashMap;
use std::path::PathBuf;

use tokio::sync::Mutex;

use crate::core::error::{AppError, AppResult};

pub struct GlossaryState {
    /// Absolute path to the SQLite database file
    db_path: PathBuf,
    pub(crate) pool: Mutex<Option<sqlx::SqlitePool>>,
    // naive in-memory cache keyed by a composed string
    pub(crate) cache:
        Mutex<HashMap<String, std::sync::Arc<Vec<crate::glossaries::model::GlossaryTerm>>>>,
}

impl GlossaryState {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            pool: Mutex::new(None),
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn ensure_pool(&self) -> AppResult<()> {
        if self.pool.lock().await.is_none() {
            use sqlx::sqlite::SqlitePoolOptions;
            use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};

            // Ensure parent directory exists (first run on a new machine)
            if let Some(parent) = self.db_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }

            let opts = SqliteConnectOptions::new()
                .filename(self.db_path.clone())
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

            // Run SQL migrations instead of ad-hoc CREATE/ALTER
            // NOTE: path is relative to this crate (src-tauri). Keep migrations in src-tauri/migrations
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            *self.pool.lock().await = Some(pool);
        }
        Ok(())
    }

    pub async fn pool(&self) -> sqlx::SqlitePool {
        self.ensure_pool().await.expect("glossary db init");
        self.pool.lock().await.as_ref().unwrap().clone()
    }
}
