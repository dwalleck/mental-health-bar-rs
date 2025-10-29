<!--
  Combobox Component with Tailwind UI patterns
  Features:
  - Search/filter functionality
  - Keyboard navigation
  - Multi-select support
  - Custom rendering
  - Loading states
  - Empty states
  - Accessibility
-->

<script lang="ts">
	import { tick } from 'svelte'
	import { fly } from 'svelte/transition'

	interface Option {
		value: string | number
		label: string
		description?: string
		icon?: string
		disabled?: boolean
	}

	interface Props {
		options?: Option[]
		value?: string | number | (string | number)[] | null
		placeholder?: string
		searchPlaceholder?: string
		multiple?: boolean
		searchable?: boolean
		disabled?: boolean
		loading?: boolean
		emptyMessage?: string
		label?: string
		description?: string
		error?: string
		required?: boolean
	}

	let {
		options = [],
		value = $bindable(null),
		placeholder = 'Select an option',
		searchPlaceholder = 'Search...',
		multiple = false,
		searchable = true,
		disabled = false,
		loading = false,
		emptyMessage = 'No options found',
		label = '',
		description = '',
		error = '',
		required = false
	}: Props = $props()

	let isOpen = $state(false)
	let searchQuery = $state('')
	let highlightedIndex = $state(0)
	let inputElement: HTMLInputElement
	let listElement: HTMLUListElement

	// Filter options based on search
	const filteredOptions = $derived(
		searchQuery
			? options.filter((option) =>
					option.label.toLowerCase().includes(searchQuery.toLowerCase()) ||
					option.description?.toLowerCase().includes(searchQuery.toLowerCase())
				)
			: options
	)

	// Get display value
	const displayValue = $derived(() => {
		if (multiple && Array.isArray(value)) {
			const selected = options.filter((opt) => value.includes(opt.value))
			return selected.length > 0
				? `${selected.length} selected`
				: placeholder
		} else if (value !== null) {
			const selected = options.find((opt) => opt.value === value)
			return selected?.label || placeholder
		}
		return placeholder
	})

	// Check if option is selected
	function isSelected(option: Option): boolean {
		if (multiple && Array.isArray(value)) {
			return value.includes(option.value)
		}
		return value === option.value
	}

	// Toggle dropdown
	function toggle() {
		if (disabled) return
		isOpen = !isOpen
		if (isOpen) {
			searchQuery = ''
			highlightedIndex = 0
			tick().then(() => {
				inputElement?.focus()
			})
		}
	}

	// Select option
	function selectOption(option: Option) {
		if (option.disabled) return

		if (multiple && Array.isArray(value)) {
			if (isSelected(option)) {
				value = value.filter((v) => v !== option.value)
			} else {
				value = [...value, option.value]
			}
		} else {
			value = option.value
			isOpen = false
		}
	}

	// Handle keyboard navigation
	function handleKeydown(event: KeyboardEvent) {
		if (!isOpen) {
			if (event.key === 'Enter' || event.key === ' ' || event.key === 'ArrowDown') {
				event.preventDefault()
				isOpen = true
			}
			return
		}

		switch (event.key) {
			case 'Escape':
				event.preventDefault()
				isOpen = false
				break
			case 'ArrowDown':
				event.preventDefault()
				highlightedIndex = Math.min(highlightedIndex + 1, filteredOptions.length - 1)
				scrollToHighlighted()
				break
			case 'ArrowUp':
				event.preventDefault()
				highlightedIndex = Math.max(highlightedIndex - 1, 0)
				scrollToHighlighted()
				break
			case 'Enter':
				event.preventDefault()
				if (filteredOptions[highlightedIndex]) {
					selectOption(filteredOptions[highlightedIndex])
				}
				break
			case 'Tab':
				isOpen = false
				break
		}
	}

	// Scroll to highlighted option
	function scrollToHighlighted() {
		tick().then(() => {
			const highlighted = listElement?.children[highlightedIndex] as HTMLElement
			if (highlighted) {
				highlighted.scrollIntoView({ block: 'nearest' })
			}
		})
	}

	// Close on outside click
	function handleClickOutside(event: MouseEvent) {
		if (!(event.target as HTMLElement).closest('.combobox-container')) {
			isOpen = false
		}
	}

	$effect(() => {
		if (typeof window !== 'undefined') {
			if (isOpen) {
				window.addEventListener('click', handleClickOutside)
				return () => window.removeEventListener('click', handleClickOutside)
			}
		}
	})
</script>

<div class="combobox-container">
	{#if label}
		<label class="block text-sm font-medium leading-6 text-gray-900 dark:text-gray-100 mb-2">
			{label}
			{#if required}
				<span class="text-red-500">*</span>
			{/if}
		</label>
	{/if}

	{#if description}
		<p class="text-sm text-gray-500 dark:text-gray-400 mb-2">{description}</p>
	{/if}

	<div class="relative">
		<!-- Trigger button -->
		<button
			type="button"
			class="relative w-full cursor-default rounded-md bg-white dark:bg-gray-800 py-2 pl-3 pr-10 text-left text-gray-900 dark:text-gray-100 shadow-sm ring-1 ring-inset ring-gray-300 dark:ring-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:focus:ring-blue-400 sm:text-sm sm:leading-6
				{disabled ? 'opacity-50 cursor-not-allowed' : ''}
				{error ? 'ring-red-600 dark:ring-red-400' : ''}"
			onclick={toggle}
			onkeydown={handleKeydown}
			aria-haspopup="listbox"
			aria-expanded={isOpen}
			{disabled}
		>
			<span class="block truncate">{displayValue()}</span>
			<span class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
				{#if loading}
					<svg class="h-5 w-5 animate-spin text-gray-400" fill="none" viewBox="0 0 24 24">
						<circle
							class="opacity-25"
							cx="12"
							cy="12"
							r="10"
							stroke="currentColor"
							stroke-width="4"
						></circle>
						<path
							class="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
				{:else}
					<svg class="h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
						<path
							fill-rule="evenodd"
							d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z"
							clip-rule="evenodd"
						/>
					</svg>
				{/if}
			</span>
		</button>

		<!-- Dropdown -->
		{#if isOpen && !loading}
			<div
				class="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md bg-white dark:bg-gray-800 py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm"
				transition:fly={{ y: -10, duration: 150 }}
				role="listbox"
			>
				{#if searchable}
					<div class="sticky top-0 z-10 bg-white dark:bg-gray-800 px-2 py-1.5">
						<input
							bind:this={inputElement}
							bind:value={searchQuery}
							type="text"
							class="w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 dark:text-gray-100 shadow-sm ring-1 ring-inset ring-gray-300 dark:ring-gray-700 placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-blue-600 dark:focus:ring-blue-400 sm:text-sm sm:leading-6 bg-white dark:bg-gray-900"
							placeholder={searchPlaceholder}
							onkeydown={handleKeydown}
						/>
					</div>
				{/if}

				<ul bind:this={listElement} class="py-1">
					{#if filteredOptions.length === 0}
						<li class="px-3 py-2 text-sm text-gray-500 dark:text-gray-400 text-center">
							{emptyMessage}
						</li>
					{:else}
						{#each filteredOptions as option, index}
							<li
								class="relative cursor-default select-none py-2 pl-3 pr-9
									{highlightedIndex === index ? 'bg-blue-600 text-white' : 'text-gray-900 dark:text-gray-100'}
									{option.disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700'}"
								onclick={() => selectOption(option)}
								onmouseenter={() => (highlightedIndex = index)}
								onkeydown={(e) => e.key === 'Enter' && selectOption(option)}
								role="option"
								aria-selected={isSelected(option)}
							>
								<div class="flex items-center">
									{#if option.icon}
										<span class="mr-3 flex-shrink-0">{option.icon}</span>
									{/if}
									<div>
										<span class="block truncate {isSelected(option) ? 'font-semibold' : 'font-normal'}">
											{option.label}
										</span>
										{#if option.description}
											<span
												class="block truncate text-sm {highlightedIndex === index
													? 'text-blue-200'
													: 'text-gray-500 dark:text-gray-400'}"
											>
												{option.description}
											</span>
										{/if}
									</div>
								</div>

								{#if isSelected(option)}
									<span
										class="absolute inset-y-0 right-0 flex items-center pr-4 {highlightedIndex === index
											? 'text-white'
											: 'text-blue-600 dark:text-blue-400'}"
									>
										<svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
											<path
												fill-rule="evenodd"
												d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
												clip-rule="evenodd"
											/>
										</svg>
									</span>
								{/if}
							</li>
						{/each}
					{/if}
				</ul>
			</div>
		{/if}
	</div>

	{#if error}
		<p class="mt-2 text-sm text-red-600 dark:text-red-400">{error}</p>
	{/if}
</div>