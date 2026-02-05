-- Migration 006: Add partial unique constraint for drafts
-- Addresses TOCTOU race condition (PR #46 review feedback)
--
-- Ensures only ONE draft can exist per assessment type at a time.
-- This enables atomic UPSERT operations in the repository layer.

-- Partial unique index: only applies WHERE status = 'draft'
-- Completed assessments are not affected (multiple allowed)
CREATE UNIQUE INDEX IF NOT EXISTS idx_one_draft_per_type
ON assessment_responses(assessment_type_id)
WHERE status = 'draft';
