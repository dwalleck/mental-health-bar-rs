-- Migration 004: Expand Mood Scale from 1-5 to 1-7
-- Provides more granular mood tracking with a 7-point scale
--
-- Scale Mapping (for existing data migration):
--   Old 1-5 Scale           New 1-7 Scale
--   1 (Very Bad)       →    1 (Terrible)
--   2 (Bad)            →    3 (Bad)
--   3 (Neutral)        →    4 (Ok)
--   4 (Good)           →    5 (Good)
--   5 (Very Good)      →    7 (Excellent)
--
-- New 1-7 Scale Labels:
--   1 = Terrible
--   2 = Very Bad
--   3 = Bad
--   4 = Ok
--   5 = Good
--   6 = Very Good
--   7 = Excellent

-- SQLite doesn't support ALTER COLUMN, so we recreate the table
-- IMPORTANT: We must disable foreign keys during table recreation because:
-- 1. mood_checkin_activities has FK to mood_checkins (from migration 001)
-- 2. We need to drop the old mood_checkins table
-- 3. SQLite prevents dropping tables referenced by FKs when PRAGMA foreign_keys = ON
PRAGMA foreign_keys = OFF;

-- Step 1: Create new mood_checkins table with updated CHECK constraint
CREATE TABLE mood_checkins_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mood_rating INTEGER NOT NULL CHECK (mood_rating BETWEEN 1 AND 7),
    notes TEXT CHECK (length(notes) <= 5000 OR notes IS NULL),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Step 2: Migrate existing data with scale conversion (1-5 → 1-7)
INSERT INTO mood_checkins_new (id, mood_rating, notes, created_at)
SELECT
    id,
    CASE mood_rating
        WHEN 1 THEN 1  -- Very Bad → Terrible
        WHEN 2 THEN 3  -- Bad → Bad
        WHEN 3 THEN 4  -- Neutral → Ok
        WHEN 4 THEN 5  -- Good → Good
        WHEN 5 THEN 7  -- Very Good → Excellent
        ELSE mood_rating  -- Fallback (should never happen)
    END as mood_rating,
    notes,
    created_at
FROM mood_checkins
WHERE EXISTS (SELECT 1 FROM mood_checkins);  -- Only if data exists

-- Step 3: Drop old table and rename new one
DROP TABLE IF EXISTS mood_checkins;
ALTER TABLE mood_checkins_new RENAME TO mood_checkins;

-- Step 4: Recreate indexes
CREATE INDEX idx_mood_checkins_created_at ON mood_checkins(created_at);
CREATE INDEX idx_mood_checkins_date ON mood_checkins(DATE(created_at));

-- Re-enable foreign key constraints (must match db/mod.rs PRAGMA setting)
-- This is critical to restore referential integrity enforcement
PRAGMA foreign_keys = ON;
