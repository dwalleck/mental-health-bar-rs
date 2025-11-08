<!--
  Enhanced Form Layout with Tailwind UI patterns
  Features:
  - Clean section dividers
  - Proper label and description formatting
  - Responsive grid layouts
  - Error state handling
  - Loading states
  - Action buttons with proper alignment
-->

<script lang="ts">
	import ErrorMessage from './ErrorMessage.svelte'
	import LoadingSpinner from './LoadingSpinner.svelte'

	interface FormSection {
		title?: string
		description?: string
	}

	interface SectionSlots {
		[key: `section_${number}`]: import('svelte').Snippet | undefined
	}

	interface Props extends SectionSlots {
		title?: string
		description?: string
		sections?: FormSection[]
		loading?: boolean
		error?: unknown
		submitLabel?: string
		cancelLabel?: string
		showCancel?: boolean
		onSubmit?: (e: Event) => void
		onCancel?: () => void
		children?: import('svelte').Snippet
	}

	let {
		title = '',
		description = '',
		sections = [],
		loading = false,
		error = undefined,
		submitLabel = 'Save',
		cancelLabel = 'Cancel',
		showCancel = true,
		onSubmit = () => {},
		onCancel = () => {},
		children,
		...slots
	}: Props = $props()
</script>

<form
	onsubmit={(e) => {
		e.preventDefault()
		onSubmit(e)
	}}
	class="space-y-8 divide-y divide-gray-200 dark:divide-gray-700"
>
	<!-- Form header -->
	{#if title || description}
		<div class="space-y-8 divide-y divide-gray-200 dark:divide-gray-700">
			<div>
				{#if title}
					<h3 class="text-base font-semibold leading-6 text-gray-900 dark:text-white">{title}</h3>
				{/if}
				{#if description}
					<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">{description}</p>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Error display -->
	{#if error}
		<div class="rounded-md bg-red-50 dark:bg-red-900/20 p-4">
			<ErrorMessage {error} />
		</div>
	{/if}

	<!-- Form sections -->
	<div class="space-y-8 divide-y divide-gray-200 dark:divide-gray-700">
		{#each sections as section, i (i)}
			{@const sectionSlot = slots[`section_${i}` as keyof typeof slots] as
				| import('svelte').Snippet
				| undefined}
			<div class="pt-8 first:pt-0">
				{#if section.title || section.description}
					<div>
						{#if section.title}
							<h3 class="text-base font-semibold leading-6 text-gray-900 dark:text-white">
								{section.title}
							</h3>
						{/if}
						{#if section.description}
							<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
								{section.description}
							</p>
						{/if}
					</div>
				{/if}

				<div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
					{@render sectionSlot?.()}
				</div>
			</div>
		{/each}

		<!-- Default slot for simple forms without sections -->
		{#if sections.length === 0}
			<div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
				{@render children?.()}
			</div>
		{/if}
	</div>

	<!-- Form actions -->
	<div class="pt-5">
		<div class="flex justify-end gap-x-3">
			{#if showCancel}
				<button
					type="button"
					class="rounded-md bg-white dark:bg-gray-800 px-3 py-2 text-sm font-semibold text-gray-900 dark:text-gray-100 shadow-xs ring-1 ring-inset ring-gray-300 dark:ring-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
					onclick={onCancel}
					disabled={loading}
				>
					{cancelLabel}
				</button>
			{/if}
			<button
				type="submit"
				class="inline-flex justify-center rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-blue-500 focus-visible:outline-solid focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-50 disabled:cursor-not-allowed"
				disabled={loading}
			>
				{#if loading}
					<LoadingSpinner size="small" class="mr-2" />
				{/if}
				{submitLabel}
			</button>
		</div>
	</div>
</form>

<style>
	@reference 'tailwindcss';

	/* Additional form field styles that can be applied to child elements */
	:global(.form-input) {
		@apply block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 dark:text-gray-100 shadow-xs ring-1 ring-inset ring-gray-300 dark:ring-gray-700 placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-blue-600 dark:focus:ring-blue-400 sm:text-sm sm:leading-6 bg-white dark:bg-gray-800;
	}

	:global(.form-label) {
		@apply block text-sm font-medium leading-6 text-gray-900 dark:text-gray-100;
	}

	:global(.form-description) {
		@apply mt-1 text-sm text-gray-500 dark:text-gray-400;
	}

	:global(.form-select) {
		@apply block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 dark:text-gray-100 ring-1 ring-inset ring-gray-300 dark:ring-gray-700 focus:ring-2 focus:ring-blue-600 dark:focus:ring-blue-400 sm:text-sm sm:leading-6 bg-white dark:bg-gray-800;
	}

	:global(.form-checkbox) {
		@apply h-4 w-4 rounded border-gray-300 dark:border-gray-700 text-blue-600 focus:ring-blue-600 dark:focus:ring-blue-400;
	}

	:global(.form-radio) {
		@apply h-4 w-4 border-gray-300 dark:border-gray-700 text-blue-600 focus:ring-blue-600 dark:focus:ring-blue-400;
	}

	:global(.form-textarea) {
		@apply block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 dark:text-gray-100 shadow-xs ring-1 ring-inset ring-gray-300 dark:ring-gray-700 placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-blue-600 dark:focus:ring-blue-400 sm:text-sm sm:leading-6 bg-white dark:bg-gray-800;
	}

	:global(.form-col-span-full) {
		@apply sm:col-span-full;
	}

	:global(.form-col-span-4) {
		@apply sm:col-span-4;
	}

	:global(.form-col-span-3) {
		@apply sm:col-span-3;
	}

	:global(.form-col-span-2) {
		@apply sm:col-span-2;
	}
</style>
