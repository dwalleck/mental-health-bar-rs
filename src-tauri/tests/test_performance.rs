// Performance tests for Activity Groups feature
// Tests: 4.15-4.19 from REVISED-tasks.md
//
// These tests verify the system can handle realistic production loads:
// - 50 activity groups
// - 250 activities (5 per group)
// - 1000+ activity logs
// - Performance targets: <200ms for list queries, <500ms for reporting queries

use mental_health_bar::db::Database;
use mental_health_bar::features::activities::repository::ActivitiesRepository;
use std::sync::Arc;
use std::time::Instant;
use tempfile::TempDir;

/// Helper to set up test repository with database
fn setup_test_repo() -> (ActivitiesRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db = Arc::new(Database::new(temp_dir.path()).expect("Failed to create database"));
    (ActivitiesRepository::new(db), temp_dir)
}

#[test]
fn test_performance_50_groups() {
    let (repo, _temp_dir) = setup_test_repo();

    // Task 4.15: Create 50 activity groups
    println!("Creating 50 activity groups...");
    let start = Instant::now();

    for i in 1..=50 {
        repo.create_activity_group(
            &format!("Group {}", i),
            Some(&format!("Description for group {}", i)),
        )
        .expect("Failed to create group");
    }

    let creation_time = start.elapsed();
    println!("Created 50 groups in {:?}", creation_time);

    // Verify all groups are retrievable
    let start = Instant::now();
    let groups = repo.get_activity_groups().expect("Failed to get groups");
    let retrieval_time = start.elapsed();

    assert_eq!(groups.len(), 50, "Should have 50 groups");
    println!("Retrieved 50 groups in {:?}", retrieval_time);

    // Performance target: <200ms for activity list operations
    assert!(
        retrieval_time.as_millis() < 200,
        "Group retrieval took {:?}, expected <200ms",
        retrieval_time
    );
}

#[test]
fn test_performance_500_logs() {
    let (repo, _temp_dir) = setup_test_repo();

    // Task 4.16: Test with 500 activity logs

    // Setup: Create 10 groups with 5 activities each (50 activities total)
    println!("Setting up test data: 10 groups, 50 activities...");
    let mut activity_ids = Vec::new();

    for i in 1..=10 {
        let group = repo
            .create_activity_group(&format!("Group {}", i), None)
            .expect("Failed to create group");

        for j in 1..=5 {
            let activity = repo
                .create_activity(
                    group.id,
                    &format!("Activity {}-{}", i, j),
                    Some("#4CAF50"),
                    Some("✓"),
                )
                .expect("Failed to create activity");
            activity_ids.push(activity.id);
        }
    }

    // Create 500 activity logs distributed across activities
    println!("Creating 500 activity logs...");
    let start = Instant::now();

    for i in 0..500 {
        let activity_id = activity_ids[i % activity_ids.len()];
        let days_ago = i / 10; // Spread over ~50 days
        let logged_at = chrono::Utc::now() - chrono::Duration::days(days_ago as i64);

        repo.log_activity(activity_id, Some(logged_at.to_rfc3339()), None)
            .expect("Failed to log activity");
    }

    let creation_time = start.elapsed();
    println!("Created 500 logs in {:?}", creation_time);

    // Test retrieval performance
    let start = Instant::now();
    let logs = repo
        .get_activity_logs(None, None, None, Some(1000))
        .expect("Failed to get logs");
    let retrieval_time = start.elapsed();

    assert_eq!(logs.len(), 500, "Should have 500 logs");
    println!("Retrieved 500 logs in {:?}", retrieval_time);

    // Performance target: <200ms for log list
    assert!(
        retrieval_time.as_millis() < 200,
        "Log retrieval took {:?}, expected <200ms",
        retrieval_time
    );
}

#[test]
fn test_performance_reporting_queries_1000_logs() {
    let (repo, _temp_dir) = setup_test_repo();

    // Task 4.17: Test reporting queries with >1000 logs

    // Setup: Create groups and activities
    println!("Setting up test data for reporting queries...");
    let group = repo
        .create_activity_group("Performance Test Group", None)
        .expect("Failed to create group");

    let activity = repo
        .create_activity(group.id, "Test Activity", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");

    // Create 1200 logs over 120 days (10 per day)
    println!("Creating 1200 activity logs...");
    let start = Instant::now();

    for i in 0..1200 {
        let days_ago = i / 10;
        let logged_at = chrono::Utc::now() - chrono::Duration::days(days_ago as i64);

        repo.log_activity(activity.id, Some(logged_at.to_rfc3339()), None)
            .expect("Failed to log activity");
    }

    let creation_time = start.elapsed();
    println!("Created 1200 logs in {:?}", creation_time);

    // Test 1: Activity Frequency Query
    println!("\n--- Testing Activity Frequency Query ---");
    let start_date = (chrono::Utc::now() - chrono::Duration::days(30)).to_rfc3339();
    let end_date = chrono::Utc::now().to_rfc3339();

    let start = Instant::now();
    let frequency = repo
        .get_activity_frequency(activity.id, &start_date, &end_date)
        .expect("Failed to get frequency");
    let frequency_time = start.elapsed();

    println!(
        "Activity frequency: {} days/week (calculated in {:?})",
        frequency.days_per_week, frequency_time
    );

    // Performance target: <500ms for reporting queries
    assert!(
        frequency_time.as_millis() < 500,
        "Frequency query took {:?}, expected <500ms",
        frequency_time
    );

    // Test 2: Activity Trend Query
    println!("\n--- Testing Activity Trend Query ---");
    let start = Instant::now();
    let trend = repo
        .get_activity_trend(activity.id, 30, &end_date)
        .expect("Failed to get trend");
    let trend_time = start.elapsed();

    println!(
        "Activity trend: {:+.1}% change (calculated in {:?})",
        trend.percent_change, trend_time
    );

    assert!(
        trend_time.as_millis() < 500,
        "Trend query took {:?}, expected <500ms",
        trend_time
    );

    // Test 3: Set and Check Goal Progress
    println!("\n--- Testing Goal Progress Query ---");
    repo.set_activity_goal(
        Some(activity.id),
        None,
        "days_per_period",
        Some(20),
        None,
        30,
    )
    .expect("Failed to set goal");

    let start = Instant::now();
    let progress = repo
        .check_goal_progress(activity.id, None, &end_date)
        .expect("Failed to check progress");
    let progress_time = start.elapsed();

    println!(
        "Goal progress: {:.1}% ({}/{}) (calculated in {:?})",
        progress.percentage, progress.current_value, progress.target_value, progress_time
    );

    assert!(
        progress_time.as_millis() < 500,
        "Progress query took {:?}, expected <500ms",
        progress_time
    );

    // Test 4: Get all logs (worst case scenario)
    println!("\n--- Testing Full Log Retrieval ---");
    let start = Instant::now();
    let all_logs = repo
        .get_activity_logs(None, None, None, Some(2000))
        .expect("Failed to get all logs");
    let all_logs_time = start.elapsed();

    assert_eq!(all_logs.len(), 1200, "Should have 1200 logs");
    println!("Retrieved all 1200 logs in {:?}", all_logs_time);

    assert!(
        all_logs_time.as_millis() < 500,
        "Full log retrieval took {:?}, expected <500ms",
        all_logs_time
    );
}

#[test]
fn test_performance_concurrent_operations() {
    let (repo, _temp_dir) = setup_test_repo();

    // Additional performance test: Simulate realistic usage patterns
    println!("Testing concurrent-like operation patterns...");

    // Create realistic dataset: 20 groups, 100 activities, 500 logs
    let mut activity_ids = Vec::new();

    for i in 1..=20 {
        let group = repo
            .create_activity_group(&format!("Group {}", i), None)
            .expect("Failed to create group");

        for j in 1..=5 {
            let activity = repo
                .create_activity(
                    group.id,
                    &format!("Activity {}-{}", i, j),
                    Some("#4CAF50"),
                    None,
                )
                .expect("Failed to create activity");
            activity_ids.push(activity.id);
        }
    }

    // Create 500 logs
    for i in 0..500 {
        let activity_id = activity_ids[i % activity_ids.len()];
        let days_ago = i / 10;
        let logged_at = chrono::Utc::now() - chrono::Duration::days(days_ago as i64);

        repo.log_activity(activity_id, Some(logged_at.to_rfc3339()), None)
            .expect("Failed to log activity");
    }

    // Simulate typical user workflow: multiple rapid queries
    println!("\n--- Simulating typical user workflow ---");
    let workflow_start = Instant::now();

    // 1. Load groups list
    let groups = repo.get_activity_groups().expect("Failed to get groups");
    println!("Step 1: Loaded {} groups", groups.len());

    // 2. Load activities for first group
    let activities = repo
        .get_activities_by_group(groups[0].id)
        .expect("Failed to get activities");
    println!("Step 2: Loaded {} activities", activities.len());

    // 3. Load logs for first activity
    let logs = repo
        .get_activity_logs(Some(activities[0].id), None, None, Some(50))
        .expect("Failed to get logs");
    println!("Step 3: Loaded {} recent logs", logs.len());

    // 4. Check goal progress
    let end_date = chrono::Utc::now().to_rfc3339();
    let _ = repo.get_activity_frequency(activities[0].id, &end_date, &end_date);
    println!("Step 4: Calculated frequency");

    let workflow_time = workflow_start.elapsed();
    println!("\nComplete workflow took {:?}", workflow_time);

    // Entire workflow should complete quickly
    assert!(
        workflow_time.as_millis() < 1000,
        "Workflow took {:?}, expected <1000ms",
        workflow_time
    );
}

#[test]
fn test_performance_index_effectiveness() {
    let (repo, _temp_dir) = setup_test_repo();

    // Task 4.18: Verify indexes are effective
    // This test creates data patterns that would be slow without proper indexes

    println!("Testing index effectiveness with date-range queries...");

    let group = repo
        .create_activity_group("Index Test", None)
        .expect("Failed to create group");

    let activity = repo
        .create_activity(group.id, "Test Activity", None, None)
        .expect("Failed to create activity");

    // Create 1000 logs spread over 1 year
    for i in 0..1000 {
        let days_ago = i % 365;
        let logged_at = chrono::Utc::now() - chrono::Duration::days(days_ago as i64);
        repo.log_activity(activity.id, Some(logged_at.to_rfc3339()), None)
            .expect("Failed to log");
    }

    // Test 1: Query recent logs (should use idx_activity_logs_logged_at)
    let start = Instant::now();
    let from_date = (chrono::Utc::now() - chrono::Duration::days(7)).to_rfc3339();
    let recent_logs = repo
        .get_activity_logs(None, Some(&from_date), None, None)
        .expect("Failed to get recent logs");
    let recent_time = start.elapsed();

    println!(
        "Retrieved {} recent logs in {:?}",
        recent_logs.len(),
        recent_time
    );
    assert!(
        recent_time.as_millis() < 100,
        "Recent logs query took {:?}, should be fast with index",
        recent_time
    );

    // Test 2: Query logs for specific activity (should use idx_activity_logs_activity)
    let start = Instant::now();
    let activity_logs = repo
        .get_activity_logs(Some(activity.id), None, None, None)
        .expect("Failed to get activity logs");
    let activity_time = start.elapsed();

    println!(
        "Retrieved {} activity logs in {:?}",
        activity_logs.len(),
        activity_time
    );
    assert_eq!(activity_logs.len(), 1000);
    assert!(
        activity_time.as_millis() < 100,
        "Activity logs query took {:?}, should be fast with index",
        activity_time
    );

    // Test 3: Exclude soft-deleted (should use partial index idx_activity_logs_deleted)
    let start = Instant::now();
    let active_logs = repo
        .get_activity_logs(None, None, None, None)
        .expect("Failed to get active logs");
    let active_time = start.elapsed();

    println!(
        "Retrieved {} active logs in {:?}",
        active_logs.len(),
        active_time
    );
    assert!(
        active_time.as_millis() < 100,
        "Active logs query took {:?}, should be fast with partial index",
        active_time
    );
}
