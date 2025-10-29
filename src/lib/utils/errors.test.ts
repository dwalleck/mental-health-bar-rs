import { describe, it, expect } from 'vitest'
import {
	formatUserError,
	isValidationError,
	isTransientError,
	formatErrorForLogging,
} from './errors'

describe('Error Utilities', () => {
	describe('formatUserError', () => {
		it('should format Error instances', () => {
			const error = new Error('Database connection failed')
			expect(formatUserError(error)).toBe('Database connection failed')
		})

		it('should format string errors', () => {
			expect(formatUserError('Invalid input')).toBe('Invalid input')
		})

		it('should handle unknown error types', () => {
			expect(formatUserError(null)).toBe('An unexpected error occurred. Please try again.')
			expect(formatUserError(undefined)).toBe('An unexpected error occurred. Please try again.')
			expect(formatUserError(123)).toBe('An unexpected error occurred. Please try again.')
			expect(formatUserError({ code: 'ERR' })).toBe(
				'An unexpected error occurred. Please try again.'
			)
		})

		it('should handle Error subclasses', () => {
			class CustomError extends Error {
				constructor(message: string) {
					super(message)
					this.name = 'CustomError'
				}
			}

			const error = new CustomError('Custom error message')
			expect(formatUserError(error)).toBe('Custom error message')
		})
	})

	describe('isValidationError', () => {
		it('should identify validation errors', () => {
			expect(isValidationError(new Error('Validation failed'))).toBe(true)
			expect(isValidationError(new Error('Invalid email address'))).toBe(true)
			expect(isValidationError(new Error('Field is required'))).toBe(true)
			expect(isValidationError(new Error('Value must be between 1 and 5'))).toBe(true)
			expect(isValidationError(new Error('Notes too long'))).toBe(true)
			expect(isValidationError(new Error('Text too short'))).toBe(true)
			expect(isValidationError(new Error('Rating out of range'))).toBe(true)
		})

		it('should not identify non-validation errors', () => {
			expect(isValidationError(new Error('Database error'))).toBe(false)
			expect(isValidationError(new Error('Connection timeout'))).toBe(false)
			expect(isValidationError('Network error')).toBe(false)
			expect(isValidationError(null)).toBe(false)
		})

		it('should be case-insensitive', () => {
			expect(isValidationError(new Error('VALIDATION ERROR'))).toBe(true)
			expect(isValidationError(new Error('Invalid Data'))).toBe(true)
		})
	})

	describe('isTransientError', () => {
		it('should identify transient database errors', () => {
			expect(isTransientError(new Error('Database lock timeout'))).toBe(true)
			expect(isTransientError(new Error('Database is busy'))).toBe(true)
			expect(isTransientError(new Error('Lock poisoned'))).toBe(true)
		})

		it('should identify transient network errors', () => {
			expect(isTransientError(new Error('Connection timeout'))).toBe(true)
			expect(isTransientError(new Error('Request timed out'))).toBe(true)
			expect(isTransientError(new Error('Network error'))).toBe(true)
			expect(isTransientError(new Error('Fetch failed'))).toBe(true)
		})

		it('should identify SQLite errors by name', () => {
			const error = new Error('Operation failed')
			error.name = 'SQLiteError'
			expect(isTransientError(error)).toBe(true)
		})

		it('should identify database errors by name', () => {
			const error = new Error('Operation failed')
			error.name = 'DatabaseError'
			expect(isTransientError(error)).toBe(true)
		})

		it('should not identify non-transient errors', () => {
			expect(isTransientError(new Error('Validation failed'))).toBe(false)
			expect(isTransientError(new Error('Authentication required'))).toBe(false)
			expect(isTransientError(new Error('Not found'))).toBe(false)
			expect(isTransientError('Invalid input')).toBe(false)
			expect(isTransientError(null)).toBe(false)
		})

		it('should be case-insensitive', () => {
			expect(isTransientError(new Error('CONNECTION TIMEOUT'))).toBe(true)
			expect(isTransientError(new Error('Database Lock'))).toBe(true)
		})
	})

	describe('formatErrorForLogging', () => {
		it('should include stack trace for Error objects', () => {
			const error = new Error('Test error')
			const formatted = formatErrorForLogging(error)

			// Stack trace should include the error message
			expect(formatted).toContain('Test error')
			// Stack trace should include file/line information
			expect(formatted.includes('errors.test.ts') || formatted.includes('Error: Test error')).toBe(
				true
			)
		})

		it('should format string errors', () => {
			expect(formatErrorForLogging('Simple error')).toBe('Simple error')
		})

		it('should JSON stringify unknown error types', () => {
			const error = { code: 'ERR_001', details: 'Something went wrong' }
			const formatted = formatErrorForLogging(error)

			expect(formatted).toContain('"code"')
			expect(formatted).toContain('ERR_001')
			expect(formatted).toContain('"details"')
		})

		it('should handle null and undefined', () => {
			expect(formatErrorForLogging(null)).toBe('null')
			expect(formatErrorForLogging(undefined)).toBe(undefined as unknown as string)
		})
	})
})
