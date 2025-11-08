-- Migration 003: Activity Groups and Tracking
-- Adds support for organizing activities into groups, logging activity occurrences, and setting goals

-- Activity Groups (2-level hierarchy: Group -> Activities)
CREATE TABLE activity_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL CHECK(length(name) <= 100),
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT  -- Soft delete timestamp
);

CREATE INDEX idx_activity_groups_deleted ON activity_groups(deleted_at);

-- Partial unique index: only enforce uniqueness for non-deleted groups
CREATE UNIQUE INDEX idx_activity_groups_name_unique ON activity_groups(name) WHERE deleted_at IS NULL;

-- Modify existing activities table to add group_id foreign key
-- SQLite doesn't support ALTER COLUMN, so we need to recreate the table
-- Since no user data exists yet, this is a safe operation

-- Step 1: Create new activities table with group_id
CREATE TABLE activities_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    group_id INTEGER NOT NULL,
    name TEXT NOT NULL CHECK(length(name) <= 100),
    color TEXT,
    icon TEXT CHECK(length(icon) <= 20 AND (icon IS NULL OR length(icon) > 0)),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT,  -- Soft delete timestamp
    FOREIGN KEY (group_id) REFERENCES activity_groups(id) ON DELETE CASCADE
);

-- Step 2: Copy existing data (if any - though none exists in practice)
INSERT INTO activities_new (id, group_id, name, color, icon, created_at, deleted_at)
SELECT id, 1 as group_id, name, color, icon, created_at, deleted_at
FROM activities
WHERE EXISTS (SELECT 1 FROM activities);  -- Only if data exists

-- Step 3: Drop old table and rename new one
DROP TABLE IF EXISTS activities;
ALTER TABLE activities_new RENAME TO activities;

-- Step 4: Recreate indexes
CREATE INDEX idx_activities_deleted_at ON activities(deleted_at);
CREATE INDEX idx_activities_group_id ON activities(group_id);
CREATE UNIQUE INDEX idx_activities_name_unique ON activities(name) WHERE deleted_at IS NULL;

-- Activity Logs (track when activities are performed)
CREATE TABLE activity_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activity_id INTEGER NOT NULL,
    logged_at TEXT NOT NULL,           -- When the activity occurred
    created_at TEXT NOT NULL DEFAULT (datetime('now')),  -- When the log was created
    notes TEXT CHECK (length(notes) <= 500 OR notes IS NULL),
    deleted_at TEXT,  -- Soft delete timestamp
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE
);

CREATE INDEX idx_activity_logs_activity ON activity_logs(activity_id);
CREATE INDEX idx_activity_logs_logged_at ON activity_logs(logged_at);
CREATE INDEX idx_activity_logs_deleted ON activity_logs(deleted_at);

-- Activity Goals (track progress toward activity goals)
-- Goals can target either a specific activity OR an entire activity group (mutually exclusive)
CREATE TABLE activity_goals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activity_id INTEGER,
    group_id INTEGER,
    goal_type TEXT NOT NULL CHECK(goal_type IN ('days_per_period', 'percent_improvement')),
    target_value INTEGER NOT NULL,
    period_days INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT,  -- Soft delete timestamp
    -- Ensure goal targets either an activity OR a group, not both (allows both NULL for testing)
    CHECK (NOT (activity_id IS NOT NULL AND group_id IS NOT NULL)),
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES activity_groups(id) ON DELETE CASCADE
);

CREATE INDEX idx_activity_goals_activity ON activity_goals(activity_id);
CREATE INDEX idx_activity_goals_group ON activity_goals(group_id);
CREATE INDEX idx_activity_goals_deleted ON activity_goals(deleted_at);

-- Partial index for active goals (common query pattern)
CREATE INDEX idx_activity_goals_active ON activity_goals(activity_id, group_id) WHERE deleted_at IS NULL;
