// Integration tests for assessment repository
use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::assessments::repository::AssessmentRepository;
use tauri_sveltekit_modern_lib::types::assessment::{AssessmentStatus, SeverityLevel};
use tempfile::TempDir;

fn setup_test_db() -> (Arc<Database>, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().to_path_buf();
    let db = Database::new(db_path).expect("Failed to create database");

    (Arc::new(db), temp_dir)
}

#[test]
fn test_get_all_assessment_types() {
    let (db, _temp_dir) = setup_test_db();
    let repo = AssessmentRepository::new(db);

    let types = repo
        .get_assessment_types()
        .expect("Failed to get assessment types");

    // Should have 4 assessment types seeded
    assert_eq!(types.len(), 4);

    // Verify types are present
    let codes: Vec<String> = types.iter().map(|t| t.code.clone()).collect();
    assert!(codes.contains(&"PHQ9".to_string()));
    assert!(codes.contains(&"GAD7".to_string()));
    assert!(codes.contains(&"CESD".to_string()));
    assert!(codes.contains(&"OASIS".to_string()));
}

#[test]
fn test_get_assessment_type_by_code() {
    let (db, _temp_dir) = setup_test_db();
    let repo = AssessmentRepository::new(db);

    let assessment_type = repo
        .get_assessment_type_by_code("PHQ9")
        .expect("Failed to get PHQ9 assessment type");

    assert_eq!(assessment_type.code, "PHQ9");
    assert_eq!(assessment_type.name, "Patient Health Questionnaire-9");
    assert_eq!(assessment_type.question_count, 9);
    assert_eq!(assessment_type.min_score, 0);
    assert_eq!(assessment_type.max_score, 27);
}

#[test]
fn test_get_assessment_type_by_invalid_code() {
    let (db, _temp_dir) = setup_test_db();
    let repo = AssessmentRepository::new(db);

    let result = repo.get_assessment_type_by_code("INVALID");

    assert!(result.is_err());
}

#[test]
fn test_save_and_retrieve_assessment() {
    let (db, _temp_dir) = setup_test_db();
    let repo = AssessmentRepository::new(db);

    // Get PHQ9 assessment type
    let assessment_type = repo
        .get_assessment_type_by_code("PHQ9")
        .expect("Failed to get PHQ9");

    // Save an assessment
    let responses = vec![1, 2, 1, 0, 1, 2, 1, 0, 1];
    let total_score = 9;
    let severity_level = SeverityLevel::Mild;
    let notes = Some("Test notes".to_string());

    let id = repo
        .save_assessment(
            assessment_type.id,
            &responses,
            total_score,
            severity_level.as_str(),
            notes.clone(),
            AssessmentStatus::Completed.as_str(),
        )
        .expect("Failed to save assessment");

    // Retrieve the assessment
    let retrieved = repo
        .get_assessment_response(id)
        .expect("Failed to retrieve assessment");

    assert_eq!(retrieved.id, id);
    assert_eq!(retrieved.responses, responses);
    assert_eq!(retrieved.total_score, total_score);
    assert_eq!(retrieved.severity_level, severity_level);
    assert_eq!(retrieved.notes, notes);
    assert_eq!(retrieved.assessment_type.code, "PHQ9");
}

#[test]
fn test_get_assessment_history() {
    let (db, _temp_dir) = setup_test_db();
    let repo = AssessmentRepository::new(db.clone());

    // Get assessment types
    let phq9 = repo
        .get_assessment_type_by_code("PHQ9")
        .expect("Failed to get PHQ9");
    let gad7 = repo
        .get_assessment_type_by_code("GAD7")
        .expect("Failed to get GAD7");

    // Save multiple assessments
    repo.save_assessment(
        phq9.id,
        &vec![1; 9],
        9,
        SeverityLevel::Mild.as_str(),
        None,
        AssessmentStatus::Completed.as_str(),
    )
    .expect("Failed to save PHQ9 assessment");
    repo.save_assessment(
        gad7.id,
        &vec![2; 7],
        14,
        SeverityLevel::Moderate.as_str(),
        None,
        AssessmentStatus::Completed.as_str(),
    )
    .expect("Failed to save GAD7 assessment");
    repo.save_assessment(
        phq9.id,
        &vec![2; 9],
        18,
        SeverityLevel::Moderate.as_str(),
        None,
        AssessmentStatus::Completed.as_str(),
    )
    .expect("Failed to save second PHQ9 assessment");

    // Get all history
    let history = repo
        .get_assessment_history(None, None, None, None)
        .expect("Failed to get history");
    assert_eq!(history.len(), 3);

    // Filter by type
    let phq9_history = repo
        .get_assessment_history(Some("PHQ9".to_string()), None, None, None)
        .expect("Failed to get PHQ9 history");
    assert_eq!(phq9_history.len(), 2);
    assert!(phq9_history
        .iter()
        .all(|a| a.assessment_type.code == "PHQ9"));

    // Test limit
    let limited_history = repo
        .get_assessment_history(None, None, None, Some(2))
        .expect("Failed to get limited history");
    assert_eq!(limited_history.len(), 2);
}

#[test]
fn test_save_assessment_without_notes() {
    let (db, _temp_dir) = setup_test_db();
    let repo = AssessmentRepository::new(db);

    let assessment_type = repo
        .get_assessment_type_by_code("GAD7")
        .expect("Failed to get GAD7");

    let id = repo
        .save_assessment(
            assessment_type.id,
            &vec![1, 1, 2, 1, 2, 1, 1],
            9,
            SeverityLevel::Mild.as_str(),
            None,
            AssessmentStatus::Completed.as_str(),
        )
        .expect("Failed to save assessment");

    let retrieved = repo
        .get_assessment_response(id)
        .expect("Failed to retrieve assessment");

    assert_eq!(retrieved.notes, None);
}

#[test]
fn test_retrieve_nonexistent_assessment() {
    let (db, _temp_dir) = setup_test_db();
    let repo = AssessmentRepository::new(db);

    let result = repo.get_assessment_response(99999);

    assert!(result.is_err());
}

#[test]
fn test_delete_assessment_type_blocked_when_responses_exist() {
    let (db, _temp_dir) = setup_test_db();
    let repo = AssessmentRepository::new(db);

    // Get PHQ9 assessment type
    let assessment_type = repo
        .get_assessment_type_by_code("PHQ9")
        .expect("Failed to get PHQ9");

    // Save an assessment response
    repo.save_assessment(
        assessment_type.id,
        &vec![1, 2, 1, 0, 1, 2, 1, 0, 1],
        9,
        SeverityLevel::Mild.as_str(),
        None,
        AssessmentStatus::Completed.as_str(),
    )
    .expect("Failed to save assessment");

    // Attempt to delete the assessment type
    let result = repo.delete_assessment_type(assessment_type.id);

    // Should fail with HasChildren error
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("1 assessment response"));
}

#[test]
fn test_delete_assessment_type_blocked_when_schedules_exist() {
    let (db, _temp_dir) = setup_test_db();
    let repo = AssessmentRepository::new(db.clone());

    // Get GAD7 assessment type
    let assessment_type = repo
        .get_assessment_type_by_code("GAD7")
        .expect("Failed to get GAD7");

    // Manually insert a schedule for this assessment type
    let conn = db.get_connection();
    let conn_guard = conn.lock();
    conn_guard
        .execute(
            "INSERT INTO assessment_schedules (assessment_type_id, frequency, time_of_day, enabled)
             VALUES (?, 'daily', '09:00:00', true)",
            [assessment_type.id],
        )
        .expect("Failed to insert schedule");
    drop(conn_guard);

    // Attempt to delete the assessment type
    let result = repo.delete_assessment_type(assessment_type.id);

    // Should fail with HasChildren error mentioning schedules
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("1 schedule"));
}
