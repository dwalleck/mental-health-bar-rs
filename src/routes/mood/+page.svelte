<script lang="ts">
	// T090: /mood route - Quick mood check-in page

	import { invokeWithRetry } from '$lib/utils/retry'
	import Card from '$lib/components/ui/Card.svelte'
	import MoodScaleInput from '$lib/components/mood/MoodScaleInput.svelte'
	import ActivitySelector from '$lib/components/mood/ActivitySelector.svelte'
	import { getMoodLabel } from '$lib/utils/colors'
	import { displayError, displaySuccess } from '$lib/utils/errors'
	import type { MoodCheckin } from '$lib/bindings'

	let moodRating = $state(3)
	let selectedActivityIds: number[] = $state([])
	let notes = $state('')
	let isSubmitting = $state(false)
	let error = $state<string | null>(null)
	let successMessage = $state<string | null>(null)
	let recentCheckins: MoodCheckin[] = $state([])

	// Load recent check-ins on mount
	$effect(() => {
		loadRecentCheckins()
	})

	async function loadRecentCheckins() {
		try {
			const history = await invokeWithRetry('get_mood_history', {
				fromDate: null,
				toDate: null,
				limit: 3,
			})
			recentCheckins = history as MoodCheckin[]
		} catch (e) {
			displayError(e)
		}
	}

	async function submitMood(event: SubmitEvent) {
		event.preventDefault()
		if (isSubmitting) return

		try {
			isSubmitting = true
			error = null
			successMessage = null

			// Validate notes length before submission
			const trimmedNotes = notes.trim()
			if (trimmedNotes.length > 5000) {
				const result = displayError(
					new Error(
						`Notes too long: ${trimmedNotes.length} characters. Maximum 5000 characters allowed.`
					)
				)
				if (result.type === 'inline') {
					error = result.message || 'Notes too long'
				}
				return
			}

			await invokeWithRetry('log_mood', {
				request: {
					mood_rating: moodRating,
					activity_ids: selectedActivityIds,
					notes: trimmedNotes || null,
				},
			})

			displaySuccess('Mood logged successfully!')
			moodRating = 3
			selectedActivityIds = []
			notes = ''
			await loadRecentCheckins()
		} catch (e) {
			const result = displayError(e)
			if (result.type === 'inline') {
				error = result.message || 'Failed to log mood'
			}
		} finally {
			isSubmitting = false
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr)
		const now = new Date()
		const isToday = date.toDateString() === now.toDateString()
		if (isToday) {
			return date.toLocaleTimeString('en-US', {
				hour: 'numeric',
				minute: '2-digit',
				hour12: true,
			})
		}
		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: 'numeric',
			minute: '2-digit',
		})
	}
</script>

<div class="max-w-4xl mx-auto">
	<div class="mb-6">
		<div class="flex items-start justify-between">
			<div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white">Mood Check-In</h1>
				<p class="text-gray-600 dark:text-gray-400 mt-1">
					How are you feeling right now? Track your mood in under 30 seconds.
				</p>
			</div>
			<a
				href="/mood/activities"
				data-sveltekit-preload-data="hover"
				class="px-4 py-2 text-sm bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-300 rounded-lg transition-colors inline-flex items-center gap-2"
			>
				<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
					></path>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
					></path>
				</svg>
				Manage Activities
			</a>
		</div>
	</div>

	{#if error}
		<div class="mb-4 p-4 bg-red-100 border border-red-300 text-red-700 rounded-lg">
			<div class="font-semibold">Error</div>
			<div class="text-sm mt-1">{error}</div>
		</div>
	{/if}

	{#if successMessage}
		<div class="mb-4 p-4 bg-green-100 border border-green-300 text-green-700 rounded-lg">
			<div class="font-semibold">✓ {successMessage}</div>
		</div>
	{/if}

	<Card title="Log Your Mood">
		<form onsubmit={submitMood} class="space-y-6">
			<div>
				<MoodScaleInput value={moodRating} onChange={(rating) => (moodRating = rating)} />
			</div>

			<div>
				<ActivitySelector
					selectedIds={selectedActivityIds}
					onChange={(ids) => (selectedActivityIds = ids)}
				/>
			</div>

			<div>
				<label for="notes" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
					Notes (optional)
				</label>
				<textarea
					id="notes"
					bind:value={notes}
					placeholder="How are you feeling? What's on your mind?"
					rows="3"
					class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md
						focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
					maxlength="5000"
				></textarea>
				<div class="text-xs text-gray-500 dark:text-gray-400 mt-1 text-right">
					{notes.length} / 5000 characters
				</div>
			</div>

			<button
				type="submit"
				class="w-full py-3 px-4 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg
					disabled:opacity-50 disabled:cursor-not-allowed transition-colors shadow-sm"
				disabled={isSubmitting}
			>
				{isSubmitting ? 'Logging Mood...' : 'Log Mood'}
			</button>
		</form>
	</Card>

	{#if recentCheckins.length > 0}
		<div class="mt-6">
			<div class="flex items-center justify-between mb-3">
				<h2 class="text-xl font-semibold text-gray-900 dark:text-white">Recent Check-Ins</h2>
				<a
					href="/mood/history"
					data-sveltekit-preload-data="hover"
					class="text-sm text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
				>
					View All →
				</a>
			</div>
			<div class="grid gap-3">
				{#each recentCheckins as checkin (checkin.id)}
					<div
						class="p-4 bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700"
					>
						<div class="flex items-center gap-3">
							<div class="text-2xl font-bold text-gray-700 dark:text-gray-300">
								{checkin.mood_rating}
							</div>
							<div class="flex-1">
								<div class="font-medium text-gray-900 dark:text-white">
									{getMoodLabel(checkin.mood_rating)}
								</div>
								<div class="text-xs text-gray-500 dark:text-gray-400">
									{formatDate(checkin.created_at)}
								</div>
							</div>
						</div>
						{#if checkin.activities && checkin.activities.length > 0}
							<div class="mt-2 flex flex-wrap gap-1">
								{#each checkin.activities as activity (activity.id)}
									<span
										class="inline-flex items-center gap-1 px-2 py-0.5 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded text-xs"
									>
										{#if activity.icon}{activity.icon}{/if}
										{activity.name}
									</span>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>
