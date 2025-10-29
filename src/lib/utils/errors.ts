/**
 * Centralized error handling utilities for user-facing error messages
 */

/**
 * Formats an unknown error into a user-friendly string message
 *
 * @param error - The error to format (can be Error, string, or unknown)
 * @returns User-friendly error message
 *
 * @example
 * ```typescript
 * try {
 *   await submitAssessment(data)
 * } catch (error) {
 *   const message = formatUserError(error)
 *   toastStore.error(message)
 * }
 * ```
 */
export function formatUserError(error: unknown): string {
	// Handle Error instances with message property
	if (error instanceof Error) {
		return error.message
	}

	// Handle string errors
	if (typeof error === 'string') {
		return error
	}

	// Fallback for unknown error types
	return 'An unexpected error occurred. Please try again.'
}

/**
 * Checks if an error is a validation error (user input issue)
 *
 * @param error - The error to check
 * @returns True if the error is related to validation
 *
 * @example
 * ```typescript
 * if (isValidationError(error)) {
 *   // Show inline form error
 * } else {
 *   // Show toast notification
 * }
 * ```
 */
export function isValidationError(error: unknown): boolean {
	if (error instanceof Error) {
		const message = error.message.toLowerCase()
		return (
			message.includes('validation') ||
			message.includes('invalid') ||
			message.includes('required') ||
			message.includes('must be') ||
			message.includes('too long') ||
			message.includes('too short') ||
			message.includes('out of range')
		)
	}
	return false
}

/**
 * Checks if an error is a transient error that may succeed on retry
 *
 * @param error - The error to check
 * @returns True if the error is likely transient
 *
 * @example
 * ```typescript
 * if (isTransientError(error)) {
 *   // Show retry button
 * } else {
 *   // Show error message without retry
 * }
 * ```
 */
export function isTransientError(error: unknown): boolean {
	if (error instanceof Error) {
		const message = error.message.toLowerCase()
		const name = error.name?.toLowerCase() || ''

		return (
			name.includes('sqliteerror') ||
			name.includes('databaseerror') ||
			message.includes('database lock') ||
			message.includes('database is busy') ||
			message.includes('lock poisoned') ||
			message.includes('connection timeout') ||
			message.includes('timed out') ||
			message.includes('network error') ||
			message.includes('fetch failed')
		)
	}
	return false
}

/**
 * Formats an error for logging purposes (includes stack trace for Error objects)
 *
 * @param error - The error to format
 * @returns Detailed error string for logging
 *
 * @example
 * ```typescript
 * console.error('Operation failed:', formatErrorForLogging(error))
 * ```
 */
export function formatErrorForLogging(error: unknown): string {
	if (error instanceof Error) {
		return error.stack || `${error.name}: ${error.message}`
	}

	if (typeof error === 'string') {
		return error
	}

	return JSON.stringify(error, null, 2)
}
