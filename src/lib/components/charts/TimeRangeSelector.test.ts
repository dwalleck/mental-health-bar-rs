import { describe, it, expect, beforeEach, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/svelte'
import TimeRangeSelector from './TimeRangeSelector.svelte'
import type { TimeRange } from '$lib/bindings'

describe('TimeRangeSelector', () => {
	beforeEach(() => {
		vi.clearAllMocks()
	})

	describe('Rendering', () => {
		it('should render the time range selector with label', () => {
			render(TimeRangeSelector, { props: { selected: 'week' } })

			expect(screen.getByLabelText('Time Range')).toBeInTheDocument()
			expect(screen.getByRole('combobox')).toBeInTheDocument()
		})

		it('should render all time range options', () => {
			render(TimeRangeSelector, { props: { selected: 'week' } })

			const select = screen.getByRole('combobox')
			const options = Array.from(select.querySelectorAll('option'))

			expect(options).toHaveLength(5)
			expect(options[0]).toHaveTextContent('Last Week')
			expect(options[1]).toHaveTextContent('Last Month')
			expect(options[2]).toHaveTextContent('Last 3 Months')
			expect(options[3]).toHaveTextContent('Last Year')
			expect(options[4]).toHaveTextContent('All Time')
		})

		it('should display the correct selected value', () => {
			render(TimeRangeSelector, { props: { selected: 'month' } })

			const select = screen.getByRole('combobox') as HTMLSelectElement
			expect(select.value).toBe('month')
		})

		it('should default to "week" if no selected prop provided', () => {
			render(TimeRangeSelector)

			const select = screen.getByRole('combobox') as HTMLSelectElement
			expect(select.value).toBe('week')
		})
	})

	describe('Interaction', () => {
		it('should call onchange callback when selection changes', async () => {
			const mockOnChange = vi.fn()
			render(TimeRangeSelector, {
				props: { selected: 'week', onchange: mockOnChange },
			})

			const select = screen.getByRole('combobox')
			await fireEvent.change(select, { target: { value: 'month' } })

			expect(mockOnChange).toHaveBeenCalledTimes(1)
			expect(mockOnChange).toHaveBeenCalledWith('month')
		})

		it('should update internal state when selection changes', async () => {
			render(TimeRangeSelector, { props: { selected: 'week' } })

			const select = screen.getByRole('combobox') as HTMLSelectElement
			await fireEvent.change(select, { target: { value: 'year' } })

			expect(select.value).toBe('year')
		})

		it('should handle changing to each time range option', async () => {
			const mockOnChange = vi.fn()
			render(TimeRangeSelector, {
				props: { selected: 'week', onchange: mockOnChange },
			})

			const select = screen.getByRole('combobox')
			const ranges: TimeRange[] = ['week', 'month', 'quarter', 'year', 'alltime']

			for (const range of ranges) {
				await fireEvent.change(select, { target: { value: range } })
			}

			expect(mockOnChange).toHaveBeenCalledTimes(5)
			expect(mockOnChange).toHaveBeenCalledWith('week')
			expect(mockOnChange).toHaveBeenCalledWith('month')
			expect(mockOnChange).toHaveBeenCalledWith('quarter')
			expect(mockOnChange).toHaveBeenCalledWith('year')
			expect(mockOnChange).toHaveBeenCalledWith('alltime')
		})

		it('should not throw error if onchange callback is not provided', async () => {
			render(TimeRangeSelector, { props: { selected: 'week' } })

			const select = screen.getByRole('combobox')
			await expect(fireEvent.change(select, { target: { value: 'month' } })).resolves.not.toThrow()
		})
	})

	describe('Accessibility', () => {
		it('should have proper label association', () => {
			render(TimeRangeSelector, { props: { selected: 'week' } })

			const label = screen.getByText('Time Range')
			const select = screen.getByRole('combobox')

			expect(label.getAttribute('for')).toBe('time-range')
			expect(select.getAttribute('id')).toBe('time-range')
		})

		it('should have descriptive option text', () => {
			render(TimeRangeSelector, { props: { selected: 'week' } })

			expect(screen.getByRole('option', { name: 'Last Week' })).toBeInTheDocument()
			expect(screen.getByRole('option', { name: 'Last Month' })).toBeInTheDocument()
			expect(screen.getByRole('option', { name: 'Last 3 Months' })).toBeInTheDocument()
			expect(screen.getByRole('option', { name: 'Last Year' })).toBeInTheDocument()
			expect(screen.getByRole('option', { name: 'All Time' })).toBeInTheDocument()
		})
	})

	describe('Option Values', () => {
		it('should have correct value attributes for all options', () => {
			render(TimeRangeSelector, { props: { selected: 'week' } })

			const select = screen.getByRole('combobox')
			const options = Array.from(select.querySelectorAll('option'))

			expect(options[0].getAttribute('value')).toBe('week')
			expect(options[1].getAttribute('value')).toBe('month')
			expect(options[2].getAttribute('value')).toBe('quarter')
			expect(options[3].getAttribute('value')).toBe('year')
			expect(options[4].getAttribute('value')).toBe('alltime')
		})
	})
})
