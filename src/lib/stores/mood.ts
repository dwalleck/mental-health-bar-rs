// T089: Mood Svelte store for reactive state management

import { writable, derived } from 'svelte/store'
import type { MoodCheckin, Activity } from '$lib/bindings'

interface MoodState {
	currentRating: number
	selectedActivityIds: number[]
	notes: string
	isSubmitting: boolean
}

const initialState: MoodState = {
	currentRating: 3,
	selectedActivityIds: [],
	notes: '',
	isSubmitting: false,
}

export const mood = writable<MoodState>(initialState)

export const moodHistory = writable<MoodCheckin[]>([])
export const activities = writable<Activity[]>([])

export const moodStats = derived(moodHistory, ($history) => {
	if ($history.length === 0) {
		return {
			averageMood: 0,
			totalCheckins: 0,
			lastCheckin: null,
		}
	}

	const total = $history.reduce((sum, checkin) => sum + checkin.mood_rating, 0)
	return {
		averageMood: total / $history.length,
		totalCheckins: $history.length,
		lastCheckin: $history[0],
	}
})

// Helper functions
export function resetMoodForm() {
	mood.set(initialState)
}

export function setMoodRating(rating: number) {
	mood.update((state) => ({ ...state, currentRating: rating }))
}

export function setSelectedActivities(activityIds: number[]) {
	mood.update((state) => ({ ...state, selectedActivityIds: activityIds }))
}

export function setNotes(notes: string) {
	mood.update((state) => ({ ...state, notes }))
}

export function setSubmitting(isSubmitting: boolean) {
	mood.update((state) => ({ ...state, isSubmitting }))
}
