use serde::{Deserialize, Serialize};
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

/// Structured error response for Tauri commands that provides:
/// - Human-readable error message
/// - Machine-readable error type for conditional logic
/// - Retry flag to guide client-side retry behavior
///
/// This enables the frontend to:
/// - Make type-safe decisions about error handling
/// - Implement smart retry logic without string parsing
/// - Display appropriate error messages to users
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CommandError {
    /// Human-readable error message
    pub message: String,

    /// Machine-readable error type for conditional logic
    /// Examples: "validation", "not_found", "database_locked", "transient"
    pub error_type: String,

    /// Whether this error is retryable (e.g., database locks, transient network issues)
    /// - true: Client should retry the operation (e.g., SQLITE_BUSY, lock timeout)
    /// - false: Client should not retry (e.g., validation error, not found)
    pub retryable: bool,

    /// Optional additional context as JSON (for debugging or detailed error info)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[specta(skip)]
    pub details: Option<serde_json::Value>,
}

impl CommandError {
    /// Create a new retryable error (e.g., database locked, transient issues)
    pub fn retryable(message: impl Into<String>, error_type: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            error_type: error_type.into(),
            retryable: true,
            details: None,
        }
    }

    /// Create a new non-retryable error (e.g., validation, not found)
    pub fn permanent(message: impl Into<String>, error_type: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            error_type: error_type.into(),
            retryable: false,
            details: None,
        }
    }

    /// Add additional context to the error
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}
