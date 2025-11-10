// Integration tests for assessment business logic end-to-end
// Tests the full flow: save assessment → retrieve → validate scoring
use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::assessments::models::*;
use tauri_sveltekit_modern_lib::features::assessments::repository::AssessmentRepository;
use tempfile::TempDir;

fn setup_test_repo() -> (AssessmentRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().to_path_buf();
    let db = Arc::new(Database::new(db_path).expect("Failed to create database"));

    let repo = AssessmentRepository::new(db);

    (repo, temp_dir)
}

#[test]
fn test_submit_assessment_phq9_end_to_end() {
    let (repo, _temp_dir) = setup_test_repo();

    // Get PHQ-9 assessment type
    let assessment_type = repo
        .get_assessment_type_by_code("PHQ9")
        .expect("Failed to get PHQ9");

    // Create responses
    let responses = vec![1, 2, 1, 0, 1, 2, 1, 0, 1]; // Score = 9

    // Calculate score using business logic
    let total_score = calculate_phq9_score(&responses).expect("Failed to calculate score");
    let severity_level = get_phq9_severity(total_score);

    assert_eq!(total_score, 9);
    assert_eq!(severity_level, "mild");

    // Save to database
    let id = repo
        .save_assessment(
            assessment_type.id,
            &responses,
            total_score,
            severity_level,
            Some("Integration test notes".to_string()),
        )
        .expect("Failed to save assessment");

    // Retrieve and verify
    let retrieved = repo
        .get_assessment_response(id)
        .expect("Failed to retrieve assessment");

    assert_eq!(retrieved.id, id);
    assert_eq!(retrieved.assessment_type.code, "PHQ9");
    assert_eq!(retrieved.responses, responses);
    assert_eq!(retrieved.total_score, 9);
    assert_eq!(retrieved.severity_level, "mild");
    assert_eq!(retrieved.notes, Some("Integration test notes".to_string()));
}

#[test]
fn test_submit_all_assessment_types_end_to_end() {
    let (repo, _temp_dir) = setup_test_repo();

    // Test PHQ-9 (9 questions, 0-3 each, severe score)
    let phq9 = repo.get_assessment_type_by_code("PHQ9").unwrap();
    let phq9_responses = vec![3, 3, 3, 3, 3, 3, 3, 3, 3]; // Score = 27
    let phq9_score = calculate_phq9_score(&phq9_responses).unwrap();
    assert_eq!(phq9_score, 27);
    assert_eq!(get_phq9_severity(phq9_score), "severe");

    let phq9_id = repo
        .save_assessment(
            phq9.id,
            &phq9_responses,
            phq9_score,
            get_phq9_severity(phq9_score),
            None,
        )
        .expect("Failed to save PHQ9");

    // Test GAD-7 (7 questions, 0-3 each, moderate score)
    let gad7 = repo.get_assessment_type_by_code("GAD7").unwrap();
    let gad7_responses = vec![2, 2, 2, 2, 2, 2, 2]; // Score = 14
    let gad7_score = calculate_gad7_score(&gad7_responses).unwrap();
    assert_eq!(gad7_score, 14);
    assert_eq!(get_gad7_severity(gad7_score), "moderate");

    let gad7_id = repo
        .save_assessment(
            gad7.id,
            &gad7_responses,
            gad7_score,
            get_gad7_severity(gad7_score),
            None,
        )
        .expect("Failed to save GAD7");

    // Test CES-D (20 questions, 0-3 each, mild score)
    let cesd = repo.get_assessment_type_by_code("CESD").unwrap();
    let cesd_responses = vec![1; 20]; // Score = 20
    let cesd_score = calculate_cesd_score(&cesd_responses).unwrap();
    assert_eq!(cesd_score, 20);
    assert_eq!(get_cesd_severity(cesd_score), "mild");

    let cesd_id = repo
        .save_assessment(
            cesd.id,
            &cesd_responses,
            cesd_score,
            get_cesd_severity(cesd_score),
            None,
        )
        .expect("Failed to save CESD");

    // Test OASIS (5 questions, 0-4 each, moderate score)
    let oasis = repo.get_assessment_type_by_code("OASIS").unwrap();
    let oasis_responses = vec![2, 2, 2, 2, 2]; // Score = 10
    let oasis_score = calculate_oasis_score(&oasis_responses).unwrap();
    assert_eq!(oasis_score, 10);
    assert_eq!(get_oasis_severity(oasis_score), "moderate");

    let oasis_id = repo
        .save_assessment(
            oasis.id,
            &oasis_responses,
            oasis_score,
            get_oasis_severity(oasis_score),
            None,
        )
        .expect("Failed to save OASIS");

    // Verify all assessments can be retrieved
    assert!(repo.get_assessment_response(phq9_id).is_ok());
    assert!(repo.get_assessment_response(gad7_id).is_ok());
    assert!(repo.get_assessment_response(cesd_id).is_ok());
    assert!(repo.get_assessment_response(oasis_id).is_ok());
}

#[test]
fn test_get_assessment_history_query_end_to_end() {
    let (repo, _temp_dir) = setup_test_repo();

    // Initially should be empty
    let initial_history = repo
        .get_assessment_history(None, None, None, None)
        .expect("Failed to get initial history");
    assert_eq!(initial_history.len(), 0);

    // Get assessment types
    let phq9 = repo.get_assessment_type_by_code("PHQ9").unwrap();
    let gad7 = repo.get_assessment_type_by_code("GAD7").unwrap();

    // Submit multiple assessments
    repo.save_assessment(phq9.id, &vec![1; 9], 9, "mild", None)
        .expect("Failed to save first PHQ9");

    repo.save_assessment(gad7.id, &vec![2; 7], 14, "moderate", None)
        .expect("Failed to save GAD7");

    repo.save_assessment(phq9.id, &vec![2; 9], 18, "moderately_severe", None)
        .expect("Failed to save second PHQ9");

    // Get all history
    let all_history = repo
        .get_assessment_history(None, None, None, None)
        .expect("Failed to get all history");
    assert_eq!(all_history.len(), 3);

    // Filter by assessment type
    let phq9_history = repo
        .get_assessment_history(Some("PHQ9".to_string()), None, None, None)
        .expect("Failed to get PHQ9 history");
    assert_eq!(phq9_history.len(), 2);
    assert!(phq9_history
        .iter()
        .all(|a| a.assessment_type.code == "PHQ9"));

    let gad7_history = repo
        .get_assessment_history(Some("GAD7".to_string()), None, None, None)
        .expect("Failed to get GAD7 history");
    assert_eq!(gad7_history.len(), 1);
    assert_eq!(gad7_history[0].assessment_type.code, "GAD7");

    // Test limit
    let limited_history = repo
        .get_assessment_history(None, None, None, Some(2))
        .expect("Failed to get limited history");
    assert_eq!(limited_history.len(), 2);

    // Verify history is ordered by completed_at DESC (most recent first)
    // The second PHQ9 should come before the first PHQ9
    let phq9_ordered = repo
        .get_assessment_history(Some("PHQ9".to_string()), None, None, None)
        .expect("Failed to get ordered PHQ9 history");
    assert_eq!(phq9_ordered[0].total_score, 18); // Second submission
    assert_eq!(phq9_ordered[1].total_score, 9); // First submission
}

#[test]
fn test_get_assessment_history_with_date_filtering() {
    let (repo, _temp_dir) = setup_test_repo();

    // Submit an assessment
    let phq9 = repo.get_assessment_type_by_code("PHQ9").unwrap();
    repo.save_assessment(phq9.id, &vec![1; 9], 9, "mild", None)
        .expect("Failed to submit assessment");

    // Get dates for filtering - using wider margins to avoid timezone issues
    let yesterday = (chrono::Local::now() - chrono::Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();
    let next_week = (chrono::Local::now() + chrono::Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();

    // Filter from yesterday onwards - should include today's assessment
    let from_yesterday = repo
        .get_assessment_history(None, Some(yesterday.clone()), None, None)
        .expect("Failed to get history from yesterday");
    assert_eq!(
        from_yesterday.len(),
        1,
        "Should find assessment from yesterday onwards"
    );

    // Filter up to next week - should include today's assessment
    let until_next_week = repo
        .get_assessment_history(None, None, Some(next_week.clone()), None)
        .expect("Failed to get history until next week");
    assert_eq!(
        until_next_week.len(),
        1,
        "Should find assessment until next week"
    );

    // Filter for combined date range (yesterday to next week) - should include today
    let date_range = repo
        .get_assessment_history(None, Some(yesterday), Some(next_week), None)
        .expect("Failed to get history with date range");
    assert_eq!(date_range.len(), 1, "Should find assessment in date range");

    // Filter for future date range - should be empty
    let future_start = (chrono::Local::now() + chrono::Duration::days(30))
        .format("%Y-%m-%d")
        .to_string();
    let future_end = (chrono::Local::now() + chrono::Duration::days(60))
        .format("%Y-%m-%d")
        .to_string();
    let future_history = repo
        .get_assessment_history(None, Some(future_start), Some(future_end), None)
        .expect("Failed to get future history");
    assert_eq!(future_history.len(), 0, "Future date range should be empty");
}

#[test]
fn test_delete_assessment_end_to_end() {
    let (repo, _temp_dir) = setup_test_repo();

    // Submit an assessment
    let phq9 = repo.get_assessment_type_by_code("PHQ9").unwrap();
    let id = repo
        .save_assessment(phq9.id, &vec![1; 9], 9, "mild", None)
        .expect("Failed to save assessment");

    // Verify it exists
    assert!(repo.get_assessment_response(id).is_ok());

    // Delete it
    repo.delete_assessment(id)
        .expect("Failed to delete assessment");

    // Verify it's gone
    assert!(repo.get_assessment_response(id).is_err());
}

#[test]
fn test_invalid_assessment_type_code() {
    let (repo, _temp_dir) = setup_test_repo();

    let result = repo.get_assessment_type_by_code("INVALID");

    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Invalid assessment type"));
}

#[test]
fn test_scoring_validation_errors() {
    // Test PHQ-9 with incomplete responses
    let result = calculate_phq9_score(&vec![1, 2, 3]);
    assert!(result.is_err());

    // Test PHQ-9 with invalid response values
    let result = calculate_phq9_score(&vec![1, 2, 3, 4, 0, 0, 0, 0, 0]);
    assert!(result.is_err());

    // Test GAD-7 with incomplete responses
    let result = calculate_gad7_score(&vec![1, 2]);
    assert!(result.is_err());

    // Test OASIS with invalid response values (must be 0-4, not 0-3)
    let result = calculate_oasis_score(&vec![0, 1, 2, 3, 5]); // 5 is invalid
    assert!(result.is_err());
}

// ============================================================================
// P0 TESTS - Query Edge Cases (T150p, T150q)
// ============================================================================

// T150p: Test get_assessment_history with from_date > to_date
#[test]
fn test_get_assessment_history_reversed_date_range() {
    let (repo, _temp_dir) = setup_test_repo();

    // Get PHQ-9 assessment type
    let phq9_type = repo
        .get_assessment_type_by_code("PHQ9")
        .expect("Failed to get PHQ-9");

    // Create some assessments
    let responses1 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
    repo.save_assessment(phq9_type.id, &responses1, 9, "minimal", None)
        .expect("Failed to save assessment 1");

    let responses2 = vec![2, 2, 2, 2, 2, 2, 2, 2, 2];
    repo.save_assessment(phq9_type.id, &responses2, 18, "mild", None)
        .expect("Failed to save assessment 2");

    // Query with from_date > to_date (reversed range)
    let from_date = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    let to_date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let result = repo.get_assessment_history(
        Some("PHQ9".to_string()),
        Some(from_date),
        Some(to_date),
        None,
    );

    // Should either return empty results or error
    match result {
        Ok(assessments) => {
            assert_eq!(
                assessments.len(),
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

// T150q: Test queries with SQL injection attempts
#[test]
fn test_sql_injection_protection() {
    let (repo, _temp_dir) = setup_test_repo();

    // Test SQL injection in assessment type code
    let sql_injections = vec![
        "PHQ9'; DROP TABLE assessments; --",
        "PHQ9' OR '1'='1",
        "PHQ9'; DELETE FROM assessment_responses WHERE '1'='1",
        "' UNION SELECT * FROM assessment_types --",
        "PHQ9\"; DROP TABLE assessments; --",
    ];

    for injection in sql_injections {
        // Attempt to get assessment type with SQL injection
        let result = repo.get_assessment_type_by_code(injection);

        // Should either return NotFound error or safely handle it
        match result {
            Ok(_) => {
                panic!(
                    "SQL injection should not succeed in finding an assessment: '{}'",
                    injection
                );
            }
            Err(e) => {
                // Should be a normal "not found" error, not a SQL error
                let error_msg = format!("{}", e);
                assert!(
                    !error_msg.contains("SQL") && !error_msg.contains("syntax"),
                    "Error should not expose SQL details: {}",
                    error_msg
                );
            }
        }
    }

    // Test SQL injection in notes field
    let phq9_type = repo
        .get_assessment_type_by_code("PHQ9")
        .expect("Failed to get PHQ-9");

    let malicious_notes = "Test'; DROP TABLE assessment_responses; --";
    let responses = vec![1; 9];
    let result = repo.save_assessment(
        phq9_type.id,
        &responses,
        9,
        "minimal",
        Some(malicious_notes.to_string()),
    );

    // Should succeed (parameterized queries should sanitize)
    assert!(
        result.is_ok(),
        "Parameterized queries should handle SQL injection attempts safely"
    );

    // Verify the ID was returned
    let saved_id = result.unwrap();
    assert!(saved_id > 0, "Should return valid assessment ID");

    // Verify database integrity - tables should still exist
    let history = repo.get_assessment_history(Some("PHQ9".to_string()), None, None, None);
    assert!(
        history.is_ok(),
        "Database should remain intact after SQL injection attempt"
    );
}

// ============================================================================
// INPUT TRIMMING TESTS
// ============================================================================

#[test]
fn test_save_assessment_trims_notes() {
    let (repo, _temp_dir) = setup_test_repo();

    // Test notes with surrounding whitespace
    let assessment_id = repo
        .save_assessment(
            1, // PHQ9
            &[2, 2, 2, 2, 2, 2, 2, 2, 2],
            18,
            "moderate",
            Some("  Feeling much better today!  ".to_string()),
        )
        .expect("Failed to save assessment with trimmed notes");

    // Retrieve and verify notes are trimmed
    let assessment = repo
        .get_assessment_response(assessment_id)
        .expect("Failed to get assessment");

    assert_eq!(
        assessment.notes,
        Some("Feeling much better today!".to_string()),
        "Notes should be trimmed"
    );
}

#[test]
fn test_save_assessment_whitespace_only_notes_becomes_none() {
    let (repo, _temp_dir) = setup_test_repo();

    // Test notes with only whitespace
    let assessment_id = repo
        .save_assessment(
            1, // PHQ9
            &[1, 1, 1, 1, 1, 1, 1, 1, 1],
            9,
            "mild",
            Some("     ".to_string()),
        )
        .expect("Failed to save assessment");

    // Retrieve and verify whitespace-only notes become None
    let assessment = repo
        .get_assessment_response(assessment_id)
        .expect("Failed to get assessment");

    assert_eq!(
        assessment.notes, None,
        "Whitespace-only notes should become None"
    );
}
