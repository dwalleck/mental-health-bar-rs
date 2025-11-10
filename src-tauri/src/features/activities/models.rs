use crate::errors::{CommandError, ErrorType, ToCommandError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

/// Activities feature errors
#[derive(Error, Debug)]
pub enum ActivityError {
    #[error("Activity group not found: {0}")]
    GroupNotFound(i32),

    #[error("Activity not found: {0}")]
    ActivityNotFound(i32),

    #[error("Activity log not found: {0}")]
    LogNotFound(i32),

    #[error("Activity goal not found: {0}")]
    GoalNotFound(i32),

    #[error("Activity group name cannot be empty")]
    EmptyGroupName,

    #[error("Activity group name too long: {0} characters. Maximum 100 characters allowed")]
    GroupNameTooLong(usize),

    #[error("Activity group description too long: {0} characters. Maximum 500 characters allowed")]
    DescriptionTooLong(usize),

    #[error("Activity group name already exists: {0}")]
    DuplicateGroupName(String),

    #[error("Activity name cannot be empty")]
    EmptyActivityName,

    #[error("Activity name too long: {0} characters. Maximum 50 characters allowed")]
    ActivityNameTooLong(usize),

    #[error("Activity name already exists: {0}")]
    DuplicateActivityName(String),

    #[error("Activity icon too long: {0} characters. Maximum 20 characters allowed")]
    ActivityIconTooLong(usize),

    #[error("Notes too long: {0} characters. Maximum 500 characters allowed")]
    NotesLengthExceeded(usize),

    #[error("Invalid goal type: {0}. Must be 'days_per_period' or 'percent_improvement'")]
    InvalidGoalType(String),

    #[error("Goal must target either an activity OR a group, not both")]
    InvalidGoalTarget,

    #[error("Goal must target either an activity or a group")]
    MissingGoalTarget,

    #[error("Target value must be positive: {0}")]
    InvalidTargetValue(i32),

    #[error("Period days must be positive: {0}")]
    InvalidPeriodDays(i32),

    #[error("Database lock poisoned - a panic occurred while holding the database lock. The application should restart.")]
    LockPoisoned,

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Transaction rollback failed: {0}. Database may be in inconsistent state")]
    TransactionFailure(String),
}

impl ToCommandError for ActivityError {
    fn to_command_error(&self) -> CommandError {
        match self {
            // Validation errors - not retryable
            ActivityError::EmptyGroupName => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::GroupNameTooLong(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::DescriptionTooLong(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::EmptyActivityName => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::ActivityNameTooLong(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::ActivityIconTooLong(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::NotesLengthExceeded(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::InvalidGoalType(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::InvalidGoalTarget => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::MissingGoalTarget => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::InvalidTargetValue(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }
            ActivityError::InvalidPeriodDays(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation)
            }

            // Not found errors - not retryable
            ActivityError::GroupNotFound(id) => {
                CommandError::permanent(self.to_string(), ErrorType::NotFound).with_details(
                    serde_json::json!({
                        "resource": "activity_group",
                        "id": id
                    }),
                )
            }
            ActivityError::ActivityNotFound(id) => {
                CommandError::permanent(self.to_string(), ErrorType::NotFound).with_details(
                    serde_json::json!({
                        "resource": "activity",
                        "id": id
                    }),
                )
            }
            ActivityError::LogNotFound(id) => {
                CommandError::permanent(self.to_string(), ErrorType::NotFound).with_details(
                    serde_json::json!({
                        "resource": "activity_log",
                        "id": id
                    }),
                )
            }
            ActivityError::GoalNotFound(id) => {
                CommandError::permanent(self.to_string(), ErrorType::NotFound).with_details(
                    serde_json::json!({
                        "resource": "activity_goal",
                        "id": id
                    }),
                )
            }

            // Duplicate errors - not retryable
            ActivityError::DuplicateGroupName(name) => {
                CommandError::permanent(self.to_string(), ErrorType::Duplicate).with_details(
                    serde_json::json!({
                        "field": "name",
                        "value": name
                    }),
                )
            }
            ActivityError::DuplicateActivityName(name) => {
                CommandError::permanent(self.to_string(), ErrorType::Duplicate).with_details(
                    serde_json::json!({
                        "field": "name",
                        "value": name
                    }),
                )
            }

            // Database lock/transient errors - retryable
            ActivityError::LockPoisoned => {
                CommandError::retryable(self.to_string(), ErrorType::LockPoisoned)
            }
            ActivityError::TransactionFailure(_) => {
                CommandError::retryable(self.to_string(), ErrorType::TransactionFailure)
            }
            ActivityError::Database(e) => CommandError::from_rusqlite_error(e),
        }
    }
}

/// Activity Group model
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ActivityGroup {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

/// Activity model (updated with group_id)
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Activity {
    pub id: i32,
    pub group_id: i32,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

/// Activity Log model
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ActivityLog {
    pub id: i32,
    pub activity_id: i32,
    pub logged_at: String,
    pub created_at: String,
    pub notes: Option<String>,
    pub deleted_at: Option<String>,
}

/// Activity Goal model
///
/// Represents a user-defined goal for tracking activity completion or improvement.
///
/// # Goal Target (Mutually Exclusive)
///
/// Goals must target either:
/// - A specific `activity_id` (e.g., "Exercise 3 times per week")
/// - An entire `group_id` (e.g., "Do any social activity 5 times per week")
///
/// Setting both or neither is invalid and enforced by:
/// - Database CHECK constraint: `NOT (activity_id IS NOT NULL AND group_id IS NOT NULL)`
/// - Validation layer: `validate_goal_target_exclusivity()` function
///
/// # Goal Types
///
/// - `"days_per_period"`: Track frequency of activity within a time period
///   - Example: "Exercise 3 days per 7-day period"
///   - `target_value`: number of days activity should be performed
///   - `period_days`: rolling window size in days
///
/// - `"percent_improvement"`: Track improvement over baseline period
///   - Example: "Increase meditation by 20% over 30-day baseline"
///   - `target_value`: percentage improvement (e.g., 20 = 20%)
///   - `period_days`: baseline comparison period in days
///
/// # Soft Deletes
///
/// Goals use soft delete pattern via `deleted_at` timestamp, allowing:
/// - Historical goal tracking and analysis
/// - Recovery of accidentally deleted goals
/// - Audit trail of goal changes over time
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ActivityGoal {
    pub id: i32,
    /// ID of specific activity this goal targets (mutually exclusive with group_id)
    pub activity_id: Option<i32>,
    /// ID of activity group this goal targets (mutually exclusive with activity_id)
    pub group_id: Option<i32>,
    /// Type of goal: 'days_per_period' or 'percent_improvement'
    pub goal_type: String,
    /// Target value: days count for 'days_per_period', percentage for 'percent_improvement'
    pub target_value: i32,
    /// Time period in days for goal measurement or baseline comparison
    pub period_days: i32,
    /// ISO 8601 timestamp when goal was created
    pub created_at: String,
    /// ISO 8601 timestamp when goal was soft-deleted (None if active)
    pub deleted_at: Option<String>,
}

/// Request to create an activity group
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
pub struct CreateActivityGroupRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

/// Request to update an activity group
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
pub struct UpdateActivityGroupRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

/// Request to create an activity
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
pub struct CreateActivityRequest {
    pub group_id: i32,
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub color: Option<String>,
    #[validate(custom(function = "validate_optional_icon"))]
    pub icon: Option<String>,
}

/// Request to update an activity
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
pub struct UpdateActivityRequest {
    pub group_id: Option<i32>,
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,
    pub color: Option<String>,
    #[validate(custom(function = "validate_optional_icon"))]
    pub icon: Option<String>,
}

/// Request to log an activity
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
pub struct LogActivityRequest {
    pub activity_id: i32,
    pub logged_at: Option<String>, // ISO 8601 timestamp, defaults to now
    #[validate(length(max = 500))]
    pub notes: Option<String>,
}

/// Request to set an activity goal
#[derive(Debug, Serialize, Deserialize, specta::Type, Validate)]
#[validate(schema(function = "validate_goal_target_exclusivity"))]
pub struct SetActivityGoalRequest {
    pub activity_id: Option<i32>,
    pub group_id: Option<i32>,
    #[validate(custom(function = "validate_goal_type"))]
    pub goal_type: String, // 'days_per_period' or 'percent_improvement'
    #[validate(range(min = 1))]
    pub target_value: i32,
    #[validate(range(min = 1))]
    pub period_days: i32,
}

/// Custom validator for goal_type field
fn validate_goal_type(goal_type: &str) -> Result<(), validator::ValidationError> {
    match goal_type {
        "days_per_period" | "percent_improvement" => Ok(()),
        _ => {
            let mut error = validator::ValidationError::new("invalid_goal_type");
            error.message = Some(std::borrow::Cow::from(
                "Goal type must be 'days_per_period' or 'percent_improvement'",
            ));
            Err(error)
        }
    }
}

/// Custom schema validator to ensure activity_id and group_id are mutually exclusive
fn validate_goal_target_exclusivity(
    request: &SetActivityGoalRequest,
) -> Result<(), validator::ValidationError> {
    match (&request.activity_id, &request.group_id) {
        (Some(_), Some(_)) => {
            let mut error = validator::ValidationError::new("invalid_goal_target");
            error.message = Some(std::borrow::Cow::from(
                "Goal must target either an activity OR a group, not both",
            ));
            Err(error)
        }
        (None, None) => {
            let mut error = validator::ValidationError::new("missing_goal_target");
            error.message = Some(std::borrow::Cow::from(
                "Goal must target either an activity or a group",
            ));
            Err(error)
        }
        _ => Ok(()),
    }
}

/// Custom validator for optional icon field to prevent Some("") (empty string wrapped in Some)
fn validate_optional_icon(icon: &str) -> Result<(), validator::ValidationError> {
    if icon.is_empty() {
        let mut error = validator::ValidationError::new("empty_icon");
        error.message = Some(std::borrow::Cow::from(
            "Icon cannot be an empty string. Use None instead.",
        ));
        return Err(error);
    }
    if icon.len() > 20 {
        let mut error = validator::ValidationError::new("icon_too_long");
        error.message = Some(std::borrow::Cow::from("Icon must be 20 characters or less"));
        return Err(error);
    }
    Ok(())
}
