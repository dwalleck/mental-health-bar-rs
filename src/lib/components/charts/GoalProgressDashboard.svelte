<script lang="ts">
	// T4.3: Goal Progress Dashboard - All active goals with progress bars
	import { onMount } from 'svelte'
	import { commands } from '$lib/bindings'
	import type { ActivityGoal, GoalProgress, Activity, ActivityGroup } from '$lib/bindings'
	import { GOAL_TYPES } from '$lib/constants/activities'
	import GoalProgressIndicator from '$lib/components/goals/GoalProgressIndicator.svelte'
	import SkeletonLoader from '$lib/components/ui/SkeletonLoader.svelte'
	import { displayError } from '$lib/utils/errors'

	// Props
	interface Props {
		activities?: Activity[] // Optional pre-loaded activities
		groups?: ActivityGroup[] // Optional pre-loaded groups
	}

	let { activities = [], groups = [] }: Props = $props()

	// State
	let goals = $state<ActivityGoal[]>([])
	let goalsProgress = $state<Map<number, GoalProgress>>(new Map())
	let activitiesMap = $state<Map<number, Activity>>(new Map())
	let groupsMap = $state<Map<number, ActivityGroup>>(new Map())
	let loading = $state(true)
	let error = $state('')

	// Separate goals by type
	const activityGoals = $derived(goals.filter((g) => g.activity_id !== null))
	const groupGoals = $derived(goals.filter((g) => g.group_id !== null))

	// Load data on mount
	onMount(async () => {
		await loadData()
	})

	async function loadData() {
		try {
			loading = true
			error = ''

			// Load all goals (null, null gets all goals)
			const goalsResult = await commands.getActivityGoals(null, null)
			if (goalsResult.status === 'error') {
				throw new Error(goalsResult.error.message)
			}
			goals = goalsResult.data

			// Load activities if not provided
			if (activities.length === 0) {
				const activitiesResult = await commands.getActivities(false)
				if (activitiesResult.status === 'ok') {
					activities = activitiesResult.data
				}
			}

			// Load groups if not provided
			if (groups.length === 0) {
				const groupsResult = await commands.getActivityGroups()
				if (groupsResult.status === 'ok') {
					groups = groupsResult.data
				}
			}

			// Create maps for quick lookup
			activitiesMap = new Map(activities.map((a) => [a.id, a]))
			groupsMap = new Map(groups.map((g) => [g.id, g]))

			// Load progress for each goal
			const currentTime = new Date().toISOString()
			const progressPromises = goals.map((goal) => commands.checkGoalProgress(goal.id, currentTime))
			const progressResults = await Promise.all(progressPromises)

			// Store progress in map
			progressResults.forEach((result, index) => {
				if (result.status === 'ok') {
					goalsProgress.set(goals[index].id, result.data)
				}
			})
		} catch (err) {
			displayError(err)
			error = err instanceof Error ? err.message : 'Failed to load goal progress'
		} finally {
			loading = false
		}
	}

	// Helper functions
	function getGoalTypeLabel(goalType: string): string {
		switch (goalType) {
			case GOAL_TYPES.DAYS_PER_PERIOD:
				return 'Days per Period'
			case GOAL_TYPES.PERCENT_IMPROVEMENT:
				return 'Percent Improvement'
			default:
				return goalType
		}
	}

	function getGoalDescription(goal: ActivityGoal): string {
		if (goal.goal_type === GOAL_TYPES.DAYS_PER_PERIOD) {
			return `${goal.target_value} days every ${goal.period_days} days`
		} else {
			return `${goal.target_value}% improvement over ${goal.period_days} days`
		}
	}
</script>

<div class="goal-progress-dashboard bg-white rounded-lg shadow-sm border border-gray-200 p-6">
	<!-- Header -->
	<div class="mb-6">
		<h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">Goal Progress</h2>
		<p class="text-sm text-gray-600 dark:text-gray-400">
			Track your progress toward all active goals
		</p>
	</div>

	{#if loading}
		<div class="space-y-4">
			<SkeletonLoader type="card" count={3} />
		</div>
	{:else if error}
		<div class="bg-red-50 border border-red-200 text-red-800 rounded-lg p-4">
			<p class="font-semibold">Error</p>
			<p>{error}</p>
		</div>
	{:else if goals.length === 0}
		<!-- Empty State -->
		<div class="text-center py-12">
			<div class="text-4xl mb-2">🎯</div>
			<p class="text-gray-600 dark:text-gray-400 mb-2">No active goals</p>
			<p class="text-sm text-gray-500 dark:text-gray-500">
				Set goals for your activities or groups to track progress
			</p>
		</div>
	{:else}
		<!-- Activity Goals Section -->
		{#if activityGoals.length > 0}
			<div class="mb-8">
				<h3
					class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4 flex items-center gap-2"
				>
					<span>🎯</span>
					Activity Goals
					<span class="text-sm font-normal text-gray-500 dark:text-gray-400">
						({activityGoals.length})
					</span>
				</h3>

				<div class="space-y-4">
					{#each activityGoals as goal (goal.id)}
						{@const activity = activitiesMap.get(goal.activity_id || 0)}
						{@const progress = goalsProgress.get(goal.id)}
						{#if activity}
							<div
								class="p-4 rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800"
							>
								<div class="flex items-start justify-between mb-3">
									<div>
										<div class="flex items-center gap-2 mb-1">
											{#if activity.icon}
												<span class="text-xl">{activity.icon}</span>
											{/if}
											<h4 class="font-semibold text-gray-900 dark:text-white">{activity.name}</h4>
										</div>
										<p class="text-sm text-gray-600 dark:text-gray-400">
											{getGoalTypeLabel(goal.goal_type)} - {getGoalDescription(goal)}
										</p>
									</div>
									{#if progress?.is_achieved}
										<span class="text-2xl">🎉</span>
									{/if}
								</div>

								{#if progress}
									<GoalProgressIndicator {progress} size="medium" />
								{:else}
									<div class="text-sm text-gray-500 dark:text-gray-400">Loading progress...</div>
								{/if}
							</div>
						{/if}
					{/each}
				</div>
			</div>
		{/if}

		<!-- Group Goals Section -->
		{#if groupGoals.length > 0}
			<div>
				<h3
					class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4 flex items-center gap-2"
				>
					<span>📊</span>
					Group Goals
					<span class="text-sm font-normal text-gray-500 dark:text-gray-400">
						({groupGoals.length})
					</span>
				</h3>

				<div class="space-y-4">
					{#each groupGoals as goal (goal.id)}
						{@const group = groupsMap.get(goal.group_id || 0)}
						{@const progress = goalsProgress.get(goal.id)}
						{#if group}
							<div
								class="p-4 rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800"
							>
								<div class="flex items-start justify-between mb-3">
									<div>
										<h4 class="font-semibold text-gray-900 dark:text-white mb-1">{group.name}</h4>
										<p class="text-sm text-gray-600 dark:text-gray-400">
											{getGoalTypeLabel(goal.goal_type)} - {getGoalDescription(goal)}
										</p>
									</div>
									{#if progress?.is_achieved}
										<span class="text-2xl">🎉</span>
									{/if}
								</div>

								{#if progress}
									<GoalProgressIndicator {progress} size="medium" />
								{:else}
									<div class="text-sm text-gray-500 dark:text-gray-400">Loading progress...</div>
								{/if}
							</div>
						{/if}
					{/each}
				</div>
			</div>
		{/if}

		<!-- Summary Stats -->
		{#if goals.length > 0}
			{@const achievedGoals = Array.from(goalsProgress.values()).filter(
				(p) => p.is_achieved
			).length}
			{@const totalGoals = goals.length}
			{@const achievementRate = totalGoals > 0 ? (achievedGoals / totalGoals) * 100 : 0}

			<div class="mt-8 pt-6 border-t border-gray-200 dark:border-gray-700">
				<div class="grid grid-cols-3 gap-4">
					<div class="text-center">
						<p class="text-3xl font-bold text-blue-600 dark:text-blue-400">{totalGoals}</p>
						<p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Total Goals</p>
					</div>
					<div class="text-center">
						<p class="text-3xl font-bold text-green-600 dark:text-green-400">{achievedGoals}</p>
						<p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Achieved</p>
					</div>
					<div class="text-center">
						<p class="text-3xl font-bold text-purple-600 dark:text-purple-400">
							{achievementRate.toFixed(0)}%
						</p>
						<p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Success Rate</p>
					</div>
				</div>
			</div>
		{/if}
	{/if}
</div>
