// Activity type definitions for improved type safety
// Replaces stringly-typed goal_type field with enum and adds validated newtypes

use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

/// Errors for activity type parsing and validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ActivityTypeError {
    #[error("Invalid goal type: '{0}'. Must be 'days_per_period' or 'percent_improvement'")]
    InvalidGoalType(String),

    #[error("Invalid goal target: {0}. Must be a positive integer")]
    InvalidGoalTarget(i32),

    #[error("Invalid color format: '{0}'. Must be #RGB, #RRGGBB, or #RRGGBBAA")]
    InvalidColorFormat(String),
}

/// Goal type for activity tracking
///
/// Replaces string matching for `"days_per_period"` and `"percent_improvement"`.
#[derive(Serialize, Deserialize, specta::Type, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum GoalType {
    /// Track frequency: achieve X days within a Y-day period
    /// Example: "Exercise 3 days per 7-day period"
    DaysPerPeriod,
    /// Track improvement: increase activity by X% over baseline
    /// Example: "Increase meditation by 20% over 30-day baseline"
    PercentImprovement,
}

impl GoalType {
    /// Get the string representation for database storage
    pub fn as_str(&self) -> &'static str {
        match self {
            GoalType::DaysPerPeriod => "days_per_period",
            GoalType::PercentImprovement => "percent_improvement",
        }
    }

    /// Get all valid goal types
    pub fn all() -> &'static [GoalType] {
        &[GoalType::DaysPerPeriod, GoalType::PercentImprovement]
    }
}

impl FromStr for GoalType {
    type Err = ActivityTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "days_per_period" => Ok(GoalType::DaysPerPeriod),
            "percent_improvement" => Ok(GoalType::PercentImprovement),
            _ => Err(ActivityTypeError::InvalidGoalType(s.to_string())),
        }
    }
}

impl std::fmt::Display for GoalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ToSql for GoalType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

impl FromSql for GoalType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s = value.as_str()?;
        GoalType::from_str(s).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

/// Positive integer target value for goals
///
/// Newtype wrapper ensuring the target is always positive (>= 1).
/// For `DaysPerPeriod`: number of days
/// For `PercentImprovement`: percentage (e.g., 20 = 20%)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(try_from = "i32", into = "i32")]
pub struct GoalTarget(i32);

impl GoalTarget {
    /// Create a new GoalTarget, validating that the value is positive
    pub fn new(value: i32) -> Result<Self, ActivityTypeError> {
        if value < 1 {
            return Err(ActivityTypeError::InvalidGoalTarget(value));
        }
        Ok(GoalTarget(value))
    }

    /// Get the underlying value
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for GoalTarget {
    type Error = ActivityTypeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        GoalTarget::new(value)
    }
}

impl From<GoalTarget> for i32 {
    fn from(target: GoalTarget) -> Self {
        target.0
    }
}

impl std::fmt::Display for GoalTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Validated hex color string
///
/// Newtype wrapper ensuring the color is a valid hex format:
/// - #RGB (4 chars, e.g., #F00 for red)
/// - #RRGGBB (7 chars, e.g., #FF0000 for red)
/// - #RRGGBBAA (9 chars, e.g., #FF000080 for semi-transparent red)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(try_from = "String", into = "String")]
pub struct HexColor(String);

impl HexColor {
    /// Create a new HexColor, validating the format
    pub fn new(value: impl Into<String>) -> Result<Self, ActivityTypeError> {
        let s = value.into();
        Self::validate(&s)?;
        Ok(HexColor(s))
    }

    /// Validate a hex color string
    fn validate(color: &str) -> Result<(), ActivityTypeError> {
        if !color.starts_with('#') {
            return Err(ActivityTypeError::InvalidColorFormat(color.to_string()));
        }

        // Valid lengths: 4 (#RGB), 7 (#RRGGBB), or 9 (#RRGGBBAA)
        let hex_part_len = color.len() - 1;
        if hex_part_len != 3 && hex_part_len != 6 && hex_part_len != 8 {
            return Err(ActivityTypeError::InvalidColorFormat(color.to_string()));
        }

        // Check that all characters after # are valid hex digits
        for ch in color[1..].chars() {
            if !ch.is_ascii_hexdigit() {
                return Err(ActivityTypeError::InvalidColorFormat(color.to_string()));
            }
        }

        Ok(())
    }

    /// Get the underlying color string
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Get the color as an owned String
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for HexColor {
    type Error = ActivityTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        HexColor::new(value)
    }
}

impl TryFrom<&str> for HexColor {
    type Error = ActivityTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        HexColor::new(value)
    }
}

impl From<HexColor> for String {
    fn from(color: HexColor) -> Self {
        color.0
    }
}

impl std::fmt::Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for HexColor {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToSql for HexColor {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.as_str()))
    }
}

impl FromSql for HexColor {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s = value.as_str()?;
        HexColor::new(s).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // GoalType tests
    #[test]
    fn test_goal_type_from_str() {
        assert_eq!(
            GoalType::from_str("days_per_period").unwrap(),
            GoalType::DaysPerPeriod
        );
        assert_eq!(
            GoalType::from_str("DAYS_PER_PERIOD").unwrap(),
            GoalType::DaysPerPeriod
        );
        assert_eq!(
            GoalType::from_str("percent_improvement").unwrap(),
            GoalType::PercentImprovement
        );
    }

    #[test]
    fn test_goal_type_from_str_invalid() {
        assert!(GoalType::from_str("weekly").is_err());
        assert!(GoalType::from_str("").is_err());
    }

    #[test]
    fn test_goal_type_as_str() {
        assert_eq!(GoalType::DaysPerPeriod.as_str(), "days_per_period");
        assert_eq!(GoalType::PercentImprovement.as_str(), "percent_improvement");
    }

    #[test]
    fn test_goal_type_serde_roundtrip() {
        let goal_type = GoalType::DaysPerPeriod;
        let json = serde_json::to_string(&goal_type).unwrap();
        assert_eq!(json, r#""days_per_period""#);
        let parsed: GoalType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, goal_type);
    }

    // GoalTarget tests
    #[test]
    fn test_goal_target_valid() {
        assert!(GoalTarget::new(1).is_ok());
        assert!(GoalTarget::new(100).is_ok());
        assert_eq!(GoalTarget::new(5).unwrap().value(), 5);
    }

    #[test]
    fn test_goal_target_invalid() {
        assert!(GoalTarget::new(0).is_err());
        assert!(GoalTarget::new(-1).is_err());
        assert!(GoalTarget::new(-100).is_err());
    }

    #[test]
    fn test_goal_target_try_from() {
        let target: Result<GoalTarget, _> = 5.try_into();
        assert!(target.is_ok());
        assert_eq!(target.unwrap().value(), 5);

        let invalid: Result<GoalTarget, _> = 0.try_into();
        assert!(invalid.is_err());
    }

    #[test]
    fn test_goal_target_into_i32() {
        let target = GoalTarget::new(42).unwrap();
        let value: i32 = target.into();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_goal_target_serde_roundtrip() {
        let target = GoalTarget::new(7).unwrap();
        let json = serde_json::to_string(&target).unwrap();
        assert_eq!(json, "7");
        let parsed: GoalTarget = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, target);
    }

    #[test]
    fn test_goal_target_serde_invalid() {
        let result: Result<GoalTarget, _> = serde_json::from_str("0");
        assert!(result.is_err());
    }

    // HexColor tests
    #[test]
    fn test_hex_color_valid_6_digit() {
        assert!(HexColor::new("#FF5733").is_ok());
        assert!(HexColor::new("#000000").is_ok());
        assert!(HexColor::new("#ffffff").is_ok());
        assert!(HexColor::new("#4CAF50").is_ok());
    }

    #[test]
    fn test_hex_color_valid_3_digit() {
        assert!(HexColor::new("#FFF").is_ok());
        assert!(HexColor::new("#000").is_ok());
        assert!(HexColor::new("#F5A").is_ok());
    }

    #[test]
    fn test_hex_color_valid_8_digit() {
        assert!(HexColor::new("#FF5733FF").is_ok());
        assert!(HexColor::new("#00000080").is_ok());
        assert!(HexColor::new("#4CAF5000").is_ok());
    }

    #[test]
    fn test_hex_color_invalid() {
        assert!(HexColor::new("FF5733").is_err()); // Missing #
        assert!(HexColor::new("#FF57").is_err()); // Wrong length
        assert!(HexColor::new("#FF57331").is_err()); // Wrong length
        assert!(HexColor::new("blue").is_err()); // Not hex
        assert!(HexColor::new("#GGGGGG").is_err()); // Invalid hex chars
        assert!(HexColor::new("#FF").is_err()); // Too short
    }

    #[test]
    fn test_hex_color_value() {
        let color = HexColor::new("#4CAF50").unwrap();
        assert_eq!(color.value(), "#4CAF50");
    }

    #[test]
    fn test_hex_color_serde_roundtrip() {
        let color = HexColor::new("#FF5733").unwrap();
        let json = serde_json::to_string(&color).unwrap();
        assert_eq!(json, "\"#FF5733\"");
        let parsed: HexColor = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, color);
    }

    #[test]
    fn test_hex_color_serde_invalid() {
        let result: Result<HexColor, _> = serde_json::from_str(r#""invalid""#);
        assert!(result.is_err());
    }
}
