// Task 4.13: Component tests for GoalSettingModal.svelte
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte'
import GoalSettingModal from './GoalSettingModal.svelte'
import type { Activity, ActivityGroup, ActivityGoal } from '$lib/bindings'

// Mock bindings
vi.mock('$lib/bindings', () => ({
	commands: {
		setActivityGoal: vi.fn(),
		updateActivityGoal: vi.fn(),
	},
}))

// Mock error handling
vi.mock('$lib/utils/errors', () => ({
	displayError: vi.fn(),
	displaySuccess: vi.fn(),
}))

// Mock constants
vi.mock('$lib/constants/activities', () => ({
	GOAL_TYPES: {
		DAYS_PER_PERIOD: 'days_per_period',
		PERCENT_IMPROVEMENT: 'percent_improvement',
	},
	ACTIVITY_GOAL: {
		MAX_PERIOD_DAYS: 365,
	},
}))

import { commands } from '$lib/bindings'
import { displayError, displaySuccess } from '$lib/utils/errors'

describe('GoalSettingModal', () => {
	let setActivityGoalMock: ReturnType<typeof vi.fn>
	let updateActivityGoalMock: ReturnType<typeof vi.fn>
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

	const mockGroup: ActivityGroup = {
		id: 1,
		name: 'Exercise',
		description: 'Physical activities',
		created_at: '2024-01-01T00:00:00Z',
		deleted_at: null,
	}

	const mockExistingGoal: ActivityGoal = {
		id: 1,
		activity_id: 1,
		group_id: null,
		goal_type: 'days_per_period',
		target_value: 3,
		period_days: 7,
		created_at: '2024-01-01T00:00:00Z',
		deleted_at: null,
	}

	beforeEach(() => {
		setActivityGoalMock = vi.mocked(commands.setActivityGoal)
		updateActivityGoalMock = vi.mocked(commands.updateActivityGoal)
		displayErrorMock = vi.mocked(displayError)
		displaySuccessMock = vi.mocked(displaySuccess)

		// Clear mocks
		setActivityGoalMock.mockClear()
		updateActivityGoalMock.mockClear()
		displayErrorMock.mockClear()
		displaySuccessMock.mockClear()
	})

	afterEach(() => {
		vi.clearAllMocks()
	})

	describe('Rendering', () => {
		it('should render in create mode when no existing goal', () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			expect(screen.getByText('Set New Goal')).toBeInTheDocument()
			expect(screen.getByText('Create Goal')).toBeInTheDocument()
			expect(screen.getByText(/Set a goal for Running/)).toBeInTheDocument()
		})

		it('should render in edit mode when existing goal provided', () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					existingGoal: mockExistingGoal,
					onSuccess,
					onCancel,
				},
			})

			expect(screen.getByText('Edit Goal')).toBeInTheDocument()
			expect(screen.getByText('Update Goal')).toBeInTheDocument()
		})

		it('should support group goals when group provided', () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					group: mockGroup,
					onSuccess,
					onCancel,
				},
			})

			expect(screen.getByText(/Set a goal for Exercise group/)).toBeInTheDocument()
		})

		it('should not render when closed', () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			const { container } = render(GoalSettingModal, {
				props: {
					open: false,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Modal should not be visible
			expect(container.querySelector('[role="dialog"]')).not.toBeInTheDocument()
		})
	})

	describe('Goal Type Selection (Task 3.19)', () => {
		it('should render both goal type radio buttons', () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			expect(screen.getByText('Days per Period')).toBeInTheDocument()
			expect(screen.getByText('Percent Improvement')).toBeInTheDocument()
		})

		it('should default to days_per_period type', () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			const daysRadio = screen.getAllByRole('radio')[0] as HTMLInputElement
			expect(daysRadio.checked).toBe(true)
		})

		it('should allow switching between goal types', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			const percentRadio = screen.getAllByRole('radio')[1]
			await fireEvent.click(percentRadio)

			const percentRadioChecked = screen.getAllByRole('radio')[1] as HTMLInputElement
			expect(percentRadioChecked.checked).toBe(true)
		})

		it('should disable goal type selection in edit mode', () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					existingGoal: mockExistingGoal,
					onSuccess,
					onCancel,
				},
			})

			const radios = screen.getAllByRole('radio')
			radios.forEach((radio) => {
				expect(radio).toBeDisabled()
			})

			expect(screen.getByText(/Goal type cannot be changed/)).toBeInTheDocument()
		})

		it('should update description based on goal type', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Initial description should mention days per period
			const initialDescriptions = screen.queryAllByText(/Track how many days you complete/)
			expect(initialDescriptions.length).toBeGreaterThan(0)

			// Switch to percent improvement
			const percentRadio = screen.getAllByRole('radio')[1]
			await fireEvent.click(percentRadio)

			// Description should update to mention percentage
			await waitFor(() => {
				const updatedDescriptions = screen.queryAllByText(/Track percentage improvement/)
				expect(updatedDescriptions.length).toBeGreaterThan(0)
			})
		})
	})

	describe('Target Value Input (Task 3.20)', () => {
		it('should render target value input', () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			const input = screen.getByLabelText(/Target Days/)
			expect(input).toBeInTheDocument()
			expect(input).toHaveAttribute('type', 'number')
		})

		it('should accept positive integers only', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			const input = screen.getByLabelText(/Target Days/) as HTMLInputElement

			await fireEvent.input(input, { target: { value: '5' } })
			expect(input.value).toBe('5')

			// Min should be 1
			expect(input).toHaveAttribute('min', '1')
		})

		it('should validate target value is positive', async () => {
			setActivityGoalMock.mockResolvedValue({ status: 'ok', data: mockExistingGoal })

			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			const input = screen.getByLabelText(/Target Days/)
			await fireEvent.input(input, { target: { value: '0' } })

			const submitButton = screen.getByText('Create Goal')
			await fireEvent.click(submitButton)

			// Should show validation error
			await waitFor(() => {
				expect(screen.getByText(/must be a positive number/)).toBeInTheDocument()
			})

			// Should not call API
			expect(setActivityGoalMock).not.toHaveBeenCalled()
		})

		it('should validate days cannot exceed period for days_per_period', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Set target to 10 days (period is 7 by default)
			const input = screen.getByLabelText(/Target Days/)
			await fireEvent.input(input, { target: { value: '10' } })

			const submitButton = screen.getByText('Create Goal')
			await fireEvent.click(submitButton)

			await waitFor(() => {
				expect(screen.getByText(/Cannot exceed 7 days in the period/)).toBeInTheDocument()
			})
		})

		it('should validate percentage does not exceed 1000%', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Switch to percent improvement
			const percentRadio = screen.getAllByRole('radio')[1]
			await fireEvent.click(percentRadio)

			// Set target to 1500%
			const input = screen.getByLabelText(/Target Percentage/)
			await fireEvent.input(input, { target: { value: '1500' } })

			const submitButton = screen.getByText('Create Goal')
			await fireEvent.click(submitButton)

			await waitFor(() => {
				expect(screen.getByText(/cannot exceed 1000%/)).toBeInTheDocument()
			})
		})

		it('should update label based on goal type', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Initial label
			expect(screen.getByLabelText(/Target Days/)).toBeInTheDocument()

			// Switch to percent improvement
			const percentRadio = screen.getAllByRole('radio')[1]
			await fireEvent.click(percentRadio)

			// Label should update
			await waitFor(() => {
				expect(screen.getByLabelText(/Target Percentage/)).toBeInTheDocument()
			})
		})
	})

	describe('Period Selector (Task 3.21)', () => {
		it('should render period dropdown with presets', () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			const select = screen.getByLabelText(/Time Period/)
			expect(select).toBeInTheDocument()

			// Check for preset options
			expect(screen.getByText('7 days (1 week)')).toBeInTheDocument()
			expect(screen.getByText('14 days (2 weeks)')).toBeInTheDocument()
			expect(screen.getByText('30 days (1 month)')).toBeInTheDocument()
			expect(screen.getByText('Custom')).toBeInTheDocument()
		})

		it('should show custom input when Custom selected', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			const select = screen.getByLabelText(/Time Period/)
			await fireEvent.change(select, { target: { value: '-1' } })

			// Custom input should appear
			await waitFor(() => {
				const customInput = screen.getByPlaceholderText(/Enter custom period/)
				expect(customInput).toBeInTheDocument()
			})
		})

		it('should validate custom period is positive', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Select custom period
			const select = screen.getByLabelText(/Time Period/)
			await fireEvent.change(select, { target: { value: '-1' } })

			// Enter invalid custom period
			const customInput = screen.getByPlaceholderText(/Enter custom period/)
			await fireEvent.input(customInput, { target: { value: '0' } })

			const submitButton = screen.getByText('Create Goal')
			await fireEvent.click(submitButton)

			await waitFor(() => {
				expect(screen.getByText(/must be a positive number/)).toBeInTheDocument()
			})
		})

		it('should validate custom period does not exceed max', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Select custom period
			const select = screen.getByLabelText(/Time Period/)
			await fireEvent.change(select, { target: { value: '-1' } })

			// Enter period > 365 days
			const customInput = screen.getByPlaceholderText(/Enter custom period/)
			await fireEvent.input(customInput, { target: { value: '400' } })

			const submitButton = screen.getByText('Create Goal')
			await fireEvent.click(submitButton)

			await waitFor(() => {
				expect(screen.getByText(/cannot exceed 365 days/)).toBeInTheDocument()
			})
		})
	})

	describe('Form Submission', () => {
		it('should create new activity goal successfully', async () => {
			setActivityGoalMock.mockResolvedValue({ status: 'ok', data: mockExistingGoal })

			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Fill form
			const targetInput = screen.getByLabelText(/Target Days/)
			await fireEvent.input(targetInput, { target: { value: '3' } })

			// Submit
			const submitButton = screen.getByText('Create Goal')
			await fireEvent.click(submitButton)

			await waitFor(() => {
				expect(setActivityGoalMock).toHaveBeenCalledWith({
					activity_id: 1,
					group_id: null,
					goal_type: 'days_per_period',
					target_value: 3,
					period_days: 7,
				})
			})

			expect(displaySuccessMock).toHaveBeenCalledWith('Goal created successfully')
			expect(onSuccess).toHaveBeenCalled()
		})

		it('should create new group goal successfully', async () => {
			setActivityGoalMock.mockResolvedValue({ status: 'ok', data: mockExistingGoal })

			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					group: mockGroup,
					onSuccess,
					onCancel,
				},
			})

			const submitButton = screen.getByText('Create Goal')
			await fireEvent.click(submitButton)

			await waitFor(() => {
				expect(setActivityGoalMock).toHaveBeenCalledWith({
					activity_id: null,
					group_id: 1,
					goal_type: 'days_per_period',
					target_value: 3,
					period_days: 7,
				})
			})
		})

		it('should update existing goal successfully', async () => {
			updateActivityGoalMock.mockResolvedValue({ status: 'ok', data: mockExistingGoal })

			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					existingGoal: mockExistingGoal,
					onSuccess,
					onCancel,
				},
			})

			// Change target value
			const targetInput = screen.getByLabelText(/Target Days/)
			await fireEvent.input(targetInput, { target: { value: '5' } })

			const submitButton = screen.getByText('Update Goal')
			await fireEvent.click(submitButton)

			await waitFor(() => {
				expect(updateActivityGoalMock).toHaveBeenCalledWith(1, 5, 7)
			})

			expect(displaySuccessMock).toHaveBeenCalledWith('Goal updated successfully')
			expect(onSuccess).toHaveBeenCalled()
		})

		it('should use custom period when submitted', async () => {
			setActivityGoalMock.mockResolvedValue({ status: 'ok', data: mockExistingGoal })

			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Select custom period
			const select = screen.getByLabelText(/Time Period/)
			await fireEvent.change(select, { target: { value: '-1' } })

			// Enter custom period
			const customInput = screen.getByPlaceholderText(/Enter custom period/)
			await fireEvent.input(customInput, { target: { value: '21' } })

			const submitButton = screen.getByText('Create Goal')
			await fireEvent.click(submitButton)

			await waitFor(() => {
				expect(setActivityGoalMock).toHaveBeenCalledWith(
					expect.objectContaining({
						period_days: 21,
					})
				)
			})
		})

		it('should handle API errors', async () => {
			setActivityGoalMock.mockResolvedValue({
				status: 'error',
				error: { message: 'Database error', type: 'Database', retryable: false },
			})

			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			const submitButton = screen.getByText('Create Goal')
			await fireEvent.click(submitButton)

			await waitFor(() => {
				expect(displayErrorMock).toHaveBeenCalled()
			})

			// Should not call onSuccess
			expect(onSuccess).not.toHaveBeenCalled()
		})

		it('should disable form while submitting', async () => {
			setActivityGoalMock.mockImplementation(
				() => new Promise((resolve) => setTimeout(resolve, 100))
			)

			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			const submitButton = screen.getByText('Create Goal')
			await fireEvent.click(submitButton)

			// Buttons should be disabled
			expect(submitButton).toBeDisabled()
			expect(screen.getByText('Cancel')).toBeDisabled()
		})
	})

	describe('Cancel Handling', () => {
		it('should call onCancel when Cancel button clicked', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			const cancelButton = screen.getByText('Cancel')
			await fireEvent.click(cancelButton)

			expect(onCancel).toHaveBeenCalled()
		})
	})

	describe('Form Reset', () => {
		it('should reset form when modal reopens', async () => {
			const onSuccess = vi.fn()
			const onCancel = vi.fn()

			// First render - modal open
			const { unmount } = render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Change target value
			const targetInput = screen.getByLabelText(/Target Days/)
			await fireEvent.input(targetInput, { target: { value: '10' } })
			expect((targetInput as HTMLInputElement).value).toBe('10')

			// Unmount and re-render (simulates modal close/reopen)
			unmount()

			// Re-render with fresh props
			render(GoalSettingModal, {
				props: {
					open: true,
					activity: mockActivity,
					onSuccess,
					onCancel,
				},
			})

			// Value should be reset to default (3)
			await waitFor(() => {
				const resetInput = screen.getByLabelText(/Target Days/) as HTMLInputElement
				expect(resetInput.value).toBe('3')
			})
		})
	})
})
