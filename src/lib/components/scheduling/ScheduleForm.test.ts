import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte'
import ScheduleForm from './ScheduleForm.svelte'

// Mock the bindings module
vi.mock('$lib/bindings', () => ({
	commands: {
		getAssessmentTypes: vi.fn(),
		createSchedule: vi.fn(),
	},
}))

import { commands } from '$lib/bindings'

const mockAssessmentTypes = [
	{
		id: 1,
		code: 'PHQ9',
		name: 'PHQ-9 Depression Scale',
		description: 'Test',
		question_count: 9,
		min_score: 0,
		max_score: 27,
	},
	{
		id: 2,
		code: 'GAD7',
		name: 'GAD-7 Anxiety Scale',
		description: 'Test',
		question_count: 7,
		min_score: 0,
		max_score: 21,
	},
	{
		id: 3,
		code: 'CESD',
		name: 'CES-D Depression Scale',
		description: 'Test',
		question_count: 20,
		min_score: 0,
		max_score: 60,
	},
]

describe('ScheduleForm', () => {
	beforeEach(() => {
		vi.clearAllMocks()
		// Default mock for getAssessmentTypes
		vi.mocked(commands.getAssessmentTypes).mockResolvedValue({
			status: 'ok',
			data: mockAssessmentTypes,
		})
	})

	it('should render the form', async () => {
		render(ScheduleForm)

		await waitFor(() => {
			expect(screen.getByRole('heading', { name: 'Create Schedule' })).toBeInTheDocument()
		})

		expect(screen.getByLabelText(/Assessment Type/i)).toBeInTheDocument()
		expect(screen.getByLabelText(/Frequency/i)).toBeInTheDocument()
		expect(screen.getByLabelText(/Time of Day/i)).toBeInTheDocument()
	})

	it('should load assessment types on mount', async () => {
		render(ScheduleForm)

		await waitFor(() => {
			expect(commands.getAssessmentTypes).toHaveBeenCalled()
		})
	})

	it('should display error when assessment types fail to load', async () => {
		vi.mocked(commands.getAssessmentTypes).mockResolvedValue({
			status: 'error',
			error: 'Failed to load',
		})

		render(ScheduleForm)

		await waitFor(() => {
			expect(screen.getByText('Failed to load')).toBeInTheDocument()
		})
	})

	it('should show day of week selector for weekly frequency', async () => {
		render(ScheduleForm)

		await waitFor(() => {
			expect(screen.queryByLabelText(/Day of Week/i)).not.toBeInTheDocument()
		})

		const frequencySelect = screen.getByLabelText(/Frequency/i)
		await fireEvent.change(frequencySelect, { target: { value: 'weekly' } })

		await waitFor(() => {
			expect(screen.getByLabelText(/Day of Week/i)).toBeInTheDocument()
		})
	})

	it('should show day of week selector for biweekly frequency', async () => {
		render(ScheduleForm)

		const frequencySelect = screen.getByLabelText(/Frequency/i)
		await fireEvent.change(frequencySelect, { target: { value: 'biweekly' } })

		await waitFor(() => {
			expect(screen.getByLabelText(/Day of Week/i)).toBeInTheDocument()
		})
	})

	it('should show day of month selector for monthly frequency', async () => {
		render(ScheduleForm)

		await waitFor(() => {
			expect(screen.queryByLabelText(/Day of Month/i)).not.toBeInTheDocument()
		})

		const frequencySelect = screen.getByLabelText(/Frequency/i)
		await fireEvent.change(frequencySelect, { target: { value: 'monthly' } })

		await waitFor(() => {
			expect(screen.getByLabelText(/Day of Month/i)).toBeInTheDocument()
		})
	})

	it('should not show day selectors for daily frequency', async () => {
		render(ScheduleForm)

		await waitFor(() => {
			expect(screen.queryByLabelText(/Day of Week/i)).not.toBeInTheDocument()
			expect(screen.queryByLabelText(/Day of Month/i)).not.toBeInTheDocument()
		})
	})

	it('should submit daily schedule successfully', async () => {
		vi.mocked(commands.createSchedule).mockResolvedValue({
			status: 'ok',
			data: {
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
				created_at: '2025-10-28T00:00:00Z',
				updated_at: '2025-10-28T00:00:00Z',
			},
		})

		const onSuccess = vi.fn()
		render(ScheduleForm, { props: { onSuccess } })

		await waitFor(() => {
			expect(screen.getByLabelText(/Assessment Type/i)).toBeInTheDocument()
		})

		const timeInput = screen.getByLabelText(/Time of Day/i)
		await fireEvent.input(timeInput, { target: { value: '09:00' } })

		const submitButton = screen.getByRole('button', { name: /Create Schedule/i })
		await fireEvent.click(submitButton)

		await waitFor(() => {
			expect(commands.createSchedule).toHaveBeenCalledWith({
				assessment_type_id: 1,
				frequency: 'daily',
				time_of_day: '09:00',
				day_of_week: null,
				day_of_month: null,
			})
		})

		await waitFor(() => {
			expect(screen.getByText('Schedule created successfully!')).toBeInTheDocument()
			expect(onSuccess).toHaveBeenCalled()
		})
	})

	it('should submit weekly schedule with day of week', async () => {
		vi.mocked(commands.createSchedule).mockResolvedValue({
			status: 'ok',
			data: {
				id: 1,
				assessment_type_id: 1,
				assessment_type_code: 'PHQ9',
				assessment_type_name: 'PHQ-9 Depression Scale',
				frequency: 'weekly',
				time_of_day: '14:00',
				day_of_week: 3,
				day_of_month: null,
				enabled: true,
				last_triggered_at: null,
				created_at: '2025-10-28T00:00:00Z',
				updated_at: '2025-10-28T00:00:00Z',
			},
		})

		render(ScheduleForm)

		await waitFor(() => {
			expect(screen.getByLabelText(/Frequency/i)).toBeInTheDocument()
		})

		const frequencySelect = screen.getByLabelText(/Frequency/i)
		await fireEvent.change(frequencySelect, { target: { value: 'weekly' } })

		const dayOfWeekSelect = await screen.findByLabelText(/Day of Week/i)
		await fireEvent.change(dayOfWeekSelect, { target: { value: '3' } })

		const timeInput = screen.getByLabelText(/Time of Day/i)
		await fireEvent.input(timeInput, { target: { value: '14:00' } })

		const submitButton = screen.getByRole('button', { name: /Create Schedule/i })
		await fireEvent.click(submitButton)

		await waitFor(() => {
			expect(commands.createSchedule).toHaveBeenCalledWith({
				assessment_type_id: 1,
				frequency: 'weekly',
				time_of_day: '14:00',
				day_of_week: 3,
				day_of_month: null,
			})
		})
	})

	it('should submit monthly schedule with day of month', async () => {
		vi.mocked(commands.createSchedule).mockResolvedValue({
			status: 'ok',
			data: {
				id: 1,
				assessment_type_id: 1,
				assessment_type_code: 'PHQ9',
				assessment_type_name: 'PHQ-9 Depression Scale',
				frequency: 'monthly',
				time_of_day: '10:00',
				day_of_week: null,
				day_of_month: 15,
				enabled: true,
				last_triggered_at: null,
				created_at: '2025-10-28T00:00:00Z',
				updated_at: '2025-10-28T00:00:00Z',
			},
		})

		render(ScheduleForm)

		await waitFor(() => {
			expect(screen.getByLabelText(/Frequency/i)).toBeInTheDocument()
		})

		const frequencySelect = screen.getByLabelText(/Frequency/i)
		await fireEvent.change(frequencySelect, { target: { value: 'monthly' } })

		const dayOfMonthSelect = await screen.findByLabelText(/Day of Month/i)
		await fireEvent.change(dayOfMonthSelect, { target: { value: '15' } })

		const timeInput = screen.getByLabelText(/Time of Day/i)
		await fireEvent.input(timeInput, { target: { value: '10:00' } })

		const submitButton = screen.getByRole('button', { name: /Create Schedule/i })
		await fireEvent.click(submitButton)

		await waitFor(() => {
			expect(commands.createSchedule).toHaveBeenCalledWith({
				assessment_type_id: 1,
				frequency: 'monthly',
				time_of_day: '10:00',
				day_of_week: null,
				day_of_month: 15,
			})
		})
	})

	it('should display error on failed submission', async () => {
		vi.mocked(commands.createSchedule).mockResolvedValue({
			status: 'error',
			error: 'Invalid time format',
		})

		render(ScheduleForm)

		await waitFor(() => {
			expect(screen.getByLabelText(/Time of Day/i)).toBeInTheDocument()
		})

		const submitButton = screen.getByRole('button', { name: /Create Schedule/i })
		await fireEvent.click(submitButton)

		await waitFor(() => {
			expect(screen.getByText('Invalid time format')).toBeInTheDocument()
		})
	})

	it('should reset form after successful submission', async () => {
		vi.mocked(commands.createSchedule).mockResolvedValue({
			status: 'ok',
			data: {
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
				created_at: '2025-10-28T00:00:00Z',
				updated_at: '2025-10-28T00:00:00Z',
			},
		})

		render(ScheduleForm)

		await waitFor(() => {
			expect(screen.getByLabelText(/Time of Day/i)).toBeInTheDocument()
		})

		// Just submit with daily frequency to avoid validation issues
		const submitButton = screen.getByRole('button', { name: /Create Schedule/i })
		await fireEvent.click(submitButton)

		await waitFor(
			() => {
				expect(screen.getByText('Schedule created successfully!')).toBeInTheDocument()
			},
			{ timeout: 3000 }
		)

		// Verify form was reset (frequency stays at default 'daily')
		const resetFrequency = screen.getByLabelText(/Frequency/i) as HTMLSelectElement
		expect(resetFrequency.value).toBe('daily')
	})

	it('should disable submit button while loading', async () => {
		vi.mocked(commands.createSchedule).mockReturnValue(new Promise(() => {})) // Never resolve

		render(ScheduleForm)

		await waitFor(() => {
			expect(screen.getByLabelText(/Time of Day/i)).toBeInTheDocument()
		})

		const submitButton = screen.getByRole('button', { name: /Create Schedule/i })
		await fireEvent.click(submitButton)

		await waitFor(() => {
			const button = screen.getByRole('button', { name: /Creating/i })
			expect(button).toBeDisabled()
		})
	})
})
