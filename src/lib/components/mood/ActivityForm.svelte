<script lang="ts">
	// T112: ActivityForm component - Form for creating/editing activities with validation

	import type { Activity } from '$lib/bindings'

	interface Props {
		activity?: Activity | null
		onSubmit: (name: string, color: string, icon: string) => Promise<void>
		onCancel: () => void
	}

	let { activity = null, onSubmit, onCancel }: Props = $props()

	let name = $state(activity?.name || '')
	let color = $state(activity?.color || '#3B82F6')
	let icon = $state(activity?.icon || '')
	let isSubmitting = $state(false)
	let errors = $state<Record<string, string>>({})

	const isEditing = activity !== null

	function validateForm(): boolean {
		const newErrors: Record<string, string> = {}

		// Validate name
		const trimmedName = name.trim()
		if (!trimmedName) {
			newErrors.name = 'Activity name is required'
		} else if (trimmedName.length > 100) {
			newErrors.name = 'Activity name must be 100 characters or less'
		}

		// Validate color format (#RRGGBB)
		if (color && !/^#[0-9A-Fa-f]{6}$/.test(color)) {
			newErrors.color = 'Color must be in #RRGGBB format'
		}

		errors = newErrors
		return Object.keys(newErrors).length === 0
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault()
		if (!validateForm()) return

		try {
			isSubmitting = true
			await onSubmit(name.trim(), color, icon.trim())
		} catch (error) {
			// Error handling is done by parent component
			console.error('Form submission error:', error)
		} finally {
			isSubmitting = false
		}
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4">
	<div>
		<label
			for="activity-name"
			class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
		>
			Activity Name <span class="text-red-500">*</span>
		</label>
		<input
			id="activity-name"
			type="text"
			bind:value={name}
			placeholder="e.g., Exercise, Meditation, Reading"
			class="w-full px-3 py-2 border rounded-md
				{errors.name ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}
				focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
			maxlength="100"
			disabled={isSubmitting}
		/>
		{#if errors.name}
			<p class="mt-1 text-sm text-red-500">{errors.name}</p>
		{/if}
		<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
			{name.trim().length} / 100 characters
		</p>
	</div>

	<div>
		<label
			for="activity-color"
			class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
		>
			Color
		</label>
		<div class="flex items-center gap-3">
			<input
				id="activity-color"
				type="color"
				bind:value={color}
				class="w-16 h-10 rounded cursor-pointer border border-gray-300 dark:border-gray-600"
				disabled={isSubmitting}
			/>
			<input
				type="text"
				bind:value={color}
				placeholder="#3B82F6"
				class="flex-1 px-3 py-2 border rounded-md font-mono text-sm
					{errors.color ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}
					focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
				maxlength="7"
				disabled={isSubmitting}
			/>
		</div>
		{#if errors.color}
			<p class="mt-1 text-sm text-red-500">{errors.color}</p>
		{/if}
	</div>

	<div>
		<label
			for="activity-icon"
			class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
		>
			Icon (optional)
		</label>
		<input
			id="activity-icon"
			type="text"
			bind:value={icon}
			placeholder="🏃 (emoji)"
			class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md
				focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
			maxlength="10"
			disabled={isSubmitting}
		/>
		<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
			Enter an emoji to represent this activity
		</p>
	</div>

	<div class="flex gap-3 pt-4">
		<button
			type="submit"
			class="flex-1 py-2 px-4 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md
				disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
			disabled={isSubmitting}
		>
			{isSubmitting
				? isEditing
					? 'Updating...'
					: 'Creating...'
				: isEditing
					? 'Update Activity'
					: 'Create Activity'}
		</button>
		<button
			type="button"
			class="px-4 py-2 bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600
				text-gray-700 dark:text-gray-300 font-medium rounded-md transition-colors"
			onclick={onCancel}
			disabled={isSubmitting}
		>
			Cancel
		</button>
	</div>
</form>
