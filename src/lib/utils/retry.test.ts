// T183: Tests for retry logic (using p-retry)
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { invokeWithRetry, createRetryCommand } from './retry'

// Mock @tauri-apps/api/core
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn(),
}))

describe('Retry Utility', () => {
	beforeEach(() => {
		vi.clearAllMocks()
	})

	afterEach(() => {
		vi.restoreAllMocks()
	})

	describe('invokeWithRetry', () => {
		it('should succeed on first attempt', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockResolvedValueOnce({ success: true })

			const result = await invokeWithRetry('test_command', { arg: 'value' })

			expect(result).toEqual({ success: true })
			expect(invoke).toHaveBeenCalledTimes(1)
			expect(invoke).toHaveBeenCalledWith('test_command', { arg: 'value' })
		})

		it('should retry on database errors', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke)
				.mockRejectedValueOnce(new Error('Database lock timeout'))
				.mockRejectedValueOnce(new Error('Database is busy'))
				.mockResolvedValueOnce({ success: true })

			const result = await invokeWithRetry('test_command', {})

			expect(result).toEqual({ success: true })
			expect(invoke).toHaveBeenCalledTimes(3)
		})

		it('should retry on connection errors', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke)
				.mockRejectedValueOnce(new Error('Connection timeout'))
				.mockResolvedValueOnce({ success: true })

			const result = await invokeWithRetry('test_command', {})

			expect(result).toEqual({ success: true })
			expect(invoke).toHaveBeenCalledTimes(2)
		})

		it('should not retry on non-retryable errors', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValueOnce(new Error('Validation error'))

			await expect(invokeWithRetry('test_command', {})).rejects.toThrow('Validation error')

			expect(invoke).toHaveBeenCalledTimes(1)
		})

		it('should respect maxAttempts option', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValue(new Error('Database lock timeout'))

			await expect(invokeWithRetry('test_command', {}, { maxAttempts: 2 })).rejects.toThrow(
				'Database lock timeout'
			)

			expect(invoke).toHaveBeenCalledTimes(2)
		})

		it('should use custom shouldRetry function', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValue(new Error('Custom retryable error'))

			const shouldRetry = vi.fn((error) => {
				return error instanceof Error && error.message.includes('Custom')
			})

			await expect(
				invokeWithRetry(
					'test_command',
					{},
					{
						maxAttempts: 2,
						shouldRetry,
					}
				)
			).rejects.toThrow('Custom retryable error')

			expect(shouldRetry).toHaveBeenCalled()
			expect(invoke).toHaveBeenCalledTimes(2)
		})

		it('should retry on lock poisoned errors', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke)
				.mockRejectedValueOnce(new Error('Lock poisoned'))
				.mockResolvedValueOnce({ success: true })

			const result = await invokeWithRetry('test_command', {})

			expect(result).toEqual({ success: true })
			expect(invoke).toHaveBeenCalledTimes(2)
		})

		it('should throw error when all retries are exhausted', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValue(new Error('Database lock timeout'))

			await expect(invokeWithRetry('test_command', {}, { maxAttempts: 3 })).rejects.toThrow(
				'Database lock timeout'
			)
			expect(invoke).toHaveBeenCalledTimes(3)
		})

		it('should not retry validation errors', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValueOnce(new Error('Validation failed: invalid rating'))

			await expect(invokeWithRetry('test_command', {})).rejects.toThrow(
				'Validation failed: invalid rating'
			)

			expect(invoke).toHaveBeenCalledTimes(1)
		})

		it('should not retry authentication errors', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValueOnce(new Error('Unauthorized: invalid token'))

			await expect(invokeWithRetry('test_command', {})).rejects.toThrow(
				'Unauthorized: invalid token'
			)

			expect(invoke).toHaveBeenCalledTimes(1)
		})

		it('should not retry when shouldRetry returns false immediately', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			const error = new Error('Non-retryable error')
			vi.mocked(invoke).mockRejectedValueOnce(error)

			const shouldRetry = vi.fn(() => false)

			await expect(invokeWithRetry('test_command', {}, { shouldRetry })).rejects.toThrow(
				'Non-retryable error'
			)

			expect(shouldRetry).toHaveBeenCalledWith(error)
			expect(invoke).toHaveBeenCalledTimes(1)
		})

		it('should not retry non-Error objects', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValueOnce('String error')

			// p-retry wraps non-retryable errors in AbortError
			await expect(invokeWithRetry('test_command', {})).rejects.toThrow('String error')

			expect(invoke).toHaveBeenCalledTimes(1)
		})

		it('should check Error.name for SQLiteError', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			const error = new Error('Operation failed')
			error.name = 'SQLiteError'

			vi.mocked(invoke).mockRejectedValueOnce(error).mockResolvedValueOnce({ success: true })

			const result = await invokeWithRetry('test_command', {})

			expect(result).toEqual({ success: true })
			expect(invoke).toHaveBeenCalledTimes(2)
		})
	})

	describe('createRetryCommand', () => {
		it('should create a command wrapper with default options', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockResolvedValueOnce({ data: 'test' })

			const testCommand = createRetryCommand<{ id: number }, { data: string }>('test_cmd', {
				maxAttempts: 5,
			})

			const result = await testCommand({ id: 123 })

			expect(result).toEqual({ data: 'test' })
			expect(invoke).toHaveBeenCalledWith('test_cmd', { id: 123 })
		})

		it('should allow overriding options', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValue(new Error('Database lock timeout'))

			const testCommand = createRetryCommand('test_cmd', { maxAttempts: 5 })

			await expect(testCommand({}, { maxAttempts: 2 })).rejects.toThrow('Database lock timeout')

			// Should use override maxAttempts (2) not default (5)
			expect(invoke).toHaveBeenCalledTimes(2)
		})

		it('should work without arguments', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockResolvedValueOnce({ status: 'ok' })

			const testCommand = createRetryCommand('no_args_cmd')

			const result = await testCommand()

			expect(result).toEqual({ status: 'ok' })
			expect(invoke).toHaveBeenCalledWith('no_args_cmd', undefined)
		})
	})
})
