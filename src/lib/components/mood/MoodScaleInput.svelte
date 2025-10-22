<script lang="ts">
	// T086: MoodScaleInput component - Interactive 1-5 mood rating selector with color coding

	interface Props {
		value: number
		onchange: (rating: number) => void
	}

	let { value = 3, onchange }: Props = $props()

	const moodOptions = [
		{ rating: 1, label: 'Very Bad', color: 'bg-red-500 hover:bg-red-600', textColor: 'text-white' },
		{
			rating: 2,
			label: 'Bad',
			color: 'bg-orange-500 hover:bg-orange-600',
			textColor: 'text-white',
		},
		{
			rating: 3,
			label: 'Neutral',
			color: 'bg-yellow-500 hover:bg-yellow-600',
			textColor: 'text-white',
		},
		{ rating: 4, label: 'Good', color: 'bg-lime-500 hover:bg-lime-600', textColor: 'text-white' },
		{
			rating: 5,
			label: 'Very Good',
			color: 'bg-green-500 hover:bg-green-600',
			textColor: 'text-white',
		},
	]

	function handleKeydown(event: KeyboardEvent, rating: number) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault()
			onchange(rating)
		}
	}
</script>

<div class="mood-scale-input">
	<div class="mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">How are you feeling?</div>
	<div class="flex gap-2 flex-wrap">
		{#each moodOptions as option (option.rating)}
			<button
				type="button"
				class="mood-button flex-1 min-w-[80px] py-3 px-4 rounded-lg font-medium transition-all transform
					{option.color} {option.textColor}
					{value === option.rating ? 'ring-4 ring-offset-2 ring-blue-500 scale-105' : 'opacity-75'}
					focus:outline-none focus:ring-4 focus:ring-offset-2 focus:ring-blue-500"
				onclick={() => onchange(option.rating)}
				onkeydown={(e) => handleKeydown(e, option.rating)}
				aria-label={`Rate your mood as ${option.label} (${option.rating} out of 5)`}
				aria-pressed={value === option.rating}
			>
				<div class="text-2xl">{option.rating}</div>
				<div class="text-xs mt-1">{option.label}</div>
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
