<script lang="ts">
	let {
		type = 'text',
		value = $bindable(''),
		placeholder = '',
		label = undefined,
		required = false,
		disabled = false,
		error = undefined,
		id = `input-${Math.random().toString(36).substring(2, 11)}`,
		oninput,
		onchange,
		onblur,
	}: {
		type?: 'text' | 'number' | 'email' | 'password' | 'date' | 'time'
		value?: string | number
		placeholder?: string
		label?: string
		required?: boolean
		disabled?: boolean
		error?: string
		id?: string
		oninput?: (event: Event) => void
		onchange?: (event: Event) => void
		onblur?: (event: FocusEvent) => void
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

	<input
		{id}
		{type}
		{placeholder}
		{required}
		{disabled}
		bind:value
		{oninput}
		{onchange}
		{onblur}
		class="w-full px-4 py-2 border rounded-lg focus:outline-hidden focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100 disabled:cursor-not-allowed {error
			? 'border-red-500'
			: 'border-gray-300'}"
	/>

	{#if error}
		<p class="mt-1 text-sm text-red-500">{error}</p>
	{/if}
</div>
