import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { render, waitFor, fireEvent } from '@testing-library/svelte'
import ActivitySelector from './ActivitySelector.svelte'
import type { Activity } from '$lib/bindings'

// Mock retry utility
vi.mock('$lib/utils/retry', () => ({
	invokeWithRetry: vi.fn(),
}))

// Mock error handling utilities
vi.mock('$lib/utils/errors', () => ({
	displayError: vi.fn((error) => ({
		type: 'inline',
		message: error instanceof Error ? error.message : String(error),
	})),
	displaySuccess: vi.fn(),
	formatUserError: vi.fn((error) => (error instanceof Error ? error.message : String(error))),
	isValidationError: vi.fn(() => false),
	isCommandError: vi.fn(() => false),
}))

describe('ActivitySelector', () => {
	let invokeWithRetry: ReturnType<typeof vi.fn>
	let displayError: ReturnType<typeof vi.fn>
	let displaySuccess: ReturnType<typeof vi.fn>

	beforeEach(async () => {
		const retry = await import('$lib/utils/retry')
		const errors = await import('$lib/utils/errors')
		invokeWithRetry = retry.invokeWithRetry as ReturnType<typeof vi.fn>
		displayError = errors.displayError as ReturnType<typeof vi.fn>
		displaySuccess = errors.displaySuccess as ReturnType<typeof vi.fn>
		invokeWithRetry.mockClear()
		displayError.mockClear()
		displaySuccess.mockClear()
	})

	afterEach(() => {
		vi.clearAllMocks()
	})

	const mockActivities: Activity[] = [
		{
			id: 1,
			group_id: 1,
			name: 'Exercise',
			color: '#22C55E',
			icon: '🏃',
			created_at: '2024-01-01',
			deleted_at: null,
		},
		{
			id: 2,
			group_id: 1,
			name: 'Reading',
			color: '#3B82F6',
			icon: '📚',
			created_at: '2024-01-02',
			deleted_at: null,
		},
	]

	describe('Loading State', () => {
		it('should show loading message initially', () => {
			invokeWithRetry.mockReturnValue(new Promise(() => {}))

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			expect(container.textContent).toContain('Loading activities...')
		})
	})

	describe('Error State', () => {
		it('should display error message when fetch fails', async () => {
			invokeWithRetry.mockRejectedValue(new Error('Failed to load'))

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('Failed to load')
			})
		})

		it('should display error in error message box', async () => {
			invokeWithRetry.mockRejectedValue(new Error('Database error'))

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const errorBox = container.querySelector('.error-message')
				expect(errorBox).toBeInTheDocument()
			})
		})
	})

	describe('Empty State', () => {
		it('should show empty message when no activities', async () => {
			invokeWithRetry.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('No activities yet')
			})
		})

		it('should suggest creating an activity', async () => {
			invokeWithRetry.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('Click "+ Add New" to create one')
			})
		})
	})

	describe('Activities Display', () => {
		it('should display all activities', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('Exercise')
				expect(container.textContent).toContain('Reading')
			})
		})

		it('should display activity icons', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('🏃')
				expect(container.textContent).toContain('📚')
			})
		})

		it('should apply custom colors to activity chips', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const buttons = container.querySelectorAll('.activity-chip')
				// Browser converts hex to RGB format
				expect(buttons[0]).toHaveAttribute('style')
				expect(buttons[1]).toHaveAttribute('style')
				expect(buttons[0].getAttribute('style')).toContain('border-color')
				expect(buttons[1].getAttribute('style')).toContain('border-color')
			})
		})
	})

	describe('Activity Selection', () => {
		it('should mark selected activities', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, {
				props: { selectedIds: [1], onChange },
			})

			await waitFor(() => {
				const buttons = container.querySelectorAll('.activity-chip')
				expect(buttons[0]).toHaveAttribute('aria-pressed', 'true')
				expect(buttons[1]).toHaveAttribute('aria-pressed', 'false')
			})
		})

		it('should show checkmark on selected activities', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, {
				props: { selectedIds: [1], onChange },
			})

			await waitFor(() => {
				expect(container.textContent).toContain('✓')
			})
		})

		it('should call onChange when activity clicked', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, {
				props: { selectedIds: [], onChange },
			})

			await waitFor(() => {
				expect(container.textContent).toContain('Exercise')
			})

			const buttons = container.querySelectorAll('.activity-chip')
			await fireEvent.click(buttons[0])

			expect(onChange).toHaveBeenCalledWith([1])
		})

		it('should add activity to selection', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, {
				props: { selectedIds: [1], onChange },
			})

			await waitFor(() => {
				expect(container.textContent).toContain('Reading')
			})

			const buttons = container.querySelectorAll('.activity-chip')
			await fireEvent.click(buttons[1])

			expect(onChange).toHaveBeenCalledWith([1, 2])
		})

		it('should remove activity from selection when clicked again', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, {
				props: { selectedIds: [1, 2], onChange },
			})

			await waitFor(() => {
				expect(container.textContent).toContain('Exercise')
			})

			const buttons = container.querySelectorAll('.activity-chip')
			await fireEvent.click(buttons[0])

			expect(onChange).toHaveBeenCalledWith([2])
		})
	})

	describe('Create Form', () => {
		it('should have Add New button', async () => {
			invokeWithRetry.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('+ Add New')
			})
		})

		it('should show create form when Add New clicked', async () => {
			invokeWithRetry.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('+ Add New')
				)
				expect(addButton).toBeTruthy()
			})

			const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('+ Add New')
			)!
			await fireEvent.click(addButton)

			await waitFor(() => {
				const nameInput = container.querySelector('input[placeholder*="Activity name"]')
				expect(nameInput).toBeInTheDocument()
			})
		})

		it('should hide create form when Cancel clicked', async () => {
			invokeWithRetry.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			// Open form
			await waitFor(() => {
				const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('+ Add New')
				)
				return addButton
			})

			const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('+ Add New')
			)!
			await fireEvent.click(addButton)

			await waitFor(() => {
				const nameInput = container.querySelector('input[placeholder*="Activity name"]')
				expect(nameInput).toBeInTheDocument()
			})

			// Close form
			await waitFor(() => {
				const cancelButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('Cancel')
				)
				expect(cancelButton).toBeTruthy()
			})

			const cancelButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('Cancel')
			)!
			await fireEvent.click(cancelButton)

			await waitFor(() => {
				const nameInput = container.querySelector('input[placeholder*="Activity name"]')
				expect(nameInput).not.toBeInTheDocument()
			})
		})

		it('should have name input in create form', async () => {
			invokeWithRetry.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('+ Add New')
				)
				return addButton
			})

			const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('+ Add New')
			)!
			await fireEvent.click(addButton)

			await waitFor(() => {
				const nameInput = container.querySelector('input[placeholder*="Activity name"]')
				expect(nameInput).toBeInTheDocument()
			})
		})

		it('should have color picker in create form', async () => {
			invokeWithRetry.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('+ Add New')
				)
				return addButton
			})

			const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('+ Add New')
			)!
			await fireEvent.click(addButton)

			await waitFor(() => {
				const colorInput = container.querySelector('input[type="color"]')
				expect(colorInput).toBeInTheDocument()
			})
		})

		it('should have icon input in create form', async () => {
			invokeWithRetry.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('+ Add New')
				)
				return addButton
			})

			const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('+ Add New')
			)!
			await fireEvent.click(addButton)

			await waitFor(() => {
				const iconInput = container.querySelector('input[placeholder*="Emoji"]')
				expect(iconInput).toBeInTheDocument()
			})
		})

		it('should disable Create button when name is empty', async () => {
			invokeWithRetry.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('+ Add New')
				)
				return addButton
			})

			const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('+ Add New')
			)!
			await fireEvent.click(addButton)

			await waitFor(() => {
				const createButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('Create Activity')
				)
				expect(createButton).toBeDisabled()
			})
		})
	})

	describe('Creating Activity', () => {
		it('should create activity with name and defaults', async () => {
			invokeWithRetry.mockResolvedValueOnce([]).mockResolvedValueOnce({
				id: 3,
				group_id: 1,
				name: 'Meditation',
				color: '#3B82F6',
				icon: null,
				created_at: '2024-01-03',
				deleted_at: null,
			})

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			// Open form
			await waitFor(() => {
				const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('+ Add New')
				)
				return addButton
			})

			const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('+ Add New')
			)!
			await fireEvent.click(addButton)

			// Enter name
			await waitFor(() => {
				const nameInput = container.querySelector('input[placeholder*="Activity name"]')
				return nameInput
			})

			const nameInput = container.querySelector('input[placeholder*="Activity name"]')!
			await fireEvent.input(nameInput, { target: { value: 'Meditation' } })

			// Click Create
			const createButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('Create Activity')
			)!
			await fireEvent.click(createButton)

			await waitFor(() => {
				expect(invokeWithRetry).toHaveBeenCalledWith('create_activity', {
					request: {
						name: 'Meditation',
						color: '#3B82F6',
						icon: null,
					},
				})
				expect(displaySuccess).toHaveBeenCalledWith('Activity created successfully!')
			})
		})

		it('should automatically select newly created activity', async () => {
			invokeWithRetry.mockResolvedValueOnce([]).mockResolvedValueOnce({
				id: 3,
				group_id: 1,
				name: 'Meditation',
				color: '#3B82F6',
				icon: null,
				created_at: '2024-01-03',
				deleted_at: null,
			})

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [1], onChange } })

			// Open form and create activity
			await waitFor(() => {
				const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('+ Add New')
				)
				return addButton
			})

			const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('+ Add New')
			)!
			await fireEvent.click(addButton)

			await waitFor(() => {
				const nameInput = container.querySelector('input[placeholder*="Activity name"]')
				return nameInput
			})

			const nameInput = container.querySelector('input[placeholder*="Activity name"]')!
			await fireEvent.input(nameInput, { target: { value: 'Meditation' } })

			const createButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('Create Activity')
			)!
			await fireEvent.click(createButton)

			await waitFor(() => {
				expect(onChange).toHaveBeenCalledWith([1, 3])
			})
		})

		it('should close form after successful creation', async () => {
			invokeWithRetry.mockResolvedValueOnce([]).mockResolvedValueOnce({
				id: 3,
				group_id: 1,
				name: 'Meditation',
				color: '#3B82F6',
				icon: null,
				created_at: '2024-01-03',
				deleted_at: null,
			})

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			// Open form and create
			await waitFor(() => {
				const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('+ Add New')
				)
				return addButton
			})

			const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('+ Add New')
			)!
			await fireEvent.click(addButton)

			await waitFor(() => {
				const nameInput = container.querySelector('input[placeholder*="Activity name"]')
				return nameInput
			})

			const nameInput = container.querySelector('input[placeholder*="Activity name"]')!
			await fireEvent.input(nameInput, { target: { value: 'Meditation' } })

			const createButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('Create Activity')
			)!
			await fireEvent.click(createButton)

			await waitFor(() => {
				expect(container.querySelector('input[type="text"]')).not.toBeInTheDocument()
			})
		})

		it('should show error if creation fails', async () => {
			invokeWithRetry.mockResolvedValueOnce([]).mockRejectedValueOnce(new Error('Creation failed'))

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			// Open form and try to create
			await waitFor(() => {
				const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
					btn.textContent?.includes('+ Add New')
				)
				return addButton
			})

			const addButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('+ Add New')
			)!
			await fireEvent.click(addButton)

			await waitFor(() => {
				const nameInput = container.querySelector('input[placeholder*="Activity name"]')
				return nameInput
			})

			const nameInput = container.querySelector('input[placeholder*="Activity name"]')!
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const createButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.textContent?.includes('Create Activity')
			)!
			await fireEvent.click(createButton)

			await waitFor(() => {
				expect(container.textContent).toContain('Creation failed')
			})
		})
	})

	describe('Accessibility', () => {
		it('should have descriptive aria-label for selecting activity', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const buttons = container.querySelectorAll('.activity-chip')
				expect(buttons[0]).toHaveAttribute('aria-label', 'Select activity: Exercise')
			})
		})

		it('should have descriptive aria-label for deselecting activity', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, {
				props: { selectedIds: [1], onChange },
			})

			await waitFor(() => {
				const buttons = container.querySelectorAll('.activity-chip')
				expect(buttons[0]).toHaveAttribute('aria-label', 'Deselect activity: Exercise')
			})
		})

		it('should have aria-pressed on activity buttons', async () => {
			invokeWithRetry.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const buttons = container.querySelectorAll('.activity-chip')
				expect(buttons[0]).toHaveAttribute('aria-pressed', 'false')
			})
		})
	})
})
