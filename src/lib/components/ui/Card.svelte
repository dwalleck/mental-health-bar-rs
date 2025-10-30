<script lang="ts">
	let {
		title = undefined,
		padding = 'medium',
		children,
	}: {
		title?: string
		padding?: 'small' | 'medium' | 'large'
		children?: import('svelte').Snippet
	} = $props()

	const paddingClasses = {
		small: 'p-4',
		medium: 'p-6',
		large: 'p-8',
	}
</script>

<div class="bg-white rounded-lg shadow-md {paddingClasses[padding]}">
	{#if title}
		<h2 class="text-xl font-semibold text-gray-800 mb-4">{title}</h2>
	{/if}

	{@render children?.()}
</div>
