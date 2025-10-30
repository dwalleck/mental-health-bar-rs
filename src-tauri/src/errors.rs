use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error type enumeration for type-safe error classification
///
/// This enum is auto-generated to TypeScript via specta, eliminating duplication
/// between Rust and TypeScript error type constants.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    Validation,
    NotFound,
    DatabaseError,
    DatabaseLocked,
    LockPoisoned,
    ConstraintViolation,
    Duplicate,
    TransactionFailure,
    NoData,
    CalculationError,
    Transient,
    Internal,
    Config,
    IoError,
    Serialization,
}

impl ErrorType {
    /// Convert ErrorType to string (for backwards compatibility with existing code)
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorType::Validation => "validation",
            ErrorType::NotFound => "not_found",
            ErrorType::DatabaseError => "database_error",
            ErrorType::DatabaseLocked => "database_locked",
            ErrorType::LockPoisoned => "lock_poisoned",
            ErrorType::ConstraintViolation => "constraint_violation",
            ErrorType::Duplicate => "duplicate",
            ErrorType::TransactionFailure => "transaction_failure",
            ErrorType::NoData => "no_data",
            ErrorType::CalculationError => "calculation_error",
            ErrorType::Transient => "transient",
            ErrorType::Internal => "internal",
            ErrorType::Config => "config",
            ErrorType::IoError => "io_error",
            ErrorType::Serialization => "serialization",
        }
    }
}

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
    /// Auto-generated to TypeScript via specta for type safety
    pub error_type: ErrorType,

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

impl CommandError {
    /// Create a new retryable error (e.g., database locked, transient issues)
    pub fn retryable(message: impl Into<String>, error_type: ErrorType) -> Self {
        Self {
            message: message.into(),
            error_type,
            retryable: true,
            details: None,
        }
    }

    /// Create a new non-retryable error (e.g., validation, not found)
    pub fn permanent(message: impl Into<String>, error_type: ErrorType) -> Self {
        Self {
            message: message.into(),
            error_type,
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
                ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked => Self::retryable(
                    "Database is temporarily busy. This request will be retried automatically.",
                    ErrorType::DatabaseLocked,
                ),
                _ => Self::permanent(err.to_string(), ErrorType::DatabaseError),
            },
            _ => Self::permanent(err.to_string(), ErrorType::DatabaseError),
        }
    }
}
