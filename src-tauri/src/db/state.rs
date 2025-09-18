use std::collections::HashMap;
use std::path::PathBuf;

use tokio::sync::Mutex;

use crate::core::error::{AppError, AppResult};

use crate::db::glossary::model::GlossaryTerm;
/// Shared database state for all database operations
/// This replaces the individual state structs and provides a unified database connection
pub struct DbState {
    /// Absolute path to the SQLite database file
    db_path: PathBuf,
    /// Shared database connection pool
    pub(crate) pool: Mutex<Option<sqlx::SqlitePool>>,
    /// Naive in-memory cache for glossary terms keyed by a composed string
    pub(crate) glossary_cache: Mutex<HashMap<String, std::sync::Arc<Vec<GlossaryTerm>>>>,
}

impl DbState {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            pool: Mutex::new(None),
            glossary_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Ensure the database pool is initialized and run migrations
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

    /// Get a reference to the database connection pool
    pub async fn pool(&self) -> sqlx::SqlitePool {
        self.ensure_pool().await.expect("database init");
        self.pool.lock().await.as_ref().unwrap().clone()
    }

    /// Get a reference to the glossary cache
    pub fn glossary_cache(&self) -> &Mutex<HashMap<String, std::sync::Arc<Vec<GlossaryTerm>>>> {
        &self.glossary_cache
    }
}

// Note: Old GlossaryState wrapper removed - use ManagedGlossaryState instead

/// Standalone GlossaryState for independent management
pub struct ManagedGlossaryState {
    db_path: PathBuf,
    pool: Mutex<Option<sqlx::SqlitePool>>,
    cache: Mutex<HashMap<String, std::sync::Arc<Vec<GlossaryTerm>>>>,
}

impl ManagedGlossaryState {
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

            // Run SQL migrations
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

    pub fn cache(&self) -> &Mutex<HashMap<String, std::sync::Arc<Vec<GlossaryTerm>>>> {
        &self.cache
    }
}

// Note: Old TranslationState wrapper removed - use ManagedTranslationState instead

/// Standalone TranslationState for independent management (for Tauri commands)
pub struct ManagedTranslationState {
    db_path: PathBuf,
    pool: Mutex<Option<sqlx::SqlitePool>>,
}

impl ManagedTranslationState {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            pool: Mutex::new(None),
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

            // Run SQL migrations
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            *self.pool.lock().await = Some(pool);
        }
        Ok(())
    }

    pub async fn pool(&self) -> sqlx::SqlitePool {
        self.ensure_pool().await.expect("translation db init");
        self.pool.lock().await.as_ref().unwrap().clone()
    }
}
