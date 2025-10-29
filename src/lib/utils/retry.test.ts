// T183: Tests for retry logic
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { invokeWithRetry, createRetryCommand } from './retry'

// Mock @tauri-apps/api/core
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn(),
}))

describe('Retry Utility', () => {
	beforeEach(() => {
		vi.clearAllMocks()
		vi.useFakeTimers()
	})

	afterEach(() => {
		vi.restoreAllMocks()
		vi.useRealTimers()
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

			const promise = invokeWithRetry('test_command', {})

			// Fast-forward through first retry delay
			await vi.advanceTimersByTimeAsync(100)
			// Fast-forward through second retry delay (200ms with exponential backoff)
			await vi.advanceTimersByTimeAsync(200)

			const result = await promise

			expect(result).toEqual({ success: true })
			expect(invoke).toHaveBeenCalledTimes(3)
		})

		it('should retry on connection errors', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke)
				.mockRejectedValueOnce(new Error('Connection timeout'))
				.mockResolvedValueOnce({ success: true })

			const promise = invokeWithRetry('test_command', {})

			await vi.advanceTimersByTimeAsync(100)

			const result = await promise

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
			vi.mocked(invoke).mockRejectedValue(new Error('Database error'))

			const promise = invokeWithRetry('test_command', {}, { maxAttempts: 2 })

			await vi.advanceTimersByTimeAsync(100)

			await expect(promise).rejects.toThrow('Database error')

			expect(invoke).toHaveBeenCalledTimes(2)
		})

		it('should use exponential backoff', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValue(new Error('Database busy'))

			const promise = invokeWithRetry(
				'test_command',
				{},
				{
					maxAttempts: 4,
					initialDelay: 100,
					backoffMultiplier: 2,
				}
			)

			// First attempt fails immediately
			await vi.advanceTimersByTimeAsync(1)

			// Second attempt (delay: 100ms)
			await vi.advanceTimersByTimeAsync(100)

			// Third attempt (delay: 200ms)
			await vi.advanceTimersByTimeAsync(200)

			// Fourth attempt (delay: 400ms)
			await vi.advanceTimersByTimeAsync(400)

			await expect(promise).rejects.toThrow('Database busy')

			expect(invoke).toHaveBeenCalledTimes(4)
		})

		it('should respect maxDelay option', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValue(new Error('Database error'))

			const promise = invokeWithRetry(
				'test_command',
				{},
				{
					maxAttempts: 4,
					initialDelay: 500,
					maxDelay: 600,
					backoffMultiplier: 2,
				}
			)

			await vi.advanceTimersByTimeAsync(1)

			// Second attempt (delay: 500ms)
			await vi.advanceTimersByTimeAsync(500)

			// Third attempt (delay: 600ms, capped by maxDelay instead of 1000ms)
			await vi.advanceTimersByTimeAsync(600)

			// Fourth attempt (delay: 600ms, capped)
			await vi.advanceTimersByTimeAsync(600)

			await expect(promise).rejects.toThrow('Database error')

			expect(invoke).toHaveBeenCalledTimes(4)
		})

		it('should use custom shouldRetry function', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke).mockRejectedValue(new Error('Custom retryable error'))

			const shouldRetry = vi.fn((error) => String(error).includes('Custom'))

			const promise = invokeWithRetry(
				'test_command',
				{},
				{
					maxAttempts: 2,
					shouldRetry,
				}
			)

			// Advance through retry delay
			await vi.advanceTimersByTimeAsync(100)

			await expect(promise).rejects.toThrow('Custom retryable error')

			expect(shouldRetry).toHaveBeenCalled()
			expect(invoke).toHaveBeenCalledTimes(2)
		})

		it('should retry on lock poisoned errors', async () => {
			const { invoke } = await import('@tauri-apps/api/core')
			vi.mocked(invoke)
				.mockRejectedValueOnce(new Error('Lock poisoned'))
				.mockResolvedValueOnce({ success: true })

			const promise = invokeWithRetry('test_command', {})

			await vi.advanceTimersByTimeAsync(100)

			const result = await promise

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
			vi.mocked(invoke).mockRejectedValue(new Error('Database error'))

			const testCommand = createRetryCommand('test_cmd', { maxAttempts: 5 })

			const promise = testCommand({}, { maxAttempts: 2 })

			await vi.advanceTimersByTimeAsync(100)

			await expect(promise).rejects.toThrow('Database error')

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
