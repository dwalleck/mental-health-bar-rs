// T183: Retry logic for persistence errors
import { invoke } from '@tauri-apps/api/core'

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
		// Retry on persistence-related errors
		const errorMessage = String(error).toLowerCase()
		return (
			errorMessage.includes('database') ||
			errorMessage.includes('lock') ||
			errorMessage.includes('busy') ||
			errorMessage.includes('connection') ||
			errorMessage.includes('timeout') ||
			errorMessage.includes('poisoned')
		)
	},
}

/**
 * Sleep for a specified duration
 */
function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms))
}

/**
 * Calculate exponential backoff delay
 */
function calculateDelay(attempt: number, options: Required<RetryOptions>): number {
	const delay = options.initialDelay * Math.pow(options.backoffMultiplier, attempt)
	return Math.min(delay, options.maxDelay)
}

/**
 * Invokes a Tauri command with automatic retry logic for transient errors
 *
 * @param command - The Tauri command name to invoke
 * @param args - Arguments to pass to the command
 * @param options - Retry configuration options
 * @returns Promise resolving to the command result
 * @throws Error if all retry attempts fail
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
 *     shouldRetry: (error) => String(error).includes('database')
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
	let lastError: unknown

	for (let attempt = 0; attempt < opts.maxAttempts; attempt++) {
		try {
			// Attempt the command invocation
			return await invoke<T>(command, args)
		} catch (error) {
			lastError = error

			// Check if we should retry this error
			if (!opts.shouldRetry(error)) {
				throw error
			}

			// Don't delay after the last attempt
			if (attempt < opts.maxAttempts - 1) {
				const delay = calculateDelay(attempt, opts)
				console.warn(
					`Tauri command '${command}' failed (attempt ${attempt + 1}/${opts.maxAttempts}). ` +
						`Retrying in ${delay}ms...`,
					error
				)
				await sleep(delay)
			}
		}
	}

	// All attempts failed
	console.error(`Tauri command '${command}' failed after ${opts.maxAttempts} attempts`, lastError)
	throw lastError
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
		return invokeWithRetry<TResult>(command, args as Record<string, unknown>, options)
	}
}
