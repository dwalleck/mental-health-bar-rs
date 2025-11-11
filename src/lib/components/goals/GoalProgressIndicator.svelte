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
	let progressColor = $derived(() => {
		if (progress.is_achieved) {
			return 'bg-green-600'
		} else if (progress.percentage >= 75) {
			return 'bg-blue-600'
		} else if (progress.percentage >= 50) {
			return 'bg-blue-500'
		} else if (progress.percentage >= 25) {
			return 'bg-yellow-500'
		} else {
			return 'bg-gray-400'
		}
	})

	// Cap percentage at 100 for visual display
	let displayPercentage = $derived(Math.min(progress.percentage, 100))

	// Status message
	let statusMessage = $derived(() => {
		if (progress.is_achieved) {
			return 'Goal Achieved! 🎉'
		} else if (progress.percentage >= 75) {
			return 'Almost there!'
		} else if (progress.percentage >= 50) {
			return 'Making good progress'
		} else if (progress.percentage >= 25) {
			return 'Keep going!'
		} else {
			return 'Just getting started'
		}
	})
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
			class="{progressColor()} {heightClasses[
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
				{statusMessage()}
			</span>
		</div>
	{/if}
</div>
