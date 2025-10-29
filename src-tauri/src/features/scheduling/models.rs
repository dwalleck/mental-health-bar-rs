// Scheduling feature models (User Story 6)
// T156-T159: Models for assessment scheduling

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

    pub fn from_str(s: &str) -> Result<Self, SchedulingError> {
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
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug)]
pub struct CreateScheduleRequest {
    pub assessment_type_id: i32,
    pub frequency: ScheduleFrequency,
    pub time_of_day: String,       // HH:MM format
    pub day_of_week: Option<i32>,  // Required for weekly/biweekly
    pub day_of_month: Option<i32>, // Required for monthly
}

impl CreateScheduleRequest {
    /// Validate the schedule request
    pub fn validate(&self) -> Result<(), SchedulingError> {
        // Validate time format (HH:MM)
        if !is_valid_time_format(&self.time_of_day) {
            return Err(SchedulingError::InvalidTimeFormat(self.time_of_day.clone()));
        }

        // Validate frequency-specific requirements
        match self.frequency {
            ScheduleFrequency::Weekly | ScheduleFrequency::Biweekly => {
                if let Some(day) = self.day_of_week {
                    if !(0..=6).contains(&day) {
                        return Err(SchedulingError::InvalidDayOfWeek(day));
                    }
                } else {
                    return Err(SchedulingError::InvalidFrequency(
                        "day_of_week required for weekly/biweekly schedules".to_string(),
                    ));
                }
            }
            ScheduleFrequency::Monthly => {
                if let Some(day) = self.day_of_month {
                    if !(1..=31).contains(&day) {
                        return Err(SchedulingError::InvalidDayOfMonth(day));
                    }
                } else {
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
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug)]
pub struct UpdateScheduleRequest {
    pub frequency: Option<ScheduleFrequency>,
    pub time_of_day: Option<String>,
    pub day_of_week: Option<i32>,
    pub day_of_month: Option<i32>,
    pub enabled: Option<bool>,
}

impl UpdateScheduleRequest {
    /// Validate the update request
    pub fn validate(&self) -> Result<(), SchedulingError> {
        // Validate time format if provided
        if let Some(ref time) = self.time_of_day {
            if !is_valid_time_format(time) {
                return Err(SchedulingError::InvalidTimeFormat(time.clone()));
            }
        }

        // Validate day_of_week if provided
        if let Some(day) = self.day_of_week {
            if !(0..=6).contains(&day) {
                return Err(SchedulingError::InvalidDayOfWeek(day));
            }
        }

        // Validate day_of_month if provided
        if let Some(day) = self.day_of_month {
            if !(1..=31).contains(&day) {
                return Err(SchedulingError::InvalidDayOfMonth(day));
            }
        }

        Ok(())
    }
}

/// Helper function to validate time format (HH:MM)
fn is_valid_time_format(time: &str) -> bool {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 {
        return false;
    }

    let hour: Result<u32, _> = parts[0].parse();
    let minute: Result<u32, _> = parts[1].parse();

    matches!((hour, minute), (Ok(h), Ok(m)) if h < 24 && m < 60)
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

    let mut next_trigger = from;

    match schedule.frequency {
        ScheduleFrequency::Daily => {
            // Set to target time today
            next_trigger = from.date_naive().and_time(target_time).and_utc();

            // If time has passed today, move to tomorrow
            if next_trigger <= from {
                next_trigger += Duration::days(1);
            }
        }
        ScheduleFrequency::Weekly => {
            let target_day = schedule.day_of_week.ok_or_else(|| {
                SchedulingError::InvalidFrequency(
                    "day_of_week missing for weekly schedule".to_string(),
                )
            })?;

            let current_day = from.weekday().num_days_from_sunday() as i32;
            let days_until_target = (target_day - current_day + 7) % 7;

            next_trigger = (from + Duration::days(days_until_target as i64))
                .date_naive()
                .and_time(target_time)
                .and_utc();

            // If same day but time passed, move to next week
            if days_until_target == 0 && next_trigger <= from {
                next_trigger += Duration::weeks(1);
            }
        }
        ScheduleFrequency::Biweekly => {
            // Similar to weekly but 2 weeks interval
            let target_day = schedule.day_of_week.ok_or_else(|| {
                SchedulingError::InvalidFrequency(
                    "day_of_week missing for biweekly schedule".to_string(),
                )
            })?;

            let current_day = from.weekday().num_days_from_sunday() as i32;
            let days_until_target = (target_day - current_day + 7) % 7;

            next_trigger = (from + Duration::days(days_until_target as i64))
                .date_naive()
                .and_time(target_time)
                .and_utc();

            if days_until_target == 0 && next_trigger <= from {
                next_trigger += Duration::weeks(2);
            }
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
            let day = target_day.min(days_in_month(year, month) as i32);

            next_trigger = chrono::NaiveDate::from_ymd_opt(year, month, day as u32)
                .ok_or_else(|| SchedulingError::DateParseError("Invalid date".to_string()))?
                .and_time(target_time)
                .and_utc();
        }
    }

    Ok(next_trigger)
}

/// Get number of days in a month
fn days_in_month(year: i32, month: u32) -> u32 {
    use chrono::NaiveDate;

    if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .unwrap()
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days() as u32
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
