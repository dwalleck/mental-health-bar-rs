<script lang="ts">
	import { onMount } from 'svelte'
	import { SvelteSet } from 'svelte/reactivity'
	import { commands } from '$lib/bindings'
	import type { ActivityGroup, ActivityGoal, GoalProgress } from '$lib/bindings'
	import { GOAL_TYPES } from '$lib/constants/activities'
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'
	import GoalProgressIndicator from '$lib/components/goals/GoalProgressIndicator.svelte'
	import { displayError, displaySuccess } from '$lib/utils/errors'
	import { formatSimpleDate } from '$lib/utils/date'

	// Props using Svelte 5 $props() rune
	let {
		groups,
		onEdit,
		onDelete,
		onSetGoal,
	}: {
		groups: ActivityGroup[]
		onEdit: (group: ActivityGroup) => void
		onDelete: (group: ActivityGroup) => void
		onSetGoal?: (group: ActivityGroup) => void
	} = $props()

	// State for expanded groups (for Task 3.3)
	// SvelteSet is already reactive - no need for $state wrapper
	let expandedGroupIds = new SvelteSet<number>()

	// State for goals and progress
	let groupGoals = $state<Map<number, ActivityGoal[]>>(new Map())
	let goalProgress = $state<Map<number, GoalProgress>>(new Map())

	// Helper functions for localStorage persistence of notified goals
	function loadNotifiedGoals(): SvelteSet<number> {
		try {
			const stored = localStorage.getItem('notifiedGoals')
			if (stored) {
				const array = JSON.parse(stored) as number[]
				return new SvelteSet(array)
			}
		} catch (error) {
			console.error('Failed to load notified goals from localStorage:', error)
		}
		return new SvelteSet<number>()
	}

	function saveNotifiedGoals(goals: SvelteSet<number>) {
		try {
			const array = Array.from(goals)
			localStorage.setItem('notifiedGoals', JSON.stringify(array))
		} catch (error) {
			console.error('Failed to save notified goals to localStorage:', error)
		}
	}

	// Track goals that have been notified about achievement (Task 3.23a)
	// Persisted to localStorage to prevent duplicate notifications on page refresh
	let notifiedGoals = loadNotifiedGoals()

	// Toggle group expansion
	function toggleExpand(groupId: number) {
		if (expandedGroupIds.has(groupId)) {
			expandedGroupIds.delete(groupId)
		} else {
			expandedGroupIds.add(groupId)
		}
	}

	// Load goals for a specific group
	async function loadGoalsForGroup(groupId: number) {
		try {
			const result = await commands.getActivityGoals(null, groupId)

			if (result.status === 'error') {
				throw new Error(result.error.message)
			}

			groupGoals.set(groupId, result.data)

			// Load progress for each goal in parallel
			const currentTime = new Date().toISOString()
			const progressPromises = result.data.map((goal) =>
				commands.checkGoalProgress(goal.id, currentTime)
			)
			const progressResults = await Promise.all(progressPromises)

			progressResults.forEach((progressResult, index) => {
				const goal = result.data[index]

				if (progressResult.status === 'ok') {
					const progress = progressResult.data
					goalProgress.set(goal.id, progress)

					// Task 3.23a: Show notification for newly achieved goals
					if (progress.is_achieved && !notifiedGoals.has(goal.id)) {
						const group = groups.find((g) => g.id === groupId)
						const goalType = getGoalTypeLabel(goal.goal_type)

						displaySuccess(
							`🎉 Goal Achieved! ${group?.name} - ${goalType} (${progress.percentage.toFixed(0)}%)`
						)

						notifiedGoals.add(goal.id)
						saveNotifiedGoals(notifiedGoals)
					}
				}
			})
		} catch (error) {
			displayError(error)
		}
	}

	// Load all goals on mount
	// NOTE: N+1 Query Pattern - This makes multiple API calls:
	// - getActivityGoals(null, groupId) for each group
	// - checkGoalProgress(goal.id, currentTime) for each goal
	// For 5 groups with 3 goals each = 20 API calls
	// Future optimization: Consider batch endpoint getGoalsWithProgressForGroups(groupIds[])
	onMount(async () => {
		// Load all goals in parallel (errors handled in loadGoalsForGroup)
		await Promise.all(groups.map((group) => loadGoalsForGroup(group.id)))
	})

	// Use shared date formatting utility
	function formatDate(dateString: string): string {
		return formatSimpleDate(dateString)
	}

	// Memoized goal type display names (Performance: Issue 2)
	// Prevents recalculation on every render
	const goalTypeLabels: Record<string, string> = {
		[GOAL_TYPES.DAYS_PER_PERIOD]: 'Days per Period',
		[GOAL_TYPES.PERCENT_IMPROVEMENT]: 'Percent Improvement',
	}

	function getGoalTypeLabel(goalType: string): string {
		return goalTypeLabels[goalType] || goalType
	}
</script>

<div class="space-y-4">
	{#each groups as group (group.id)}
		<Card>
			<div class="space-y-4">
				<!-- Group Header -->
				<div class="flex items-start justify-between">
					<div class="flex-1">
						<div class="flex items-center gap-2">
							<h3 class="text-lg font-semibold text-gray-800">{group.name}</h3>
							<button
								onclick={() => toggleExpand(group.id)}
								class="text-gray-500 hover:text-gray-700 transition-colors"
								aria-label={expandedGroupIds.has(group.id) ? 'Collapse group' : 'Expand group'}
							>
								<svg
									class="w-5 h-5 transition-transform {expandedGroupIds.has(group.id)
										? 'rotate-180'
										: ''}"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M19 9l-7 7-7-7"
									/>
								</svg>
							</button>
						</div>
						{#if group.description}
							<p class="text-sm text-gray-600 mt-1">{group.description}</p>
						{/if}
						<p class="text-xs text-gray-500 mt-2">
							Created {formatDate(group.created_at)}
						</p>
					</div>
					<div class="flex items-center gap-2">
						<Button variant="secondary" onclick={() => onEdit(group)}>Edit</Button>
						<Button variant="secondary" onclick={() => onDelete(group)}>Delete</Button>
					</div>
				</div>

				<!-- Active Goals Section (Task 3.23) -->
				{#if groupGoals.get(group.id)?.length}
					<div class="border-t border-gray-200 dark:border-gray-700 pt-4 mt-4">
						<div class="flex items-center justify-between mb-3">
							<h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300">Active Goals</h4>
							{#if onSetGoal}
								<button
									type="button"
									onclick={() => onSetGoal?.(group)}
									class="text-sm text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
								>
									+ Add Goal
								</button>
							{/if}
						</div>

						<div class="space-y-4">
							{#each groupGoals.get(group.id) || [] as goal (goal.id)}
								{@const progress = goalProgress.get(goal.id)}
								<div
									class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
								>
									<div class="flex items-start justify-between mb-2">
										<div>
											<div class="font-medium text-gray-900 dark:text-white">
												{getGoalTypeLabel(goal.goal_type)}
											</div>
											<div class="text-sm text-gray-600 dark:text-gray-400 mt-1">
												{#if goal.goal_type === GOAL_TYPES.DAYS_PER_PERIOD}
													Target: {goal.target_value} days every {goal.period_days} days
												{:else}
													Target: {goal.target_value}% improvement over {goal.period_days} days
												{/if}
											</div>
										</div>
										<span class="text-xs text-gray-500 dark:text-gray-400">
											Started {formatDate(goal.created_at)}
										</span>
									</div>

									{#if progress}
										<GoalProgressIndicator {progress} size="medium" />
									{:else}
										<!-- Skeleton loader for better perceived performance -->
										<div class="animate-pulse space-y-2">
											<div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
											<div class="h-2 bg-gray-200 dark:bg-gray-700 rounded w-full"></div>
										</div>
									{/if}
								</div>
							{/each}
						</div>
					</div>
				{:else if onSetGoal}
					<div class="border-t border-gray-200 dark:border-gray-700 pt-4 mt-4">
						<div class="text-center py-4">
							<div class="text-sm text-gray-600 dark:text-gray-400 mb-3">
								No goals set for this group
							</div>
							<button
								type="button"
								onclick={() => onSetGoal?.(group)}
								class="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-md transition-colors"
							>
								<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M12 4v16m8-8H4"
									/>
								</svg>
								Set Your First Goal
							</button>
						</div>
					</div>
				{/if}

				<!-- Expanded Content (placeholder for Task 3.3) -->
				{#if expandedGroupIds.has(group.id)}
					<div class="border-t border-gray-200 dark:border-gray-700 pt-4">
						<p class="text-sm text-gray-600 dark:text-gray-400">
							Activities for this group will be displayed here (Task 3.8)
						</p>
					</div>
				{/if}
			</div>
		</Card>
	{/each}
</div>
