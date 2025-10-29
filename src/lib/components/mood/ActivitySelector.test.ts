import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { render, waitFor, fireEvent } from '@testing-library/svelte'
import ActivitySelector from './ActivitySelector.svelte'
import type { Activity } from '$lib/bindings'

// Mock Tauri's invoke
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn(),
}))

describe('ActivitySelector', () => {
	let invokeMock: ReturnType<typeof vi.fn>

	beforeEach(async () => {
		const { invoke } = await import('@tauri-apps/api/core')
		invokeMock = invoke as ReturnType<typeof vi.fn>
		invokeMock.mockClear()
	})

	afterEach(() => {
		vi.clearAllMocks()
	})

	const mockActivities: Activity[] = [
		{
			id: 1,
			name: 'Exercise',
			color: '#22C55E',
			icon: '🏃',
			created_at: '2024-01-01',
			updated_at: null,
			deleted_at: null,
		},
		{
			id: 2,
			name: 'Reading',
			color: '#3B82F6',
			icon: '📚',
			created_at: '2024-01-02',
			updated_at: null,
			deleted_at: null,
		},
	]

	describe('Loading State', () => {
		it('should show loading message initially', () => {
			invokeMock.mockReturnValue(new Promise(() => {}))

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			expect(container.textContent).toContain('Loading activities...')
		})
	})

	describe('Error State', () => {
		it('should display error message when fetch fails', async () => {
			invokeMock.mockRejectedValue(new Error('Failed to load'))

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('Failed to load')
			})
		})

		it('should display error in red box', async () => {
			invokeMock.mockRejectedValue(new Error('Database error'))

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const errorBox = container.querySelector('.bg-red-100')
				expect(errorBox).toBeInTheDocument()
			})
		})
	})

	describe('Empty State', () => {
		it('should show empty message when no activities', async () => {
			invokeMock.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('No activities yet')
			})
		})

		it('should suggest creating an activity', async () => {
			invokeMock.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('Click "+ Add New" to create one')
			})
		})
	})

	describe('Activities Display', () => {
		it('should display all activities', async () => {
			invokeMock.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('Exercise')
				expect(container.textContent).toContain('Reading')
			})
		})

		it('should display activity icons', async () => {
			invokeMock.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('🏃')
				expect(container.textContent).toContain('📚')
			})
		})

		it('should apply custom colors to activity chips', async () => {
			invokeMock.mockResolvedValue(mockActivities)

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
			invokeMock.mockResolvedValue(mockActivities)

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
			invokeMock.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, {
				props: { selectedIds: [1], onChange },
			})

			await waitFor(() => {
				expect(container.textContent).toContain('✓')
			})
		})

		it('should call onChange when activity clicked', async () => {
			invokeMock.mockResolvedValue(mockActivities)

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
			invokeMock.mockResolvedValue(mockActivities)

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
			invokeMock.mockResolvedValue(mockActivities)

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
			invokeMock.mockResolvedValue([])

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				expect(container.textContent).toContain('+ Add New')
			})
		})

		it('should show create form when Add New clicked', async () => {
			invokeMock.mockResolvedValue([])

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
			invokeMock.mockResolvedValue([])

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
			invokeMock.mockResolvedValue([])

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
			invokeMock.mockResolvedValue([])

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
			invokeMock.mockResolvedValue([])

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
			invokeMock.mockResolvedValue([])

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
			invokeMock.mockResolvedValueOnce([]).mockResolvedValueOnce({
				id: 3,
				name: 'Meditation',
				color: '#3B82F6',
				icon: null,
				created_at: '2024-01-03',
				updated_at: null,
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
				expect(invokeMock).toHaveBeenCalledWith('create_activity', {
					request: {
						name: 'Meditation',
						color: '#3B82F6',
						icon: null,
					},
				})
			})
		})

		it('should automatically select newly created activity', async () => {
			invokeMock.mockResolvedValueOnce([]).mockResolvedValueOnce({
				id: 3,
				name: 'Meditation',
				color: '#3B82F6',
				icon: null,
				created_at: '2024-01-03',
				updated_at: null,
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
			invokeMock.mockResolvedValueOnce([]).mockResolvedValueOnce({
				id: 3,
				name: 'Meditation',
				color: '#3B82F6',
				icon: null,
				created_at: '2024-01-03',
				updated_at: null,
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
			invokeMock.mockResolvedValueOnce([]).mockRejectedValueOnce(new Error('Creation failed'))

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
			invokeMock.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const buttons = container.querySelectorAll('.activity-chip')
				expect(buttons[0]).toHaveAttribute('aria-label', 'Select activity: Exercise')
			})
		})

		it('should have descriptive aria-label for deselecting activity', async () => {
			invokeMock.mockResolvedValue(mockActivities)

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
			invokeMock.mockResolvedValue(mockActivities)

			const onChange = vi.fn()
			const { container } = render(ActivitySelector, { props: { selectedIds: [], onChange } })

			await waitFor(() => {
				const buttons = container.querySelectorAll('.activity-chip')
				expect(buttons[0]).toHaveAttribute('aria-pressed', 'false')
			})
		})
	})
})
