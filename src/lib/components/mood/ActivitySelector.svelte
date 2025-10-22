<script lang="ts">
	// T087: ActivitySelector component - Multi-select activity picker with create new capability

	import { invoke } from '@tauri-apps/api/core'
	import { onMount } from 'svelte'
	import type { Activity } from '$lib/bindings'

	interface Props {
		selectedIds: number[]
		onchange: (ids: number[]) => void
	}

	let { selectedIds = [], onchange }: Props = $props()

	let activities: Activity[] = $state([])
	let loading = $state(true)
	let error = $state<string | null>(null)
	let showCreateForm = $state(false)
	let newActivityName = $state('')
	let newActivityColor = $state('#3B82F6')
	let newActivityIcon = $state('')
	let creating = $state(false)

	onMount(async () => {
		await loadActivities()
	})

	async function loadActivities() {
		try {
			loading = true
			error = null
			activities = await invoke('get_activities', { includeDeleted: false })
		} catch (e) {
			error = e as string
			console.error('Failed to load activities:', e)
		} finally {
			loading = false
		}
	}

	function toggleActivity(id: number) {
		if (selectedIds.includes(id)) {
			onchange(selectedIds.filter((selectedId) => selectedId !== id))
		} else {
			onchange([...selectedIds, id])
		}
	}

	async function createActivity() {
		if (!newActivityName.trim()) return

		try {
			creating = true
			error = null
			const activity: Activity = await invoke('create_activity', {
				request: {
					name: newActivityName.trim(),
					color: newActivityColor,
					icon: newActivityIcon || null,
				},
			})
			activities = [...activities, activity]
			onchange([...selectedIds, activity.id])
			newActivityName = ''
			newActivityColor = '#3B82F6'
			newActivityIcon = ''
			showCreateForm = false
		} catch (e) {
			error = e as string
			console.error('Failed to create activity:', e)
		} finally {
			creating = false
		}
	}
</script>

<div class="activity-selector">
	<div class="flex items-center justify-between mb-2">
		<div class="text-sm font-medium text-gray-700 dark:text-gray-300">Activities (optional)</div>
		<button
			type="button"
			class="text-sm text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
			onclick={() => (showCreateForm = !showCreateForm)}
			aria-label="Add new activity"
		>
			{showCreateForm ? 'Cancel' : '+ Add New'}
		</button>
	</div>

	{#if error}
		<div class="mb-3 p-2 bg-red-100 border border-red-300 text-red-700 rounded text-sm">
			{error}
		</div>
	{/if}

	{#if showCreateForm}
		<div
			class="mb-3 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700"
		>
			<input
				type="text"
				bind:value={newActivityName}
				placeholder="Activity name (e.g., Exercise)"
				class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md mb-2
					focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
				maxlength="100"
			/>
			<div class="flex gap-2 items-center mb-2">
				<input
					type="color"
					bind:value={newActivityColor}
					class="w-12 h-10 rounded cursor-pointer"
					aria-label="Choose activity color"
				/>
				<input
					type="text"
					bind:value={newActivityIcon}
					placeholder="Emoji (optional)"
					class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md
						focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
					maxlength="10"
				/>
			</div>
			<button
				type="button"
				class="w-full py-2 px-4 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md
					disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
				onclick={createActivity}
				disabled={creating || !newActivityName.trim()}
			>
				{creating ? 'Creating...' : 'Create Activity'}
			</button>
		</div>
	{/if}

	{#if loading}
		<div class="text-sm text-gray-500 dark:text-gray-400">Loading activities...</div>
	{:else if activities.length === 0}
		<div class="text-sm text-gray-500 dark:text-gray-400 italic">
			No activities yet. Click "+ Add New" to create one.
		</div>
	{:else}
		<div class="flex flex-wrap gap-2">
			{#each activities as activity (activity.id)}
				<button
					type="button"
					class="activity-chip inline-flex items-center gap-1.5 px-3 py-2 rounded-full text-sm font-medium
						transition-all border-2
						{selectedIds.includes(activity.id)
						? 'border-blue-500 bg-blue-50 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300'
						: 'border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:border-gray-400'}"
					onclick={() => toggleActivity(activity.id)}
					aria-pressed={selectedIds.includes(activity.id)}
					style={activity.color ? `border-color: ${activity.color}` : ''}
				>
					{#if activity.icon}
						<span>{activity.icon}</span>
					{/if}
					<span>{activity.name}</span>
					{#if selectedIds.includes(activity.id)}
						<span class="text-blue-600 dark:text-blue-400">✓</span>
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	.activity-chip:active {
		transform: scale(0.95);
	}
</style>
