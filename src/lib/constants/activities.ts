/**
 * Activity-related constants
 */

/**
 * Goal type constants
 */
export const GOAL_TYPES = {
	DAYS_PER_PERIOD: 'days_per_period',
	PERCENT_IMPROVEMENT: 'percent_improvement',
} as const

export type GoalType = (typeof GOAL_TYPES)[keyof typeof GOAL_TYPES]

/**
 * Activity log constants
 */
export const ACTIVITY_LOG = {
	MAX_NOTE_LENGTH: 500,
} as const
