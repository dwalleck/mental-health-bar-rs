import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte'
import ScheduleList from './ScheduleList.svelte'
import type { AssessmentSchedule } from '$lib/bindings'

// Mock the bindings module
vi.mock('$lib/bindings', () => ({
	commands: {
		getSchedules: vi.fn(),
	},
}))

// Mock retry utility
vi.mock('$lib/utils/retry', () => ({
	invokeWithRetry: vi.fn(),
}))

// Mock error handling utilities
vi.mock('$lib/utils/errors', () => ({
	displayError: vi.fn((error) => ({
		type: 'inline',
		message: typeof error === 'string' ? error : error?.message || 'Error',
	})),
	displaySuccess: vi.fn(),
	formatUserError: vi.fn((error) =>
		typeof error === 'string' ? error : error?.message || 'Error'
	),
	isValidationError: vi.fn(() => false),
	isCommandError: vi.fn(() => false),
}))

import { commands } from '$lib/bindings'
import { invokeWithRetry } from '$lib/utils/retry'
import { displayError, displaySuccess } from '$lib/utils/errors'

const mockSchedules: AssessmentSchedule[] = [
	{
		id: 1,
		assessment_type_id: 1,
		assessment_type_code: 'PHQ9',
		assessment_type_name: 'PHQ-9 Depression Scale',
		frequency: 'daily',
		time_of_day: '09:00',
		day_of_week: null,
		day_of_month: null,
		enabled: true,
		last_triggered_at: null,
		created_at: '2025-10-28T10:00:00Z',
		updated_at: '2025-10-28T10:00:00Z',
	},
	{
		id: 2,
		assessment_type_id: 2,
		assessment_type_code: 'GAD7',
		assessment_type_name: 'GAD-7 Anxiety Scale',
		frequency: 'weekly',
		time_of_day: '14:30',
		day_of_week: 3,
		day_of_month: null,
		enabled: false,
		last_triggered_at: '2025-10-27T14:30:00Z',
		created_at: '2025-10-20T10:00:00Z',
		updated_at: '2025-10-27T10:00:00Z',
	},
	{
		id: 3,
		assessment_type_id: 3,
		assessment_type_code: 'CESD',
		assessment_type_name: 'CES-D Depression Scale',
		frequency: 'monthly',
		time_of_day: '10:00',
		day_of_week: null,
		day_of_month: 15,
		enabled: true,
		last_triggered_at: null,
		created_at: '2025-10-28T10:00:00Z',
		updated_at: '2025-10-15T10:00:00Z',
	},
]

describe('ScheduleList', () => {
	beforeEach(() => {
		vi.clearAllMocks()
		// Default mock for getSchedules
		vi.mocked(commands.getSchedules).mockResolvedValue({ status: 'ok', data: mockSchedules })
		// Default mock for displayError returns inline type
		vi.mocked(displayError).mockReturnValue({ type: 'inline', message: 'Error' })
	})

	it('should render the list header', async () => {
		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText('Assessment Schedules')).toBeInTheDocument()
		})
	})

	it('should load schedules on mount', async () => {
		render(ScheduleList)

		await waitFor(() => {
			expect(commands.getSchedules).toHaveBeenCalledWith(false)
		})
	})

	it('should display schedules', async () => {
		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText('PHQ-9 Depression Scale')).toBeInTheDocument()
			expect(screen.getByText('GAD-7 Anxiety Scale')).toBeInTheDocument()
			expect(screen.getByText('CES-D Depression Scale')).toBeInTheDocument()
		})
	})

	it('should display empty state when no schedules', async () => {
		vi.mocked(commands.getSchedules).mockResolvedValue({ status: 'ok', data: [] })

		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText('No schedules configured yet.')).toBeInTheDocument()
			expect(screen.getByText(/Create a schedule above to get started/i)).toBeInTheDocument()
		})
	})

	it('should display error message on load failure', async () => {
		vi.mocked(commands.getSchedules).mockResolvedValue({
			status: 'error',
			error: {
				message: 'Failed to load schedules',
				error_type: 'database_error',
				retryable: false,
			},
		})

		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText('Failed to load schedules')).toBeInTheDocument()
		})
	})

	it('should format daily schedule description', async () => {
		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText(/Daily at 09:00/i)).toBeInTheDocument()
		})
	})

	it('should format weekly schedule description with day', async () => {
		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText(/Weekly at 14:30 on Wednesday/i)).toBeInTheDocument()
		})
	})

	it('should format monthly schedule description with day', async () => {
		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText(/Monthly at 10:00 on day 15/i)).toBeInTheDocument()
		})
	})

	it('should display last triggered timestamp', async () => {
		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText(/Last triggered:/i)).toBeInTheDocument()
		})
	})

	it('should show enabled status correctly', async () => {
		render(ScheduleList)

		await waitFor(() => {
			const enabledLabels = screen.getAllByText('Enabled')
			const disabledLabels = screen.getAllByText('Disabled')
			expect(enabledLabels.length).toBe(2) // PHQ9 and CESD
			expect(disabledLabels.length).toBe(1) // GAD7
		})
	})

	it('should toggle schedule enabled status', async () => {
		vi.mocked(invokeWithRetry).mockResolvedValue({ ...mockSchedules[0], enabled: false })

		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText('PHQ-9 Depression Scale')).toBeInTheDocument()
		})

		// Find and click the first toggle
		const toggles = screen.getAllByRole('checkbox')
		await fireEvent.click(toggles[0])

		await waitFor(() => {
			expect(invokeWithRetry).toHaveBeenCalledWith('update_schedule', {
				id: 1,
				request: {
					enabled: false,
					frequency: null,
					time_of_day: null,
					day_of_week: null,
					day_of_month: null,
				},
			})
			expect(displaySuccess).toHaveBeenCalledWith('Schedule updated successfully!')
		})
	})

	it('should delete schedule with confirmation', async () => {
		// Mock window.confirm
		const originalConfirm = window.confirm
		window.confirm = vi.fn(() => true)

		vi.mocked(invokeWithRetry).mockResolvedValue(null)

		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText('PHQ-9 Depression Scale')).toBeInTheDocument()
		})

		const deleteButtons = screen.getAllByText('Delete')
		await fireEvent.click(deleteButtons[0])

		expect(window.confirm).toHaveBeenCalledWith('Are you sure you want to delete this schedule?')

		await waitFor(() => {
			expect(invokeWithRetry).toHaveBeenCalledWith('delete_schedule', { id: 1 })
			expect(displaySuccess).toHaveBeenCalledWith('Schedule deleted successfully!')
		})

		// Restore original confirm
		window.confirm = originalConfirm
	})

	it('should not delete schedule when confirmation is cancelled', async () => {
		// Mock window.confirm to return false
		const originalConfirm = window.confirm
		window.confirm = vi.fn(() => false)

		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText('PHQ-9 Depression Scale')).toBeInTheDocument()
		})

		const deleteButtons = screen.getAllByText('Delete')
		await fireEvent.click(deleteButtons[0])

		expect(window.confirm).toHaveBeenCalled()

		// Should NOT call invokeWithRetry for delete
		await new Promise((resolve) => setTimeout(resolve, 100))
		expect(invokeWithRetry).not.toHaveBeenCalled()

		// Restore original confirm
		window.confirm = originalConfirm
	})

	it('should display error on toggle failure', async () => {
		const mockError = { message: 'Update failed', error_type: 'database', retryable: false }
		vi.mocked(invokeWithRetry).mockRejectedValue(mockError)
		vi.mocked(displayError).mockReturnValue({ type: 'inline', message: 'Update failed' })

		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText('PHQ-9 Depression Scale')).toBeInTheDocument()
		})

		const toggles = screen.getAllByRole('checkbox')
		await fireEvent.click(toggles[0])

		await waitFor(() => {
			expect(displayError).toHaveBeenCalledWith(mockError)
			expect(screen.getByText('Update failed')).toBeInTheDocument()
		})
	})

	it('should display error on delete failure', async () => {
		const originalConfirm = window.confirm
		window.confirm = vi.fn(() => true)

		const mockError = { message: 'Delete failed', error_type: 'database', retryable: false }
		vi.mocked(invokeWithRetry).mockRejectedValue(mockError)
		vi.mocked(displayError).mockReturnValue({ type: 'inline', message: 'Delete failed' })

		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText('PHQ-9 Depression Scale')).toBeInTheDocument()
		})

		const deleteButtons = screen.getAllByText('Delete')
		await fireEvent.click(deleteButtons[0])

		await waitFor(() => {
			expect(displayError).toHaveBeenCalledWith(mockError)
			expect(screen.getByText('Delete failed')).toBeInTheDocument()
		})

		window.confirm = originalConfirm
	})

	it('should show loading state', async () => {
		vi.mocked(commands.getSchedules).mockReturnValue(new Promise(() => {})) // Never resolve

		render(ScheduleList)

		await waitFor(() => {
			expect(screen.getByText('Loading schedules...')).toBeInTheDocument()
		})
	})

	it('should call getSchedules with correct parameter on mount', async () => {
		render(ScheduleList, { props: { refresh: 0 } })

		await waitFor(() => {
			expect(commands.getSchedules).toHaveBeenCalledWith(false)
		})

		// Verify it was called with the correct parameter
		expect(commands.getSchedules).toHaveBeenCalledWith(false)
	})

	it('should have accessible delete buttons', async () => {
		render(ScheduleList)

		await waitFor(() => {
			const deleteButtons = screen.getAllByRole('button', { name: 'Delete schedule' })
			expect(deleteButtons.length).toBe(3)
		})
	})
})
