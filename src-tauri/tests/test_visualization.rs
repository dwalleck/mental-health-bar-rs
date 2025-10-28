// Integration tests for visualization feature (User Stories 4 & 5)
// T118: Integration test - get_assessment_chart_data query with time ranges
// T119: Integration test - Chart data aggregation for year+ data
// T136: Integration test - get_mood_chart_data query
// T137: Integration test - Activity correlation calculation

use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::assessments::repository::AssessmentRepository;
use tauri_sveltekit_modern_lib::features::mood::repository::MoodRepository;
use tauri_sveltekit_modern_lib::features::visualization::models::{
    TimeRange, TrendDirection, VisualizationError,
};
use tauri_sveltekit_modern_lib::features::visualization::repository::VisualizationRepository;
use tempfile::TempDir;

/// Setup test environment with temporary database
fn setup_test_repo() -> (
    VisualizationRepository,
    AssessmentRepository,
    MoodRepository,
    TempDir,
) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().to_path_buf();
    let db = Arc::new(Database::new(db_path).expect("Failed to create database"));

    let viz_repo = VisualizationRepository::new(db.clone());
    let assessment_repo = AssessmentRepository::new(db.clone());
    let mood_repo = MoodRepository::new(db.clone());

    (viz_repo, assessment_repo, mood_repo, temp_dir)
}

// T118: Integration test - get_assessment_chart_data query with time ranges
#[test]
fn test_get_assessment_chart_data_with_week_range() {
    let (viz_repo, assessment_repo, _, _temp_dir) = setup_test_repo();

    // Create several PHQ-9 assessments over the past week
    for i in 0..5 {
        let score = 5 + i * 2;

        // Create assessment response (using PHQ-9 type id=1)
        let responses: Vec<i32> = vec![1; 9]; // Simple responses that sum to score
        let total_score = score as i32;
        let severity = "mild"; // Simplified for test
        assessment_repo
            .save_assessment(1, &responses, total_score, severity, None)
            .expect(&format!("Failed to create assessment {}", i));
    }

    // Query with Week time range
    let time_range = TimeRange::Week;
    let (from_date, to_date) = time_range.to_date_range().unwrap();

    let chart_data = viz_repo
        .get_assessment_chart_data("PHQ9", Some(&from_date), Some(&to_date))
        .expect("Failed to get chart data");

    // Verify results
    assert_eq!(chart_data.data_points.len(), 5);
    assert_eq!(chart_data.assessment_type.code, "PHQ9");
    assert_eq!(chart_data.statistics.total_assessments, 5);
    assert!(chart_data.statistics.min > 0.0);
    assert!(chart_data.statistics.max > chart_data.statistics.min);
    assert!(!chart_data.thresholds.is_empty());
}

#[test]
fn test_get_assessment_chart_data_with_custom_range() {
    let (viz_repo, assessment_repo, _, _temp_dir) = setup_test_repo();

    // Create assessments
    for i in 0..3 {
        let responses: Vec<i32> = vec![1; 9];
        let total_score = (i as i32 + 1) * 9; // Score increases each time
        let severity = "mild";
        assessment_repo
            .save_assessment(1, &responses, total_score, severity, None)
            .expect(&format!("Failed to create assessment {}", i));
    }

    // Query with AllTime (no date filter)
    let chart_data = viz_repo
        .get_assessment_chart_data("PHQ9", None, None)
        .expect("Failed to get chart data");

    assert_eq!(chart_data.data_points.len(), 3);
}

#[test]
fn test_get_assessment_chart_data_invalid_type() {
    let (viz_repo, _, _, _temp_dir) = setup_test_repo();

    let result = viz_repo.get_assessment_chart_data("INVALID", None, None);
    assert!(result.is_err());

    // Verify it's the correct error variant
    match result.unwrap_err() {
        VisualizationError::InvalidAssessmentType(code) => {
            assert_eq!(code, "INVALID");
        }
        e => panic!("Expected InvalidAssessmentType error, got: {:?}", e),
    }
}

#[test]
fn test_get_assessment_chart_data_no_data() {
    let (viz_repo, _, _, _temp_dir) = setup_test_repo();

    // Query with no assessments in database
    let result = viz_repo.get_assessment_chart_data("PHQ9", None, None);
    assert!(result.is_err());

    // Verify it's the NoData error
    match result.unwrap_err() {
        VisualizationError::NoData => {
            // Expected
        }
        e => panic!("Expected NoData error, got: {:?}", e),
    }
}

#[test]
fn test_assessment_chart_statistics_calculation() {
    let (viz_repo, assessment_repo, _, _temp_dir) = setup_test_repo();

    // Create assessments with known scores: 5, 10, 15, 20
    for score in [5, 10, 15, 20] {
        let responses: Vec<i32> = vec![1; 9];
        let severity = match score {
            5 => "minimal",
            10 => "mild",
            15 => "moderate",
            20 => "moderate",
            _ => "mild",
        };

        assessment_repo
            .save_assessment(1, &responses, score, severity, None)
            .expect("Failed to create assessment");
    }

    let chart_data = viz_repo
        .get_assessment_chart_data("PHQ9", None, None)
        .expect("Failed to get chart data");

    // Verify statistics
    assert_eq!(chart_data.statistics.min, 5.0);
    assert_eq!(chart_data.statistics.max, 20.0);
    assert_eq!(chart_data.statistics.total_assessments, 4);

    // Average should be (5 + 10 + 15 + 20) / 4 = 12.5
    assert!((chart_data.statistics.average - 12.5).abs() < 0.1);
}

#[test]
fn test_assessment_chart_trend_improving() {
    let (viz_repo, assessment_repo, _, _temp_dir) = setup_test_repo();

    // Create assessments with decreasing scores (improving trend)
    // 20 -> 10 is 50% reduction (> 20% threshold)
    for score in [20, 18, 15, 12, 10] {
        let responses: Vec<i32> = vec![1; 9];
        let severity = if score >= 15 { "moderate" } else { "mild" };

        assessment_repo
            .save_assessment(1, &responses, score, severity, None)
            .expect("Failed to create assessment");
    }

    let chart_data = viz_repo
        .get_assessment_chart_data("PHQ9", None, None)
        .expect("Failed to get chart data");

    // Should show improving trend (scores decreasing)
    assert!(matches!(
        chart_data.statistics.trend,
        TrendDirection::Improving
    ));
}

#[test]
fn test_assessment_chart_trend_worsening() {
    let (viz_repo, assessment_repo, _, _temp_dir) = setup_test_repo();

    // Create assessments with increasing scores (worsening trend)
    // 5 -> 15 is 200% increase (> 20% threshold)
    for score in [5, 7, 10, 12, 15] {
        let responses: Vec<i32> = vec![1; 9];
        let severity = if score >= 10 { "moderate" } else { "minimal" };

        assessment_repo
            .save_assessment(1, &responses, score, severity, None)
            .expect("Failed to create assessment");
    }

    let chart_data = viz_repo
        .get_assessment_chart_data("PHQ9", None, None)
        .expect("Failed to get chart data");

    // Should show worsening trend (scores increasing)
    assert!(matches!(
        chart_data.statistics.trend,
        TrendDirection::Worsening
    ));
}

#[test]
fn test_assessment_chart_trend_stable() {
    let (viz_repo, assessment_repo, _, _temp_dir) = setup_test_repo();

    // Create assessments with stable scores (< 20% change)
    // 10 -> 11 is 10% change (< 20% threshold)
    for score in [10, 10, 11, 10, 11] {
        let responses: Vec<i32> = vec![1; 9];
        let severity = "mild";

        assessment_repo
            .save_assessment(1, &responses, score, severity, None)
            .expect("Failed to create assessment");
    }

    let chart_data = viz_repo
        .get_assessment_chart_data("PHQ9", None, None)
        .expect("Failed to get chart data");

    // Should show stable trend (minimal change)
    assert!(matches!(
        chart_data.statistics.trend,
        TrendDirection::Stable
    ));
}

#[test]
fn test_assessment_chart_thresholds_included() {
    let (viz_repo, assessment_repo, _, _temp_dir) = setup_test_repo();

    // Create one assessment
    let responses: Vec<i32> = vec![1; 9];
    let total_score = 9;
    let severity = "mild";
    assessment_repo
        .save_assessment(1, &responses, total_score, severity, None)
        .expect("Failed to create assessment");

    let chart_data = viz_repo
        .get_assessment_chart_data("PHQ9", None, None)
        .expect("Failed to get chart data");

    // Verify thresholds are present
    assert!(!chart_data.thresholds.is_empty());

    // PHQ-9 should have thresholds for minimal, mild, moderate, moderately_severe, severe
    assert!(chart_data.thresholds.len() >= 3);

    // Verify threshold structure
    let first_threshold = &chart_data.thresholds[0];
    assert!(!first_threshold.label.is_empty());
    assert!(first_threshold.value > 0.0);
    assert!(first_threshold.color.starts_with('#'));
}

// T119: Integration test - Chart data aggregation for year+ data
#[test]
fn test_chart_data_aggregation_large_dataset() {
    let (viz_repo, assessment_repo, _, _temp_dir) = setup_test_repo();

    // Create 100 assessments (simulating ~3 months of data)
    for i in 0..100 {
        let score = (i % 27) as i32; // Cycle through possible scores
        let responses: Vec<i32> = vec![1; 9];
        let severity = match score {
            0..=4 => "minimal",
            5..=9 => "mild",
            10..=14 => "moderate",
            15..=19 => "moderately_severe",
            _ => "severe",
        };

        assessment_repo
            .save_assessment(1, &responses, score, severity, None)
            .expect(&format!("Failed to create assessment {}", i));
    }

    let chart_data = viz_repo
        .get_assessment_chart_data("PHQ9", None, None)
        .expect("Failed to get chart data");

    // Verify all data points are returned
    assert_eq!(chart_data.data_points.len(), 100);
    assert_eq!(chart_data.statistics.total_assessments, 100);

    // Statistics should be calculated correctly
    assert!(chart_data.statistics.min >= 0.0);
    assert!(chart_data.statistics.max <= 27.0); // PHQ-9 max score
    assert!(chart_data.statistics.average >= 0.0 && chart_data.statistics.average <= 27.0);
}

// T136: Integration test - get_mood_chart_data query
#[test]
fn test_get_mood_chart_data_basic() {
    let (viz_repo, _, mood_repo, _temp_dir) = setup_test_repo();

    // Create mood check-ins
    for rating in [3, 4, 5, 2, 4] {
        mood_repo
            .create_mood_checkin(rating, vec![], None)
            .expect("Failed to create mood check-in");
    }

    let mood_data = viz_repo
        .get_mood_chart_data(None, None, false)
        .expect("Failed to get mood chart data");

    // Verify results
    assert_eq!(mood_data.data_points.len(), 5);
    assert_eq!(mood_data.statistics.total_checkins, 5);
    assert_eq!(mood_data.statistics.min, 2);
    assert_eq!(mood_data.statistics.max, 5);

    // Average should be (3+4+5+2+4)/5 = 3.6
    assert!((mood_data.statistics.average - 3.6).abs() < 0.1);
}

#[test]
fn test_get_mood_chart_data_with_time_range() {
    let (viz_repo, _, mood_repo, _temp_dir) = setup_test_repo();

    // Create mood check-ins
    for rating in [3, 4, 5, 2, 4] {
        mood_repo
            .create_mood_checkin(rating, vec![], None)
            .expect("Failed to create mood check-in");
    }

    // Query with Week time range
    let time_range = TimeRange::Week;
    let (from_date, to_date) = time_range.to_date_range().unwrap();

    let mood_data = viz_repo
        .get_mood_chart_data(Some(&from_date), Some(&to_date), false)
        .expect("Failed to get mood chart data");

    assert_eq!(mood_data.data_points.len(), 5);
}

#[test]
fn test_get_mood_chart_data_no_data() {
    let (viz_repo, _, _, _temp_dir) = setup_test_repo();

    let result = viz_repo.get_mood_chart_data(None, None, false);
    assert!(result.is_err());

    // Verify it's the NoData error
    match result.unwrap_err() {
        VisualizationError::NoData => {
            // Expected
        }
        e => panic!("Expected NoData error, got: {:?}", e),
    }
}

#[test]
fn test_mood_statistics_calculation() {
    let (viz_repo, _, mood_repo, _temp_dir) = setup_test_repo();

    // Create mood check-ins: 1, 2, 3, 3, 3, 4, 5
    // Mode should be 3 (appears 3 times)
    for rating in [1, 2, 3, 3, 3, 4, 5] {
        mood_repo
            .create_mood_checkin(rating, vec![], None)
            .expect("Failed to create mood check-in");
    }

    let mood_data = viz_repo
        .get_mood_chart_data(None, None, false)
        .expect("Failed to get mood chart data");

    assert_eq!(mood_data.statistics.min, 1);
    assert_eq!(mood_data.statistics.max, 5);
    assert_eq!(mood_data.statistics.mode, 3); // Most common
    assert_eq!(mood_data.statistics.median, 3.0); // Middle value

    // Average: (1+2+3+3+3+4+5)/7 = 21/7 = 3.0
    assert!((mood_data.statistics.average - 3.0).abs() < 0.1);
}

// T137: Integration test - Activity correlation calculation
#[test]
fn test_activity_correlation_calculation() {
    let (viz_repo, _, mood_repo, _temp_dir) = setup_test_repo();

    // Create activities
    let exercise = mood_repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create Exercise activity");

    let meditation = mood_repo
        .create_activity("Meditation", Some("#9C27B0"), Some("🧘"))
        .expect("Failed to create Meditation activity");

    let work = mood_repo
        .create_activity("Work", Some("#FF9800"), Some("💼"))
        .expect("Failed to create Work activity");

    // Create mood check-ins with activities
    // Exercise: ratings 5, 5, 4 (avg 4.67)
    mood_repo
        .create_mood_checkin(5, vec![exercise.id], None)
        .unwrap();
    mood_repo
        .create_mood_checkin(5, vec![exercise.id], None)
        .unwrap();
    mood_repo
        .create_mood_checkin(4, vec![exercise.id], None)
        .unwrap();

    // Meditation: ratings 4, 4 (avg 4.0)
    mood_repo
        .create_mood_checkin(4, vec![meditation.id], None)
        .unwrap();
    mood_repo
        .create_mood_checkin(4, vec![meditation.id], None)
        .unwrap();

    // Work: ratings 2, 2, 3 (avg 2.33)
    mood_repo
        .create_mood_checkin(2, vec![work.id], None)
        .unwrap();
    mood_repo
        .create_mood_checkin(2, vec![work.id], None)
        .unwrap();
    mood_repo
        .create_mood_checkin(3, vec![work.id], None)
        .unwrap();

    // Query with activity breakdown
    let mood_data = viz_repo
        .get_mood_chart_data(None, None, true)
        .expect("Failed to get mood chart data");

    // Verify activity breakdown
    assert_eq!(mood_data.activity_breakdown.len(), 3);

    // Activities should be sorted by average mood (descending)
    assert_eq!(mood_data.activity_breakdown[0].activity.name, "Exercise");
    assert!((mood_data.activity_breakdown[0].average_mood - 4.67).abs() < 0.1);

    assert_eq!(mood_data.activity_breakdown[1].activity.name, "Meditation");
    assert!((mood_data.activity_breakdown[1].average_mood - 4.0).abs() < 0.1);

    assert_eq!(mood_data.activity_breakdown[2].activity.name, "Work");
    assert!((mood_data.activity_breakdown[2].average_mood - 2.33).abs() < 0.1);
}

#[test]
fn test_activity_correlation_minimum_sample_size() {
    let (viz_repo, _, mood_repo, _temp_dir) = setup_test_repo();

    // Create activity
    let reading = mood_repo
        .create_activity("Reading", Some("#FF5733"), Some("📚"))
        .expect("Failed to create Reading activity");

    // Create only 1 mood check-in (below minimum of 2)
    mood_repo
        .create_mood_checkin(5, vec![reading.id], None)
        .expect("Failed to create mood check-in");

    // Query with activity breakdown
    let mood_data = viz_repo
        .get_mood_chart_data(None, None, true)
        .expect("Failed to get mood chart data");

    // Should not include activity with only 1 check-in
    assert_eq!(mood_data.activity_breakdown.len(), 0);
}

#[test]
fn test_activity_correlation_ignores_deleted_activities() {
    let (viz_repo, _, mood_repo, _temp_dir) = setup_test_repo();

    // Create activity
    let deleted_activity = mood_repo
        .create_activity("Deleted", Some("#000000"), Some("❌"))
        .expect("Failed to create activity");

    // Create mood check-ins
    mood_repo
        .create_mood_checkin(5, vec![deleted_activity.id], None)
        .unwrap();
    mood_repo
        .create_mood_checkin(4, vec![deleted_activity.id], None)
        .unwrap();

    // Soft delete the activity
    mood_repo
        .delete_activity(deleted_activity.id)
        .expect("Failed to delete activity");

    // Query with activity breakdown
    let mood_data = viz_repo
        .get_mood_chart_data(None, None, true)
        .expect("Failed to get mood chart data");

    // Should not include deleted activity
    assert_eq!(mood_data.activity_breakdown.len(), 0);
}

#[test]
fn test_mood_chart_without_activity_breakdown() {
    let (viz_repo, _, mood_repo, _temp_dir) = setup_test_repo();

    // Create activities
    let exercise = mood_repo
        .create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");

    // Create mood check-ins with activities
    mood_repo
        .create_mood_checkin(5, vec![exercise.id], None)
        .unwrap();
    mood_repo
        .create_mood_checkin(4, vec![exercise.id], None)
        .unwrap();

    // Query WITHOUT activity breakdown
    let mood_data = viz_repo
        .get_mood_chart_data(None, None, false)
        .expect("Failed to get mood chart data");

    // Activity breakdown should be empty
    assert_eq!(mood_data.activity_breakdown.len(), 0);

    // But data points should still be present
    assert_eq!(mood_data.data_points.len(), 2);
}
