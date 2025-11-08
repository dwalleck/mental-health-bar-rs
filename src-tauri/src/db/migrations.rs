use anyhow::{Context, Result};
use tracing::info;

use super::Database;

/// Run all pending database migrations
pub fn run_migrations(db: &Database) -> Result<()> {
    info!("Running database migrations");

    // Create migrations tracking table if it doesn't exist
    db.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT DEFAULT (datetime('now'))
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

    if current_version < 2 {
        apply_migration_002(db)?;
        record_migration(db, 2)?;
        info!("Applied migration 002: Add schedule index");
    }

    if current_version < 3 {
        apply_migration_003(db)?;
        record_migration(db, 3)?;
        info!("Applied migration 003: Activity groups and tracking");
    }

    info!("All migrations applied successfully");
    Ok(())
}

/// Get the current schema version
fn get_current_version(db: &Database) -> Result<i32> {
    let conn = db.get_connection();
    let conn = conn.lock();

    let version: Result<i32, _> = conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
        [],
        |row| row.get(0),
    );

    version.or(Ok(0))
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
    let mut conn = conn.lock();

    // Wrap migration in explicit transaction for atomicity
    // If any DDL statement fails, entire migration rolls back
    let tx = conn
        .transaction()
        .context("Failed to begin transaction for migration 001")?;

    tx.execute_batch(schema_sql)
        .context("Failed to execute migration 001 DDL statements")?;

    tx.commit()
        .context("Failed to commit migration 001 transaction")?;

    Ok(())
}

/// Migration 002: Add performance index for schedule queries
fn apply_migration_002(db: &Database) -> Result<()> {
    let schema_sql = include_str!("migrations/002_add_schedule_index.sql");

    let conn = db.get_connection();
    let mut conn = conn.lock();

    // Wrap migration in explicit transaction for atomicity
    // If any DDL statement fails, entire migration rolls back
    let tx = conn
        .transaction()
        .context("Failed to begin transaction for migration 002")?;

    tx.execute_batch(schema_sql)
        .context("Failed to execute migration 002 DDL statements")?;

    tx.commit()
        .context("Failed to commit migration 002 transaction")?;

    Ok(())
}

/// Migration 003: Activity groups and tracking
fn apply_migration_003(db: &Database) -> Result<()> {
    let schema_sql = include_str!("migrations/003_activity_groups.sql");

    let conn = db.get_connection();
    let mut conn = conn.lock();

    // Wrap migration in explicit transaction for atomicity
    // If any DDL statement fails, entire migration rolls back
    let tx = conn
        .transaction()
        .context("Failed to begin transaction for migration 003")?;

    tx.execute_batch(schema_sql)
        .context("Failed to execute migration 003 DDL statements")?;

    tx.commit()
        .context("Failed to commit migration 003 transaction")?;

    Ok(())
}
