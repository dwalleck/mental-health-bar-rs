// Integration tests for activity management (User Story 3)
// T096: Integration test - create_activity command
// T097: Integration test - Soft delete preserves historical data
// T097b: Integration test - Verify deleted activity names still display correctly in historical mood check-ins

use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::mood::repository::MoodRepository;
use tempfile::TempDir;

/// Setup test environment with temporary database
fn setup_test_repo() -> (MoodRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().to_path_buf();
    let db = Arc::new(Database::new(db_path).expect("Failed to create database"));
    let repo = MoodRepository::new(db);
    (repo, temp_dir)
}

// T096: Integration test - create_activity command
#[test]
fn test_create_activity_success() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create activity with all fields
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");

    assert_eq!(activity.name, "Exercise");
    assert_eq!(activity.color, Some("#4CAF50".to_string()));
    assert_eq!(activity.icon, Some("🏃".to_string()));
    assert!(activity.deleted_at.is_none());
}

#[test]
fn test_create_activity_minimal() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create activity with only name
    let activity = repo
        .create_activity("Meditation", None, None)
        .expect("Failed to create activity");

    assert_eq!(activity.name, "Meditation");
    assert!(activity.color.is_none());
    assert!(activity.icon.is_none());
}

#[test]
fn test_create_activity_duplicate_name() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create first activity
    repo.create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create first activity");

    // Try to create duplicate
    let result = repo.create_activity("Exercise", Some("#FF0000"), Some("🚴"));
    assert!(result.is_err());
}

#[test]
fn test_create_activity_name_trimming() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create activity with whitespace
    let activity = repo
        .create_activity("  Reading  ", Some("#FF5733"), Some("📚"))
        .expect("Failed to create activity");

    assert_eq!(activity.name, "Reading");
}

#[test]
fn test_create_activity_empty_name() {
    let (repo, _temp_dir) = setup_test_repo();

    // Try to create activity with empty name
    let result = repo.create_activity("", None, None);
    assert!(result.is_err());

    let result = repo.create_activity("   ", None, None);
    assert!(result.is_err());
}

#[test]
fn test_create_activity_name_too_long() {
    let (repo, _temp_dir) = setup_test_repo();

    // Try to create activity with name > 100 chars
    let long_name = "a".repeat(101);
    let result = repo.create_activity(&long_name, None, None);
    assert!(result.is_err());
}

#[test]
fn test_create_activity_invalid_color() {
    let (repo, _temp_dir) = setup_test_repo();

    // Try to create activity with invalid color format
    let result = repo.create_activity("Exercise", Some("red"), None);
    assert!(result.is_err());

    let result = repo.create_activity("Exercise", Some("#FF"), None);
    assert!(result.is_err());

    let result = repo.create_activity("Exercise", Some("FF5733"), None);
    assert!(result.is_err());
}

#[test]
fn test_update_activity_success() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create activity
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");

    // Update all fields
    let updated = repo
        .update_activity(activity.id, Some("Running"), Some("#FF5733"), Some("🏃‍♂️"))
        .expect("Failed to update activity");

    assert_eq!(updated.name, "Running");
    assert_eq!(updated.color, Some("#FF5733".to_string()));
    assert_eq!(updated.icon, Some("🏃‍♂️".to_string()));
}

#[test]
fn test_update_activity_partial() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create activity
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");

    // Update only name
    let updated = repo
        .update_activity(activity.id, Some("Workout"), None, None)
        .expect("Failed to update activity");

    assert_eq!(updated.name, "Workout");
    assert_eq!(updated.color, Some("#4CAF50".to_string())); // Unchanged
    assert_eq!(updated.icon, Some("🏃".to_string())); // Unchanged
}

#[test]
fn test_update_activity_not_found() {
    let (repo, _temp_dir) = setup_test_repo();

    let result = repo.update_activity(9999, Some("Test"), None, None);
    assert!(result.is_err());
}

#[test]
fn test_update_activity_duplicate_name() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create two activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity 1");
    repo.create_activity("Meditation", Some("#9C27B0"), Some("🧘"))
        .expect("Failed to create activity 2");

    // Try to rename activity1 to existing name
    let result = repo.update_activity(activity1.id, Some("Meditation"), None, None);
    assert!(result.is_err());
}

// T097: Integration test - Soft delete basic functionality
#[test]
fn test_soft_delete_basic() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create activity (without any mood check-ins referencing it)
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");

    // Soft delete the activity
    repo.delete_activity(activity.id)
        .expect("Failed to delete activity");

    // Verify activity is soft deleted - not in active list
    let active_activities = repo
        .get_activities(false)
        .expect("Failed to get active activities");
    assert_eq!(active_activities.len(), 0);

    // Verify activity still exists when including deleted
    let all_activities = repo
        .get_activities(true)
        .expect("Failed to get all activities");
    assert_eq!(all_activities.len(), 1);
    assert!(all_activities[0].deleted_at.is_some());
}

// T097b: Integration test - Verify deleted activity appears in historical mood check-ins
// SQLite properly supports soft deletes with FK constraints (unlike DuckDB)
#[test]
fn test_deleted_activity_in_history() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create two activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity 1");
    let activity2 = repo
        .create_activity("Meditation", Some("#9C27B0"), Some("🧘"))
        .expect("Failed to create activity 2");

    // Create mood check-in with both activities
    let mood = repo
        .create_mood_checkin(4, vec![activity1.id, activity2.id], Some("Great day!"))
        .expect("Failed to create mood check-in");

    // Soft delete activity1
    repo.delete_activity(activity1.id)
        .expect("Failed to delete activity");

    // Verify historical mood check-in still shows both activities
    let historical_mood = repo
        .get_mood_checkin(mood.id)
        .expect("Failed to get mood check-in");
    assert_eq!(historical_mood.activities.len(), 2);

    // Find the deleted activity in the history
    let deleted_activity = historical_mood
        .activities
        .iter()
        .find(|a| a.name == "Exercise")
        .expect("Deleted activity not found in history");

    assert_eq!(deleted_activity.name, "Exercise");
    assert!(
        deleted_activity.deleted_at.is_some(),
        "Activity should be marked as deleted"
    );
}

#[test]
fn test_get_activities_filter_deleted() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create 3 activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity 1");
    repo.create_activity("Meditation", Some("#9C27B0"), Some("🧘"))
        .expect("Failed to create activity 2");
    let activity3 = repo
        .create_activity("Reading", Some("#FF5733"), Some("📚"))
        .expect("Failed to create activity 3");

    // Delete 2 activities
    repo.delete_activity(activity1.id)
        .expect("Failed to delete activity 1");
    repo.delete_activity(activity3.id)
        .expect("Failed to delete activity 3");

    // Get active activities only
    let active = repo
        .get_activities(false)
        .expect("Failed to get active activities");
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].name, "Meditation");

    // Get all activities
    let all = repo
        .get_activities(true)
        .expect("Failed to get all activities");
    assert_eq!(all.len(), 3);
}

#[test]
fn test_delete_activity_not_found() {
    let (repo, _temp_dir) = setup_test_repo();

    let result = repo.delete_activity(9999);
    assert!(result.is_err());
}

#[test]
fn test_soft_delete_is_idempotent() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create activity
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");

    // Delete twice
    repo.delete_activity(activity.id)
        .expect("Failed to delete activity first time");
    repo.delete_activity(activity.id)
        .expect("Failed to delete activity second time");

    // Should still be soft deleted
    let all_activities = repo
        .get_activities(true)
        .expect("Failed to get all activities");
    assert_eq!(all_activities.len(), 1);
    assert!(all_activities[0].deleted_at.is_some());
}

#[test]
fn test_duplicate_name_even_after_soft_delete() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create and delete activity
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");
    repo.delete_activity(activity.id)
        .expect("Failed to delete activity");

    // Try to create new activity with same name
    // This should fail due to UNIQUE constraint on name column
    // (Schema has UNIQUE constraint without considering deleted_at)
    let result = repo.create_activity("Exercise", Some("#FF0000"), Some("🚴"));
    assert!(
        result.is_err(),
        "Should not allow duplicate name even after soft delete"
    );

    // User must choose a different name
    let new_activity = repo
        .create_activity("Running", Some("#FF0000"), Some("🚴"))
        .expect("Failed to create activity with different name");

    assert_eq!(new_activity.name, "Running");
    assert!(new_activity.deleted_at.is_none());
}
