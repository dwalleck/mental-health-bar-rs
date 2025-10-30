/**
 * Shared TypeScript type utilities and guards
 */

import type { CommandError } from '$lib/bindings'

/**
 * Type guard to check if an error is a CommandError
 *
 * @param error - The error to check
 * @returns True if the error is a CommandError
 *
 * @example
 * ```typescript
 * if (isCommandError(error)) {
 *   console.log(error.error_type) // Type-safe access
 * }
 * ```
 */
export function isCommandError(error: unknown): error is CommandError {
	return (
		typeof error === 'object' &&
		error !== null &&
		'message' in error &&
		'error_type' in error &&
		'retryable' in error &&
		typeof (error as CommandError).message === 'string' &&
		typeof (error as CommandError).error_type === 'string' &&
		typeof (error as CommandError).retryable === 'boolean'
	)
}

/**
 * Error type constants matching the Rust backend
 */
export const ERROR_TYPES = {
	VALIDATION: 'validation',
	NOT_FOUND: 'not_found',
	DATABASE_ERROR: 'database_error',
	DATABASE_LOCKED: 'database_locked',
	LOCK_POISONED: 'lock_poisoned',
	CONSTRAINT_VIOLATION: 'constraint_violation',
	TRANSIENT: 'transient',
	INTERNAL: 'internal',
	CONFIG: 'config',
	IO_ERROR: 'io_error',
	SERIALIZATION: 'serialization',
} as const

export type ErrorType = (typeof ERROR_TYPES)[keyof typeof ERROR_TYPES]
