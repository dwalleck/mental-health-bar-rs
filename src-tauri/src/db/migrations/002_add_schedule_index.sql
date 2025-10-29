-- Migration 002: Add performance index for schedule queries
-- Partial index on enabled schedules, covering time_of_day for ORDER BY optimization

CREATE INDEX IF NOT EXISTS idx_schedules_enabled_time
ON assessment_schedules(enabled, time_of_day)
WHERE enabled = 1;
