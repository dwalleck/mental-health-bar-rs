import { describe, it, expect, vi, beforeEach, afterEach, type MockInstance } from 'vitest'
import { render, fireEvent } from '@testing-library/svelte'
import ActivityList from './ActivityList.svelte'
import type { Activity, ActivityGroup } from '$lib/bindings'

describe('ActivityList', () => {
	const mockActivities: Activity[] = [
		{
			id: 1,
			group_id: 1,
			name: 'Exercise',
			color: '#22C55E',
			icon: '🏃',
			created_at: '2024-01-01T00:00:00Z',
			deleted_at: null,
		},
		{
			id: 2,
			group_id: 1,
			name: 'Meditation',
			color: '#3B82F6',
			icon: '🧘',
			created_at: '2024-01-02T00:00:00Z',
			deleted_at: null,
		},
		{
			id: 3,
			group_id: 1,
			name: 'Reading',
			color: null,
			icon: null,
			created_at: '2024-01-03T00:00:00Z',
			deleted_at: null,
		},
	]

	const mockGroups: ActivityGroup[] = [
		{
			id: 1,
			name: 'Wellness',
			description: 'Wellness activities',
			created_at: '2024-01-01T00:00:00Z',
			deleted_at: null,
		},
	]

	describe('Loading State', () => {
		it('should display loading message when loading prop is true', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [], groups: [], onEdit, onDelete, loading: true },
			})

			expect(container.textContent).toContain('Loading activities...')
		})

		it('should have loading animation class', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [], groups: [], onEdit, onDelete, loading: true },
			})

			const loadingDiv = container.querySelector('.animate-pulse')
			expect(loadingDiv).toBeInTheDocument()
		})

		it('should not show activities when loading', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete, loading: true },
			})

			const activityCards = container.querySelectorAll('.activity-card')
			expect(activityCards).toHaveLength(0)
		})
	})

	describe('Empty State', () => {
		it('should display empty message when no activities', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [], groups: [], onEdit, onDelete, loading: false },
			})

			expect(container.textContent).toContain('No activities yet')
		})

		it('should show descriptive text in empty state', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [], groups: [], onEdit, onDelete, loading: false },
			})

			expect(container.textContent).toContain('Create your first activity to start tracking')
		})

		it('should show emoji icon in empty state', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [], groups: [], onEdit, onDelete, loading: false },
			})

			expect(container.textContent).toContain('📝')
		})
	})

	describe('Activities Display', () => {
		it('should render all activities', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			const activityCards = container.querySelectorAll('.activity-card')
			expect(activityCards).toHaveLength(3)
		})

		it('should display activity names', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			expect(container.textContent).toContain('Exercise')
			expect(container.textContent).toContain('Meditation')
			expect(container.textContent).toContain('Reading')
		})

		it('should display activity icons when present', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			expect(container.textContent).toContain('🏃')
			expect(container.textContent).toContain('🧘')
		})

		it('should not show icon for activities without icons', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[2]], groups: mockGroups, onEdit, onDelete },
			})

			// Reading activity has no icon
			const icons = container.querySelectorAll('.text-2xl')
			expect(icons).toHaveLength(0)
		})

		it('should display color indicators when present', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			expect(container.textContent).toContain('#22C55E')
			expect(container.textContent).toContain('#3B82F6')
		})

		it('should show color swatch for activities with colors', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const colorSwatch = container.querySelector('.w-6.h-6.rounded-sm')
			expect(colorSwatch).toBeInTheDocument()
			expect(colorSwatch).toHaveStyle('background-color: #22C55E')
		})

		it('should not show color indicator for activities without colors', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[2]], groups: mockGroups, onEdit, onDelete },
			})

			const colorSwatch = container.querySelector('.w-6.h-6.rounded-sm')
			expect(colorSwatch).not.toBeInTheDocument()
		})

		it('should have grid layout', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			const grid = container.querySelector('.grid')
			expect(grid).toBeInTheDocument()
			expect(grid).toHaveClass('gap-3', 'sm:grid-cols-2', 'lg:grid-cols-3')
		})

		it('should have fadeIn animation on cards', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const activityCard = container.querySelector('.activity-card')
			expect(activityCard).toHaveClass('activity-card')
		})

		it('should have hover shadow-sm effect on cards', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[2]], groups: mockGroups, onEdit, onDelete },
			})

			const activityCard = container.querySelector('.activity-card')
			expect(activityCard).toHaveClass('hover:shadow-md', 'transition-shadow')
		})
	})

	describe('Edit Button', () => {
		it('should render edit button for each activity', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			const editButtons = Array.from(container.querySelectorAll('button')).filter((btn) =>
				btn.getAttribute('aria-label')?.includes('Edit')
			)
			expect(editButtons).toHaveLength(3)
		})

		it('should call onEdit when edit button clicked', async () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			const editButtons = Array.from(container.querySelectorAll('button')).filter((btn) =>
				btn.getAttribute('aria-label')?.includes('Edit')
			)
			await fireEvent.click(editButtons[0])

			expect(onEdit).toHaveBeenCalledWith(mockActivities[0])
		})

		it('should have proper aria-label for edit button', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			const editButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Edit')
			)
			expect(editButton).toHaveAttribute('aria-label', 'Edit Exercise')
		})

		it('should have title attribute on edit button', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const editButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Edit')
			)
			expect(editButton).toHaveAttribute('title', 'Edit')
		})

		it('should show edit icon in button', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const editButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Edit')
			)
			const svg = editButton?.querySelector('svg')
			expect(svg).toBeInTheDocument()
		})

		it('should have hover styles on edit button', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const editButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Edit')
			)
			expect(editButton).toHaveClass('hover:bg-blue-50', 'dark:hover:bg-blue-900/30')
		})
	})

	describe('Delete Button', () => {
		let confirmSpy: MockInstance<(message?: string) => boolean>

		beforeEach(() => {
			confirmSpy = vi.spyOn(window, 'confirm')
		})

		afterEach(() => {
			confirmSpy.mockRestore()
		})

		it('should render delete button for each activity', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			const deleteButtons = Array.from(container.querySelectorAll('button')).filter((btn) =>
				btn.getAttribute('aria-label')?.includes('Delete')
			)
			expect(deleteButtons).toHaveLength(3)
		})

		it('should show confirmation dialog when delete button clicked', async () => {
			confirmSpy.mockReturnValue(false)
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const deleteButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Delete')
			)!
			await fireEvent.click(deleteButton)

			expect(confirmSpy).toHaveBeenCalledWith(
				'Are you sure you want to delete "Exercise"? This action cannot be undone.'
			)
		})

		it('should call onDelete when confirmed', async () => {
			confirmSpy.mockReturnValue(true)
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const deleteButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Delete')
			)!
			await fireEvent.click(deleteButton)

			expect(onDelete).toHaveBeenCalledWith(1)
		})

		it('should not call onDelete when cancelled', async () => {
			confirmSpy.mockReturnValue(false)
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const deleteButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Delete')
			)!
			await fireEvent.click(deleteButton)

			expect(onDelete).not.toHaveBeenCalled()
		})

		it('should have proper aria-label for delete button', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const deleteButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Delete')
			)
			expect(deleteButton).toHaveAttribute('aria-label', 'Delete Exercise')
		})

		it('should have title attribute on delete button', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const deleteButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Delete')
			)
			expect(deleteButton).toHaveAttribute('title', 'Delete')
		})

		it('should show delete icon in button', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const deleteButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Delete')
			)
			const svg = deleteButton?.querySelector('svg')
			expect(svg).toBeInTheDocument()
		})

		it('should have hover styles on delete button', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const deleteButton = Array.from(container.querySelectorAll('button')).find((btn) =>
				btn.getAttribute('aria-label')?.includes('Delete')
			)
			expect(deleteButton).toHaveClass('hover:bg-red-50', 'dark:hover:bg-red-900/30')
		})
	})

	describe('Accessibility', () => {
		it('should have aria-labels on all action buttons', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			const buttons = container.querySelectorAll('button')
			buttons.forEach((button) => {
				expect(button).toHaveAttribute('aria-label')
			})
		})

		it('should have title attributes on all action buttons', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			const buttons = container.querySelectorAll('button')
			buttons.forEach((button) => {
				expect(button).toHaveAttribute('title')
			})
		})

		it('should have proper text styling for dark mode', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [mockActivities[0]], groups: mockGroups, onEdit, onDelete },
			})

			const activityName = container.querySelector('h3')
			expect(activityName).toHaveClass('dark:text-white')
		})
	})

	describe('Props Defaults', () => {
		it('should default loading to false', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: mockActivities, groups: mockGroups, onEdit, onDelete },
			})

			expect(container.textContent).not.toContain('Loading activities...')
			const activityCards = container.querySelectorAll('.activity-card')
			expect(activityCards).toHaveLength(3)
		})

		it('should default activities to empty array', () => {
			const onEdit = vi.fn()
			const onDelete = vi.fn()
			const { container } = render(ActivityList, {
				props: { activities: [], groups: [], onEdit, onDelete },
			})

			expect(container.textContent).toContain('No activities yet')
		})
	})
})
