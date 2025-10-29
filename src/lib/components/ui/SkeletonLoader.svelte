<script lang="ts">
	// T181: Skeleton loader for content loading states
	interface Props {
		type?: 'text' | 'card' | 'list' | 'chart'
		count?: number
	}

	let { type = 'text', count = 1 }: Props = $props()
</script>

<div class="skeleton-container" role="status" aria-label="Loading content">
	{#if type === 'text'}
		{#each Array.from({ length: count }, (_, i) => i) as i (i)}
			<div class="skeleton skeleton-text"></div>
		{/each}
	{:else if type === 'card'}
		{#each Array.from({ length: count }, (_, i) => i) as i (i)}
			<div class="skeleton skeleton-card">
				<div class="skeleton skeleton-title"></div>
				<div class="skeleton skeleton-text"></div>
				<div class="skeleton skeleton-text short"></div>
			</div>
		{/each}
	{:else if type === 'list'}
		{#each Array.from({ length: count }, (_, i) => i) as i (i)}
			<div class="skeleton skeleton-list-item">
				<div class="skeleton skeleton-circle"></div>
				<div class="flex-1">
					<div class="skeleton skeleton-text"></div>
					<div class="skeleton skeleton-text short"></div>
				</div>
			</div>
		{/each}
	{:else if type === 'chart'}
		<div class="skeleton skeleton-chart"></div>
	{/if}
</div>

<style>
	/* CSS custom properties for theming */
	:global(:root) {
		--skeleton-base: #f0f0f0;
		--skeleton-highlight: #e0e0e0;
		--skeleton-card-bg: white;
		--skeleton-card-border: #e5e7eb;
	}

	:global(:root.dark) {
		--skeleton-base: #374151;
		--skeleton-highlight: #4b5563;
		--skeleton-card-bg: #1f2937;
		--skeleton-card-border: #374151;
	}

	.skeleton-container {
		animation: fadeIn 0.2s ease-in;
	}

	.skeleton {
		background: linear-gradient(
			90deg,
			var(--skeleton-base) 25%,
			var(--skeleton-highlight) 50%,
			var(--skeleton-base) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.skeleton-text {
		height: 16px;
		margin-bottom: 8px;
	}

	.skeleton-text.short {
		width: 60%;
	}

	.skeleton-title {
		height: 24px;
		width: 50%;
		margin-bottom: 12px;
	}

	.skeleton-card {
		padding: 16px;
		background: var(--skeleton-card-bg);
		border-radius: 8px;
		margin-bottom: 12px;
		border: 1px solid var(--skeleton-card-border);
	}

	.skeleton-list-item {
		display: flex;
		gap: 12px;
		align-items: center;
		padding: 12px;
		background: var(--skeleton-card-bg);
		border-radius: 8px;
		margin-bottom: 8px;
		border: 1px solid var(--skeleton-card-border);
	}

	.skeleton-circle {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.skeleton-chart {
		height: 300px;
		width: 100%;
		border-radius: 8px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
</style>
