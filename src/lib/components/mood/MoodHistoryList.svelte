<script lang="ts">
	// T088, T181: MoodHistoryList component - Display past mood check-ins with activities and loading animations

	import { fade } from 'svelte/transition'
	import { MOOD_COLORS, MOOD_LABELS } from '$lib/utils/colors'
	import type { MoodCheckin } from '$lib/bindings'
	import SkeletonLoader from '$lib/components/ui/SkeletonLoader.svelte'

	interface Props {
		checkins: MoodCheckin[]
		loading?: boolean
		error?: string | null
	}

	let { checkins = [], loading = false, error = null }: Props = $props()

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr)
		const now = new Date()
		const diffMs = now.getTime() - date.getTime()
		const diffMins = Math.floor(diffMs / 60000)
		const diffHours = Math.floor(diffMs / 3600000)
		const diffDays = Math.floor(diffMs / 86400000)

		if (diffMins < 60) {
			return `${diffMins} minute${diffMins !== 1 ? 's' : ''} ago`
		} else if (diffHours < 24) {
			return `${diffHours} hour${diffHours !== 1 ? 's' : ''} ago`
		} else if (diffDays < 7) {
			return `${diffDays} day${diffDays !== 1 ? 's' : ''} ago`
		} else {
			return date.toLocaleDateString('en-US', {
				month: 'short',
				day: 'numeric',
				year: date.getFullYear() !== now.getFullYear() ? 'numeric' : undefined,
			})
		}
	}

	function formatTime(dateStr: string): string {
		const date = new Date(dateStr)
		return date.toLocaleTimeString('en-US', {
			hour: 'numeric',
			minute: '2-digit',
			hour12: true,
		})
	}
</script>

<div class="mood-history-list">
	{#if error}
		<div
			class="p-4 bg-red-100 border border-red-300 text-red-700 rounded-lg"
			transition:fade={{ duration: 200 }}
		>
			<div class="font-semibold">Error loading mood history</div>
			<div class="text-sm mt-1">{error}</div>
		</div>
	{:else if loading}
		<!-- T181: Loading skeleton for list items -->
		<SkeletonLoader type="list" count={3} />
	{:else if checkins.length === 0}
		<div
			class="text-center py-8 text-gray-500 dark:text-gray-400"
			transition:fade={{ duration: 200 }}
		>
			<div class="text-4xl mb-2">📊</div>
			<div class="font-medium">No mood check-ins yet</div>
			<div class="text-sm mt-1">Start tracking your mood to see your history here</div>
		</div>
	{:else}
		<div class="space-y-3">
			{#each checkins as checkin (checkin.id)}
				<div
					class="mood-checkin-card p-4 bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-shadow"
				>
					<div class="flex items-start justify-between gap-3">
						<div class="flex-1">
							<div class="flex items-center gap-3 mb-2">
								<div
									class="mood-badge flex items-center justify-center w-12 h-12 rounded-full text-lg font-bold {MOOD_COLORS[
										checkin.mood_rating
									]}"
									aria-label={`Mood rating: ${checkin.mood_rating} out of 5 (${MOOD_LABELS[checkin.mood_rating]})`}
								>
									{checkin.mood_rating}
								</div>
								<div>
									<div class="font-semibold text-gray-900 dark:text-white">
										{MOOD_LABELS[checkin.mood_rating]}
									</div>
									<div class="text-xs text-gray-500 dark:text-gray-400">
										{formatDate(checkin.created_at)} • {formatTime(checkin.created_at)}
									</div>
								</div>
							</div>

							{#if checkin.notes}
								<div class="mt-2 text-sm text-gray-700 dark:text-gray-300 italic">
									"{checkin.notes}"
								</div>
							{/if}

							{#if checkin.activities && checkin.activities.length > 0}
								<div class="mt-3 flex flex-wrap gap-2">
									{#each checkin.activities as activity (activity.id)}
										<span
											class="inline-flex items-center gap-1 px-2 py-1 rounded text-xs
												{activity.deleted_at
												? 'bg-gray-200 dark:bg-gray-600 text-gray-500 dark:text-gray-400 line-through'
												: 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300'}"
											style={activity.color && !activity.deleted_at
												? `border-left: 3px solid ${activity.color}`
												: ''}
										>
											{#if activity.icon}
												<span>{activity.icon}</span>
											{/if}
											<span>{activity.name}</span>
											{#if activity.deleted_at}
												<span class="text-xs opacity-75">(deleted)</span>
											{/if}
										</span>
									{/each}
								</div>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.mood-checkin-card {
		animation: fadeIn 0.3s ease-in-out;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.mood-badge {
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}
</style>
