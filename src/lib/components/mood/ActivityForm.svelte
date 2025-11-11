<script lang="ts">
	// T112: ActivityForm component - Form for creating/editing activities with validation

	import type { Activity, ActivityGroup } from '$lib/bindings'
	import { displayError, displaySuccess } from '$lib/utils/errors'
	import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte'
	import IconPicker from '$lib/components/ui/IconPicker.svelte'

	interface Props {
		activity?: Activity | null
		groups: ActivityGroup[]
		onSubmit: (name: string, color: string, icon: string, groupId: number) => Promise<void>
		onCancel: () => void
	}

	let { activity = null, groups = [], onSubmit, onCancel }: Props = $props()

	let name = $state(activity?.name || '')
	let color = $state(activity?.color || '#3B82F6')
	let icon = $state(activity?.icon || '')
	let groupId = $state(activity?.group_id || 0)
	let isSubmitting = $state(false)
	let errors = $state<Record<string, string>>({})
	let formError = $state<unknown>(undefined)

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

		// Validate group selection (required)
		if (!groupId || groupId === 0) {
			newErrors.groupId = 'Please select an activity group'
		}

		errors = newErrors
		return Object.keys(newErrors).length === 0
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault()
		if (!validateForm()) return

		try {
			isSubmitting = true
			formError = undefined
			await onSubmit(name.trim(), color, icon.trim(), groupId)
			displaySuccess(`Activity ${isEditing ? 'updated' : 'created'} successfully!`)
		} catch (error) {
			const result = displayError(error)
			if (result.type === 'inline') {
				formError = error
			}
			console.error('Form submission error:', error)
		} finally {
			isSubmitting = false
		}
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4">
	<ErrorMessage error={formError} />

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
				focus:outline-hidden focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
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
			for="activity-group"
			class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
		>
			Activity Group <span class="text-red-500">*</span>
		</label>
		<select
			id="activity-group"
			bind:value={groupId}
			class="w-full px-3 py-2 border rounded-md
				{errors.groupId ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}
				focus:outline-hidden focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
			disabled={isSubmitting}
		>
			<option value={0} disabled>Select a group...</option>
			{#each groups as group (group.id)}
				<option value={group.id}>{group.name}</option>
			{/each}
		</select>
		{#if errors.groupId}
			<p class="mt-1 text-sm text-red-500">{errors.groupId}</p>
		{/if}
		<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
			Activities must belong to a group for organization
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
				class="w-16 h-10 rounded-sm cursor-pointer border border-gray-300 dark:border-gray-600"
				disabled={isSubmitting}
			/>
			<input
				type="text"
				bind:value={color}
				placeholder="#3B82F6"
				class="flex-1 px-3 py-2 border rounded-md font-mono text-sm
					{errors.color ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}
					focus:outline-hidden focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
				maxlength="7"
				disabled={isSubmitting}
			/>
		</div>
		{#if errors.color}
			<p class="mt-1 text-sm text-red-500">{errors.color}</p>
		{/if}
	</div>

	<IconPicker bind:value={icon} label="Icon (optional)" disabled={isSubmitting} />

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
