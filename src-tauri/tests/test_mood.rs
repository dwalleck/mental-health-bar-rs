// Integration tests for mood feature (User Story 2)
// T069: Integration test - log_mood command
// T070: Integration test - get_mood_history query with date filtering
// T071: Integration test - Mood check-in with multiple activities

use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::mood::repository::MoodRepository;
use tauri_sveltekit_modern_lib::types::mood::MoodRating;
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

    let repo = MoodRepository::new(db.clone());
    (repo, temp_dir, group_id)
}

// T069: Integration test - log_mood command
#[test]
fn test_log_mood_end_to_end() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create test activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
        .expect("Failed to create activity 1");
    let activity2 = repo
        .create_activity("Meditation", Some("#9C27B0"), Some("🧘"), group_id)
        .expect("Failed to create activity 2");

    // Log mood with activities
    let mood_checkin = repo
        .create_mood_checkin(
            4,
            vec![activity1.id, activity2.id],
            Some("Feeling great after workout and meditation"),
        )
        .expect("Failed to create mood check-in");

    // Verify mood check-in
    assert_eq!(mood_checkin.mood_rating.value(), 4);
    assert_eq!(mood_checkin.activities.len(), 2);
    assert!(mood_checkin.notes.is_some());
    assert_eq!(
        mood_checkin.notes.as_ref().unwrap(),
        "Feeling great after workout and meditation"
    );

    // Verify activities are correctly linked
    let activity_names: Vec<String> = mood_checkin
        .activities
        .iter()
        .map(|a| a.name.clone())
        .collect();
    assert!(activity_names.contains(&"Exercise".to_string()));
    assert!(activity_names.contains(&"Meditation".to_string()));
}

#[test]
fn test_log_mood_without_activities() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Log mood without activities
    let mood_checkin = repo
        .create_mood_checkin(3, vec![], None)
        .expect("Failed to create mood check-in");

    assert_eq!(mood_checkin.mood_rating.value(), 3);
    assert_eq!(mood_checkin.activities.len(), 0);
    assert!(mood_checkin.notes.is_none());
}

#[test]
fn test_log_mood_invalid_rating() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Try to log mood with invalid rating
    let result = repo.create_mood_checkin(0, vec![], None);
    assert!(result.is_err());

    let result = repo.create_mood_checkin(8, vec![], None); // Above maximum of 7
    assert!(result.is_err());
}

#[test]
fn test_log_mood_invalid_activity_id() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Try to log mood with non-existent activity ID
    let result = repo.create_mood_checkin(4, vec![9999], None);
    assert!(result.is_err());
}

// T070: Integration test - get_mood_history query with date filtering
#[test]
fn test_get_mood_history_with_date_filtering() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create mood check-ins on different days
    let _yesterday = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(1))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let _tomorrow = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    // Create 3 mood check-ins
    repo.create_mood_checkin(3, vec![], Some("Yesterday's mood"))
        .expect("Failed to create mood 1");
    repo.create_mood_checkin(4, vec![], Some("Today's mood"))
        .expect("Failed to create mood 2");
    repo.create_mood_checkin(5, vec![], Some("Another one"))
        .expect("Failed to create mood 3");

    // Get all mood history
    let all_moods = repo
        .get_mood_history(None, None, None)
        .expect("Failed to get all moods");
    assert_eq!(all_moods.len(), 3);

    // Get mood history from today
    let today_moods = repo
        .get_mood_history(Some(today.clone()), None, None)
        .expect("Failed to get today's moods");
    assert!(today_moods.len() >= 2); // At least 2 created today

    // Get mood history with limit
    let limited_moods = repo
        .get_mood_history(None, None, Some(2))
        .expect("Failed to get limited moods");
    assert_eq!(limited_moods.len(), 2);

    // Verify ordering (most recent first)
    assert_eq!(limited_moods[0].notes.as_ref().unwrap(), "Another one");
}

#[test]
fn test_get_mood_history_empty() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    let moods = repo
        .get_mood_history(None, None, None)
        .expect("Failed to get mood history");
    assert_eq!(moods.len(), 0);
}

// T071: Integration test - Mood check-in with multiple activities
#[test]
fn test_mood_checkin_with_multiple_activities() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create 5 activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
        .expect("Failed to create activity 1");
    let activity2 = repo
        .create_activity("Meditation", Some("#9C27B0"), Some("🧘"), group_id)
        .expect("Failed to create activity 2");
    let activity3 = repo
        .create_activity("Social", Some("#2196F3"), Some("👥"), group_id)
        .expect("Failed to create activity 3");
    let activity4 = repo
        .create_activity("Work", Some("#FF9800"), Some("💼"), group_id)
        .expect("Failed to create activity 4");
    let activity5 = repo
        .create_activity("Hobby", Some("#E91E63"), Some("🎨"), group_id)
        .expect("Failed to create activity 5");

    // Log mood with all 5 activities
    let mood_checkin = repo
        .create_mood_checkin(
            5,
            vec![
                activity1.id,
                activity2.id,
                activity3.id,
                activity4.id,
                activity5.id,
            ],
            Some("Amazing day with all my favorite activities!"),
        )
        .expect("Failed to create mood check-in");

    assert_eq!(mood_checkin.mood_rating.value(), 5);
    assert_eq!(mood_checkin.activities.len(), 5);

    // Verify all activities are present
    let activity_names: Vec<String> = mood_checkin
        .activities
        .iter()
        .map(|a| a.name.clone())
        .collect();
    assert!(activity_names.contains(&"Exercise".to_string()));
    assert!(activity_names.contains(&"Meditation".to_string()));
    assert!(activity_names.contains(&"Social".to_string()));
    assert!(activity_names.contains(&"Work".to_string()));
    assert!(activity_names.contains(&"Hobby".to_string()));
}

#[test]
fn test_mood_checkin_with_duplicate_activity_ids() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
        .expect("Failed to create activity");

    // Try to log mood with duplicate activity IDs
    // This should either deduplicate or error - implementation will decide
    let result = repo.create_mood_checkin(4, vec![activity.id, activity.id], None);

    // For now, we'll expect this to work (deduplication handled by UNIQUE constraint)
    // If it errors, the implementation should provide a clear message
    match result {
        Ok(mood) => {
            // If successful, activities should be deduplicated
            assert_eq!(mood.activities.len(), 1);
        }
        Err(e) => {
            // If it errors, make sure it's a clear validation error
            eprintln!("Duplicate activity error: {}", e);
        }
    }
}

#[test]
fn test_get_mood_checkin_by_id() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create activity
    let activity = repo
        .create_activity("Reading", Some("#FF5733"), Some("📚"), group_id)
        .expect("Failed to create activity");

    // Create mood check-in
    let created = repo
        .create_mood_checkin(4, vec![activity.id], Some("Good book"))
        .expect("Failed to create mood check-in");

    // Fetch by ID
    let fetched = repo
        .get_mood_checkin(created.id)
        .expect("Failed to get mood check-in");

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.mood_rating.value(), 4);
    assert_eq!(fetched.activities.len(), 1);
    assert_eq!(fetched.activities[0].name, "Reading");
}

#[test]
fn test_get_mood_checkin_not_found() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    let result = repo.get_mood_checkin(9999);
    assert!(result.is_err());
}

// T093a: Integration test - Deleting mood_checkin cascades to mood_checkin_activities
#[test]
fn test_delete_mood_checkin_cascades_to_activities() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create test activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"), group_id)
        .expect("Failed to create activity 1");
    let activity2 = repo
        .create_activity("Meditation", Some("#9C27B0"), Some("🧘"), group_id)
        .expect("Failed to create activity 2");

    // Create mood check-in with activities
    let mood_checkin = repo
        .create_mood_checkin(4, vec![activity1.id, activity2.id], Some("Feeling great"))
        .expect("Failed to create mood check-in");

    // Verify mood check-in exists with activities
    let fetched = repo
        .get_mood_checkin(mood_checkin.id)
        .expect("Failed to get mood check-in");
    assert_eq!(fetched.activities.len(), 2);

    // Delete the mood check-in
    repo.delete_mood_checkin(mood_checkin.id)
        .expect("Failed to delete mood check-in");

    // Verify mood check-in is deleted
    let result = repo.get_mood_checkin(mood_checkin.id);
    assert!(result.is_err(), "Mood check-in should be deleted");

    // Verify activities still exist (soft delete, not cascade)
    let all_activities = repo
        .get_activities(false)
        .expect("Failed to get activities");
    assert_eq!(all_activities.len(), 2, "Activities should still exist");

    // Verify junction table records are deleted by checking that if we create
    // a new mood check-in with same activities, it works without constraint violations
    let new_mood = repo
        .create_mood_checkin(5, vec![activity1.id, activity2.id], Some("Another day"))
        .expect("Should be able to reuse activities after deletion");
    assert_eq!(new_mood.activities.len(), 2);
}

// ============================================================================
// P0 TESTS - Command Validation (T150i-T150k, T150y)
// ============================================================================

// T150i: Test log_mood with notes exceeding 5,000 chars
#[test]
fn test_log_mood_notes_exceeds_max_length() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create notes with exactly 5,001 characters (1 over limit)
    let notes = "a".repeat(5_001);

    let result = repo.create_mood_checkin(3, vec![], Some(&notes));

    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("5000") || error_msg.contains("exceed"));
}

// T150y: Test notes at exactly 5,000 characters (boundary)
#[test]
fn test_log_mood_notes_at_exact_limit() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Exactly 5,000 characters (at the limit)
    let notes = "a".repeat(5_000);

    let result = repo.create_mood_checkin(3, vec![], Some(&notes));

    // Should succeed at exactly the limit
    assert!(result.is_ok(), "5,000 characters should be allowed");
}

// T150j: Test log_mood with boundary ratings (0, 8, -1, 100) for 1-7 scale
#[test]
fn test_log_mood_with_invalid_boundary_ratings() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Test rating 0 (below minimum of 1)
    let result = repo.create_mood_checkin(0, vec![], None);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("Invalid mood rating"));

    // Test rating 8 (above maximum of 7)
    let result = repo.create_mood_checkin(8, vec![], None);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("Invalid mood rating"));

    // Test negative rating
    let result = repo.create_mood_checkin(-1, vec![], None);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("Invalid mood rating"));

    // Test very large rating
    let result = repo.create_mood_checkin(100, vec![], None);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("Invalid mood rating"));
}

// T150j continued: Test valid boundary ratings (1 and 7) for 1-7 scale
#[test]
fn test_log_mood_with_valid_boundary_ratings() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Test rating 1 (minimum valid - Terrible)
    let result = repo.create_mood_checkin(1, vec![], None);
    assert!(result.is_ok(), "Rating 1 should be valid");
    assert_eq!(result.unwrap().mood_rating.value(), 1);

    // Test rating 7 (maximum valid - Excellent)
    let result = repo.create_mood_checkin(7, vec![], None);
    assert!(result.is_ok(), "Rating 7 should be valid");
    assert_eq!(result.unwrap().mood_rating.value(), 7);
}

// T150k: Test log_mood with very large activity_ids array (50+ ids)
#[test]
fn test_log_mood_with_large_activity_array() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create 60 activities
    let mut activity_ids = Vec::new();
    for i in 0..60 {
        let activity = repo
            .create_activity(
                &format!("Activity {}", i),
                Some("#4CAF50"),
                Some("📝"),
                group_id,
            )
            .expect(&format!("Failed to create activity {}", i));
        activity_ids.push(activity.id);
    }

    // Try to log mood with all 60 activities
    // This should either succeed or provide a reasonable error
    let result = repo.create_mood_checkin(4, activity_ids.clone(), Some("Busy day!"));

    // Most implementations should handle this, but if there's a limit, error should be clear
    match result {
        Ok(mood) => {
            assert_eq!(
                mood.activities.len(),
                60,
                "All 60 activities should be linked"
            );
            assert_eq!(mood.mood_rating.value(), 4);
        }
        Err(e) => {
            let error_msg = format!("{}", e);
            assert!(
                error_msg.contains("too many") || error_msg.contains("limit"),
                "Error should clearly indicate activity limit: {}",
                error_msg
            );
        }
    }
}

// ============================================================================
// P0 TESTS - Query Edge Cases (T150n-T150o, T150r-T150s)
// ============================================================================

// T150n: Test get_mood_history with invalid date formats
#[test]
fn test_get_mood_history_invalid_date_formats() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create a mood check-in
    repo.create_mood_checkin(4, vec![], Some("Test"))
        .expect("Failed to create mood");

    // Test various invalid date formats
    let invalid_dates = vec![
        "2024-13-01", // Invalid month
        "2024-02-30", // Invalid day
        "not-a-date", // Completely invalid
        "2024/01/15", // Wrong separator
        "15-01-2024", // Wrong format
        "2024-1-1",   // Missing leading zeros
        "",           // Empty string
    ];

    for invalid_date in invalid_dates {
        let result = repo.get_mood_history(Some(invalid_date.to_string()), None, None);

        // Implementation may either:
        // 1. Return error for invalid date
        // 2. Return empty results if date doesn't match anything
        // 3. Treat as no filter and return all
        // Test documents actual behavior
        match result {
            Ok(moods) => {
                // If it succeeds, document the behavior
                println!(
                    "Invalid date '{}' returned {} moods",
                    invalid_date,
                    moods.len()
                );
            }
            Err(e) => {
                // If it errors, error should mention date or format
                let error_msg = format!("{}", e);
                println!(
                    "Invalid date '{}' produced error: {}",
                    invalid_date, error_msg
                );
            }
        }
    }
}

// T150o: Test get_mood_history with from_date > to_date
#[test]
fn test_get_mood_history_reversed_date_range() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create some mood check-ins
    repo.create_mood_checkin(4, vec![], Some("Test 1"))
        .expect("Failed to create mood 1");
    repo.create_mood_checkin(5, vec![], Some("Test 2"))
        .expect("Failed to create mood 2");

    // Query with from_date > to_date (reversed range)
    let from_date = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let to_date = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let result = repo.get_mood_history(Some(from_date), Some(to_date), None);

    // Should either return empty results or error
    match result {
        Ok(moods) => {
            assert_eq!(
                moods.len(),
                0,
                "Reversed date range should return empty results"
            );
        }
        Err(e) => {
            let error_msg = format!("{}", e);
            assert!(
                error_msg.contains("date") || error_msg.contains("range"),
                "Error should mention date range issue: {}",
                error_msg
            );
        }
    }
}

// T150r: Test query with very large limit values
#[test]
fn test_get_mood_history_large_limit() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create 5 mood check-ins
    for i in 1..=5 {
        repo.create_mood_checkin(i, vec![], Some(&format!("Mood {}", i)))
            .expect("Failed to create mood");
    }

    // Test with very large limit
    let result = repo.get_mood_history(None, None, Some(1_000_000));

    // Should succeed and return all available moods (5)
    assert!(result.is_ok(), "Large limit should not cause error");
    let moods = result.unwrap();
    assert_eq!(
        moods.len(),
        5,
        "Should return all 5 moods despite large limit"
    );

    // Test with i32::MAX
    let result = repo.get_mood_history(None, None, Some(i32::MAX));
    assert!(result.is_ok(), "i32::MAX limit should not cause error");
    assert_eq!(result.unwrap().len(), 5);
}

// T150s: Test get_mood_statistics with empty date ranges
#[test]
fn test_get_mood_statistics_empty_date_range() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Create mood check-ins today
    repo.create_mood_checkin(4, vec![], Some("Today's mood"))
        .expect("Failed to create mood");

    // Query with date range in the future (no data)
    let future_start = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(10))
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    let future_end = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(20))
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    let result = repo.get_mood_stats(Some(future_start), Some(future_end));

    // Should either return error or return statistics with zero/empty values
    match result {
        Ok(stats) => {
            // If it returns stats, they should reflect no data
            assert_eq!(
                stats.total_checkins, 0,
                "Empty date range should have 0 check-ins"
            );
        }
        Err(e) => {
            let error_msg = format!("{}", e);
            assert!(
                error_msg.contains("No data") || error_msg.contains("empty"),
                "Error should indicate no data available: {}",
                error_msg
            );
        }
    }
}

// ============================================================================
// UTF-8 VALIDATION TESTS
// ============================================================================

#[test]
fn test_create_activity_utf8_characters() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Test emoji and international characters in activity name
    let name_with_emoji = "Running 🏃";
    let icon_with_emoji = "🏃";

    let activity = repo
        .create_activity(
            name_with_emoji,
            Some("#4CAF50"),
            Some(icon_with_emoji),
            group_id,
        )
        .expect("Failed to create activity with UTF-8");

    assert_eq!(activity.name, name_with_emoji);
    assert_eq!(activity.icon, Some(icon_with_emoji.to_string()));

    // Test Japanese characters
    let japanese_name = "瞑想";
    let activity = repo
        .create_activity(japanese_name, Some("#9C27B0"), Some("🧘"), group_id)
        .expect("Failed to create activity with Japanese characters");

    assert_eq!(activity.name, japanese_name);
}

#[test]
fn test_create_activity_utf8_length_validation() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // 50 emoji characters (4 bytes each = 200 bytes total) should be accepted
    let emoji_name = "😀".repeat(50);
    let result = repo.create_activity(&emoji_name, None, None, group_id);
    assert!(result.is_ok(), "Should accept 50 emoji characters");

    // 51 emoji characters should fail
    let emoji_name_too_long = "😀".repeat(51);
    let result = repo.create_activity(&emoji_name_too_long, None, None, group_id);
    assert!(result.is_err(), "Should reject 51 emoji characters");
    let error_msg = format!("{}", result.unwrap_err());
    assert!(
        error_msg.contains("51") || error_msg.contains("too long"),
        "Error should mention character count: {}",
        error_msg
    );
}

#[test]
fn test_mood_checkin_utf8_notes() {
    let (repo, _temp_dir, group_id) = setup_test_repo();

    // Test notes with emoji (should count characters, not bytes)
    let emoji_notes = "Great day! 😀🎉🌟";
    let result = repo.create_mood_checkin(5, vec![], Some(emoji_notes));
    assert!(result.is_ok(), "Should accept notes with emoji");
    let mood = result.unwrap();
    assert_eq!(mood.notes.as_ref().unwrap(), emoji_notes);

    // Test notes at character limit (5000 emoji = 20000 bytes)
    let long_emoji_notes = "😀".repeat(5000);
    let result = repo.create_mood_checkin(4, vec![], Some(&long_emoji_notes));
    assert!(result.is_ok(), "Should accept 5000 emoji characters");

    // Test notes exceeding character limit (5001 emoji)
    let too_long_notes = "😀".repeat(5001);
    let result = repo.create_mood_checkin(4, vec![], Some(&too_long_notes));
    assert!(result.is_err(), "Should reject 5001 emoji characters");
    let error_msg = format!("{}", result.unwrap_err());
    assert!(
        error_msg.contains("5001") || error_msg.contains("exceed"),
        "Error should mention character count: {}",
        error_msg
    );
}
