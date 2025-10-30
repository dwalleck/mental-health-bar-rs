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

/// Trait for converting feature-specific errors to CommandError
/// This provides a consistent pattern across all features
pub trait ToCommandError {
    fn to_command_error(&self) -> CommandError;
}

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

    /// Optional additional context as JSON (for server-side debugging only)
    ///
    /// IMPORTANT: This field is intentionally hidden from TypeScript (#[specta(skip)])
    /// to avoid exposing internal implementation details (field names, values, etc.)
    /// to the frontend. Use server-side logging to access this information.
    ///
    /// The `message` field already contains user-friendly error text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[specta(skip)]
    pub details: Option<serde_json::Value>,
}

/// Error type constants for consistency across the application
pub mod error_types {
    pub const VALIDATION: &str = "validation";
    pub const NOT_FOUND: &str = "not_found";
    pub const DATABASE_ERROR: &str = "database_error";
    pub const DATABASE_LOCKED: &str = "database_locked";
    pub const LOCK_POISONED: &str = "lock_poisoned";
    pub const CONSTRAINT_VIOLATION: &str = "constraint_violation";
    pub const DUPLICATE: &str = "duplicate";
    pub const TRANSACTION_FAILURE: &str = "transaction_failure";
    pub const NO_DATA: &str = "no_data";
    pub const CALCULATION_ERROR: &str = "calculation_error";
    #[allow(dead_code)]
    pub const TRANSIENT: &str = "transient";
    pub const INTERNAL: &str = "internal";
    #[allow(dead_code)]
    pub const CONFIG: &str = "config";
    #[allow(dead_code)]
    pub const IO_ERROR: &str = "io_error";
    #[allow(dead_code)]
    pub const SERIALIZATION: &str = "serialization";
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

    /// Helper to convert rusqlite errors to CommandError with proper classification
    pub fn from_rusqlite_error(err: &rusqlite::Error) -> Self {
        use rusqlite::ErrorCode;

        match err {
            rusqlite::Error::SqliteFailure(err, _) => match err.code {
                ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked => {
                    Self::retryable("Database is temporarily busy", error_types::DATABASE_LOCKED)
                }
                _ => Self::permanent(err.to_string(), error_types::DATABASE_ERROR),
            },
            _ => Self::permanent(err.to_string(), error_types::DATABASE_ERROR),
        }
    }
}
