use crate::{
    errors::{CommandError, ErrorType, ToCommandError},
    types::{HexColor, MoodRating},
    MAX_NOTES_LENGTH,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

// Re-export Activity from types for backwards compatibility
// (previously defined in this module, now consolidated in types/activity.rs)
pub use crate::types::Activity;

/// Mood feature errors
#[derive(Error, Debug)]
pub enum MoodError {
    #[error("Invalid mood rating: {0}. Must be 1-7")]
    InvalidRating(i32),

    #[error("Activity not found: {0}")]
    ActivityNotFound(i32),

    #[error("Activity name cannot be empty")]
    EmptyActivityName,

    #[error("Activity name too long: {0} characters. Maximum 50 characters allowed")]
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

impl ToCommandError for MoodError {
    fn to_command_error(&self) -> CommandError {
        match self {
            // Validation errors - not retryable
            MoodError::InvalidRating(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            MoodError::EmptyActivityName => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            MoodError::ActivityNameTooLong(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            MoodError::InvalidColorFormat(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            MoodError::ActivityIconTooLong(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            MoodError::NotesLengthExceeded(_, _) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }

            // Not found errors - not retryable
            MoodError::ActivityNotFound(id) => {
                CommandError::permanent(self.to_string(), ErrorType::NotFound).with_details(
                    serde_json::json!({
                        "resource": "activity",
                        "id": id
                    }),
                )
            }
            MoodError::MoodCheckinNotFound(id) => {
                CommandError::permanent(self.to_string(), ErrorType::NotFound).with_details(
                    serde_json::json!({
                        "resource": "mood_checkin",
                        "id": id
                    }),
                )
            }

            // Duplicate errors - not retryable
            MoodError::DuplicateActivityName(name) => {
                CommandError::permanent(self.to_string(), ErrorType::Duplicate).with_details(
                    serde_json::json!({
                        "field": "name",
                        "value": name
                    }),
                )
            }

            // Database lock/transient errors - retryable
            MoodError::LockPoisoned => {
                CommandError::retryable(self.to_string(), ErrorType::LockPoisoned)
            }
            MoodError::TransactionFailure(_) => {
                CommandError::retryable(self.to_string(), ErrorType::TransactionFailure)
            }
            MoodError::Database(e) => CommandError::from_rusqlite_error(e),
        }
    }
}

// Note: Activity struct has been moved to types/activity.rs to avoid duplication
// across features/mood and features/activities modules. It is re-exported below.

/// Mood check-in model
///
/// Tracks a mood rating on a 7-point scale with optional activities and notes.
/// The `MoodRating` newtype ensures the value is always valid (1-7).
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct MoodCheckin {
    pub id: i32,
    /// Mood rating (1-7) with type-enforced validation
    pub mood_rating: MoodRating,
    pub notes: Option<String>,
    pub activities: Vec<Activity>,
    pub created_at: String,
}

/// Request to log a mood check-in
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
pub struct LogMoodRequest {
    #[validate(range(min = 1, max = 7))]
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
    /// Color validated on deserialization via HexColor newtype
    pub color: Option<HexColor>,
    #[validate(custom(function = "validate_optional_icon"))]
    pub icon: Option<String>,
    pub group_id: i32,
}

/// Request to update an activity
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
pub struct UpdateActivityRequest {
    #[validate(custom(function = "validate_trimmed_name"))]
    pub name: Option<String>,
    /// Color validated on deserialization via HexColor newtype
    pub color: Option<HexColor>,
    #[validate(custom(function = "validate_optional_icon"))]
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

/// Validate mood rating is between 1 and 7
/// 1=Terrible, 2=Very Bad, 3=Bad, 4=Ok, 5=Good, 6=Very Good, 7=Excellent
pub fn validate_mood_rating(rating: i32) -> Result<(), MoodError> {
    if !(1..=7).contains(&rating) {
        return Err(MoodError::InvalidRating(rating));
    }
    Ok(())
}

/// Validate activity name (1-50 characters, non-empty after trim)
pub fn validate_activity_name(name: &str) -> Result<String, MoodError> {
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err(MoodError::EmptyActivityName);
    }
    let char_count = trimmed.chars().count();
    if char_count > 50 {
        return Err(MoodError::ActivityNameTooLong(char_count));
    }
    Ok(trimmed)
}

/// Validate notes length (uses centralized MAX_NOTES_LENGTH constant)
pub fn validate_notes(notes: &str) -> Result<(), MoodError> {
    let char_count = notes.chars().count();
    if char_count > MAX_NOTES_LENGTH {
        return Err(MoodError::NotesLengthExceeded(char_count, MAX_NOTES_LENGTH));
    }
    Ok(())
}

/// Custom validator function for activity name (for use with validator crate)
fn validate_trimmed_name(name: &str) -> Result<(), validator::ValidationError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        let mut error = validator::ValidationError::new("empty_name");
        error.message = Some(std::borrow::Cow::from("Activity name cannot be empty"));
        return Err(error);
    }
    let char_count = trimmed.chars().count();
    if char_count > 50 {
        let mut error = validator::ValidationError::new("name_too_long");
        error.message = Some(std::borrow::Cow::from(
            "Activity name too long (max 50 characters)",
        ));
        return Err(error);
    }
    Ok(())
}

/// Validate activity icon (max 20 characters to accommodate compound emoji sequences)
pub fn validate_icon(icon: &str) -> Result<(), MoodError> {
    let char_count = icon.chars().count();
    if char_count > 20 {
        return Err(MoodError::ActivityIconTooLong(char_count));
    }
    Ok(())
}

/// Custom validator function for optional icon (for use with validator crate)
/// Prevents Some("") by requiring non-empty strings when icon is provided
fn validate_optional_icon(icon: &str) -> Result<(), validator::ValidationError> {
    if icon.is_empty() {
        let mut error = validator::ValidationError::new("empty_icon");
        error.message = Some(std::borrow::Cow::from(
            "Icon cannot be an empty string. Use None instead.",
        ));
        return Err(error);
    }

    validate_icon(icon).map_err(|e| {
        let mut error = validator::ValidationError::new("icon_validation");
        error.message = Some(std::borrow::Cow::from(e.to_string()));
        error
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // T068: Unit test - Mood rating validation (1-7 scale)
    #[test]
    fn test_mood_rating_validation_valid() {
        assert!(validate_mood_rating(1).is_ok()); // Terrible
        assert!(validate_mood_rating(2).is_ok()); // Very Bad
        assert!(validate_mood_rating(3).is_ok()); // Bad
        assert!(validate_mood_rating(4).is_ok()); // Ok
        assert!(validate_mood_rating(5).is_ok()); // Good
        assert!(validate_mood_rating(6).is_ok()); // Very Good
        assert!(validate_mood_rating(7).is_ok()); // Excellent
    }

    #[test]
    fn test_mood_rating_validation_invalid() {
        assert!(validate_mood_rating(0).is_err());
        assert!(validate_mood_rating(8).is_err());
        assert!(validate_mood_rating(-1).is_err());
        assert!(validate_mood_rating(100).is_err());
    }

    #[test]
    fn test_mood_rating_validation_error_message() {
        let result = validate_mood_rating(0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid mood rating: 0. Must be 1-7"
        );

        let result = validate_mood_rating(8);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid mood rating: 8. Must be 1-7"
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

    // Note: Color validation tests are in types/activity.rs (HexColor newtype tests)
}
