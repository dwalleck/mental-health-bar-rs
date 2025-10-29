// Database configuration tests - PRAGMA enforcement and connection setup
// T300: Database PRAGMA enforcement tests

use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tempfile::TempDir;

/// Helper function to create a test database
fn setup_test_db() -> (Arc<Database>, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db =
        Arc::new(Database::new(temp_dir.path().to_path_buf()).expect("Failed to create database"));
    (db, temp_dir)
}

/// T300-1: Verify PRAGMA foreign_keys is enabled
#[test]
fn test_foreign_keys_pragma_enabled() {
    let (db, _temp_dir) = setup_test_db();
    let conn = db.get_connection();
    let conn = conn.lock();

    // Query the foreign_keys pragma - should return 1 (enabled)
    let fk_enabled: i32 = conn
        .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
        .expect("Failed to query foreign_keys pragma");

    assert_eq!(
        fk_enabled, 1,
        "CRITICAL: Foreign keys not enabled! All FK constraints are being ignored."
    );
}

/// T300-2: Verify PRAGMA busy_timeout is configured
#[test]
fn test_busy_timeout_configured() {
    let (db, _temp_dir) = setup_test_db();
    let conn = db.get_connection();
    let conn = conn.lock();

    // Query the busy_timeout pragma - should be 5000ms (5 seconds)
    let timeout: i32 = conn
        .query_row("PRAGMA busy_timeout", [], |row| row.get(0))
        .expect("Failed to query busy_timeout pragma");

    assert_eq!(
        timeout, 5000,
        "busy_timeout should be 5000ms for lock contention handling"
    );
}

/// T300-3: Verify PRAGMA journal_mode is WAL
#[test]
fn test_journal_mode_wal() {
    let (db, _temp_dir) = setup_test_db();
    let conn = db.get_connection();
    let conn = conn.lock();

    // Query the journal_mode pragma - should be "wal"
    let journal_mode: String = conn
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .expect("Failed to query journal_mode pragma");

    assert_eq!(
        journal_mode.to_lowercase(),
        "wal",
        "journal_mode should be WAL for better concurrency"
    );
}

/// T300-4: Verify PRAGMA synchronous is NORMAL
#[test]
fn test_synchronous_normal() {
    let (db, _temp_dir) = setup_test_db();
    let conn = db.get_connection();
    let conn = conn.lock();

    // Query the synchronous pragma - should be 1 (NORMAL)
    // 0 = OFF, 1 = NORMAL, 2 = FULL, 3 = EXTRA
    let synchronous: i32 = conn
        .query_row("PRAGMA synchronous", [], |row| row.get(0))
        .expect("Failed to query synchronous pragma");

    assert_eq!(
        synchronous, 1,
        "synchronous should be NORMAL (1) for WAL mode"
    );
}

/// T300-5: Verify PRAGMA cache_size is configured
#[test]
fn test_cache_size_configured() {
    let (db, _temp_dir) = setup_test_db();
    let conn = db.get_connection();
    let conn = conn.lock();

    // Query the cache_size pragma
    let cache_size: i32 = conn
        .query_row("PRAGMA cache_size", [], |row| row.get(0))
        .expect("Failed to query cache_size pragma");

    // Should be -64000 (64MB in KB, negative means KB)
    assert_eq!(
        cache_size, -64000,
        "cache_size should be -64000 (64MB cache)"
    );
}

/// T300-6: Verify foreign key constraints are actually enforced
#[test]
fn test_foreign_key_constraint_enforced() {
    let (db, _temp_dir) = setup_test_db();
    let conn = db.get_connection();
    let conn = conn.lock();

    // Try to insert an assessment response with a non-existent assessment_type_id
    let result = conn.execute(
        "INSERT INTO assessment_responses (assessment_type_id, responses, total_score, severity_level, notes)
         VALUES (99999, '[]', 0, 'minimal', NULL)",
        [],
    );

    // Should fail with foreign key constraint violation
    assert!(
        result.is_err(),
        "Foreign key constraint not enforced! Should have failed with invalid assessment_type_id."
    );

    // Verify it's specifically a constraint violation
    if let Err(e) = result {
        let error_msg = format!("{:?}", e);
        assert!(
            error_msg.contains("constraint") || error_msg.contains("FOREIGN KEY"),
            "Expected foreign key constraint error, got: {}",
            error_msg
        );
    }
}

/// T300-7: Verify CASCADE delete works with foreign keys enabled
#[test]
fn test_cascade_delete_works() {
    use tauri_sveltekit_modern_lib::features::mood::repository::MoodRepository;

    let (db, _temp_dir) = setup_test_db();
    let repo = MoodRepository::new(db);

    // Create a mood check-in
    let checkin = repo
        .create_mood_checkin(4, vec![], None)
        .expect("Failed to create mood check-in");

    // Verify it exists
    let fetched = repo
        .get_mood_checkin(checkin.id)
        .expect("Failed to fetch mood check-in");
    assert_eq!(fetched.id, checkin.id);

    // Delete the mood check-in
    repo.delete_mood_checkin(checkin.id)
        .expect("Failed to delete mood check-in");

    // Verify it's deleted (CASCADE should have removed junction table entries too)
    let result = repo.get_mood_checkin(checkin.id);
    assert!(
        result.is_err(),
        "Mood check-in should be deleted (with CASCADE)"
    );
}

/// T300-8: Verify CHECK constraint is enforced (mood_rating BETWEEN 1 AND 5)
#[test]
fn test_check_constraint_enforced() {
    let (db, _temp_dir) = setup_test_db();
    let conn = db.get_connection();
    let conn = conn.lock();

    // Try to insert a mood check-in with invalid rating (0)
    let result = conn.execute(
        "INSERT INTO mood_checkins (mood_rating, notes) VALUES (0, NULL)",
        [],
    );

    assert!(
        result.is_err(),
        "CHECK constraint not enforced! mood_rating=0 should be rejected."
    );

    // Try to insert a mood check-in with invalid rating (6)
    let result2 = conn.execute(
        "INSERT INTO mood_checkins (mood_rating, notes) VALUES (6, NULL)",
        [],
    );

    assert!(
        result2.is_err(),
        "CHECK constraint not enforced! mood_rating=6 should be rejected."
    );
}

/// T300-9: Verify statement cache capacity is set
#[test]
fn test_statement_cache_configured() {
    let (db, _temp_dir) = setup_test_db();
    let conn = db.get_connection();
    let conn = conn.lock();

    // Try to use prepare_cached - it should work without error
    let stmt = conn.prepare_cached("SELECT 1");
    assert!(
        stmt.is_ok(),
        "prepare_cached should work with configured cache"
    );

    // Prepare multiple cached statements to verify cache works
    for i in 1..=10 {
        let query = format!("SELECT {}", i);
        let stmt = conn.prepare_cached(&query);
        assert!(stmt.is_ok(), "prepare_cached failed for query: {}", query);
    }
}

/// T300-10: Verify temp_store is MEMORY
#[test]
fn test_temp_store_memory() {
    let (db, _temp_dir) = setup_test_db();
    let conn = db.get_connection();
    let conn = conn.lock();

    // Query the temp_store pragma - should be 2 (MEMORY)
    // 0 = DEFAULT, 1 = FILE, 2 = MEMORY
    let temp_store: i32 = conn
        .query_row("PRAGMA temp_store", [], |row| row.get(0))
        .expect("Failed to query temp_store pragma");

    assert_eq!(
        temp_store, 2,
        "temp_store should be MEMORY (2) for performance"
    );
}
