<script lang="ts">
	// T181: Skeleton loader for content loading states
	interface Props {
		type?: 'text' | 'card' | 'list' | 'chart'
		count?: number
	}

	let { type = 'text', count = 1 }: Props = $props()
</script>

<!-- eslint-disable @typescript-eslint/no-unused-vars -->
<div class="skeleton-container" role="status" aria-label="Loading content">
	{#if type === 'text'}
		{#each Array(count) as _, i (i)}
			<div class="skeleton skeleton-text"></div>
		{/each}
	{:else if type === 'card'}
		{#each Array(count) as _, i (i)}
			<div class="skeleton skeleton-card">
				<div class="skeleton skeleton-title"></div>
				<div class="skeleton skeleton-text"></div>
				<div class="skeleton skeleton-text short"></div>
			</div>
		{/each}
	{:else if type === 'list'}
		{#each Array(count) as _, i (i)}
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

<!-- eslint-enable @typescript-eslint/no-unused-vars -->

<style>
	.skeleton-container {
		animation: fadeIn 0.2s ease-in;
	}

	.skeleton {
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	@media (prefers-color-scheme: dark) {
		.skeleton {
			background: linear-gradient(90deg, #374151 25%, #4b5563 50%, #374151 75%);
			background-size: 200% 100%;
		}
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
		background: white;
		border-radius: 8px;
		margin-bottom: 12px;
		border: 1px solid #e5e7eb;
	}

	@media (prefers-color-scheme: dark) {
		.skeleton-card {
			background: #1f2937;
			border-color: #374151;
		}
	}

	.skeleton-list-item {
		display: flex;
		gap: 12px;
		align-items: center;
		padding: 12px;
		background: white;
		border-radius: 8px;
		margin-bottom: 8px;
		border: 1px solid #e5e7eb;
	}

	@media (prefers-color-scheme: dark) {
		.skeleton-list-item {
			background: #1f2937;
			border-color: #374151;
		}
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
