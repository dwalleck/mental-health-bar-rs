// Integration tests for activity management (User Story 3)
// T096: Integration test - create_activity command
// T097: Integration test - Soft delete preserves historical data
// T097b: Integration test - Verify deleted activity names still display correctly in historical mood check-ins

use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::mood::repository::MoodRepository;
use tempfile::TempDir;

/// Setup test environment with temporary database and default activity group
fn setup_test_repo() -> (MoodRepository, TempDir, i32) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().to_path_buf();
    let db = Arc::new(Database::new(db_path).expect("Failed to create database"));

    // Create a default activity group for testing
    let conn = db.get_connection();
    let conn = conn.lock();
    let group_id: i32 = conn
        .query_row(
            "INSERT INTO activity_groups (name, description) VALUES (?, ?) RETURNING id",
            ["Default Group", "Default group for testing"],
            |row| row.get(0),
        )
        .expect("Failed to create default activity group");
    drop(conn); // Release lock before creating repo

    let repo = MoodRepository::new(db);
    (repo, temp_dir, group_id)
}

// T096: Integration test - create_activity command
#[test]
fn test_create_activity_success() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create activity with all fields
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
        .expect("Failed to create activity");

    assert_eq!(activity.name, "Exercise");
    assert_eq!(activity.color, Some("#4CAF50".to_string()));
    assert_eq!(activity.icon, Some("🏃".to_string()));
    assert!(activity.deleted_at.is_none());
}

#[test]
fn test_create_activity_minimal() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create activity with only name
    let activity = repo
        .create_activity("Meditation", None, None, group_id)
        .expect("Failed to create activity");

    assert_eq!(activity.name, "Meditation");
    assert!(activity.color.is_none());
    assert!(activity.icon.is_none());
}

#[test]
fn test_create_activity_duplicate_name() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create first activity
    repo.create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
        .expect("Failed to create first activity");

    // Try to create duplicate
    let result = repo.create_activity("Exercise", Some("#FF0000"), Some("🚴"), group_id);
    assert!(result.is_err());
}

#[test]
fn test_create_activity_name_trimming() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create activity with whitespace
    let activity = repo
        .create_activity("  Reading  ", Some("#FF5733"), Some("📚"), group_id)
        .expect("Failed to create activity");

    assert_eq!(activity.name, "Reading");
}

#[test]
fn test_create_activity_empty_name() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Try to create activity with empty name
    let result = repo.create_activity("", None, None, group_id);
    assert!(result.is_err());

    let result = repo.create_activity("   ", None, None, group_id);
    assert!(result.is_err());
}

#[test]
fn test_create_activity_name_too_long() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Try to create activity with name > 50 chars
    let long_name = "a".repeat(51);
    let result = repo.create_activity(&long_name, None, None, group_id);
    assert!(result.is_err());
}

#[test]
fn test_create_activity_invalid_color() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Try to create activity with invalid color format
    let result = repo.create_activity("Exercise", Some("red"), None, group_id);
    assert!(result.is_err());

    let result = repo.create_activity("Exercise", Some("#FF"), None, group_id);
    assert!(result.is_err());

    let result = repo.create_activity("Exercise", Some("FF5733"), None, group_id);
    assert!(result.is_err());
}

#[test]
fn test_update_activity_success() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create activity
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
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
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create activity
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
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
    let (repo, _temp_dir, group_id) = setup_test_repo();

    let result = repo.update_activity(9999, Some("Test"), None, None);
    assert!(result.is_err());
}

#[test]
fn test_update_activity_duplicate_name() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create two activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
        .expect("Failed to create activity 1");
    repo.create_activity("Meditation", Some("#9C27B0"), Some("🧘"), group_id)
        .expect("Failed to create activity 2");

    // Try to rename activity1 to existing name
    let result = repo.update_activity(activity1.id, Some("Meditation"), None, None);
    assert!(result.is_err());
}

// T097: Integration test - Soft delete basic functionality
#[test]
fn test_soft_delete_basic() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create activity (without any mood check-ins referencing it)
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
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
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create two activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
        .expect("Failed to create activity 1");
    let activity2 = repo
        .create_activity("Meditation", Some("#9C27B0"), Some("🧘"), group_id)
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
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create 3 activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
        .expect("Failed to create activity 1");
    repo.create_activity("Meditation", Some("#9C27B0"), Some("🧘"), group_id)
        .expect("Failed to create activity 2");
    let activity3 = repo
        .create_activity("Reading", Some("#FF5733"), Some("📚"), group_id)
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
    let (repo, _temp_dir, group_id) = setup_test_repo();

    let result = repo.delete_activity(9999);
    assert!(result.is_err());
}

#[test]
fn test_soft_delete_is_idempotent() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create activity
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
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
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create and delete activity
    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
        .expect("Failed to create activity");
    repo.delete_activity(activity.id)
        .expect("Failed to delete activity");

    // Create new activity with same name - should succeed now
    // The partial unique index only enforces uniqueness for non-deleted activities
    let new_activity = repo
        .create_activity("Exercise", Some("#FF0000"), Some("🚴"), group_id)
        .expect("Should allow recreating activity with same name after soft delete");

    assert_eq!(new_activity.name, "Exercise");
    assert_eq!(new_activity.color, Some("#FF0000".to_string()));
    assert_eq!(new_activity.icon, Some("🚴".to_string()));
    assert!(new_activity.deleted_at.is_none());
    assert_ne!(
        new_activity.id, activity.id,
        "Should be a new activity with different ID"
    );
}

// ============================================================================
// P0 TESTS - Command Validation (T150l-T150m) and Boundary Conditions (T150v-T150w)
// ============================================================================

// T150l: Test create_activity with icon exceeding 20 chars
#[test]
fn test_create_activity_icon_exceeds_max_length() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create icon with more than 20 characters
    // Using compound emoji to exceed limit
    let long_icon = "🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨🎨"; // 21 emojis = 21 characters (84 bytes)

    let result = repo.create_activity("Art", Some("#E91E63"), Some(long_icon), group_id);

    // Should either error or truncate
    match result {
        Ok(activity) => {
            // If implementation truncates, verify it's within limit (using chars, not bytes)
            assert!(
                activity.icon.as_ref().unwrap().chars().count() <= 20,
                "Icon should be truncated to 20 characters"
            );
        }
        Err(e) => {
            // If implementation errors, message should mention icon and limit
            let error_msg = format!("{}", e);
            assert!(
                error_msg.contains("icon")
                    && (error_msg.contains("20") || error_msg.contains("length")),
                "Error should mention icon length limit: {}",
                error_msg
            );
        }
    }
}

// T150l continued: Test icon at exactly 20 characters (boundary)
#[test]
fn test_create_activity_icon_at_exact_limit() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create icon with exactly 20 ASCII characters
    let icon_20_chars = "12345678901234567890"; // Exactly 20 chars

    let result = repo.create_activity("Test", Some("#4CAF50"), Some(icon_20_chars), group_id);

    // Should succeed at exactly the limit
    assert!(result.is_ok(), "20 characters should be allowed for icon");
    assert_eq!(result.unwrap().icon.unwrap(), icon_20_chars);
}

// T150m: Test create_activity with various color formats
#[test]
fn test_create_activity_color_format_validation() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Valid formats that should succeed
    let valid_colors = vec![
        "#abc",      // 3-digit hex (lowercase)
        "#123",      // 3-digit hex (numbers)
        "#F0F",      // 3-digit hex (uppercase)
        "#ABC123",   // 6-digit hex (mixed case)
        "#ff00ff",   // 6-digit hex (lowercase)
        "#FF00FF80", // 8-digit hex with alpha
    ];

    for (i, color) in valid_colors.iter().enumerate() {
        let result = repo.create_activity(&format!("Activity{}", i), Some(color), None, group_id);
        assert!(
            result.is_ok(),
            "Valid color format '{}' should be accepted",
            color
        );
    }

    // Invalid formats that should fail
    let invalid_colors = vec![
        "RGB",        // Missing #
        "#12345",     // 5 digits (invalid)
        "#GGG",       // Invalid hex characters
        "##AABBCC",   // Double #
        "#",          // Just #
        "red",        // Color name
        "#12",        // Too short
        "#123456789", // Too long
    ];

    for color in invalid_colors.iter() {
        let result = repo.create_activity("Test", Some(color), None, group_id);
        assert!(
            result.is_err(),
            "Invalid color format '{}' should be rejected",
            color
        );
    }
}

// T150v: Test create_activity with name at exactly 50 characters
#[test]
fn test_create_activity_name_at_exact_limit() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create name with exactly 50 characters
    let name_50_chars = "a".repeat(50);

    let result = repo.create_activity(&name_50_chars, Some("#4CAF50"), Some("📝"), group_id);

    // Should succeed at exactly the limit
    assert!(result.is_ok(), "50 characters should be allowed");
    assert_eq!(result.unwrap().name, name_50_chars);
}

// T150w: Test create_activity with name boundary conditions
#[test]
fn test_create_activity_name_boundary_conditions() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // 49 characters - should succeed
    let name_49_chars = "a".repeat(49);
    let result = repo.create_activity(&name_49_chars, Some("#4CAF50"), None, group_id);
    assert!(result.is_ok(), "49 characters should be valid");
    assert_eq!(result.unwrap().name.len(), 49);

    // 50 characters - should succeed (at boundary)
    let name_50_chars = "b".repeat(50);
    let result = repo.create_activity(&name_50_chars, Some("#2196F3"), None, group_id);
    assert!(result.is_ok(), "50 characters should be valid");
    assert_eq!(result.unwrap().name.len(), 50);

    // 51 characters - should fail (over limit)
    let name_51_chars = "c".repeat(51);
    let result = repo.create_activity(&name_51_chars, Some("#FF9800"), None, group_id);
    assert!(result.is_err(), "51 characters should be rejected");

    let error_msg = format!("{}", result.unwrap_err());
    assert!(
        error_msg.contains("50") || error_msg.contains("length") || error_msg.contains("name"),
        "Error should indicate name length violation: {}",
        error_msg
    );
}
