/**
 * Shared TypeScript type utilities and guards
 */

import type { CommandError, ErrorType as GeneratedErrorType } from '$lib/bindings'

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
 * Error type constants - auto-generated from Rust via specta
 * Source of truth: src-tauri/src/errors.rs::ErrorType enum
 *
 * These constants provide a convenient way to reference error types
 * while maintaining compile-time type safety with the generated ErrorType union.
 */
export const ERROR_TYPES: Record<string, GeneratedErrorType> = {
	VALIDATION: 'validation',
	NOT_FOUND: 'not_found',
	DATABASE_ERROR: 'database_error',
	DATABASE_LOCKED: 'database_locked',
	LOCK_POISONED: 'lock_poisoned',
	CONSTRAINT_VIOLATION: 'constraint_violation',
	DUPLICATE: 'duplicate',
	TRANSACTION_FAILURE: 'transaction_failure',
	NO_DATA: 'no_data',
	CALCULATION_ERROR: 'calculation_error',
	TRANSIENT: 'transient',
	INTERNAL: 'internal',
	CONFIG: 'config',
	IO_ERROR: 'io_error',
	SERIALIZATION: 'serialization',
} as const

/**
 * Re-export the generated ErrorType for convenience
 * This is auto-generated from the Rust ErrorType enum via specta
 */
export type ErrorType = GeneratedErrorType
