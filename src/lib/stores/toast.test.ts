import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest'
import { get } from 'svelte/store'
import { toastStore } from './toast'

describe('Toast Store', () => {
	beforeEach(() => {
		// Clear all toasts before each test
		toastStore.clear()
		vi.useFakeTimers()
	})

	afterEach(() => {
		vi.restoreAllMocks()
	})

	describe('Initial State', () => {
		it('should start with no toasts', () => {
			const state = get(toastStore)
			expect(state.toasts).toEqual([])
		})
	})

	describe('show()', () => {
		it('should add a toast with custom message and type', () => {
			toastStore.show('Test message', 'info')
			const state = get(toastStore)

			expect(state.toasts).toHaveLength(1)
			expect(state.toasts[0].message).toBe('Test message')
			expect(state.toasts[0].type).toBe('info')
		})

		it('should generate unique IDs for each toast', () => {
			const id1 = toastStore.show('Message 1')
			const id2 = toastStore.show('Message 2')

			expect(id1).not.toBe(id2)

			const state = get(toastStore)
			expect(state.toasts).toHaveLength(2)
			expect(state.toasts[0].id).toBe(id1)
			expect(state.toasts[1].id).toBe(id2)
		})

		it('should default to info type when not specified', () => {
			toastStore.show('Test message')
			const state = get(toastStore)

			expect(state.toasts[0].type).toBe('info')
		})

		it('should use custom duration when provided', () => {
			toastStore.show('Test message', 'info', 10000)
			const state = get(toastStore)

			expect(state.toasts[0].duration).toBe(10000)
		})

		it('should default to 5000ms duration', () => {
			toastStore.show('Test message')
			const state = get(toastStore)

			expect(state.toasts[0].duration).toBe(5000)
		})

		it('should auto-dismiss after duration expires', () => {
			toastStore.show('Test message', 'info', 3000)

			// Toast should exist initially
			let state = get(toastStore)
			expect(state.toasts).toHaveLength(1)

			// Fast-forward time by 3000ms
			vi.advanceTimersByTime(3000)

			// Toast should be dismissed
			state = get(toastStore)
			expect(state.toasts).toHaveLength(0)
		})

		it('should not auto-dismiss when duration is 0', () => {
			toastStore.show('Persistent message', 'info', 0)

			// Fast-forward a long time
			vi.advanceTimersByTime(100000)

			// Toast should still exist
			const state = get(toastStore)
			expect(state.toasts).toHaveLength(1)
		})
	})

	describe('success()', () => {
		it('should create a success toast', () => {
			toastStore.success('Success message')
			const state = get(toastStore)

			expect(state.toasts).toHaveLength(1)
			expect(state.toasts[0].message).toBe('Success message')
			expect(state.toasts[0].type).toBe('success')
		})

		it('should use custom duration', () => {
			toastStore.success('Success message', 8000)
			const state = get(toastStore)

			expect(state.toasts[0].duration).toBe(8000)
		})
	})

	describe('error()', () => {
		it('should create an error toast', () => {
			toastStore.error('Error message')
			const state = get(toastStore)

			expect(state.toasts).toHaveLength(1)
			expect(state.toasts[0].message).toBe('Error message')
			expect(state.toasts[0].type).toBe('error')
		})

		it('should use custom duration', () => {
			toastStore.error('Error message', 6000)
			const state = get(toastStore)

			expect(state.toasts[0].duration).toBe(6000)
		})
	})

	describe('info()', () => {
		it('should create an info toast', () => {
			toastStore.info('Info message')
			const state = get(toastStore)

			expect(state.toasts).toHaveLength(1)
			expect(state.toasts[0].message).toBe('Info message')
			expect(state.toasts[0].type).toBe('info')
		})

		it('should use custom duration', () => {
			toastStore.info('Info message', 4000)
			const state = get(toastStore)

			expect(state.toasts[0].duration).toBe(4000)
		})
	})

	describe('warning()', () => {
		it('should create a warning toast', () => {
			toastStore.warning('Warning message')
			const state = get(toastStore)

			expect(state.toasts).toHaveLength(1)
			expect(state.toasts[0].message).toBe('Warning message')
			expect(state.toasts[0].type).toBe('warning')
		})

		it('should use custom duration', () => {
			toastStore.warning('Warning message', 7000)
			const state = get(toastStore)

			expect(state.toasts[0].duration).toBe(7000)
		})
	})

	describe('dismiss()', () => {
		it('should remove a specific toast by ID', () => {
			const id1 = toastStore.show('Message 1')
			const id2 = toastStore.show('Message 2')
			const id3 = toastStore.show('Message 3')

			let state = get(toastStore)
			expect(state.toasts).toHaveLength(3)

			// Dismiss middle toast
			toastStore.dismiss(id2)

			state = get(toastStore)
			expect(state.toasts).toHaveLength(2)
			expect(state.toasts.find((t) => t.id === id1)).toBeDefined()
			expect(state.toasts.find((t) => t.id === id2)).toBeUndefined()
			expect(state.toasts.find((t) => t.id === id3)).toBeDefined()
		})

		it('should do nothing if ID does not exist', () => {
			toastStore.show('Message 1')
			const state = get(toastStore)
			expect(state.toasts).toHaveLength(1)

			toastStore.dismiss('non-existent-id')

			const newState = get(toastStore)
			expect(newState.toasts).toHaveLength(1)
		})
	})

	describe('clear()', () => {
		it('should remove all toasts', () => {
			toastStore.show('Message 1')
			toastStore.show('Message 2')
			toastStore.show('Message 3')

			let state = get(toastStore)
			expect(state.toasts).toHaveLength(3)

			toastStore.clear()

			state = get(toastStore)
			expect(state.toasts).toHaveLength(0)
		})

		it('should work when there are no toasts', () => {
			toastStore.clear()
			const state = get(toastStore)
			expect(state.toasts).toEqual([])
		})
	})

	describe('Multiple Toasts', () => {
		it('should handle multiple toasts of different types', () => {
			toastStore.success('Success!')
			toastStore.error('Error!')
			toastStore.warning('Warning!')
			toastStore.info('Info!')

			const state = get(toastStore)
			expect(state.toasts).toHaveLength(4)
			expect(state.toasts[0].type).toBe('success')
			expect(state.toasts[1].type).toBe('error')
			expect(state.toasts[2].type).toBe('warning')
			expect(state.toasts[3].type).toBe('info')
		})

		it('should maintain toast order (FIFO)', () => {
			toastStore.show('First')
			toastStore.show('Second')
			toastStore.show('Third')

			const state = get(toastStore)
			expect(state.toasts[0].message).toBe('First')
			expect(state.toasts[1].message).toBe('Second')
			expect(state.toasts[2].message).toBe('Third')
		})

		it('should auto-dismiss multiple toasts independently', () => {
			toastStore.show('Message 1', 'info', 1000)
			toastStore.show('Message 2', 'info', 3000)
			toastStore.show('Message 3', 'info', 5000)

			// Initially all 3 should exist
			let state = get(toastStore)
			expect(state.toasts).toHaveLength(3)

			// After 1000ms, first should be dismissed
			vi.advanceTimersByTime(1000)
			state = get(toastStore)
			expect(state.toasts).toHaveLength(2)
			expect(state.toasts[0].message).toBe('Message 2')

			// After another 2000ms (total 3000ms), second should be dismissed
			vi.advanceTimersByTime(2000)
			state = get(toastStore)
			expect(state.toasts).toHaveLength(1)
			expect(state.toasts[0].message).toBe('Message 3')

			// After another 2000ms (total 5000ms), third should be dismissed
			vi.advanceTimersByTime(2000)
			state = get(toastStore)
			expect(state.toasts).toHaveLength(0)
		})
	})

	describe('Return Values', () => {
		it('should return toast ID from show()', () => {
			const id = toastStore.show('Test')
			expect(id).toMatch(/^toast-\d+-/)
		})

		it('should return toast ID from convenience methods', () => {
			const successId = toastStore.success('Success')
			const errorId = toastStore.error('Error')
			const warningId = toastStore.warning('Warning')
			const infoId = toastStore.info('Info')

			expect(successId).toMatch(/^toast-\d+-/)
			expect(errorId).toMatch(/^toast-\d+-/)
			expect(warningId).toMatch(/^toast-\d+-/)
			expect(infoId).toMatch(/^toast-\d+-/)
		})
	})
})
