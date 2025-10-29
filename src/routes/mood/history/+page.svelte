<script lang="ts">
	// T091: /mood/history route - View all past mood check-ins

	import { invoke } from '@tauri-apps/api/core'
	import { onMount } from 'svelte'
	import Card from '$lib/components/ui/Card.svelte'
	import MoodHistoryList from '$lib/components/mood/MoodHistoryList.svelte'
	import type { MoodCheckin, MoodStats } from '$lib/bindings'

	let checkins: MoodCheckin[] = $state([])
	let stats: MoodStats | null = $state(null)
	let loading = $state(true)
	let error = $state<string | null>(null)
	let selectedFilter = $state<'all' | 'week' | 'month'>('all')

	onMount(async () => {
		await loadHistory()
		await loadStats()
	})

	async function loadHistory() {
		try {
			loading = true
			error = null

			let fromDate: string | null = null
			const now = Date.now()

			if (selectedFilter === 'week') {
				// eslint-disable-next-line svelte/prefer-svelte-reactivity
				const weekAgo = new Date(now)
				weekAgo.setDate(weekAgo.getDate() - 7)
				fromDate = weekAgo.toISOString()
			} else if (selectedFilter === 'month') {
				// eslint-disable-next-line svelte/prefer-svelte-reactivity
				const monthAgo = new Date(now)
				monthAgo.setMonth(monthAgo.getMonth() - 1)
				fromDate = monthAgo.toISOString()
			}

			const history = await invoke('get_mood_history', {
				fromDate,
				toDate: null,
				limit: 100,
			})
			checkins = history as MoodCheckin[]
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
			console.error('Failed to load mood history:', e)
		} finally {
			loading = false
		}
	}

	async function loadStats() {
		try {
			stats = await invoke('get_mood_stats', {
				fromDate: null,
				toDate: null,
			})
		} catch (e) {
			console.error('Failed to load mood stats:', e)
		}
	}

	async function handleFilterChange(filter: 'all' | 'week' | 'month') {
		selectedFilter = filter
		await loadHistory()
	}
</script>

<svelte:head>
	<title>Mood History - Mental Health Tracker</title>
</svelte:head>

<div class="max-w-4xl mx-auto">
	<div class="mb-6">
		<h1 class="text-3xl font-bold text-gray-900">Mood History</h1>
		<p class="text-gray-600 mt-1">
			Review your mood patterns and track your emotional well-being over time.
		</p>
	</div>

	{#if stats}
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
			<Card title="Average Mood">
				<div class="text-4xl font-bold text-blue-600">
					{stats.average_mood.toFixed(1)}
				</div>
				<div class="text-sm text-gray-500 mt-1">out of 5.0</div>
			</Card>
			<Card title="Total Check-Ins">
				<div class="text-4xl font-bold text-green-600">
					{stats.total_checkins}
				</div>
				<div class="text-sm text-gray-500 mt-1">logged moods</div>
			</Card>
			<Card title="Activity Correlations">
				<div class="text-sm space-y-1">
					{#if stats.activity_correlations && stats.activity_correlations.length > 0}
						{#each stats.activity_correlations.slice(0, 3) as correlation (correlation.activity.id)}
							<div class="flex items-center justify-between">
								<div class="flex items-center gap-1">
									{#if correlation.activity.icon}
										<span>{correlation.activity.icon}</span>
									{/if}
									<span class="text-gray-700">{correlation.activity.name}</span>
								</div>
								<span class="font-semibold text-blue-600">
									{correlation.average_mood.toFixed(1)}
								</span>
							</div>
						{/each}
					{:else}
						<div class="text-gray-500 italic">Log moods with activities to see correlations</div>
					{/if}
				</div>
			</Card>
		</div>
	{/if}

	<Card title="Your Check-Ins">
		<div class="mb-4 flex gap-2">
			<button
				class="px-4 py-2 rounded-md font-medium transition-colors
					{selectedFilter === 'all'
					? 'bg-blue-600 text-white'
					: 'bg-gray-100 text-gray-700 hover:bg-gray-200'}"
				onclick={() => handleFilterChange('all')}
			>
				All Time
			</button>
			<button
				class="px-4 py-2 rounded-md font-medium transition-colors
					{selectedFilter === 'week'
					? 'bg-blue-600 text-white'
					: 'bg-gray-100 text-gray-700 hover:bg-gray-200'}"
				onclick={() => handleFilterChange('week')}
			>
				Last Week
			</button>
			<button
				class="px-4 py-2 rounded-md font-medium transition-colors
					{selectedFilter === 'month'
					? 'bg-blue-600 text-white'
					: 'bg-gray-100 text-gray-700 hover:bg-gray-200'}"
				onclick={() => handleFilterChange('month')}
			>
				Last Month
			</button>
		</div>

		<MoodHistoryList {checkins} {loading} {error} />
	</Card>

	<div class="mt-6">
		<a
			href="/mood"
			data-sveltekit-preload-data="hover"
			class="inline-block px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition-colors shadow-sm"
		>
			← Back to Mood Check-In
		</a>
	</div>
</div>
