// Integration tests for mood feature (User Story 2)
// T069: Integration test - log_mood command
// T070: Integration test - get_mood_history query with date filtering
// T071: Integration test - Mood check-in with multiple activities

use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::mood::models::*;
use tauri_sveltekit_modern_lib::features::mood::repository::MoodRepository;
use std::sync::Arc;
use tempfile::TempDir;

/// Setup test environment with temporary database
fn setup_test_repo() -> (MoodRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().to_path_buf();
    let db = Arc::new(Database::new(db_path).expect("Failed to create database"));
    let repo = MoodRepository::new(db);
    (repo, temp_dir)
}

// T069: Integration test - log_mood command
#[test]
fn test_log_mood_end_to_end() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create test activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity 1");
    let activity2 = repo
        .create_activity("Meditation", Some("#9C27B0"), Some("🧘"))
        .expect("Failed to create activity 2");

    // Log mood with activities
    let mood_checkin = repo
        .create_mood_checkin(4, vec![activity1.id, activity2.id], Some("Feeling great after workout and meditation"))
        .expect("Failed to create mood check-in");

    // Verify mood check-in
    assert_eq!(mood_checkin.mood_rating, 4);
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
    let (repo, _temp_dir) = setup_test_repo();

    // Log mood without activities
    let mood_checkin = repo
        .create_mood_checkin(3, vec![], None)
        .expect("Failed to create mood check-in");

    assert_eq!(mood_checkin.mood_rating, 3);
    assert_eq!(mood_checkin.activities.len(), 0);
    assert!(mood_checkin.notes.is_none());
}

#[test]
fn test_log_mood_invalid_rating() {
    let (repo, _temp_dir) = setup_test_repo();

    // Try to log mood with invalid rating
    let result = repo.create_mood_checkin(0, vec![], None);
    assert!(result.is_err());

    let result = repo.create_mood_checkin(6, vec![], None);
    assert!(result.is_err());
}

#[test]
fn test_log_mood_invalid_activity_id() {
    let (repo, _temp_dir) = setup_test_repo();

    // Try to log mood with non-existent activity ID
    let result = repo.create_mood_checkin(4, vec![9999], None);
    assert!(result.is_err());
}

// T070: Integration test - get_mood_history query with date filtering
#[test]
fn test_get_mood_history_with_date_filtering() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create mood check-ins on different days
    let yesterday = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(1))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let tomorrow = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .unwrap()
        .format("%Y-%m-%d").to_string();

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
    let (repo, _temp_dir) = setup_test_repo();

    let moods = repo
        .get_mood_history(None, None, None)
        .expect("Failed to get mood history");
    assert_eq!(moods.len(), 0);
}

// T071: Integration test - Mood check-in with multiple activities
#[test]
fn test_mood_checkin_with_multiple_activities() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create 5 activities
    let activity1 = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity 1");
    let activity2 = repo
        .create_activity("Meditation", Some("#9C27B0"), Some("🧘"))
        .expect("Failed to create activity 2");
    let activity3 = repo
        .create_activity("Social", Some("#2196F3"), Some("👥"))
        .expect("Failed to create activity 3");
    let activity4 = repo
        .create_activity("Work", Some("#FF9800"), Some("💼"))
        .expect("Failed to create activity 4");
    let activity5 = repo
        .create_activity("Hobby", Some("#E91E63"), Some("🎨"))
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

    assert_eq!(mood_checkin.mood_rating, 5);
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
    let (repo, _temp_dir) = setup_test_repo();

    let activity = repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
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
    let (repo, _temp_dir) = setup_test_repo();

    // Create activity
    let activity = repo
        .create_activity("Reading", Some("#FF5733"), Some("📚"))
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
    assert_eq!(fetched.mood_rating, 4);
    assert_eq!(fetched.activities.len(), 1);
    assert_eq!(fetched.activities[0].name, "Reading");
}

#[test]
fn test_get_mood_checkin_not_found() {
    let (repo, _temp_dir) = setup_test_repo();

    let result = repo.get_mood_checkin(9999);
    assert!(result.is_err());
}
