-- Rollback Migration 003: Activity Groups and Tracking
-- Removes activity groups, goals, and logs tables
-- Restores original activities table structure

-- Drop tables in reverse order of dependencies
DROP TABLE IF EXISTS activity_goals;
DROP TABLE IF EXISTS activity_logs;

-- Recreate original activities table (without group_id)
-- Include CHECK constraints to prevent invalid data during rollback state
CREATE TABLE activities_rollback (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL CHECK(length(name) <= 50),
    color TEXT,
    icon TEXT CHECK(length(icon) <= 20 AND (icon IS NULL OR length(icon) > 0)),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT  -- Soft delete timestamp
);

-- Copy data back (excluding group_id)
INSERT INTO activities_rollback (id, name, color, icon, created_at, deleted_at)
SELECT id, name, color, icon, created_at, deleted_at
FROM activities
WHERE EXISTS (SELECT 1 FROM activities);

-- Drop modified activities table and rename rollback version
DROP TABLE IF EXISTS activities;
ALTER TABLE activities_rollback RENAME TO activities;

-- Recreate original indexes
CREATE INDEX idx_activities_deleted_at ON activities(deleted_at);
CREATE UNIQUE INDEX idx_activities_name_unique ON activities(name) WHERE deleted_at IS NULL;

-- Drop activity groups table
DROP TABLE IF EXISTS activity_groups;
