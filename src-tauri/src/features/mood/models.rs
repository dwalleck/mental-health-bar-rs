use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Mood feature errors
#[derive(Error, Debug)]
pub enum MoodError {
    #[error("Invalid mood rating: {0}. Must be 1-5")]
    InvalidRating(i32),

    #[error("Activity not found: {0}")]
    ActivityNotFound(i64),

    #[error("Activity name cannot be empty")]
    EmptyActivityName,

    #[error("Activity name already exists: {0}")]
    DuplicateActivityName(String),

    #[error("Invalid color format: {0}. Must be #RRGGBB")]
    InvalidColorFormat(String),

    #[error("Database error: {0}")]
    Database(#[from] duckdb::Error),

    #[error("Mood check-in not found: {0}")]
    MoodCheckinNotFound(i64),
}

/// Activity model
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Activity {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

/// Mood check-in model
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct MoodCheckin {
    pub id: i64,
    pub mood_rating: i32,
    pub notes: Option<String>,
    pub activities: Vec<Activity>,
    pub created_at: String,
}

/// Request to log a mood check-in
#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct LogMoodRequest {
    pub mood_rating: i32,
    pub activity_ids: Vec<i64>,
    pub notes: Option<String>,
}

/// Request to create an activity
#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct CreateActivityRequest {
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

/// Request to update an activity
#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct UpdateActivityRequest {
    pub name: Option<String>,
    pub color: Option<String>,
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
    if rating < 1 || rating > 5 {
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
        return Err(MoodError::EmptyActivityName); // Will improve error message later
    }
    Ok(trimmed)
}

/// Validate hex color format (#RRGGBB)
pub fn validate_color(color: &str) -> Result<(), MoodError> {
    if color.len() != 7 {
        return Err(MoodError::InvalidColorFormat(color.to_string()));
    }

    if !color.starts_with('#') {
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
        assert_eq!(
            validate_activity_name("  Reading  ").unwrap(),
            "Reading"
        );

        // Invalid names
        assert!(validate_activity_name("").is_err());
        assert!(validate_activity_name("   ").is_err());
    }

    #[test]
    fn test_color_validation() {
        // Valid colors
        assert!(validate_color("#FF5733").is_ok());
        assert!(validate_color("#000000").is_ok());
        assert!(validate_color("#ffffff").is_ok());
        assert!(validate_color("#4CAF50").is_ok());

        // Invalid colors
        assert!(validate_color("FF5733").is_err());
        assert!(validate_color("#FF57").is_err());
        assert!(validate_color("#FF57331").is_err());
        assert!(validate_color("blue").is_err());
        assert!(validate_color("#GGGGGG").is_err());
    }
}
