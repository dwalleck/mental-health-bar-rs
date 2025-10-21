<script lang="ts">
	export let value: string | number = '';
	export let options: Array<{ value: string | number; label: string }> = [];
	export let label: string | undefined = undefined;
	export let required = false;
	export let disabled = false;
	export let placeholder = 'Select an option';
	export let error: string | undefined = undefined;
	export let id: string = `select-${Math.random().toString(36).substring(2, 11)}`;
</script>

<div class="w-full">
	{#if label}
		<label for={id} class="block text-sm font-medium text-gray-700 mb-1">
			{label}
			{#if required}
				<span class="text-red-500">*</span>
			{/if}
		</label>
	{/if}

	<select
		{id}
		{required}
		{disabled}
		bind:value
		class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100 disabled:cursor-not-allowed {error
			? 'border-red-500'
			: 'border-gray-300'}"
		on:change
	>
		<option value="" disabled selected={value === ''}>{placeholder}</option>
		{#each options as option}
			<option value={option.value}>{option.label}</option>
		{/each}
	</select>

	{#if error}
		<p class="mt-1 text-sm text-red-500">{error}</p>
	{/if}
</div>
