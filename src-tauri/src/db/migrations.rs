use anyhow::{Context, Result};
use duckdb::params;
use tracing::info;

use super::Database;

/// Run all pending database migrations
pub fn run_migrations(db: &Database) -> Result<()> {
    info!("Running database migrations");

    // Create migrations tracking table if it doesn't exist
    db.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        &[],
    )?;

    // Get current schema version
    let current_version = get_current_version(db)?;
    info!("Current schema version: {}", current_version);

    // Apply migrations in sequence
    if current_version < 1 {
        apply_migration_001(db)?;
        record_migration(db, 1)?;
        info!("Applied migration 001: Initial schema");
    }

    info!("All migrations applied successfully");
    Ok(())
}

/// Get the current schema version
fn get_current_version(db: &Database) -> Result<i32> {
    let conn = db.get_connection();
    let conn = conn.lock().unwrap();

    let version: Result<i32, _> = conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
        [],
        |row| row.get(0),
    );

    Ok(version.unwrap_or(0))
}

/// Record that a migration has been applied
fn record_migration(db: &Database, version: i32) -> Result<()> {
    db.execute(
        "INSERT INTO schema_migrations (version) VALUES (?)",
        &[&version],
    )?;
    Ok(())
}

/// Migration 001: Initial schema
fn apply_migration_001(db: &Database) -> Result<()> {
    let schema_sql = include_str!("migrations/001_initial_schema.sql");

    let conn = db.get_connection();
    let conn = conn.lock().unwrap();

    conn.execute_batch(schema_sql)
        .context("Failed to apply migration 001")?;

    Ok(())
}
