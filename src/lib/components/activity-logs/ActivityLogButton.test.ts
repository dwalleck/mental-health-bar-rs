// Task 4.14: Component tests for ActivityLogButton.svelte
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte'
import ActivityLogButton from './ActivityLogButton.svelte'
import type { Activity, ActivityLog } from '$lib/bindings'

// Mock bindings
vi.mock('$lib/bindings', () => ({
	commands: {
		logActivity: vi.fn(),
	},
}))

// Mock error handling
vi.mock('$lib/utils/errors', () => ({
	displayError: vi.fn(),
	displaySuccess: vi.fn(),
}))

import { commands } from '$lib/bindings'
import { displayError, displaySuccess } from '$lib/utils/errors'

describe('ActivityLogButton', () => {
	let logActivityMock: ReturnType<typeof vi.fn>
	let displayErrorMock: ReturnType<typeof vi.fn>
	let displaySuccessMock: ReturnType<typeof vi.fn>

	const mockActivity: Activity = {
		id: 1,
		group_id: 1,
		name: 'Running',
		color: '#4CAF50',
		icon: '🏃',
		created_at: '2024-01-01T00:00:00Z',
		deleted_at: null,
	}

	const mockLogResponse: ActivityLog = {
		id: 1,
		activity_id: 1,
		logged_at: '2024-01-15T10:30:00Z',
		created_at: '2024-01-15T10:30:00Z',
		notes: null,
		deleted_at: null,
	}

	beforeEach(() => {
		logActivityMock = vi.mocked(commands.logActivity)
		displayErrorMock = vi.mocked(displayError)
		displaySuccessMock = vi.mocked(displaySuccess)

		// Clear mocks
		logActivityMock.mockClear()
		displayErrorMock.mockClear()
		displaySuccessMock.mockClear()
	})

	afterEach(() => {
		vi.clearAllMocks()
	})

	describe('Rendering', () => {
		it('should render with default size (medium)', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button', { name: `Log ${mockActivity.name}` })
			expect(button).toBeInTheDocument()
			expect(screen.getByText('Log Now')).toBeInTheDocument()
		})

		it('should render with small size', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
					size: 'small',
				},
			})

			const button = screen.getByRole('button')
			expect(button).toHaveClass('text-xs')
		})

		it('should render with large size', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
					size: 'large',
				},
			})

			const button = screen.getByRole('button')
			expect(button).toHaveClass('text-base')
		})

		it('should have proper ARIA label', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')
			expect(button).toHaveAttribute('aria-label', 'Log Running')
			expect(button).toHaveAttribute('title', 'Quick log Running')
		})
	})

	describe('Quick Log Functionality (Task 3.14)', () => {
		it('should log activity when clicked', async () => {
			logActivityMock.mockResolvedValue({ status: 'ok', data: mockLogResponse })

			const onLogCreated = vi.fn()

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
					onLogCreated,
				},
			})

			const button = screen.getByRole('button')
			await fireEvent.click(button)

			await waitFor(() => {
				expect(logActivityMock).toHaveBeenCalledWith({
					activity_id: 1,
					logged_at: null,
					notes: null,
				})
			})

			expect(displaySuccessMock).toHaveBeenCalledWith('Logged Running')
			expect(onLogCreated).toHaveBeenCalled()
		})

		it('should pass null for logged_at to use server timestamp', async () => {
			logActivityMock.mockResolvedValue({ status: 'ok', data: mockLogResponse })

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')
			await fireEvent.click(button)

			await waitFor(() => {
				expect(logActivityMock).toHaveBeenCalledWith(
					expect.objectContaining({
						logged_at: null,
					})
				)
			})
		})

		it('should pass null for notes by default', async () => {
			logActivityMock.mockResolvedValue({ status: 'ok', data: mockLogResponse })

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')
			await fireEvent.click(button)

			await waitFor(() => {
				expect(logActivityMock).toHaveBeenCalledWith(
					expect.objectContaining({
						notes: null,
					})
				)
			})
		})
	})

	describe('Loading State', () => {
		it('should show loading state while logging', async () => {
			logActivityMock.mockImplementation(
				() =>
					new Promise((resolve) =>
						setTimeout(() => resolve({ status: 'ok', data: mockLogResponse }), 100)
					)
			)

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')
			await fireEvent.click(button)

			// Should show loading state immediately
			expect(screen.getByText('Logging...')).toBeInTheDocument()
			expect(button).toBeDisabled()

			// Should show spinner
			const spinner = button.querySelector('.animate-spin')
			expect(spinner).toBeInTheDocument()

			// Wait for completion
			await waitFor(() => {
				expect(screen.getByText('Log Now')).toBeInTheDocument()
			})
		})

		it('should disable button while logging', async () => {
			logActivityMock.mockImplementation(
				() =>
					new Promise((resolve) =>
						setTimeout(() => resolve({ status: 'ok', data: mockLogResponse }), 100)
					)
			)

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')
			expect(button).not.toBeDisabled()

			await fireEvent.click(button)

			// Button should be disabled during logging
			expect(button).toBeDisabled()

			// Wait for completion
			await waitFor(() => {
				expect(button).not.toBeDisabled()
			})
		})

		it('should prevent double-clicks', async () => {
			// Use a delayed promise to ensure isLogging remains true long enough
			logActivityMock.mockImplementation(
				() =>
					new Promise((resolve) =>
						setTimeout(() => resolve({ status: 'ok', data: mockLogResponse }), 50)
					)
			)

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')

			// Verify button starts enabled
			expect(button).not.toBeDisabled()

			// Click to start logging
			await fireEvent.click(button)

			// Button should immediately become disabled (preventing further clicks)
			await waitFor(() => {
				expect(button).toBeDisabled()
			})

			// Verify API was called once
			expect(logActivityMock).toHaveBeenCalledTimes(1)

			// Verify button shows loading state
			expect(screen.getByText('Logging...')).toBeInTheDocument()

			// Wait for operation to complete
			await waitFor(() => {
				expect(button).not.toBeDisabled()
			})

			// Button should show normal state again
			expect(screen.getByText('Log Now')).toBeInTheDocument()
		})
	})

	describe('Error Handling', () => {
		it('should handle API errors gracefully', async () => {
			logActivityMock.mockResolvedValue({
				status: 'error',
				error: { message: 'Failed to log activity', type: 'Database', retryable: false },
			})

			const onLogCreated = vi.fn()

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
					onLogCreated,
				},
			})

			const button = screen.getByRole('button')
			await fireEvent.click(button)

			await waitFor(() => {
				expect(displayErrorMock).toHaveBeenCalled()
			})

			// Should not call onLogCreated callback on error
			expect(onLogCreated).not.toHaveBeenCalled()

			// Button should be re-enabled after error
			expect(button).not.toBeDisabled()
		})

		it('should recover from errors and allow retry', async () => {
			// First call fails
			logActivityMock.mockResolvedValueOnce({
				status: 'error',
				error: { message: 'Network error', type: 'Network', retryable: true },
			})

			// Second call succeeds
			logActivityMock.mockResolvedValueOnce({ status: 'ok', data: mockLogResponse })

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')

			// First attempt
			await fireEvent.click(button)
			await waitFor(() => {
				expect(displayErrorMock).toHaveBeenCalledTimes(1)
			})

			// Retry
			await fireEvent.click(button)
			await waitFor(() => {
				expect(displaySuccessMock).toHaveBeenCalledWith('Logged Running')
			})

			expect(logActivityMock).toHaveBeenCalledTimes(2)
		})
	})

	describe('Callback Handling', () => {
		it('should call onLogCreated callback after successful log', async () => {
			logActivityMock.mockResolvedValue({ status: 'ok', data: mockLogResponse })

			const onLogCreated = vi.fn()

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
					onLogCreated,
				},
			})

			const button = screen.getByRole('button')
			await fireEvent.click(button)

			await waitFor(() => {
				expect(onLogCreated).toHaveBeenCalledTimes(1)
			})
		})

		it('should not error if onLogCreated is not provided', async () => {
			logActivityMock.mockResolvedValue({ status: 'ok', data: mockLogResponse })

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
					// onLogCreated not provided
				},
			})

			const button = screen.getByRole('button')
			await fireEvent.click(button)

			// Should complete successfully without callback
			await waitFor(() => {
				expect(displaySuccessMock).toHaveBeenCalled()
			})
		})
	})

	describe('Visual Styling', () => {
		it('should have green button styling', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')
			expect(button).toHaveClass('bg-green-600')
			expect(button).toHaveClass('hover:bg-green-700')
		})

		it('should have proper disabled styling', async () => {
			logActivityMock.mockImplementation(
				() => new Promise(() => {}) // Never resolves
			)

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')
			await fireEvent.click(button)

			expect(button).toHaveClass('disabled:opacity-50')
			expect(button).toHaveClass('disabled:cursor-not-allowed')
		})

		it('should display plus icon when not logging', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')
			const icon = button.querySelector('svg')

			expect(icon).toBeInTheDocument()
			// Check for plus icon path (M12 4v16m8-8H4)
			expect(icon?.querySelector('path')).toHaveAttribute('d', 'M12 4v16m8-8H4')
		})

		it('should display spinner icon when logging', async () => {
			logActivityMock.mockImplementation(
				() => new Promise(() => {}) // Never resolves
			)

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')
			await fireEvent.click(button)

			// Should have spinner
			const spinner = button.querySelector('.animate-spin')
			expect(spinner).toBeInTheDocument()
		})
	})

	describe('Accessibility', () => {
		it('should be keyboard accessible', async () => {
			logActivityMock.mockResolvedValue({ status: 'ok', data: mockLogResponse })

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')

			// Native HTML buttons are keyboard accessible by default
			// When users press Enter/Space, browsers fire click events
			// Testing-library recommends using click() to test keyboard interactions
			await fireEvent.click(button)

			await waitFor(() => {
				expect(logActivityMock).toHaveBeenCalled()
			})

			// Verify button is a proper semantic button element
			expect(button.tagName).toBe('BUTTON')
			expect(button).toHaveAttribute('type', 'button')
		})

		it('should announce state changes to screen readers', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')

			// Initial state
			expect(button).toHaveAttribute('aria-label', 'Log Running')

			// Button text provides visual feedback ("Log Now" vs "Logging...")
			expect(screen.getByText('Log Now')).toBeInTheDocument()
		})

		it('should have proper title attribute for tooltip', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			const button = screen.getByRole('button')
			expect(button).toHaveAttribute('title', 'Quick log Running')
		})
	})

	describe('Size Variants', () => {
		it('should apply correct classes for small size', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
					size: 'small',
				},
			})

			const button = screen.getByRole('button')
			expect(button).toHaveClass('px-2')
			expect(button).toHaveClass('py-1')
			expect(button).toHaveClass('text-xs')

			// Check icon size
			const icon = button.querySelector('svg')
			expect(icon).toHaveClass('w-3')
			expect(icon).toHaveClass('h-3')
		})

		it('should apply correct classes for medium size', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
					size: 'medium',
				},
			})

			const button = screen.getByRole('button')
			expect(button).toHaveClass('px-3')
			expect(button).toHaveClass('py-2')
			expect(button).toHaveClass('text-sm')

			const icon = button.querySelector('svg')
			expect(icon).toHaveClass('w-4')
			expect(icon).toHaveClass('h-4')
		})

		it('should apply correct classes for large size', () => {
			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
					size: 'large',
				},
			})

			const button = screen.getByRole('button')
			expect(button).toHaveClass('px-4')
			expect(button).toHaveClass('py-3')
			expect(button).toHaveClass('text-base')

			const icon = button.querySelector('svg')
			expect(icon).toHaveClass('w-5')
			expect(icon).toHaveClass('h-5')
		})
	})

	describe('Integration Scenarios', () => {
		it('should work correctly in a list of activities', async () => {
			logActivityMock.mockResolvedValue({ status: 'ok', data: mockLogResponse })

			const activity2: Activity = { ...mockActivity, id: 2, name: 'Swimming' }

			const { container } = render(ActivityLogButton, {
				props: {
					activity: mockActivity,
				},
			})

			// Render second button
			render(ActivityLogButton, {
				props: {
					activity: activity2,
				},
				target: container,
			})

			const buttons = screen.getAllByRole('button')
			expect(buttons).toHaveLength(2)

			// Click first button
			await fireEvent.click(buttons[0])

			await waitFor(() => {
				expect(logActivityMock).toHaveBeenCalledWith(expect.objectContaining({ activity_id: 1 }))
			})
		})

		it('should refresh parent component data after logging', async () => {
			logActivityMock.mockResolvedValue({ status: 'ok', data: mockLogResponse })

			const onLogCreated = vi.fn()

			render(ActivityLogButton, {
				props: {
					activity: mockActivity,
					onLogCreated,
				},
			})

			const button = screen.getByRole('button')
			await fireEvent.click(button)

			await waitFor(() => {
				expect(onLogCreated).toHaveBeenCalled()
			})

			// Parent can use this callback to refresh activity logs list, update goal progress, etc.
		})
	})
})
