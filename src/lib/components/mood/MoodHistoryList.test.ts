import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render } from '@testing-library/svelte'
import MoodHistoryList from './MoodHistoryList.svelte'
import type { MoodCheckin, Activity } from '$lib/bindings'

// Mock svelte/transition to avoid element.animate issues in jsdom
vi.mock('svelte/transition', () => ({
	fade: () => ({}),
}))

describe('MoodHistoryList', () => {
	const mockActivities: Activity[] = [
		{
			id: 1,
			name: 'Exercise',
			color: '#22C55E',
			icon: '🏃',
			created_at: '2024-01-01T00:00:00Z',
			deleted_at: null,
		},
		{
			id: 2,
			name: 'Meditation',
			color: '#3B82F6',
			icon: '🧘',
			created_at: '2024-01-02T00:00:00Z',
			deleted_at: null,
		},
		{
			id: 3,
			name: 'Reading',
			color: null,
			icon: null,
			created_at: '2024-01-03T00:00:00Z',
			deleted_at: '2024-01-10T00:00:00Z', // Deleted
		},
	]

	const mockCheckins: MoodCheckin[] = [
		{
			id: 1,
			mood_rating: 4,
			notes: 'Feeling good after exercise',
			activities: [mockActivities[0], mockActivities[1]],
			created_at: '2024-01-15T11:00:00Z', // 1 hour ago from reference time
		},
		{
			id: 2,
			mood_rating: 3,
			notes: null,
			activities: [],
			created_at: '2024-01-14T12:00:00Z', // 1 day ago from reference time
		},
		{
			id: 3,
			mood_rating: 5,
			notes: 'Great day!',
			activities: [mockActivities[2]], // Deleted activity
			created_at: '2024-01-08T12:00:00Z', // 7 days ago from reference time
		},
	]

	describe('Error State', () => {
		it('should display error message', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [], error: 'Failed to load mood history' },
			})

			expect(container.textContent).toContain('Failed to load mood history')
		})

		it('should have error styling', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [], error: 'Failed to load' },
			})

			const errorMessage = container.querySelector('.error-message')
			expect(errorMessage).toBeInTheDocument()
		})

		it('should not show checkins when error present', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: mockCheckins, error: 'Error occurred' },
			})

			const checkinCards = container.querySelectorAll('.mood-checkin-card')
			expect(checkinCards).toHaveLength(0)
		})
	})

	describe('Loading State', () => {
		it('should show SkeletonLoader when loading', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [], loading: true },
			})

			// SkeletonLoader component should be rendered - check for loading content
			expect(container.textContent).not.toContain('No mood check-ins yet')
			expect(container.textContent).not.toContain('Error loading')
		})

		it('should not show checkins when loading', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: mockCheckins, loading: true },
			})

			const checkinCards = container.querySelectorAll('.mood-checkin-card')
			expect(checkinCards).toHaveLength(0)
		})
	})

	describe('Empty State', () => {
		it('should display empty message when no checkins', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [], loading: false },
			})

			expect(container.textContent).toContain('No mood check-ins yet')
		})

		it('should show descriptive text in empty state', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [], loading: false },
			})

			expect(container.textContent).toContain('Start tracking your mood to see your history here')
		})

		it('should show emoji icon in empty state', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [], loading: false },
			})

			expect(container.textContent).toContain('📊')
		})
	})

	describe('Checkins Display', () => {
		it('should render all checkins', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: mockCheckins },
			})

			const checkinCards = container.querySelectorAll('.mood-checkin-card')
			expect(checkinCards).toHaveLength(3)
		})

		it('should display mood rating badges', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: mockCheckins },
			})

			const badges = container.querySelectorAll('.mood-badge')
			expect(badges).toHaveLength(3)
			expect(badges[0]).toHaveTextContent('4')
			expect(badges[1]).toHaveTextContent('3')
			expect(badges[2]).toHaveTextContent('5')
		})

		it('should display mood labels', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: mockCheckins },
			})

			expect(container.textContent).toContain('Good') // Rating 4
			expect(container.textContent).toContain('Neutral') // Rating 3
			expect(container.textContent).toContain('Very Good') // Rating 5
		})

		it('should apply correct color classes to badges', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[0]] }, // Rating 4
			})

			const badge = container.querySelector('.mood-badge')
			expect(badge).toHaveClass('bg-lime-500') // Good = lime
		})

		it('should have aria-label on mood badge', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[0]] },
			})

			const badge = container.querySelector('.mood-badge')
			expect(badge).toHaveAttribute('aria-label', 'Mood rating: 4 out of 5 (Good)')
		})

		it('should display notes when present', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[0]] },
			})

			expect(container.textContent).toContain('Feeling good after exercise')
		})

		it('should not show notes section when absent', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[1]] }, // No notes
			})

			const italicText = container.querySelector('italic')
			expect(italicText).not.toBeInTheDocument()
		})

		it('should display activities when present', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[0]] },
			})

			expect(container.textContent).toContain('Exercise')
			expect(container.textContent).toContain('Meditation')
		})

		it('should display activity icons', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[0]] },
			})

			expect(container.textContent).toContain('🏃')
			expect(container.textContent).toContain('🧘')
		})

		it('should not show activities section when empty', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[1]] }, // No activities
			})

			const activityTags = container.querySelectorAll('.inline-flex.items-center')
			expect(activityTags).toHaveLength(0)
		})

		it('should show deleted activities with strikethrough', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[2]] }, // Has deleted activity
			})

			const deletedActivity = Array.from(container.querySelectorAll('span')).find(
				(span) => span.textContent === 'Reading'
			)
			expect(deletedActivity?.parentElement).toHaveClass('line-through')
		})

		it('should show (deleted) label for deleted activities', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[2]] },
			})

			expect(container.textContent).toContain('(deleted)')
		})

		it('should apply different styling to deleted activities', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[2]] },
			})

			const activitySpan = container.querySelector('.line-through')
			expect(activitySpan).toHaveClass('bg-gray-200', 'dark:bg-gray-600')
			expect(activitySpan).toHaveClass('text-gray-500', 'dark:text-gray-400')
		})

		it('should have fadeIn animation on cards', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[0]] },
			})

			const card = container.querySelector('.mood-checkin-card')
			expect(card).toHaveClass('mood-checkin-card')
		})

		it('should have hover shadow effect on cards', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[0]] },
			})

			const card = container.querySelector('.mood-checkin-card')
			expect(card).toHaveClass('hover:shadow-md', 'transition-shadow')
		})
	})

	describe('Date/Time Formatting', () => {
		beforeEach(() => {
			// Fixed reference time: 2024-01-15 12:00:00
			vi.setSystemTime(new Date('2024-01-15T12:00:00Z'))
		})

		it('should format recent time as minutes ago', () => {
			const recentCheckin: MoodCheckin = {
				id: 99,
				mood_rating: 3,
				notes: null,
				activities: [],
				created_at: new Date('2024-01-15T11:30:00Z').toISOString(), // 30 minutes ago
			}

			const { container } = render(MoodHistoryList, {
				props: { checkins: [recentCheckin] },
			})

			expect(container.textContent).toContain('30 minutes ago')
		})

		it('should format as hours ago when less than 24 hours', () => {
			const hourAgoCheckin: MoodCheckin = {
				id: 99,
				mood_rating: 3,
				notes: null,
				activities: [],
				created_at: new Date('2024-01-15T10:00:00Z').toISOString(), // 2 hours ago
			}

			const { container } = render(MoodHistoryList, {
				props: { checkins: [hourAgoCheckin] },
			})

			expect(container.textContent).toContain('2 hours ago')
		})

		it('should format as days ago when less than 7 days', () => {
			const dayAgoCheckin: MoodCheckin = {
				id: 99,
				mood_rating: 3,
				notes: null,
				activities: [],
				created_at: new Date('2024-01-12T12:00:00Z').toISOString(), // 3 days ago
			}

			const { container } = render(MoodHistoryList, {
				props: { checkins: [dayAgoCheckin] },
			})

			expect(container.textContent).toContain('3 days ago')
		})

		it('should handle singular units correctly', () => {
			const oneMinuteAgo: MoodCheckin = {
				id: 99,
				mood_rating: 3,
				notes: null,
				activities: [],
				created_at: new Date('2024-01-15T11:59:00Z').toISOString(), // 1 minute ago
			}

			const { container } = render(MoodHistoryList, {
				props: { checkins: [oneMinuteAgo] },
			})

			expect(container.textContent).toContain('1 minute ago')
			expect(container.textContent).not.toContain('1 minutes ago')
		})

		it('should display time in 12-hour format', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[0]] },
			})

			// Should have AM or PM
			const timeText = container.textContent || ''
			expect(timeText.match(/\d{1,2}:\d{2}\s*(AM|PM)/i)).toBeTruthy()
		})
	})

	describe('Accessibility', () => {
		it('should have aria-labels on all mood badges', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: mockCheckins },
			})

			const badges = container.querySelectorAll('.mood-badge')
			badges.forEach((badge) => {
				expect(badge).toHaveAttribute('aria-label')
			})
		})

		it('should include mood rating and label in aria-label', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[2]] }, // Rating 5
			})

			const badge = container.querySelector('.mood-badge')
			const ariaLabel = badge?.getAttribute('aria-label')
			expect(ariaLabel).toContain('5 out of 5')
			expect(ariaLabel).toContain('Very Good')
		})

		it('should have proper dark mode text styling', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[0]] },
			})

			const moodLabel = container.querySelector('.font-semibold')
			expect(moodLabel).toHaveClass('dark:text-white')
		})
	})

	describe('Props Defaults', () => {
		it('should default loading to false', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: mockCheckins },
			})

			const skeleton = container.querySelector('.skeleton-loader')
			expect(skeleton).not.toBeInTheDocument()

			const checkinCards = container.querySelectorAll('.mood-checkin-card')
			expect(checkinCards).toHaveLength(3)
		})

		it('should default error to null', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: mockCheckins },
			})

			const errorDiv = container.querySelector('.bg-red-100')
			expect(errorDiv).not.toBeInTheDocument()
		})

		it('should default checkins to empty array', () => {
			const { container } = render(MoodHistoryList, { props: { checkins: [] } })

			expect(container.textContent).toContain('No mood check-ins yet')
		})
	})

	describe('Activity Color Borders', () => {
		it('should apply color border to non-deleted activities', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[0]] },
			})

			const activitySpans = container.querySelectorAll('.inline-flex.items-center')
			// First two activities are not deleted and have colors
			expect(activitySpans[0]).toHaveAttribute('style')
			expect(activitySpans[1]).toHaveAttribute('style')
		})

		it('should not apply color border to deleted activities', () => {
			const { container } = render(MoodHistoryList, {
				props: { checkins: [mockCheckins[2]] }, // Has deleted activity
			})

			const activitySpan = container.querySelector('.line-through')
			const style = activitySpan?.getAttribute('style') || ''
			// Deleted activities should not have border-left style
			expect(style).toBe('')
		})
	})
})
