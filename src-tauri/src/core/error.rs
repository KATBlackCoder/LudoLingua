use std::error::Error;
use std::fmt;

/// The main error type for the application.
/// This wraps other error types and provides a unified error handling approach.
#[derive(Debug)]
pub enum AppError {
    /// Errors related to file system operations
    FileSystem(String),
    /// Errors related to parsing game data
    Parsing(String),
    /// Errors related to translation operations
    Translation(String),
    /// Errors related to database operations
    Database(String),
    /// Errors related to the LLM operations
    Llm(String),
    /// Other errors that don't fit into the above categories
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::FileSystem(msg) => write!(f, "File system error: {}", msg),
            AppError::Parsing(msg) => write!(f, "Parsing error: {}", msg),
            AppError::Translation(msg) => write!(f, "Translation error: {}", msg),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Llm(msg) => write!(f, "LLM error: {}", msg),
            AppError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl Error for AppError {}

/// A specialized Result type for the application.
pub type AppResult<T> = Result<T, AppError>;
