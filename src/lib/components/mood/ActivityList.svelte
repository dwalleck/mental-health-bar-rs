<script lang="ts">
	// T113: ActivityList component - Display activities with edit/delete actions

	import { SvelteMap } from 'svelte/reactivity'
	import type { Activity, ActivityGroup } from '$lib/bindings'

	interface Props {
		activities: Activity[]
		groups: ActivityGroup[]
		onEdit: (activity: Activity) => void
		onDelete: (id: number) => void
		loading?: boolean
	}

	let { activities = [], groups = [], onEdit, onDelete, loading = false }: Props = $props()

	// Group activities by their group_id using SvelteMap for reactivity
	let groupedActivities = $derived(() => {
		const grouped = new SvelteMap<number, Activity[]>()

		// Initialize map with all groups
		groups.forEach((group) => {
			grouped.set(group.id, [])
		})

		// Group activities
		activities.forEach((activity) => {
			const existing = grouped.get(activity.group_id) || []
			grouped.set(activity.group_id, [...existing, activity])
		})

		return grouped
	})

	function confirmDelete(activity: Activity) {
		if (
			confirm(`Are you sure you want to delete "${activity.name}"? This action cannot be undone.`)
		) {
			onDelete(activity.id)
		}
	}
</script>

<div class="activity-list space-y-6">
	{#if loading}
		<div class="text-center py-8 text-gray-500 dark:text-gray-400">
			<div class="animate-pulse">Loading activities...</div>
		</div>
	{:else if activities.length === 0}
		<div class="text-center py-8 text-gray-500 dark:text-gray-400">
			<div class="text-4xl mb-2">📝</div>
			<div class="font-medium">No activities yet</div>
			<div class="text-sm mt-1">Create your first activity to start tracking</div>
		</div>
	{:else}
		{#each groups as group (group.id)}
			{@const groupActivities = groupedActivities().get(group.id) || []}
			{#if groupActivities.length > 0}
				<div class="group-section">
					<!-- Group Header -->
					<div class="flex items-center gap-3 mb-3">
						<h2 class="text-xl font-semibold text-gray-800 dark:text-gray-200">{group.name}</h2>
						<span
							class="text-sm text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded-full"
						>
							{groupActivities.length}
							{groupActivities.length === 1 ? 'activity' : 'activities'}
						</span>
					</div>

					<!-- Activities Grid -->
					<div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
						{#each groupActivities as activity (activity.id)}
							<div
								class="activity-card p-4 bg-white dark:bg-gray-800 rounded-lg shadow-xs border border-gray-200 dark:border-gray-700 hover:shadow-md transition-shadow"
								style={activity.color ? `border-left: 4px solid ${activity.color}` : ''}
							>
								<div class="flex items-start justify-between gap-2">
									<div class="flex-1 min-w-0">
										<div class="flex items-center gap-2 mb-1">
											{#if activity.icon}
												<span class="text-2xl">{activity.icon}</span>
											{/if}
											<h3 class="font-semibold text-gray-900 dark:text-white truncate">
												{activity.name}
											</h3>
										</div>
										{#if activity.color}
											<div class="flex items-center gap-2 mt-2">
												<div
													class="w-6 h-6 rounded-sm border border-gray-300 dark:border-gray-600"
													style={`background-color: ${activity.color}`}
												></div>
												<span class="text-xs font-mono text-gray-500 dark:text-gray-400">
													{activity.color}
												</span>
											</div>
										{/if}
									</div>
									<div class="flex flex-col gap-1">
										<button
											class="p-2 text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-sm transition-colors"
											onclick={() => onEdit(activity)}
											aria-label="Edit {activity.name}"
											title="Edit"
										>
											<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path
													stroke-linecap="round"
													stroke-linejoin="round"
													stroke-width="2"
													d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
												></path>
											</svg>
										</button>
										<button
											class="p-2 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/30 rounded-sm transition-colors"
											onclick={() => confirmDelete(activity)}
											aria-label="Delete {activity.name}"
											title="Delete"
										>
											<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path
													stroke-linecap="round"
													stroke-linejoin="round"
													stroke-width="2"
													d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
												></path>
											</svg>
										</button>
									</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		{/each}
	{/if}
</div>

<style>
	.activity-card {
		animation: fadeIn 0.2s ease-in-out;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(-5px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
