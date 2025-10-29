<script lang="ts">
	// T181: Reusable loading spinner with animations
	import { fade } from 'svelte/transition'

	interface Props {
		size?: 'small' | 'medium' | 'large'
		text?: string
		center?: boolean
	}

	let { size = 'medium', text = 'Loading...', center = false }: Props = $props()

	const sizeClasses = {
		small: 'w-5 h-5',
		medium: 'w-8 h-8',
		large: 'w-12 h-12',
	}

	const textSizeClasses = {
		small: 'text-sm',
		medium: 'text-base',
		large: 'text-lg',
	}
</script>

<div
	class="loading-spinner {center ? 'flex items-center justify-center' : ''}"
	transition:fade={{ duration: 150 }}
	role="status"
	aria-live="polite"
	aria-label={text}
>
	<div class="flex flex-col items-center gap-3">
		<!-- Animated spinner -->
		<div class="relative">
			<svg
				class="{sizeClasses[size]} animate-spin text-blue-600"
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				aria-hidden="true"
			>
				<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
				></circle>
				<path
					class="opacity-75"
					fill="currentColor"
					d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
				></path>
			</svg>
		</div>

		<!-- Loading text -->
		{#if text}
			<p class="text-gray-600 dark:text-gray-400 {textSizeClasses[size]} font-medium">
				{text}
			</p>
		{/if}
	</div>
</div>

<style>
	.loading-spinner {
		animation: fadeIn 0.15s ease-in;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* Ensure smooth spinning */
	:global(.animate-spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>
