-- Migration 005: Add status column to assessment_responses
-- Enables draft assessments feature (FR-009a)
--
-- Status values:
--   'draft'     - Assessment saved but not completed
--   'completed' - Assessment fully submitted (default)

-- SQLite doesn't support ALTER TABLE ADD COLUMN with CHECK constraints in old versions
-- Use ALTER TABLE ADD COLUMN (supported in all SQLite versions)
ALTER TABLE assessment_responses ADD COLUMN status TEXT NOT NULL DEFAULT 'completed';

-- Add CHECK constraint validation in application layer
-- SQLite 3.25.0+ supports CHECK in ALTER TABLE, but for compatibility we validate in Rust

-- Create index for filtering drafts vs completed assessments
CREATE INDEX idx_assessment_responses_status ON assessment_responses(status);

-- Create composite index for finding user's drafts by type
CREATE INDEX idx_assessment_responses_type_status ON assessment_responses(assessment_type_id, status);
