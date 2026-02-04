// Mood type definitions for improved type safety
// Replaces i32 mood_rating with validated newtype

use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors for mood type validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum MoodTypeError {
    #[error("Invalid mood rating: {0}. Must be between 1 and 7")]
    InvalidRating(i32),
}

/// Mood rating on a 7-point scale
///
/// Newtype wrapper ensuring the rating is always valid (1-7).
/// - 1 = Terrible
/// - 2 = Very Bad
/// - 3 = Bad
/// - 4 = Ok
/// - 5 = Good
/// - 6 = Very Good
/// - 7 = Excellent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(try_from = "i32", into = "i32")]
pub struct MoodRating(i32);

impl MoodRating {
    /// Minimum valid mood rating
    pub const MIN: i32 = 1;
    /// Maximum valid mood rating
    pub const MAX: i32 = 7;

    /// Create a new MoodRating, validating that the value is 1-7
    pub fn new(value: i32) -> Result<Self, MoodTypeError> {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(MoodTypeError::InvalidRating(value));
        }
        Ok(MoodRating(value))
    }

    /// Get the underlying value
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Get a human-readable label for this mood rating
    pub fn label(&self) -> &'static str {
        match self.0 {
            1 => "Terrible",
            2 => "Very Bad",
            3 => "Bad",
            4 => "Ok",
            5 => "Good",
            6 => "Very Good",
            7 => "Excellent",
            _ => unreachable!("MoodRating is always validated to be 1-7"),
        }
    }

    /// Check if this is a negative mood (1-3)
    pub fn is_negative(&self) -> bool {
        self.0 <= 3
    }

    /// Check if this is a neutral mood (4)
    pub fn is_neutral(&self) -> bool {
        self.0 == 4
    }

    /// Check if this is a positive mood (5-7)
    pub fn is_positive(&self) -> bool {
        self.0 >= 5
    }
}

impl TryFrom<i32> for MoodRating {
    type Error = MoodTypeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        MoodRating::new(value)
    }
}

impl From<MoodRating> for i32 {
    fn from(rating: MoodRating) -> Self {
        rating.0
    }
}

impl std::fmt::Display for MoodRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for MoodRating {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MoodRating {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl ToSql for MoodRating {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0))
    }
}

impl FromSql for MoodRating {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let i64_val = value.as_i64()?;
        let i = i32::try_from(i64_val).map_err(|_| {
            FromSqlError::Other(Box::new(MoodTypeError::InvalidRating(
                i64_val.clamp(i32::MIN as i64, i32::MAX as i64) as i32,
            )))
        })?;
        MoodRating::new(i).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mood_rating_valid() {
        for i in 1..=7 {
            assert!(MoodRating::new(i).is_ok());
            assert_eq!(MoodRating::new(i).unwrap().value(), i);
        }
    }

    #[test]
    fn test_mood_rating_invalid() {
        assert!(MoodRating::new(0).is_err());
        assert!(MoodRating::new(-1).is_err());
        assert!(MoodRating::new(8).is_err());
        assert!(MoodRating::new(100).is_err());
    }

    #[test]
    fn test_mood_rating_error_message() {
        let result = MoodRating::new(0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid mood rating: 0. Must be between 1 and 7"
        );
    }

    #[test]
    fn test_mood_rating_try_from() {
        let rating: Result<MoodRating, _> = 5.try_into();
        assert!(rating.is_ok());
        assert_eq!(rating.unwrap().value(), 5);

        let invalid: Result<MoodRating, _> = 0.try_into();
        assert!(invalid.is_err());
    }

    #[test]
    fn test_mood_rating_into_i32() {
        let rating = MoodRating::new(4).unwrap();
        let value: i32 = rating.into();
        assert_eq!(value, 4);
    }

    #[test]
    fn test_mood_rating_label() {
        assert_eq!(MoodRating::new(1).unwrap().label(), "Terrible");
        assert_eq!(MoodRating::new(2).unwrap().label(), "Very Bad");
        assert_eq!(MoodRating::new(3).unwrap().label(), "Bad");
        assert_eq!(MoodRating::new(4).unwrap().label(), "Ok");
        assert_eq!(MoodRating::new(5).unwrap().label(), "Good");
        assert_eq!(MoodRating::new(6).unwrap().label(), "Very Good");
        assert_eq!(MoodRating::new(7).unwrap().label(), "Excellent");
    }

    #[test]
    fn test_mood_rating_categories() {
        // Negative moods (1-3)
        assert!(MoodRating::new(1).unwrap().is_negative());
        assert!(MoodRating::new(2).unwrap().is_negative());
        assert!(MoodRating::new(3).unwrap().is_negative());
        assert!(!MoodRating::new(4).unwrap().is_negative());

        // Neutral mood (4)
        assert!(MoodRating::new(4).unwrap().is_neutral());
        assert!(!MoodRating::new(3).unwrap().is_neutral());
        assert!(!MoodRating::new(5).unwrap().is_neutral());

        // Positive moods (5-7)
        assert!(MoodRating::new(5).unwrap().is_positive());
        assert!(MoodRating::new(6).unwrap().is_positive());
        assert!(MoodRating::new(7).unwrap().is_positive());
        assert!(!MoodRating::new(4).unwrap().is_positive());
    }

    #[test]
    fn test_mood_rating_ord() {
        let low = MoodRating::new(1).unwrap();
        let mid = MoodRating::new(4).unwrap();
        let high = MoodRating::new(7).unwrap();

        assert!(low < mid);
        assert!(mid < high);
        assert!(low < high);
        assert_eq!(mid, MoodRating::new(4).unwrap());
    }

    #[test]
    fn test_mood_rating_serde_roundtrip() {
        let rating = MoodRating::new(5).unwrap();
        let json = serde_json::to_string(&rating).unwrap();
        assert_eq!(json, "5");
        let parsed: MoodRating = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, rating);
    }

    #[test]
    fn test_mood_rating_serde_invalid() {
        let result: Result<MoodRating, _> = serde_json::from_str("0");
        assert!(result.is_err());

        let result: Result<MoodRating, _> = serde_json::from_str("8");
        assert!(result.is_err());
    }
}
