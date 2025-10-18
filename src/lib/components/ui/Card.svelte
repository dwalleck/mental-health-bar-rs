<script lang="ts">
	export let title: string | undefined = undefined;
	export let padding: 'small' | 'medium' | 'large' = 'medium';

	const paddingClasses = {
		small: 'p-4',
		medium: 'p-6',
		large: 'p-8'
	};
</script>

<div class="bg-white rounded-lg shadow-md {paddingClasses[padding]}">
	{#if title}
		<h2 class="text-xl font-semibold text-gray-800 mb-4">{title}</h2>
	{/if}

	<slot />
</div>
