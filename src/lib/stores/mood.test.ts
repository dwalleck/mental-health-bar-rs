import { describe, it, expect, beforeEach } from 'vitest'
import { get } from 'svelte/store'
import {
	mood,
	moodHistory,
	activities,
	moodStats,
	resetMoodForm,
	setMoodRating,
	setSelectedActivities,
	setNotes,
	setSubmitting,
} from './mood'
import type { MoodCheckin, Activity } from '$lib/bindings'

describe('Mood Store', () => {
	beforeEach(() => {
		// Reset stores before each test
		resetMoodForm()
		moodHistory.set([])
		activities.set([])
	})

	describe('mood store', () => {
		it('should have initial state', () => {
			const state = get(mood)
			expect(state).toEqual({
				currentRating: 3,
				selectedActivityIds: [],
				notes: '',
				isSubmitting: false,
			})
		})

		it('should be subscribable', () => {
			let value
			const unsubscribe = mood.subscribe((v) => {
				value = v
			})

			expect(value).toBeDefined()
			unsubscribe()
		})
	})

	describe('moodHistory store', () => {
		it('should initialize as empty array', () => {
			const history = get(moodHistory)
			expect(history).toEqual([])
		})

		it('should store mood checkins', () => {
			const checkins: MoodCheckin[] = [
				{
					id: 1,
					mood_rating: 4,
					notes: 'Feeling good',
					activities: [],
					created_at: '2024-01-01T00:00:00Z',
				},
			]

			moodHistory.set(checkins)
			expect(get(moodHistory)).toEqual(checkins)
		})

		it('should update with multiple checkins', () => {
			const checkins: MoodCheckin[] = [
				{
					id: 1,
					mood_rating: 4,
					notes: 'Good',
					activities: [],
					created_at: '2024-01-01T00:00:00Z',
				},
				{
					id: 2,
					mood_rating: 3,
					notes: 'Okay',
					activities: [],
					created_at: '2024-01-02T00:00:00Z',
				},
			]

			moodHistory.set(checkins)
			expect(get(moodHistory)).toHaveLength(2)
		})
	})

	describe('activities store', () => {
		it('should initialize as empty array', () => {
			const acts = get(activities)
			expect(acts).toEqual([])
		})

		it('should store activities', () => {
			const acts: Activity[] = [
				{
					id: 1,
					name: 'Exercise',
					color: '#22C55E',
					icon: '🏃',
					created_at: '2024-01-01T00:00:00Z',
					updated_at: '2024-01-01T00:00:00Z',
					deleted_at: null,
				},
			]

			activities.set(acts)
			expect(get(activities)).toEqual(acts)
		})
	})

	describe('moodStats derived store', () => {
		it('should return zero stats when no history', () => {
			const stats = get(moodStats)
			expect(stats).toEqual({
				averageMood: 0,
				totalCheckins: 0,
				lastCheckin: null,
			})
		})

		it('should calculate average mood from single checkin', () => {
			moodHistory.set([
				{
					id: 1,
					mood_rating: 4,
					notes: null,
					activities: [],
					created_at: '2024-01-01T00:00:00Z',
				},
			])

			const stats = get(moodStats)
			expect(stats.averageMood).toBe(4)
			expect(stats.totalCheckins).toBe(1)
			expect(stats.lastCheckin?.id).toBe(1)
		})

		it('should calculate average mood from multiple checkins', () => {
			moodHistory.set([
				{
					id: 1,
					mood_rating: 5,
					notes: null,
					activities: [],
					created_at: '2024-01-03T00:00:00Z',
				},
				{
					id: 2,
					mood_rating: 3,
					notes: null,
					activities: [],
					created_at: '2024-01-02T00:00:00Z',
				},
				{
					id: 3,
					mood_rating: 4,
					notes: null,
					activities: [],
					created_at: '2024-01-01T00:00:00Z',
				},
			])

			const stats = get(moodStats)
			expect(stats.averageMood).toBe(4) // (5 + 3 + 4) / 3 = 4
			expect(stats.totalCheckins).toBe(3)
		})

		it('should return first checkin as lastCheckin', () => {
			const firstCheckin = {
				id: 1,
				mood_rating: 5,
				notes: 'Most recent',
				activities: [],
				created_at: '2024-01-03T00:00:00Z',
			}

			moodHistory.set([
				firstCheckin,
				{
					id: 2,
					mood_rating: 3,
					notes: 'Older',
					activities: [],
					created_at: '2024-01-01T00:00:00Z',
				},
			])

			const stats = get(moodStats)
			expect(stats.lastCheckin).toEqual(firstCheckin)
		})

		it('should reactively update when moodHistory changes', () => {
			moodHistory.set([
				{
					id: 1,
					mood_rating: 3,
					notes: null,
					activities: [],
					created_at: '2024-01-01T00:00:00Z',
				},
			])

			expect(get(moodStats).averageMood).toBe(3)

			moodHistory.update((history) => [
				...history,
				{
					id: 2,
					mood_rating: 5,
					notes: null,
					activities: [],
					created_at: '2024-01-02T00:00:00Z',
				},
			])

			expect(get(moodStats).averageMood).toBe(4) // (3 + 5) / 2 = 4
			expect(get(moodStats).totalCheckins).toBe(2)
		})
	})

	describe('resetMoodForm', () => {
		it('should reset mood to initial state', () => {
			mood.set({
				currentRating: 5,
				selectedActivityIds: [1, 2, 3],
				notes: 'Some notes',
				isSubmitting: true,
			})

			resetMoodForm()

			const state = get(mood)
			expect(state).toEqual({
				currentRating: 3,
				selectedActivityIds: [],
				notes: '',
				isSubmitting: false,
			})
		})

		it('should be callable multiple times', () => {
			resetMoodForm()
			resetMoodForm()

			const state = get(mood)
			expect(state.currentRating).toBe(3)
		})
	})

	describe('setMoodRating', () => {
		it('should update current rating', () => {
			setMoodRating(5)
			expect(get(mood).currentRating).toBe(5)
		})

		it('should preserve other state fields', () => {
			mood.set({
				currentRating: 3,
				selectedActivityIds: [1, 2],
				notes: 'Test notes',
				isSubmitting: false,
			})

			setMoodRating(4)

			const state = get(mood)
			expect(state.currentRating).toBe(4)
			expect(state.selectedActivityIds).toEqual([1, 2])
			expect(state.notes).toBe('Test notes')
			expect(state.isSubmitting).toBe(false)
		})

		it('should accept all valid mood ratings', () => {
			for (let rating = 1; rating <= 5; rating++) {
				setMoodRating(rating)
				expect(get(mood).currentRating).toBe(rating)
			}
		})
	})

	describe('setSelectedActivities', () => {
		it('should update selected activity IDs', () => {
			setSelectedActivities([1, 2, 3])
			expect(get(mood).selectedActivityIds).toEqual([1, 2, 3])
		})

		it('should replace previous selections', () => {
			setSelectedActivities([1, 2])
			setSelectedActivities([3, 4, 5])

			expect(get(mood).selectedActivityIds).toEqual([3, 4, 5])
		})

		it('should accept empty array', () => {
			setSelectedActivities([1, 2])
			setSelectedActivities([])

			expect(get(mood).selectedActivityIds).toEqual([])
		})

		it('should preserve other state fields', () => {
			mood.set({
				currentRating: 4,
				selectedActivityIds: [],
				notes: 'Test',
				isSubmitting: false,
			})

			setSelectedActivities([1, 2])

			const state = get(mood)
			expect(state.currentRating).toBe(4)
			expect(state.notes).toBe('Test')
			expect(state.isSubmitting).toBe(false)
		})
	})

	describe('setNotes', () => {
		it('should update notes', () => {
			setNotes('Feeling great today!')
			expect(get(mood).notes).toBe('Feeling great today!')
		})

		it('should replace previous notes', () => {
			setNotes('First note')
			setNotes('Second note')

			expect(get(mood).notes).toBe('Second note')
		})

		it('should accept empty string', () => {
			setNotes('Some notes')
			setNotes('')

			expect(get(mood).notes).toBe('')
		})

		it('should preserve other state fields', () => {
			mood.set({
				currentRating: 5,
				selectedActivityIds: [1],
				notes: '',
				isSubmitting: false,
			})

			setNotes('New notes')

			const state = get(mood)
			expect(state.currentRating).toBe(5)
			expect(state.selectedActivityIds).toEqual([1])
			expect(state.isSubmitting).toBe(false)
		})
	})

	describe('setSubmitting', () => {
		it('should update isSubmitting to true', () => {
			setSubmitting(true)
			expect(get(mood).isSubmitting).toBe(true)
		})

		it('should update isSubmitting to false', () => {
			setSubmitting(true)
			setSubmitting(false)

			expect(get(mood).isSubmitting).toBe(false)
		})

		it('should preserve other state fields', () => {
			mood.set({
				currentRating: 4,
				selectedActivityIds: [1, 2],
				notes: 'Test',
				isSubmitting: false,
			})

			setSubmitting(true)

			const state = get(mood)
			expect(state.currentRating).toBe(4)
			expect(state.selectedActivityIds).toEqual([1, 2])
			expect(state.notes).toBe('Test')
		})
	})

	describe('Integration scenarios', () => {
		it('should handle full mood submission workflow', () => {
			// Initial state
			resetMoodForm()

			// User selects rating
			setMoodRating(4)
			expect(get(mood).currentRating).toBe(4)

			// User selects activities
			setSelectedActivities([1, 2])
			expect(get(mood).selectedActivityIds).toEqual([1, 2])

			// User adds notes
			setNotes('Had a good workout')
			expect(get(mood).notes).toBe('Had a good workout')

			// Submission starts
			setSubmitting(true)
			expect(get(mood).isSubmitting).toBe(true)

			// Submission completes
			setSubmitting(false)
			expect(get(mood).isSubmitting).toBe(false)

			// Form resets
			resetMoodForm()
			expect(get(mood)).toEqual({
				currentRating: 3,
				selectedActivityIds: [],
				notes: '',
				isSubmitting: false,
			})
		})

		it('should handle mood history updates with stats recalculation', () => {
			// Add first checkin
			moodHistory.set([
				{
					id: 1,
					mood_rating: 4,
					notes: 'Good day',
					activities: [],
					created_at: '2024-01-01T00:00:00Z',
				},
			])

			expect(get(moodStats).averageMood).toBe(4)
			expect(get(moodStats).totalCheckins).toBe(1)

			// Add second checkin
			moodHistory.update((h) => [
				{
					id: 2,
					mood_rating: 2,
					notes: 'Not great',
					activities: [],
					created_at: '2024-01-02T00:00:00Z',
				},
				...h,
			])

			expect(get(moodStats).averageMood).toBe(3) // (4 + 2) / 2 = 3
			expect(get(moodStats).totalCheckins).toBe(2)
			expect(get(moodStats).lastCheckin?.id).toBe(2)
		})
	})
})
