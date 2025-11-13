<script lang="ts">
	// T086: MoodScaleInput component - Interactive 1-7 mood rating selector with color-coded circles

	import { MOOD_COLORS, MOOD_HOVER_COLORS, MOOD_LABELS } from '$lib/utils/colors'

	interface Props {
		value: number
		onChange: (rating: number) => void
	}

	let { value = 4, onChange }: Props = $props()

	// Mood options for 1-7 scale with circle icons
	const moodOptions = [1, 2, 3, 4, 5, 6, 7].map((rating) => ({
		rating,
		label: MOOD_LABELS[rating],
		color: `${MOOD_COLORS[rating]} ${MOOD_HOVER_COLORS[rating]}`,
		textColor: 'text-white',
	}))

	function handleKeydown(event: KeyboardEvent, rating: number) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault()
			onChange(rating)
		} else if (event.key === 'ArrowLeft' && rating > 1) {
			event.preventDefault()
			onChange(rating - 1)
		} else if (event.key === 'ArrowRight' && rating < 7) {
			event.preventDefault()
			onChange(rating + 1)
		}
	}

	// Get icon SVG for mood rating
	function getMoodIcon(rating: number): string {
		if (rating === 1) return '✖' // X for Terrible
		if (rating === 2 || rating === 3) return '!' // Exclamation for Very Bad/Bad
		if (rating === 4) return '−' // Minus for Ok
		return '✓' // Check for Good/Very Good/Excellent
	}
</script>

<div class="mood-scale-input">
	<div class="mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">How are you feeling?</div>
	<div class="flex gap-2 flex-wrap justify-center">
		{#each moodOptions as option (option.rating)}
			<button
				type="button"
				class="mood-button flex flex-col items-center justify-center min-w-[70px] py-3 px-2 rounded-lg font-medium transition-all transform
					{option.color} {option.textColor}
					{value === option.rating
					? 'ring-4 ring-offset-2 ring-blue-500 dark:ring-blue-400 scale-110'
					: 'opacity-80'}
					focus:outline-hidden focus:ring-4 focus:ring-offset-2 focus:ring-blue-500 dark:ring-blue-400"
				onclick={() => onChange(option.rating)}
				onkeydown={(e) => handleKeydown(e, option.rating)}
				aria-label={`Rate your mood as ${option.label} (${option.rating} out of 7)`}
				aria-pressed={value === option.rating}
				tabindex="0"
			>
				<div
					class="w-10 h-10 mb-1 rounded-full flex items-center justify-center text-2xl font-bold border-2 border-white/30"
				>
					{getMoodIcon(option.rating)}
				</div>
				<div class="text-xs font-semibold">{option.label}</div>
			</button>
		{/each}
	</div>
</div>

<style>
	.mood-scale-input {
		user-select: none;
	}

	.mood-button:active {
		transform: scale(0.95);
	}

	@media (max-width: 640px) {
		.mood-button {
			min-width: 60px;
			padding: 0.5rem 0.75rem;
		}
	}
</style>
