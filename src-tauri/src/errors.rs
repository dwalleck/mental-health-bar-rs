use thiserror::Error;

/// Application-level errors using thiserror for structured error handling
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias for application errors
pub type AppResult<T> = Result<T, AppError>;

/// Convert AppError to String for Tauri commands
impl From<AppError> for String {
    fn from(error: AppError) -> String {
        error.to_string()
    }
}
