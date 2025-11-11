<script lang="ts">
	// Props
	let {
		value = $bindable(''),
		label = 'Icon',
		required = false,
		disabled = false,
		error = undefined,
		id = `icon-picker-${Math.random().toString(36).substring(2, 11)}`,
	}: {
		value?: string
		label?: string
		required?: boolean
		disabled?: boolean
		error?: string
		id?: string
	} = $props()

	// Common Heroicons (outline variant) for suggestions
	const commonIcons = [
		'academic-cap',
		'arrow-trending-up',
		'book-open',
		'calendar',
		'chat-bubble-left-right',
		'check-circle',
		'heart',
		'home',
		'light-bulb',
		'musical-note',
		'pencil',
		'puzzle-piece',
		'sparkles',
		'star',
		'sun',
		'trophy',
		'user-group',
		'face-smile',
		'fire',
		'moon',
	]

	let showSuggestions = $state(false)
	let filteredSuggestions = $derived(
		value
			? commonIcons.filter((icon) => icon.toLowerCase().includes(value.toLowerCase()))
			: commonIcons
	)

	function selectIcon(icon: string) {
		value = icon
		showSuggestions = false
	}

	function handleFocus() {
		showSuggestions = true
	}

	function handleBlur() {
		// Delay to allow click on suggestion
		setTimeout(() => {
			showSuggestions = false
		}, 200)
	}

	// Validate icon name format (lowercase with hyphens)
	function isValidIconFormat(iconName: string): boolean {
		if (!iconName) return true // Empty is valid if not required
		return /^[a-z]+(-[a-z]+)*$/.test(iconName)
	}

	let formatError = $derived(
		!isValidIconFormat(value) ? 'Icon name must be lowercase with hyphens' : ''
	)
</script>

<div class="w-full relative">
	{#if label}
		<label for={id} class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
			{label}
			{#if required}
				<span class="text-red-500">*</span>
			{/if}
		</label>
	{/if}

	<div class="relative">
		<input
			{id}
			type="text"
			bind:value
			placeholder="e.g., heart, star, academic-cap"
			{required}
			{disabled}
			onfocus={handleFocus}
			onblur={handleBlur}
			class="w-full px-4 py-2 border rounded-lg focus:outline-hidden focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100 disabled:cursor-not-allowed {error ||
			formatError
				? 'border-red-500'
				: 'border-gray-300 dark:border-gray-600'} dark:bg-gray-700 dark:text-white"
			maxlength="20"
		/>

		<!-- Icon Preview -->
		{#if value && isValidIconFormat(value)}
			<div class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400">
				<svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<!-- Generic icon preview -->
					<circle cx="12" cy="12" r="10" stroke-width="2" />
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4" />
				</svg>
			</div>
		{/if}

		<!-- Suggestions Dropdown -->
		{#if showSuggestions && filteredSuggestions.length > 0}
			<div
				class="absolute z-10 w-full mt-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg shadow-lg max-h-60 overflow-y-auto"
			>
				{#each filteredSuggestions as icon (icon)}
					<button
						type="button"
						onclick={() => selectIcon(icon)}
						class="w-full px-4 py-2 text-left hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-2 transition-colors"
					>
						<svg
							class="w-5 h-5 text-gray-400"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
						>
							<circle cx="12" cy="12" r="10" stroke-width="2" />
						</svg>
						<span class="text-sm font-mono text-gray-700 dark:text-gray-300">{icon}</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>

	{#if error}
		<p class="mt-1 text-sm text-red-500">{error}</p>
	{:else if formatError}
		<p class="mt-1 text-sm text-red-500">{formatError}</p>
	{/if}

	<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
		Enter a Heroicon name (e.g., "heart", "star") or emoji
	</p>
</div>
