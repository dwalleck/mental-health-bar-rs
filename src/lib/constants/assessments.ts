/**
 * Assessment status constants matching backend AssessmentStatus enum
 * These should stay in sync with src-tauri/src/types/assessment.rs
 */
export const ASSESSMENT_STATUS = {
	DRAFT: 'draft',
	COMPLETED: 'completed',
} as const

export type AssessmentStatus = (typeof ASSESSMENT_STATUS)[keyof typeof ASSESSMENT_STATUS]
