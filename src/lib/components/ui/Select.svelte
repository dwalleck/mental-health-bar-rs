<script lang="ts">
	let {
		value = $bindable(''),
		options = [],
		label = undefined,
		required = false,
		disabled = false,
		placeholder = 'Select an option',
		error = undefined,
		id = `select-${Math.random().toString(36).substring(2, 11)}`,
		onchange,
	}: {
		value?: string | number
		options?: Array<{ value: string | number; label: string }>
		label?: string
		required?: boolean
		disabled?: boolean
		placeholder?: string
		error?: string
		id?: string
		onchange?: (event: Event) => void
	} = $props()
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
		{onchange}
		class="w-full px-4 py-2 border rounded-lg focus:outline-hidden focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100 disabled:cursor-not-allowed {error
			? 'border-red-500'
			: 'border-gray-300'}"
	>
		<option value="" disabled selected={value === ''}>{placeholder}</option>
		{#each options as option (option.value)}
			<option value={option.value}>{option.label}</option>
		{/each}
	</select>

	{#if error}
		<p class="mt-1 text-sm text-red-500">{error}</p>
	{/if}
</div>
