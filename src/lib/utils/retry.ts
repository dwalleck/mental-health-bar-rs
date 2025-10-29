// T183: Retry logic for persistence errors using p-retry
import { invoke } from '@tauri-apps/api/core'
import pRetry, { AbortError } from 'p-retry'
import type { CommandError } from '$lib/bindings'

/**
 * Type guard to check if an error is a CommandError
 * Note: This is duplicated here to avoid circular dependency with errors.ts
 */
function isCommandError(error: unknown): error is CommandError {
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
 * Configuration options for retry behavior
 */
export interface RetryOptions {
	/** Maximum number of retry attempts (default: 3) */
	maxAttempts?: number
	/** Initial delay in milliseconds before first retry (default: 100ms) */
	initialDelay?: number
	/** Maximum delay in milliseconds between retries (default: 2000ms) */
	maxDelay?: number
	/** Multiplier for exponential backoff (default: 2) */
	backoffMultiplier?: number
	/** Function to determine if an error should trigger a retry */
	shouldRetry?: (error: unknown) => boolean
}

/**
 * Default retry configuration
 */
const DEFAULT_OPTIONS: Required<RetryOptions> = {
	maxAttempts: 3,
	initialDelay: 100,
	maxDelay: 2000,
	backoffMultiplier: 2,
	shouldRetry: (error: unknown) => {
		// Check if error is a CommandError with retryable flag
		if (isCommandError(error)) {
			return error.retryable
		}

		// Fallback: Check Error objects for transient error patterns
		if (error instanceof Error) {
			const errorMessage = error.message.toLowerCase()
			const errorName = error.name?.toLowerCase() || ''

			return (
				// Check error name first (most specific)
				errorName.includes('sqliteerror') ||
				errorName.includes('databaseerror') ||
				// Then check error message for transient conditions
				errorMessage.includes('database lock') ||
				errorMessage.includes('database is busy') ||
				errorMessage.includes('lock poisoned') ||
				errorMessage.includes('connection timeout') ||
				errorMessage.includes('timed out')
			)
		}

		// Don't retry non-Error objects
		return false
	},
}

/**
 * Invokes a Tauri command with automatic retry logic for transient errors
 *
 * Uses p-retry library for robust retry handling with exponential backoff and jitter.
 *
 * @param command - The Tauri command name to invoke
 * @param args - Arguments to pass to the command
 * @param options - Retry configuration options
 * @returns Promise resolving to the command result
 * @throws Error if all retry attempts fail or error is not retryable
 *
 * @example
 * ```typescript
 * const result = await invokeWithRetry('submit_assessment', {
 *   request: assessmentData
 * })
 * ```
 *
 * @example
 * ```typescript
 * // Custom retry configuration
 * const result = await invokeWithRetry(
 *   'log_mood',
 *   { request: moodData },
 *   {
 *     maxAttempts: 5,
 *     initialDelay: 200,
 *     shouldRetry: (error) => error instanceof Error && error.message.includes('database')
 *   }
 * )
 * ```
 */
export async function invokeWithRetry<T>(
	command: string,
	args?: Record<string, unknown>,
	options?: RetryOptions
): Promise<T> {
	const opts = { ...DEFAULT_OPTIONS, ...options }

	return pRetry(
		async () => {
			try {
				return await invoke<T>(command, args)
			} catch (error) {
				// Check if this error should be retried
				if (!opts.shouldRetry(error)) {
					// AbortError tells p-retry to stop immediately without retrying
					throw new AbortError(error instanceof Error ? error.message : String(error))
				}
				// Re-throw retryable errors so p-retry can handle them
				throw error
			}
		},
		{
			retries: opts.maxAttempts - 1, // p-retry counts retries, not total attempts
			factor: opts.backoffMultiplier,
			minTimeout: opts.initialDelay,
			maxTimeout: opts.maxDelay,
			randomize: true, // Built-in jitter to prevent thundering herd
			onFailedAttempt: (error) => {
				console.warn(
					`Tauri command '${command}' failed (attempt ${error.attemptNumber}/${opts.maxAttempts}). ` +
						`Retrying... (${error.retriesLeft} retries left)`,
					error
				)
			},
		}
	)
}

/**
 * Creates a retry wrapper for a specific command with preset options
 *
 * @param command - The Tauri command name
 * @param defaultOptions - Default retry options for this command
 * @returns Function that invokes the command with retry logic
 *
 * @example
 * ```typescript
 * const submitAssessment = createRetryCommand('submit_assessment', {
 *   maxAttempts: 5
 * })
 *
 * const result = await submitAssessment({ request: data })
 * ```
 */
export function createRetryCommand<TArgs extends Record<string, unknown>, TResult>(
	command: string,
	defaultOptions?: RetryOptions
) {
	return async (args?: TArgs, overrideOptions?: RetryOptions): Promise<TResult> => {
		const options = { ...defaultOptions, ...overrideOptions }
		return invokeWithRetry<TResult>(command, args, options)
	}
}
