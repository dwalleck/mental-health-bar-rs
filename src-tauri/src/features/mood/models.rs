use crate::{CommandError, MAX_NOTES_LENGTH};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

/// Mood feature errors
#[derive(Error, Debug)]
pub enum MoodError {
    #[error("Invalid mood rating: {0}. Must be 1-5")]
    InvalidRating(i32),

    #[error("Activity not found: {0}")]
    ActivityNotFound(i32),

    #[error("Activity name cannot be empty")]
    EmptyActivityName,

    #[error("Activity name too long: {0} characters. Maximum 100 characters allowed")]
    ActivityNameTooLong(usize),

    #[error("Activity name already exists: {0}")]
    DuplicateActivityName(String),

    #[error("Invalid color format: {0}. Must be #RGB, #RRGGBB, or #RRGGBBAA")]
    InvalidColorFormat(String),

    #[error("Activity icon too long: {0} characters. Maximum 20 characters allowed")]
    ActivityIconTooLong(usize),

    #[error("Notes too long: {0} characters. Maximum {1} characters allowed")]
    NotesLengthExceeded(usize, usize),

    #[error("Database lock poisoned - a panic occurred while holding the database lock. The application should restart.")]
    LockPoisoned,

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Mood check-in not found: {0}")]
    MoodCheckinNotFound(i32),

    #[error("Transaction rollback failed: {0}. Database may be in inconsistent state")]
    TransactionFailure(String),
}

impl MoodError {
    /// Convert to structured CommandError for frontend consumption
    pub fn to_command_error(&self) -> CommandError {
        match self {
            // Validation errors - not retryable
            MoodError::InvalidRating(_) => CommandError::permanent(self.to_string(), "validation"),
            MoodError::EmptyActivityName => CommandError::permanent(self.to_string(), "validation"),
            MoodError::ActivityNameTooLong(_) => {
                CommandError::permanent(self.to_string(), "validation")
            }
            MoodError::InvalidColorFormat(_) => {
                CommandError::permanent(self.to_string(), "validation")
            }
            MoodError::ActivityIconTooLong(_) => {
                CommandError::permanent(self.to_string(), "validation")
            }
            MoodError::NotesLengthExceeded(_, _) => {
                CommandError::permanent(self.to_string(), "validation")
            }

            // Not found errors - not retryable
            MoodError::ActivityNotFound(id) => {
                CommandError::permanent(self.to_string(), "not_found").with_details(
                    serde_json::json!({
                        "resource": "activity",
                        "id": id
                    }),
                )
            }
            MoodError::MoodCheckinNotFound(id) => {
                CommandError::permanent(self.to_string(), "not_found").with_details(
                    serde_json::json!({
                        "resource": "mood_checkin",
                        "id": id
                    }),
                )
            }

            // Duplicate errors - not retryable
            MoodError::DuplicateActivityName(name) => {
                CommandError::permanent(self.to_string(), "duplicate").with_details(
                    serde_json::json!({
                        "field": "name",
                        "value": name
                    }),
                )
            }

            // Database lock/transient errors - retryable
            MoodError::LockPoisoned => CommandError::retryable(self.to_string(), "lock_poisoned"),
            MoodError::TransactionFailure(_) => {
                CommandError::retryable(self.to_string(), "transaction_failure")
            }
            MoodError::Database(e) => {
                // Classify SQLite errors as retryable or permanent
                match e {
                    rusqlite::Error::SqliteFailure(err, _) => {
                        // SQLITE_BUSY (5), SQLITE_LOCKED (6) are retryable
                        if err.code == rusqlite::ErrorCode::DatabaseBusy
                            || err.code == rusqlite::ErrorCode::DatabaseLocked
                        {
                            CommandError::retryable(self.to_string(), "database_locked")
                        } else {
                            CommandError::permanent(self.to_string(), "database")
                        }
                    }
                    _ => CommandError::permanent(self.to_string(), "database"),
                }
            }
        }
    }
}

/// Activity model
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Activity {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

/// Mood check-in model
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct MoodCheckin {
    pub id: i32,
    pub mood_rating: i32,
    pub notes: Option<String>,
    pub activities: Vec<Activity>,
    pub created_at: String,
}

/// Request to log a mood check-in
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
pub struct LogMoodRequest {
    #[validate(range(min = 1, max = 5))]
    pub mood_rating: i32,
    pub activity_ids: Vec<i32>,
    #[validate(length(max = 5000))]
    pub notes: Option<String>,
}

/// Request to create an activity
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
pub struct CreateActivityRequest {
    #[validate(custom(function = "validate_trimmed_name"))]
    pub name: String,
    #[validate(custom(function = "validate_hex_color"))]
    pub color: Option<String>,
    #[validate(length(max = 20))]
    pub icon: Option<String>,
}

/// Request to update an activity
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
pub struct UpdateActivityRequest {
    #[validate(custom(function = "validate_trimmed_name"))]
    pub name: Option<String>,
    #[validate(custom(function = "validate_hex_color"))]
    pub color: Option<String>,
    #[validate(length(max = 20))]
    pub icon: Option<String>,
}

/// Mood statistics
#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct MoodStats {
    pub average_mood: f64,
    pub total_checkins: i32,
    pub mood_distribution: std::collections::HashMap<i32, i32>,
    pub activity_correlations: Vec<ActivityCorrelation>,
}

/// Activity correlation with mood
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ActivityCorrelation {
    pub activity: Activity,
    pub average_mood: f64,
    pub checkin_count: i32,
}

/// Validate mood rating is between 1 and 5
pub fn validate_mood_rating(rating: i32) -> Result<(), MoodError> {
    if !(1..=5).contains(&rating) {
        return Err(MoodError::InvalidRating(rating));
    }
    Ok(())
}

/// Validate activity name (1-100 characters, non-empty after trim)
pub fn validate_activity_name(name: &str) -> Result<String, MoodError> {
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err(MoodError::EmptyActivityName);
    }
    if trimmed.len() > 100 {
        return Err(MoodError::ActivityNameTooLong(trimmed.len()));
    }
    Ok(trimmed)
}

/// Validate notes length (uses centralized MAX_NOTES_LENGTH constant)
pub fn validate_notes(notes: &str) -> Result<(), MoodError> {
    if notes.len() > MAX_NOTES_LENGTH {
        return Err(MoodError::NotesLengthExceeded(
            notes.len(),
            MAX_NOTES_LENGTH,
        ));
    }
    Ok(())
}

/// Validate hex color format (#RGB, #RRGGBB, or #RRGGBBAA)
pub fn validate_color(color: &str) -> Result<(), MoodError> {
    if !color.starts_with('#') {
        return Err(MoodError::InvalidColorFormat(color.to_string()));
    }

    // Valid lengths: 4 (#RGB), 7 (#RRGGBB), or 9 (#RRGGBBAA)
    let hex_part_len = color.len() - 1;
    if hex_part_len != 3 && hex_part_len != 6 && hex_part_len != 8 {
        return Err(MoodError::InvalidColorFormat(color.to_string()));
    }

    // Check that all characters after # are valid hex digits
    for ch in color[1..].chars() {
        if !ch.is_ascii_hexdigit() {
            return Err(MoodError::InvalidColorFormat(color.to_string()));
        }
    }

    Ok(())
}

/// Custom validator function for hex color (for use with validator crate)
fn validate_hex_color(color: &str) -> Result<(), validator::ValidationError> {
    validate_color(color).map_err(|_| {
        let mut error = validator::ValidationError::new("hex_color");
        error.message = Some(std::borrow::Cow::from(
            "Must be valid hex color format: #RGB, #RRGGBB, or #RRGGBBAA",
        ));
        error
    })
}

/// Custom validator function for activity name (for use with validator crate)
fn validate_trimmed_name(name: &str) -> Result<(), validator::ValidationError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        let mut error = validator::ValidationError::new("empty_name");
        error.message = Some(std::borrow::Cow::from("Activity name cannot be empty"));
        return Err(error);
    }
    if trimmed.len() > 100 {
        let mut error = validator::ValidationError::new("name_too_long");
        error.message = Some(std::borrow::Cow::from(
            "Activity name too long (max 100 characters)",
        ));
        return Err(error);
    }
    Ok(())
}

/// Validate activity icon (max 20 characters to accommodate compound emoji sequences)
pub fn validate_icon(icon: &str) -> Result<(), MoodError> {
    if icon.len() > 20 {
        return Err(MoodError::ActivityIconTooLong(icon.len()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // T068: Unit test - Mood rating validation (1-5 only)
    #[test]
    fn test_mood_rating_validation_valid() {
        assert!(validate_mood_rating(1).is_ok());
        assert!(validate_mood_rating(2).is_ok());
        assert!(validate_mood_rating(3).is_ok());
        assert!(validate_mood_rating(4).is_ok());
        assert!(validate_mood_rating(5).is_ok());
    }

    #[test]
    fn test_mood_rating_validation_invalid() {
        assert!(validate_mood_rating(0).is_err());
        assert!(validate_mood_rating(6).is_err());
        assert!(validate_mood_rating(-1).is_err());
        assert!(validate_mood_rating(100).is_err());
    }

    #[test]
    fn test_mood_rating_validation_error_message() {
        let result = validate_mood_rating(0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid mood rating: 0. Must be 1-5"
        );

        let result = validate_mood_rating(6);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid mood rating: 6. Must be 1-5"
        );
    }

    #[test]
    fn test_activity_name_validation() {
        // Valid names
        assert!(validate_activity_name("Exercise").is_ok());
        assert!(validate_activity_name("  Meditation  ").is_ok());
        assert_eq!(validate_activity_name("  Reading  ").unwrap(), "Reading");

        // Invalid names
        assert!(validate_activity_name("").is_err());
        assert!(validate_activity_name("   ").is_err());
    }

    #[test]
    fn test_color_validation() {
        // Valid 6-digit colors (#RRGGBB)
        assert!(validate_color("#FF5733").is_ok());
        assert!(validate_color("#000000").is_ok());
        assert!(validate_color("#ffffff").is_ok());
        assert!(validate_color("#4CAF50").is_ok());

        // Valid 3-digit colors (#RGB)
        assert!(validate_color("#FFF").is_ok());
        assert!(validate_color("#000").is_ok());
        assert!(validate_color("#F5A").is_ok());

        // Valid 8-digit colors with alpha (#RRGGBBAA)
        assert!(validate_color("#FF5733FF").is_ok());
        assert!(validate_color("#00000080").is_ok());
        assert!(validate_color("#4CAF5000").is_ok());

        // Invalid colors
        assert!(validate_color("FF5733").is_err()); // Missing #
        assert!(validate_color("#FF57").is_err()); // Wrong length
        assert!(validate_color("#FF57331").is_err()); // Wrong length
        assert!(validate_color("blue").is_err()); // Not hex
        assert!(validate_color("#GGGGGG").is_err()); // Invalid hex chars
        assert!(validate_color("#FF").is_err()); // Too short
    }
}
