// Scheduling commands (User Story 6)
// T165-T167: Tauri commands for schedule management

use tauri::State;

use crate::{AppState, CommandError};

use super::models::{AssessmentSchedule, CreateScheduleRequest, UpdateScheduleRequest};
use super::repository::SchedulingRepository;
use super::repository_trait::SchedulingRepositoryTrait;

/// T165: Create a new assessment schedule
#[tauri::command]
#[specta::specta]
pub fn create_schedule(
    request: CreateScheduleRequest,
    state: State<AppState>,
) -> Result<AssessmentSchedule, CommandError> {
    let repo = SchedulingRepository::new(state.db.clone());
    create_schedule_impl(&repo, request)
}

/// Business logic for creating schedule - uses trait bound for testability
fn create_schedule_impl(
    repo: &impl SchedulingRepositoryTrait,
    request: CreateScheduleRequest,
) -> Result<AssessmentSchedule, CommandError> {
    repo.create_schedule(request)
        .map_err(|e| e.to_command_error())
}

/// T166: Update an existing schedule
#[tauri::command]
#[specta::specta]
pub fn update_schedule(
    id: i32,
    request: UpdateScheduleRequest,
    state: State<AppState>,
) -> Result<AssessmentSchedule, CommandError> {
    let repo = SchedulingRepository::new(state.db.clone());
    update_schedule_impl(&repo, id, request)
}

/// Business logic for updating schedule - uses trait bound for testability
fn update_schedule_impl(
    repo: &impl SchedulingRepositoryTrait,
    id: i32,
    request: UpdateScheduleRequest,
) -> Result<AssessmentSchedule, CommandError> {
    repo.update_schedule(id, request)
        .map_err(|e| e.to_command_error())
}

/// T167: Delete a schedule
#[tauri::command]
#[specta::specta]
pub fn delete_schedule(id: i32, state: State<AppState>) -> Result<(), CommandError> {
    let repo = SchedulingRepository::new(state.db.clone());
    delete_schedule_impl(&repo, id)
}

/// Business logic for deleting schedule - uses trait bound for testability
fn delete_schedule_impl(
    repo: &impl SchedulingRepositoryTrait,
    id: i32,
) -> Result<(), CommandError> {
    repo.delete_schedule(id).map_err(|e| e.to_command_error())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::scheduling::{
        repository_trait::MockSchedulingRepositoryTrait, ScheduleFrequency, SchedulingError,
        SchedulingRepositoryTrait,
    };

    // ========================================================================
    // Unit Tests: Command Validation
    // ========================================================================

    #[test]
    fn test_create_schedule_request_validation_valid_time() {
        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_schedule_request_validation_invalid_time_format() {
        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "9:00".to_string(), // Invalid: needs HH:MM
            day_of_week: None,
            day_of_month: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_schedule_request_validation_invalid_time_hour() {
        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "25:00".to_string(), // Hour > 23
            day_of_week: None,
            day_of_month: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_schedule_request_validation_day_of_week_in_range() {
        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Weekly,
            time_of_day: "09:00".to_string(),
            day_of_week: Some(3), // Wednesday
            day_of_month: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_schedule_request_validation_day_of_week_out_of_range() {
        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Weekly,
            time_of_day: "09:00".to_string(),
            day_of_week: Some(7), // Invalid: 0-6 only
            day_of_month: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_schedule_request_validation_day_of_month_in_range() {
        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Monthly,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: Some(15),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_schedule_request_validation_day_of_month_out_of_range() {
        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Monthly,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: Some(32), // Invalid: 1-31 only
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_schedule_request_validation_weekly_requires_day_of_week() {
        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Weekly,
            time_of_day: "09:00".to_string(),
            day_of_week: None, // Missing required field
            day_of_month: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_schedule_request_validation_monthly_requires_day_of_month() {
        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Monthly,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: None, // Missing required field
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_schedule_request_validation_valid() {
        let request = UpdateScheduleRequest {
            frequency: Some(ScheduleFrequency::Daily),
            time_of_day: Some("14:30".to_string()),
            day_of_week: Some(2),
            day_of_month: Some(10),
            enabled: Some(false),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_schedule_request_validation_invalid_time() {
        let request = UpdateScheduleRequest {
            frequency: None,
            time_of_day: Some("25:00".to_string()), // Invalid hour
            day_of_week: None,
            day_of_month: None,
            enabled: None,
        };

        assert!(request.validate().is_err());
    }

    // ========================================================================
    // Unit Tests: Error Message Formatting
    // ========================================================================

    /// Helper function to simulate the create_schedule command logic
    fn create_schedule_with_trait(
        repo: &dyn SchedulingRepositoryTrait,
        request: CreateScheduleRequest,
    ) -> Result<AssessmentSchedule, String> {
        repo.create_schedule(request)
            .map_err(|e| format!("Failed to create schedule: {}", e))
    }

    #[test]
    fn test_error_message_formatting_invalid_time_format() {
        let mut mock_repo = MockSchedulingRepositoryTrait::new();

        mock_repo
            .expect_create_schedule()
            .returning(|_| Err(SchedulingError::InvalidTimeFormat("25:00".to_string())));

        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "25:00".to_string(),
            day_of_week: None,
            day_of_month: None,
        };

        let result = create_schedule_with_trait(&mock_repo, request);

        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("Invalid time format: 25:00"));
    }

    #[test]
    fn test_error_message_formatting_not_found() {
        let mut mock_repo = MockSchedulingRepositoryTrait::new();

        mock_repo
            .expect_create_schedule()
            .returning(|_| Err(SchedulingError::NotFound(999)));

        let request = CreateScheduleRequest {
            assessment_type_id: 999,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: None,
        };

        let result = create_schedule_with_trait(&mock_repo, request);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Schedule not found: 999"));
    }

    /// Helper function to simulate the update_schedule command logic
    fn update_schedule_with_trait(
        repo: &dyn SchedulingRepositoryTrait,
        id: i32,
        request: UpdateScheduleRequest,
    ) -> Result<AssessmentSchedule, String> {
        repo.update_schedule(id, request)
            .map_err(|e| format!("Failed to update schedule: {}", e))
    }

    #[test]
    fn test_error_message_formatting_update_not_found() {
        let mut mock_repo = MockSchedulingRepositoryTrait::new();

        mock_repo
            .expect_update_schedule()
            .returning(|_, _| Err(SchedulingError::NotFound(999)));

        let request = UpdateScheduleRequest {
            frequency: Some(ScheduleFrequency::Weekly),
            time_of_day: None,
            day_of_week: Some(1),
            day_of_month: None,
            enabled: None,
        };

        let result = update_schedule_with_trait(&mock_repo, 999, request);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Schedule not found: 999"));
    }

    // ========================================================================
    // Unit Tests: Error Propagation and Conversion
    // ========================================================================

    /// Helper for delete_schedule command logic
    fn delete_schedule_with_trait(
        repo: &dyn SchedulingRepositoryTrait,
        id: i32,
    ) -> Result<(), String> {
        repo.delete_schedule(id)
            .map_err(|e| format!("Failed to delete schedule: {}", e))
    }

    #[test]
    fn test_delete_schedule_not_found() {
        let mut mock_repo = MockSchedulingRepositoryTrait::new();

        mock_repo
            .expect_delete_schedule()
            .with(mockall::predicate::eq(999))
            .returning(|_| Err(SchedulingError::NotFound(999)));

        let result = delete_schedule_with_trait(&mock_repo, 999);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Schedule not found: 999"));
    }

    #[test]
    fn test_delete_schedule_success() {
        let mut mock_repo = MockSchedulingRepositoryTrait::new();

        mock_repo
            .expect_delete_schedule()
            .with(mockall::predicate::eq(123))
            .returning(|_| Ok(()));

        let result = delete_schedule_with_trait(&mock_repo, 123);

        assert!(result.is_ok());
    }

    #[test]
    fn test_create_schedule_database_error() {
        let mut mock_repo = MockSchedulingRepositoryTrait::new();

        mock_repo
            .expect_create_schedule()
            .returning(|_| Err(SchedulingError::Database(rusqlite::Error::InvalidQuery)));

        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: None,
        };

        let result = create_schedule_with_trait(&mock_repo, request);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Database error"));
    }

    // ========================================================================
    // Unit Tests: Conditional Logic (Different Frequencies)
    // ========================================================================

    #[test]
    fn test_daily_schedule_no_day_requirements() {
        let request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: None,
        };

        // Daily schedule should not require day_of_week or day_of_month
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_weekly_schedule_requires_day_of_week() {
        let valid_request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Weekly,
            time_of_day: "09:00".to_string(),
            day_of_week: Some(1), // Monday
            day_of_month: None,
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Weekly,
            time_of_day: "09:00".to_string(),
            day_of_week: None, // Missing required field
            day_of_month: None,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_monthly_schedule_requires_day_of_month() {
        let valid_request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Monthly,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: Some(15), // 15th of month
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Monthly,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: None, // Missing required field
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_biweekly_schedule_requires_day_of_week() {
        let valid_request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Biweekly,
            time_of_day: "09:00".to_string(),
            day_of_week: Some(3), // Wednesday
            day_of_month: None,
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Biweekly,
            time_of_day: "09:00".to_string(),
            day_of_week: None, // Missing required field
            day_of_month: None,
        };
        assert!(invalid_request.validate().is_err());
    }

    // ========================================================================
    // Unit Tests: Input Sanitization (Edge Cases)
    // ========================================================================

    #[test]
    fn test_time_format_edge_cases() {
        // Valid edge cases
        let midnight_request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "00:00".to_string(),
            day_of_week: None,
            day_of_month: None,
        };
        assert!(midnight_request.validate().is_ok());

        let last_minute_request = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "23:59".to_string(),
            day_of_week: None,
            day_of_month: None,
        };
        assert!(last_minute_request.validate().is_ok());

        // Invalid edge cases
        let invalid_hour = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "24:00".to_string(),
            day_of_week: None,
            day_of_month: None,
        };
        assert!(invalid_hour.validate().is_err());

        let invalid_minute = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Daily,
            time_of_day: "09:60".to_string(),
            day_of_week: None,
            day_of_month: None,
        };
        assert!(invalid_minute.validate().is_err());
    }

    #[test]
    fn test_day_boundary_values() {
        // day_of_week boundaries (0-6)
        let sunday = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Weekly,
            time_of_day: "09:00".to_string(),
            day_of_week: Some(0), // Sunday
            day_of_month: None,
        };
        assert!(sunday.validate().is_ok());

        let saturday = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Weekly,
            time_of_day: "09:00".to_string(),
            day_of_week: Some(6), // Saturday
            day_of_month: None,
        };
        assert!(saturday.validate().is_ok());

        // day_of_month boundaries (1-31)
        let first_day = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Monthly,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: Some(1),
        };
        assert!(first_day.validate().is_ok());

        let last_day = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Monthly,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: Some(31),
        };
        assert!(last_day.validate().is_ok());

        // Out of range
        let invalid_day_of_month = CreateScheduleRequest {
            assessment_type_id: 1,
            frequency: ScheduleFrequency::Monthly,
            time_of_day: "09:00".to_string(),
            day_of_week: None,
            day_of_month: Some(0), // Invalid: must be 1-31
        };
        assert!(invalid_day_of_month.validate().is_err());
    }
}
