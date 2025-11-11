<script lang="ts">
	// T122: GoalProgressIndicator - Visual progress bar for goal tracking
	import type { GoalProgress } from '$lib/bindings'

	interface Props {
		progress: GoalProgress
		size?: 'small' | 'medium' | 'large'
		showLabel?: boolean
	}

	let { progress, size = 'medium', showLabel = true }: Props = $props()

	// Size variants for progress bar height
	const heightClasses = {
		small: 'h-2',
		medium: 'h-4',
		large: 'h-6',
	}

	// Text size variants
	const textSizeClasses = {
		small: 'text-xs',
		medium: 'text-sm',
		large: 'text-base',
	}

	// Color based on achievement status and percentage
	let progressColor = $derived(
		progress.is_achieved
			? 'bg-green-600'
			: progress.percentage >= 75
				? 'bg-blue-600'
				: progress.percentage >= 50
					? 'bg-blue-500'
					: progress.percentage >= 25
						? 'bg-yellow-500'
						: 'bg-gray-400'
	)

	// Cap percentage at 100 for visual display
	let displayPercentage = $derived(Math.min(progress.percentage, 100))

	// Status message
	let statusMessage = $derived(
		progress.is_achieved
			? 'Goal Achieved! 🎉'
			: progress.percentage >= 75
				? 'Almost there!'
				: progress.percentage >= 50
					? 'Making good progress'
					: progress.percentage >= 25
						? 'Keep going!'
						: 'Just getting started'
	)
</script>

<div class="space-y-2">
	<!-- Progress Bar -->
	<div
		class="w-full bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden {heightClasses[size]}"
		role="progressbar"
		aria-valuenow={displayPercentage}
		aria-valuemin="0"
		aria-valuemax="100"
		aria-label="Goal progress: {progress.percentage.toFixed(1)}%"
	>
		<div
			class="{progressColor} {heightClasses[
				size
			]} rounded-full transition-all duration-500 ease-out"
			style="width: {displayPercentage}%"
		></div>
	</div>

	<!-- Label and Stats -->
	{#if showLabel}
		<div class="flex items-center justify-between gap-2 {textSizeClasses[size]}">
			<div class="flex items-center gap-2">
				<!-- Current vs Target Values -->
				<span class="font-semibold text-gray-900 dark:text-white">
					{progress.current_value.toFixed(1)} / {progress.target_value.toFixed(1)}
				</span>

				<!-- Percentage -->
				<span
					class="font-medium {progress.is_achieved
						? 'text-green-600 dark:text-green-400'
						: 'text-gray-600 dark:text-gray-400'}"
				>
					({progress.percentage.toFixed(1)}%)
				</span>
			</div>

			<!-- Status Message -->
			<span
				class="text-xs font-medium {progress.is_achieved
					? 'text-green-600 dark:text-green-400'
					: 'text-gray-500 dark:text-gray-400'}"
			>
				{statusMessage}
			</span>
		</div>
	{/if}
</div>
