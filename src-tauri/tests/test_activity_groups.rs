// Integration tests for activity groups (Phase 1: Activity Groups)
// TDD approach: Write tests first, then implement repository methods

use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::activities::repository::ActivityRepository;
use tempfile::TempDir;

/// Setup test environment with temporary database
fn setup_test_repo() -> (ActivityRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().to_path_buf();
    let db = Arc::new(Database::new(db_path).expect("Failed to create database"));
    let repo = ActivityRepository::new(db);
    (repo, temp_dir)
}

// Task 1.12: Test for create_activity_group (TDD: red phase)
#[test]
fn test_create_activity_group() {
    let (repo, _temp_dir) = setup_test_repo();

    // Arrange: Prepare test data
    let name = "Exercise";
    let description = Some("Physical activities");

    // Act: Create activity group
    let group = repo
        .create_activity_group(name, description)
        .expect("Failed to create activity group");

    // Assert: Verify group was created correctly
    assert_eq!(group.name, "Exercise");
    assert_eq!(group.description, Some("Physical activities".to_string()));
    assert!(group.id > 0, "Group should have a positive ID");
    assert!(
        group.deleted_at.is_none(),
        "Newly created group should not be deleted"
    );
    assert!(
        !group.created_at.is_empty(),
        "Group should have created_at timestamp"
    );
}

// Additional test: Create group with minimal data (name only)
#[test]
fn test_create_activity_group_minimal() {
    let (repo, _temp_dir) = setup_test_repo();

    let group = repo
        .create_activity_group("Hobbies", None)
        .expect("Failed to create activity group");

    assert_eq!(group.name, "Hobbies");
    assert!(group.description.is_none());
}

// Additional test: Duplicate group name should fail
#[test]
fn test_create_activity_group_duplicate_name() {
    let (repo, _temp_dir) = setup_test_repo();

    // Create first group
    repo.create_activity_group("Exercise", None)
        .expect("Failed to create first group");

    // Try to create duplicate
    let result = repo.create_activity_group("Exercise", Some("Duplicate"));
    assert!(result.is_err(), "Duplicate group name should fail");
}

// Additional test: Empty group name should fail
#[test]
fn test_create_activity_group_empty_name() {
    let (repo, _temp_dir) = setup_test_repo();

    let result = repo.create_activity_group("", None);
    assert!(result.is_err(), "Empty group name should fail");
}

// Additional test: Group name too long should fail
#[test]
fn test_create_activity_group_name_too_long() {
    let (repo, _temp_dir) = setup_test_repo();

    let long_name = "a".repeat(101); // Exceeds 100 char limit
    let result = repo.create_activity_group(&long_name, None);
    assert!(
        result.is_err(),
        "Group name exceeding 100 characters should fail"
    );
}

// Additional test: Group name at exactly 100 chars should succeed
#[test]
fn test_create_activity_group_name_at_limit() {
    let (repo, _temp_dir) = setup_test_repo();

    let name_at_limit = "a".repeat(100); // Exactly 100 chars
    let result = repo.create_activity_group(&name_at_limit, None);
    assert!(
        result.is_ok(),
        "Group name with exactly 100 characters should succeed"
    );
}
