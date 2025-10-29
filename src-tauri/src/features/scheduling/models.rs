// Scheduling feature models (User Story 6)
// T156-T159: Models for assessment scheduling

use crate::CommandError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;
use validator::Validate;

/// Scheduling-specific errors
#[derive(Error, Debug)]
pub enum SchedulingError {
    #[error("Invalid time format: {0}")]
    InvalidTimeFormat(String),

    #[error("Invalid frequency: {0}")]
    InvalidFrequency(String),

    #[error("Schedule not found: {0}")]
    NotFound(i32),

    #[error("Invalid day of week: {0}. Must be 0-6 (Sunday-Saturday)")]
    InvalidDayOfWeek(i32),

    #[error("Invalid day of month: {0}. Must be 1-31")]
    InvalidDayOfMonth(i32),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Lock poisoned - database in inconsistent state")]
    LockPoisoned,

    #[error("Date parsing error: {0}")]
    DateParseError(String),
}

impl SchedulingError {
    /// Convert to structured CommandError for frontend consumption
    pub fn to_command_error(&self) -> CommandError {
        match self {
            // Validation errors - not retryable
            SchedulingError::InvalidTimeFormat(time) => {
                CommandError::permanent(self.to_string(), "validation").with_details(
                    serde_json::json!({
                        "field": "time_of_day",
                        "value": time
                    }),
                )
            }
            SchedulingError::InvalidFrequency(freq) => {
                CommandError::permanent(self.to_string(), "validation").with_details(
                    serde_json::json!({
                        "field": "frequency",
                        "value": freq
                    }),
                )
            }
            SchedulingError::InvalidDayOfWeek(day) => {
                CommandError::permanent(self.to_string(), "validation").with_details(
                    serde_json::json!({
                        "field": "day_of_week",
                        "value": day
                    }),
                )
            }
            SchedulingError::InvalidDayOfMonth(day) => {
                CommandError::permanent(self.to_string(), "validation").with_details(
                    serde_json::json!({
                        "field": "day_of_month",
                        "value": day
                    }),
                )
            }
            SchedulingError::DateParseError(msg) => {
                CommandError::permanent(self.to_string(), "validation").with_details(
                    serde_json::json!({
                        "details": msg
                    }),
                )
            }

            // Not found errors - not retryable
            SchedulingError::NotFound(id) => CommandError::permanent(self.to_string(), "not_found")
                .with_details(serde_json::json!({
                    "resource": "schedule",
                    "id": id
                })),

            // Database lock/transient errors - retryable
            SchedulingError::LockPoisoned => {
                CommandError::retryable(self.to_string(), "lock_poisoned")
            }
            SchedulingError::Database(e) => {
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

/// Schedule frequency options
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ScheduleFrequency {
    Daily,
    Weekly,
    Biweekly,
    Monthly,
}

impl ScheduleFrequency {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScheduleFrequency::Daily => "daily",
            ScheduleFrequency::Weekly => "weekly",
            ScheduleFrequency::Biweekly => "biweekly",
            ScheduleFrequency::Monthly => "monthly",
        }
    }
}

impl FromStr for ScheduleFrequency {
    type Err = SchedulingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "daily" => Ok(ScheduleFrequency::Daily),
            "weekly" => Ok(ScheduleFrequency::Weekly),
            "biweekly" => Ok(ScheduleFrequency::Biweekly),
            "monthly" => Ok(ScheduleFrequency::Monthly),
            _ => Err(SchedulingError::InvalidFrequency(s.to_string())),
        }
    }
}

/// Assessment schedule configuration
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug)]
pub struct AssessmentSchedule {
    pub id: i32,
    pub assessment_type_id: i32,
    pub assessment_type_code: String, // Denormalized for convenience
    pub assessment_type_name: String, // Denormalized for convenience
    pub frequency: ScheduleFrequency,
    pub time_of_day: String,       // HH:MM format (e.g., "09:00")
    pub day_of_week: Option<i32>,  // 0-6 (Sunday-Saturday) for weekly/biweekly
    pub day_of_month: Option<i32>, // 1-31 for monthly
    pub enabled: bool,
    pub last_triggered_at: Option<String>, // ISO 8601 timestamp
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create a new schedule
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug, Validate)]
pub struct CreateScheduleRequest {
    pub assessment_type_id: i32,
    pub frequency: ScheduleFrequency,
    #[validate(custom(function = "validate_time_format"))]
    pub time_of_day: String, // HH:MM format
    #[validate(range(min = 0, max = 6))]
    pub day_of_week: Option<i32>, // Required for weekly/biweekly
    #[validate(range(min = 1, max = 31))]
    pub day_of_month: Option<i32>, // Required for monthly
}

impl CreateScheduleRequest {
    /// Validate the schedule request
    /// Combines validator crate's derive validation with custom frequency logic
    pub fn validate(&self) -> Result<(), SchedulingError> {
        // First, run validator crate's derive validations (range checks, custom validators)
        
        <Self as validator::Validate>::validate(self).map_err(|e| {
            // Convert validator::ValidationErrors to SchedulingError
            let errors = e.field_errors();
            if let Some(time_errors) = errors.get("time_of_day") {
                if !time_errors.is_empty() {
                    return SchedulingError::InvalidTimeFormat(self.time_of_day.clone());
                }
            }
            if let Some(day_week_errors) = errors.get("day_of_week") {
                if !day_week_errors.is_empty() {
                    if let Some(day) = self.day_of_week {
                        return SchedulingError::InvalidDayOfWeek(day);
                    }
                }
            }
            if let Some(day_month_errors) = errors.get("day_of_month") {
                if !day_month_errors.is_empty() {
                    if let Some(day) = self.day_of_month {
                        return SchedulingError::InvalidDayOfMonth(day);
                    }
                }
            }
            // Fallback for any other validation errors
            SchedulingError::InvalidTimeFormat(format!("Validation failed: {}", e))
        })?;

        // Then run custom frequency-specific requirement validation
        // Range validation already handled by validator crate above
        // We only need to check frequency-specific requirements (e.g., weekly needs day_of_week)
        match self.frequency {
            ScheduleFrequency::Weekly | ScheduleFrequency::Biweekly => {
                if self.day_of_week.is_none() {
                    return Err(SchedulingError::InvalidFrequency(
                        "day_of_week required for weekly/biweekly schedules".to_string(),
                    ));
                }
            }
            ScheduleFrequency::Monthly => {
                if self.day_of_month.is_none() {
                    return Err(SchedulingError::InvalidFrequency(
                        "day_of_month required for monthly schedules".to_string(),
                    ));
                }
            }
            ScheduleFrequency::Daily => {
                // No additional validation needed for daily
            }
        }

        Ok(())
    }
}

/// Request to update an existing schedule
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug, Validate)]
pub struct UpdateScheduleRequest {
    pub frequency: Option<ScheduleFrequency>,
    #[validate(custom(function = "validate_time_format"))]
    pub time_of_day: Option<String>,
    #[validate(range(min = 0, max = 6))]
    pub day_of_week: Option<i32>,
    #[validate(range(min = 1, max = 31))]
    pub day_of_month: Option<i32>,
    pub enabled: Option<bool>,
}

impl UpdateScheduleRequest {
    /// Validate the update request
    /// Combines validator crate's derive validation with custom logic
    pub fn validate(&self) -> Result<(), SchedulingError> {
        // First, run validator crate's derive validations (range checks, custom validators)
        
        <Self as validator::Validate>::validate(self).map_err(|e| {
            // Convert validator::ValidationErrors to SchedulingError
            let errors = e.field_errors();
            if let Some(time_errors) = errors.get("time_of_day") {
                if !time_errors.is_empty() {
                    if let Some(ref time) = self.time_of_day {
                        return SchedulingError::InvalidTimeFormat(time.clone());
                    }
                }
            }
            if let Some(day_week_errors) = errors.get("day_of_week") {
                if !day_week_errors.is_empty() {
                    if let Some(day) = self.day_of_week {
                        return SchedulingError::InvalidDayOfWeek(day);
                    }
                }
            }
            if let Some(day_month_errors) = errors.get("day_of_month") {
                if !day_month_errors.is_empty() {
                    if let Some(day) = self.day_of_month {
                        return SchedulingError::InvalidDayOfMonth(day);
                    }
                }
            }
            // Fallback for any other validation errors
            SchedulingError::InvalidFrequency(format!("Validation failed: {}", e))
        })?;

        // No additional custom validation needed for update requests
        // Range validation already handled by validator crate above
        Ok(())
    }
}

/// Helper function to validate time format (HH:MM)
fn is_valid_time_format(time: &str) -> bool {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 {
        return false;
    }

    // Check for HH:MM format (exactly 2 digits for hour and minute)
    if parts[0].len() != 2 || parts[1].len() != 2 {
        return false;
    }

    let hour: Result<u32, _> = parts[0].parse();
    let minute: Result<u32, _> = parts[1].parse();

    matches!((hour, minute), (Ok(h), Ok(m)) if h < 24 && m < 60)
}

/// Custom validator for time format (for use with validator crate)
fn validate_time_format(time: &str) -> Result<(), validator::ValidationError> {
    if !is_valid_time_format(time) {
        let mut error = validator::ValidationError::new("time_format");
        error.message = Some(std::borrow::Cow::from(
            "Must be in HH:MM format (e.g., 09:00)",
        ));
        return Err(error);
    }
    Ok(())
}

/// Calculate next trigger time for a schedule
pub fn calculate_next_trigger(
    schedule: &AssessmentSchedule,
    from: DateTime<Utc>,
) -> Result<DateTime<Utc>, SchedulingError> {
    use chrono::{Datelike, Duration, NaiveTime};

    // Parse time_of_day
    let parts: Vec<&str> = schedule.time_of_day.split(':').collect();
    let hour: u32 = parts[0]
        .parse()
        .map_err(|_| SchedulingError::InvalidTimeFormat(schedule.time_of_day.clone()))?;
    let minute: u32 = parts[1]
        .parse()
        .map_err(|_| SchedulingError::InvalidTimeFormat(schedule.time_of_day.clone()))?;

    let target_time = NaiveTime::from_hms_opt(hour, minute, 0)
        .ok_or_else(|| SchedulingError::InvalidTimeFormat(schedule.time_of_day.clone()))?;

    let next_trigger = match schedule.frequency {
        ScheduleFrequency::Daily => {
            // Set to target time today
            let mut trigger = from.date_naive().and_time(target_time).and_utc();

            // If time has passed today, move to tomorrow
            if trigger <= from {
                trigger += Duration::days(1);
            }
            trigger
        }
        ScheduleFrequency::Weekly => {
            let target_day = schedule.day_of_week.ok_or_else(|| {
                SchedulingError::InvalidFrequency(
                    "day_of_week missing for weekly schedule".to_string(),
                )
            })?;

            let current_day = from.weekday().num_days_from_sunday() as i32;
            let days_until_target = (target_day - current_day + 7) % 7;

            let mut trigger = (from + Duration::days(days_until_target as i64))
                .date_naive()
                .and_time(target_time)
                .and_utc();

            // If same day but time passed, move to next week
            if days_until_target == 0 && trigger <= from {
                trigger += Duration::weeks(1);
            }
            trigger
        }
        ScheduleFrequency::Biweekly => {
            // BIWEEKLY SCHEDULING ALGORITHM
            //
            // For biweekly schedules, we must maintain strict 14-day intervals to prevent
            // "drift" where the schedule would trigger every occurrence of the target weekday
            // instead of maintaining proper 2-week spacing.
            //
            // Example problem without this logic:
            //   Schedule: Every other Monday at 9:00 AM
            //   Last trigger: Monday, Oct 1
            //   Without proper tracking: Would trigger again on Oct 8 (only 7 days later!)
            //   With proper tracking: Correctly triggers on Oct 15 (14 days later)

            let target_day = schedule.day_of_week.ok_or_else(|| {
                SchedulingError::InvalidFrequency(
                    "day_of_week missing for biweekly schedule".to_string(),
                )
            })?;

            let trigger = if let Some(last_triggered) = &schedule.last_triggered_at {
                // RECURRING SCHEDULE: Calculate from last trigger to ensure 14-day intervals
                // This prevents "drift" where the schedule would trigger every occurrence of
                // the target weekday instead of maintaining strict 2-week spacing.
                let last_trigger_dt = chrono::DateTime::parse_from_rfc3339(last_triggered)
                    .map_err(|e| {
                        SchedulingError::DateParseError(format!(
                            "Failed to parse last_triggered_at: {}",
                            e
                        ))
                    })?
                    .with_timezone(&chrono::Utc);
                let mut t = last_trigger_dt + Duration::weeks(2);

                // CATCH-UP LOGIC: If calculated trigger is in the past (e.g., app was offline
                // for multiple weeks), keep adding 2-week intervals until we reach a future date.
                // This ensures we don't miss multiple triggers or create notification spam.
                while t <= from {
                    t += Duration::weeks(2);
                }
                t
            } else {
                // FIRST-TIME SCHEDULE: Find next occurrence of target weekday
                // Subsequent triggers will use the path above to maintain 14-day spacing.
                let current_day = from.weekday().num_days_from_sunday() as i32;
                let days_until_target = (target_day - current_day + 7) % 7;

                let mut t = (from + Duration::days(days_until_target as i64))
                    .date_naive()
                    .and_time(target_time)
                    .and_utc();

                // If target day is today but time has passed, move to next week
                if days_until_target == 0 && t <= from {
                    t += Duration::weeks(1);
                }
                t
            };
            trigger
        }
        ScheduleFrequency::Monthly => {
            let target_day = schedule.day_of_month.ok_or_else(|| {
                SchedulingError::InvalidFrequency(
                    "day_of_month missing for monthly schedule".to_string(),
                )
            })?;

            let current_day = from.day() as i32;

            // Try current month first
            let mut year = from.year();
            let mut month = from.month();

            if target_day < current_day || (target_day == current_day && from.time() >= target_time)
            {
                // Move to next month
                month += 1;
                if month > 12 {
                    month = 1;
                    year += 1;
                }
            }

            // Handle months with fewer days (e.g., February, April)
            let day = target_day.min(days_in_month(year, month)? as i32);

            chrono::NaiveDate::from_ymd_opt(year, month, day as u32)
                .ok_or_else(|| SchedulingError::DateParseError("Invalid date".to_string()))?
                .and_time(target_time)
                .and_utc()
        }
    };

    Ok(next_trigger)
}

/// Get number of days in a month
fn days_in_month(year: i32, month: u32) -> Result<u32, SchedulingError> {
    use chrono::NaiveDate;

    let first_of_next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };

    let first_of_month = NaiveDate::from_ymd_opt(year, month, 1);

    match (first_of_next_month, first_of_month) {
        (Some(next), Some(current)) => Ok(next.signed_duration_since(current).num_days() as u32),
        _ => Err(SchedulingError::DateParseError(format!(
            "Invalid year/month combination: year={}, month={}",
            year, month
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_time_format() {
        assert!(is_valid_time_format("09:00"));
        assert!(is_valid_time_format("00:00"));
        assert!(is_valid_time_format("23:59"));
        assert!(!is_valid_time_format("24:00"));
        assert!(!is_valid_time_format("09:60"));
        assert!(!is_valid_time_format("9:00"));
        assert!(!is_valid_time_format("09:0"));
    }

    #[test]
    fn test_create_schedule_request_validation() {
        // Valid daily schedule
        let req = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: None,
        };
        assert!(req.validate().is_ok());

        // Invalid time format
        let req = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "25:00".to_string(),
            day_of_week: None,
            day_of_month: None,
        };
        assert!(req.validate().is_err());

        // Weekly without day_of_week
        let req = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Weekly,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: None,
        };
        assert!(req.validate().is_err());

        // Monthly with valid day
        let req = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Monthly,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: Some(15),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_schedule_frequency_conversion() {
        assert_eq!(
            ScheduleFrequency::from_str("daily").unwrap(),
            ScheduleFrequency::Daily
        );
        assert_eq!(
            ScheduleFrequency::from_str("WEEKLY").unwrap(),
            ScheduleFrequency::Weekly
        );
        assert!(ScheduleFrequency::from_str("invalid").is_err());
    }
}
