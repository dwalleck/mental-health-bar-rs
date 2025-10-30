/**
 * Centralized error handling utilities for user-facing error messages
 */

import { toastStore } from '$lib/stores/toast'
import { isCommandError, ERROR_TYPES } from '$lib/utils/types'

/**
 * Get the error type from an unknown error
 *
 * @param error - The error to check
 * @returns The error type string, or null if not a CommandError
 *
 * @example
 * ```typescript
 * const errorType = getErrorType(error)
 * if (errorType === 'validation') {
 *   // Handle validation error
 * }
 * ```
 */
export function getErrorType(error: unknown): string | null {
	if (isCommandError(error)) {
		return error.error_type
	}
	return null
}

/**
 * Formats an unknown error into a user-friendly string message
 *
 * @param error - The error to format (can be CommandError, Error, string, or unknown)
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
	// Handle CommandError (structured error from backend)
	if (isCommandError(error)) {
		return error.message
	}

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
	// Check CommandError error_type first (most reliable)
	if (isCommandError(error)) {
		return error.error_type === ERROR_TYPES.VALIDATION
	}

	// Fallback: Check Error message for validation keywords
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
	// Check CommandError retryable flag first (most reliable)
	if (isCommandError(error)) {
		return error.retryable
	}

	// Fallback: Check Error message for transient keywords
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
	if (isCommandError(error)) {
		return `CommandError[${error.error_type}]: ${error.message} (retryable: ${error.retryable})`
	}

	if (error instanceof Error) {
		return error.stack || `${error.name}: ${error.message}`
	}

	if (typeof error === 'string') {
		return error
	}

	return JSON.stringify(error, null, 2)
}

/**
 * Display an error appropriately based on its type
 * Validation errors show inline, system errors show as toast
 *
 * @param error - The error to display
 * @returns Object indicating how the error was displayed
 *
 * @example
 * ```typescript
 * const result = displayError(error)
 * if (result.type === 'inline') {
 *   errorMessage = result.message
 * }
 * ```
 */
export function displayError(error: unknown): { type: 'inline' | 'toast'; message?: string } {
	if (isValidationError(error)) {
		// Validation errors should be shown inline with the form
		return { type: 'inline', message: formatUserError(error) }
	} else {
		// System errors should be shown as toast notifications
		const message = formatUserError(error)

		// Use longer duration for transient errors (user needs time to read before retry)
		// Shorter duration for permanent errors (no retry available)
		const duration = isTransientError(error) ? 8000 : 5000

		toastStore.error(message, duration)
		return { type: 'toast' }
	}
}

/**
 * Display a success message as a toast notification
 *
 * @param message - The success message to display
 * @param duration - Duration in milliseconds (default: 3000)
 *
 * @example
 * ```typescript
 * await submitAssessment(data)
 * displaySuccess('Assessment submitted successfully!')
 * ```
 */
export function displaySuccess(message: string, duration: number = 3000): void {
	toastStore.success(message, duration)
}

/**
 * Display a warning message as a toast notification
 *
 * @param message - The warning message to display
 * @param duration - Duration in milliseconds (default: 5000)
 *
 * @example
 * ```typescript
 * if (assessmentIncomplete) {
 *   displayWarning('Some questions were skipped')
 * }
 * ```
 */
export function displayWarning(message: string, duration: number = 5000): void {
	toastStore.warning(message, duration)
}

/**
 * Display an info message as a toast notification
 *
 * @param message - The info message to display
 * @param duration - Duration in milliseconds (default: 4000)
 *
 * @example
 * ```typescript
 * displayInfo('New assessment types are available')
 * ```
 */
export function displayInfo(message: string, duration: number = 4000): void {
	toastStore.info(message, duration)
}
