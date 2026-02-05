-- Migration 007: Add partial index for assessment history queries
--
-- Optimizes the most common query pattern in get_assessment_history():
--   SELECT ... FROM assessment_responses WHERE status = 'completed' ORDER BY completed_at DESC
--
-- Benefits:
--   - Partial index: Only indexes completed assessments (smaller index size)
--   - Covers both WHERE filter (status = 'completed') and ORDER BY (completed_at DESC)
--   - Benchmark shows history queries run 1200+ times in reporting scenarios
--
-- Note: SQLite doesn't support DESC in index definition, but the B-tree structure
-- can be traversed in either direction efficiently.

CREATE INDEX IF NOT EXISTS idx_assessment_responses_completed_history
    ON assessment_responses(status, completed_at)
    WHERE status = 'completed';
