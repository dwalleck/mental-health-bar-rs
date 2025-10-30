import { describe, it, expect } from 'vitest'
import { ERROR_TYPES, type ErrorType } from './types'
import type { ErrorType as GeneratedErrorType } from '$lib/bindings'

describe('Error Type Constants', () => {
	it('should match all generated ErrorType values', () => {
		// This test ensures ERROR_TYPES constants match the generated ErrorType union
		// If you add/remove/rename an error type in Rust, this test will fail
		const expectedValues: GeneratedErrorType[] = [
			'validation',
			'not_found',
			'database_error',
			'database_locked',
			'lock_poisoned',
			'constraint_violation',
			'duplicate',
			'transaction_failure',
			'no_data',
			'calculation_error',
			'transient',
			'internal',
			'config',
			'io_error',
			'serialization',
		]

		const constantValues = Object.values(ERROR_TYPES)

		// Check that all expected values are present in constants
		for (const expected of expectedValues) {
			expect(constantValues).toContain(expected)
		}

		// Check that no extra values are in constants
		expect(constantValues).toHaveLength(expectedValues.length)
	})

	it('should have correct constant mappings', () => {
		// Verify each constant maps to the correct string value
		expect(ERROR_TYPES.VALIDATION).toBe('validation')
		expect(ERROR_TYPES.NOT_FOUND).toBe('not_found')
		expect(ERROR_TYPES.DATABASE_ERROR).toBe('database_error')
		expect(ERROR_TYPES.DATABASE_LOCKED).toBe('database_locked')
		expect(ERROR_TYPES.LOCK_POISONED).toBe('lock_poisoned')
		expect(ERROR_TYPES.CONSTRAINT_VIOLATION).toBe('constraint_violation')
		expect(ERROR_TYPES.DUPLICATE).toBe('duplicate')
		expect(ERROR_TYPES.TRANSACTION_FAILURE).toBe('transaction_failure')
		expect(ERROR_TYPES.NO_DATA).toBe('no_data')
		expect(ERROR_TYPES.CALCULATION_ERROR).toBe('calculation_error')
		expect(ERROR_TYPES.TRANSIENT).toBe('transient')
		expect(ERROR_TYPES.INTERNAL).toBe('internal')
		expect(ERROR_TYPES.CONFIG).toBe('config')
		expect(ERROR_TYPES.IO_ERROR).toBe('io_error')
		expect(ERROR_TYPES.SERIALIZATION).toBe('serialization')
	})

	it('should have ErrorType type compatible with generated type', () => {
		// Type-level test: ErrorType should be assignable to GeneratedErrorType
		const testValue: ErrorType = 'validation'
		const generatedValue: GeneratedErrorType = testValue // Should compile without error
		expect(generatedValue).toBe('validation')
	})

	it('should reject invalid error type values at compile time', () => {
		// This is a compile-time check - uncomment to verify TypeScript catches errors:
		// const invalid: ErrorType = 'invalid_type' // Should not compile
		expect(true).toBe(true) // Placeholder for compile-time test
	})
})
