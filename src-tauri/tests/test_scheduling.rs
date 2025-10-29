// Integration tests for scheduling feature
// Tests repository CRUD operations, validation, and due schedule logic
use std::str::FromStr;
use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::scheduling::models::*;
use tauri_sveltekit_modern_lib::features::scheduling::repository::SchedulingRepository;
use tempfile::TempDir;

fn setup_test_repo() -> (SchedulingRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().to_path_buf();
    let db = Arc::new(Database::new(db_path).expect("Failed to create database"));

    let repo = SchedulingRepository::new(db);

    (repo, temp_dir)
}

// ============================================================================
// CREATE SCHEDULE TESTS
// ============================================================================

#[test]
fn test_create_schedule_daily() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1, // PHQ-9
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create daily schedule");

    assert_eq!(schedule.assessment_type_id, 1);
    assert_eq!(schedule.assessment_type_code, "PHQ9");
    assert_eq!(schedule.frequency, ScheduleFrequency::Daily);
    assert_eq!(schedule.time_of_day, "09:00");
    assert_eq!(schedule.day_of_week, None);
    assert_eq!(schedule.day_of_month, None);
    assert!(schedule.enabled);
    assert_eq!(schedule.last_triggered_at, None);
}

#[test]
fn test_create_schedule_weekly() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 2, // GAD-7
        frequency: ScheduleFrequency::Weekly,
        time_of_day: "14:30".to_string(),
        day_of_week: Some(3), // Wednesday
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create weekly schedule");

    assert_eq!(schedule.assessment_type_code, "GAD7");
    assert_eq!(schedule.frequency, ScheduleFrequency::Weekly);
    assert_eq!(schedule.time_of_day, "14:30");
    assert_eq!(schedule.day_of_week, Some(3));
    assert_eq!(schedule.day_of_month, None);
}

#[test]
fn test_create_schedule_biweekly() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 3, // CESD
        frequency: ScheduleFrequency::Biweekly,
        time_of_day: "10:00".to_string(),
        day_of_week: Some(1), // Monday
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create biweekly schedule");

    assert_eq!(schedule.assessment_type_code, "CESD");
    assert_eq!(schedule.frequency, ScheduleFrequency::Biweekly);
    assert_eq!(schedule.day_of_week, Some(1));
}

#[test]
fn test_create_schedule_monthly() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 4, // OASIS
        frequency: ScheduleFrequency::Monthly,
        time_of_day: "08:00".to_string(),
        day_of_week: None,
        day_of_month: Some(15),
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create monthly schedule");

    assert_eq!(schedule.assessment_type_code, "OASIS");
    assert_eq!(schedule.frequency, ScheduleFrequency::Monthly);
    assert_eq!(schedule.day_of_week, None);
    assert_eq!(schedule.day_of_month, Some(15));
}

#[test]
fn test_create_schedule_invalid_assessment_type() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 999, // Non-existent
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let result = repo.create_schedule(&request);
    assert!(result.is_err());
}

#[test]
fn test_create_schedule_invalid_time_format() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "25:00".to_string(), // Invalid hour
        day_of_week: None,
        day_of_month: None,
    };

    let result = repo.create_schedule(&request);
    assert!(result.is_err());
}

#[test]
fn test_create_schedule_weekly_missing_day_of_week() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Weekly,
        time_of_day: "09:00".to_string(),
        day_of_week: None, // Required for weekly
        day_of_month: None,
    };

    let result = repo.create_schedule(&request);
    assert!(
        result.is_err(),
        "Weekly schedule should require day_of_week"
    );
}

#[test]
fn test_create_schedule_monthly_missing_day_of_month() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Monthly,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None, // Required for monthly
    };

    let result = repo.create_schedule(&request);
    assert!(
        result.is_err(),
        "Monthly schedule should require day_of_month"
    );
}

#[test]
fn test_create_schedule_invalid_day_of_week() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Weekly,
        time_of_day: "09:00".to_string(),
        day_of_week: Some(7), // Invalid (0-6 only)
        day_of_month: None,
    };

    let result = repo.create_schedule(&request);
    assert!(result.is_err(), "Day of week must be 0-6");
}

#[test]
fn test_create_schedule_invalid_day_of_month() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Monthly,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: Some(32), // Invalid (1-31 only)
    };

    let result = repo.create_schedule(&request);
    assert!(result.is_err(), "Day of month must be 1-31");
}

// ============================================================================
// UPDATE SCHEDULE TESTS
// ============================================================================

#[test]
fn test_update_schedule_frequency() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create initial schedule
    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create schedule");

    // Update frequency to weekly
    let update = UpdateScheduleRequest {
        frequency: Some(ScheduleFrequency::Weekly),
        time_of_day: None,
        day_of_week: Some(3), // Required for weekly
        day_of_month: None,
        enabled: None,
    };

    let updated = repo
        .update_schedule(schedule.id, &update)
        .expect("Failed to update schedule");

    assert_eq!(updated.frequency, ScheduleFrequency::Weekly);
    assert_eq!(updated.day_of_week, Some(3));
}

#[test]
fn test_update_schedule_time() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create schedule");

    let update = UpdateScheduleRequest {
        frequency: None,
        time_of_day: Some("15:30".to_string()),
        day_of_week: None,
        day_of_month: None,
        enabled: None,
    };

    let updated = repo
        .update_schedule(schedule.id, &update)
        .expect("Failed to update schedule");

    assert_eq!(updated.time_of_day, "15:30");
}

#[test]
fn test_update_schedule_disable() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create schedule");
    assert!(schedule.enabled);

    let update = UpdateScheduleRequest {
        frequency: None,
        time_of_day: None,
        day_of_week: None,
        day_of_month: None,
        enabled: Some(false),
    };

    let updated = repo
        .update_schedule(schedule.id, &update)
        .expect("Failed to update schedule");

    assert!(!updated.enabled);
}

#[test]
fn test_update_schedule_invalid_time() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create schedule");

    let update = UpdateScheduleRequest {
        frequency: None,
        time_of_day: Some("25:00".to_string()), // Invalid
        day_of_week: None,
        day_of_month: None,
        enabled: None,
    };

    let result = repo.update_schedule(schedule.id, &update);
    assert!(result.is_err());
}

#[test]
fn test_update_schedule_not_found() {
    let (repo, _temp_dir) = setup_test_repo();

    let update = UpdateScheduleRequest {
        frequency: None,
        time_of_day: Some("15:30".to_string()),
        day_of_week: None,
        day_of_month: None,
        enabled: None,
    };

    let result = repo.update_schedule(999, &update);
    assert!(result.is_err());
}

#[test]
fn test_update_schedule_no_changes() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create schedule");

    // Empty update
    let update = UpdateScheduleRequest {
        frequency: None,
        time_of_day: None,
        day_of_week: None,
        day_of_month: None,
        enabled: None,
    };

    let updated = repo
        .update_schedule(schedule.id, &update)
        .expect("Failed to update schedule");

    // Should return unchanged schedule
    assert_eq!(updated.time_of_day, "09:00");
    assert_eq!(updated.frequency, ScheduleFrequency::Daily);
}

// ============================================================================
// DELETE SCHEDULE TESTS
// ============================================================================

#[test]
fn test_delete_schedule() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create schedule");

    repo.delete_schedule(schedule.id)
        .expect("Failed to delete schedule");

    // Verify it's gone
    let result = repo.get_schedule(schedule.id);
    assert!(result.is_err());
}

#[test]
fn test_delete_schedule_not_found() {
    let (repo, _temp_dir) = setup_test_repo();

    let result = repo.delete_schedule(999);
    assert!(result.is_err());
}

// ============================================================================
// GET SCHEDULES TESTS
// ============================================================================

#[test]
fn test_get_schedules_empty() {
    let (repo, _temp_dir) = setup_test_repo();

    let schedules = repo.get_schedules(false).expect("Failed to get schedules");
    assert_eq!(schedules.len(), 0);
}

#[test]
fn test_get_schedules_all() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create multiple schedules
    let request1 = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let request2 = CreateScheduleRequest {
        assessment_type_id: 2,
        frequency: ScheduleFrequency::Weekly,
        time_of_day: "14:00".to_string(),
        day_of_week: Some(1),
        day_of_month: None,
    };

    repo.create_schedule(&request1)
        .expect("Failed to create schedule 1");
    repo.create_schedule(&request2)
        .expect("Failed to create schedule 2");

    let schedules = repo.get_schedules(false).expect("Failed to get schedules");
    assert_eq!(schedules.len(), 2);
}

#[test]
fn test_get_schedules_enabled_only() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create enabled schedule
    let request1 = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule1 = repo
        .create_schedule(&request1)
        .expect("Failed to create schedule 1");

    // Create and disable second schedule
    let request2 = CreateScheduleRequest {
        assessment_type_id: 2,
        frequency: ScheduleFrequency::Weekly,
        time_of_day: "14:00".to_string(),
        day_of_week: Some(1),
        day_of_month: None,
    };

    let schedule2 = repo
        .create_schedule(&request2)
        .expect("Failed to create schedule 2");

    let update = UpdateScheduleRequest {
        frequency: None,
        time_of_day: None,
        day_of_week: None,
        day_of_month: None,
        enabled: Some(false),
    };

    repo.update_schedule(schedule2.id, &update)
        .expect("Failed to disable schedule");

    // Get enabled only
    let enabled_schedules = repo
        .get_schedules(true)
        .expect("Failed to get enabled schedules");
    assert_eq!(enabled_schedules.len(), 1);
    assert_eq!(enabled_schedules[0].id, schedule1.id);

    // Get all
    let all_schedules = repo
        .get_schedules(false)
        .expect("Failed to get all schedules");
    assert_eq!(all_schedules.len(), 2);
}

#[test]
fn test_get_schedule_by_id() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let created = repo
        .create_schedule(&request)
        .expect("Failed to create schedule");

    let retrieved = repo
        .get_schedule(created.id)
        .expect("Failed to get schedule");

    assert_eq!(retrieved.id, created.id);
    assert_eq!(retrieved.assessment_type_id, created.assessment_type_id);
    assert_eq!(retrieved.frequency, created.frequency);
}

#[test]
fn test_get_schedule_not_found() {
    let (repo, _temp_dir) = setup_test_repo();

    let result = repo.get_schedule(999);
    assert!(result.is_err());
}

// ============================================================================
// DUE SCHEDULES TESTS
// ============================================================================

#[test]
fn test_get_due_schedules_empty() {
    let (repo, _temp_dir) = setup_test_repo();

    let due = repo
        .get_due_schedules()
        .expect("Failed to get due schedules");
    assert_eq!(due.len(), 0);
}

#[test]
fn test_get_due_schedules_future_time() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create schedule for late in the day (likely future)
    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "23:59".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    repo.create_schedule(&request)
        .expect("Failed to create schedule");

    let due = repo
        .get_due_schedules()
        .expect("Failed to get due schedules");
    // Should not include future schedule
    assert_eq!(due.len(), 0);
}

#[test]
fn test_get_due_schedules_past_time() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create schedule for early in the day (likely past)
    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "00:01".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    repo.create_schedule(&request)
        .expect("Failed to create schedule");

    let due = repo
        .get_due_schedules()
        .expect("Failed to get due schedules");
    // Should include past time that hasn't been triggered today
    assert_eq!(due.len(), 1);
}

#[test]
fn test_get_due_schedules_disabled_excluded() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "00:01".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create schedule");

    // Disable it
    let update = UpdateScheduleRequest {
        frequency: None,
        time_of_day: None,
        day_of_week: None,
        day_of_month: None,
        enabled: Some(false),
    };

    repo.update_schedule(schedule.id, &update)
        .expect("Failed to disable schedule");

    let due = repo
        .get_due_schedules()
        .expect("Failed to get due schedules");
    // Disabled schedules should not appear in due list
    assert_eq!(due.len(), 0);
}

// ============================================================================
// MARK TRIGGERED TESTS
// ============================================================================

#[test]
fn test_mark_triggered() {
    let (repo, _temp_dir) = setup_test_repo();

    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create schedule");
    assert_eq!(schedule.last_triggered_at, None);

    repo.mark_triggered(schedule.id)
        .expect("Failed to mark triggered");

    let updated = repo
        .get_schedule(schedule.id)
        .expect("Failed to get schedule");
    assert!(updated.last_triggered_at.is_some());
}

#[test]
fn test_mark_triggered_not_found() {
    let (repo, _temp_dir) = setup_test_repo();

    let result = repo.mark_triggered(999);
    assert!(result.is_err());
}

#[test]
fn test_mark_triggered_excludes_from_due() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create schedule with past time
    let request = CreateScheduleRequest {
        assessment_type_id: 1,
        frequency: ScheduleFrequency::Daily,
        time_of_day: "00:01".to_string(),
        day_of_week: None,
        day_of_month: None,
    };

    let schedule = repo
        .create_schedule(&request)
        .expect("Failed to create schedule");

    // Should be due
    let due = repo
        .get_due_schedules()
        .expect("Failed to get due schedules");
    assert_eq!(due.len(), 1);

    // Mark as triggered
    repo.mark_triggered(schedule.id)
        .expect("Failed to mark triggered");

    // Should no longer be due
    let due_after = repo
        .get_due_schedules()
        .expect("Failed to get due schedules");
    assert_eq!(due_after.len(), 0);
}

// ============================================================================
// FREQUENCY ENUM TESTS
// ============================================================================

#[test]
fn test_schedule_frequency_as_str() {
    assert_eq!(ScheduleFrequency::Daily.as_str(), "daily");
    assert_eq!(ScheduleFrequency::Weekly.as_str(), "weekly");
    assert_eq!(ScheduleFrequency::Biweekly.as_str(), "biweekly");
    assert_eq!(ScheduleFrequency::Monthly.as_str(), "monthly");
}

#[test]
fn test_schedule_frequency_from_str() {
    assert_eq!(
        ScheduleFrequency::from_str("daily").unwrap(),
        ScheduleFrequency::Daily
    );
    assert_eq!(
        ScheduleFrequency::from_str("WEEKLY").unwrap(),
        ScheduleFrequency::Weekly
    );
    assert_eq!(
        ScheduleFrequency::from_str("BiWeekly").unwrap(),
        ScheduleFrequency::Biweekly
    );
    assert_eq!(
        ScheduleFrequency::from_str("MONTHLY").unwrap(),
        ScheduleFrequency::Monthly
    );
    assert!(ScheduleFrequency::from_str("invalid").is_err());
}
