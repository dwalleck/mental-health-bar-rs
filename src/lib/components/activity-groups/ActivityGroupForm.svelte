<script lang="ts">
	import { invoke } from '@tauri-apps/api/core'
	import type { ActivityGroup } from '$lib/bindings'
	import Modal from '$lib/components/ui/Modal.svelte'
	import Input from '$lib/components/ui/Input.svelte'
	import { displayError } from '$lib/utils/errors'

	// Props
	let {
		open = $bindable(false),
		group = undefined,
		onSuccess,
	}: {
		open?: boolean
		group?: ActivityGroup
		onSuccess: (group: ActivityGroup) => void
	} = $props()

	// Determine if we're editing or creating
	let isEditMode = $derived(group !== undefined)

	// Form state
	let name = $state(group?.name ?? '')
	let description = $state(group?.description ?? '')
	let isSubmitting = $state(false)
	let nameError = $state('')

	// Reset form when modal opens/closes or group changes
	// Use $effect.pre() to avoid flicker/race conditions during re-renders
	$effect.pre(() => {
		if (open) {
			name = group?.name ?? ''
			description = group?.description ?? ''
			nameError = ''
		}
	})

	// Validate name field
	function validateName(): boolean {
		if (!name.trim()) {
			nameError = 'Name is required'
			return false
		}
		if (name.length > 100) {
			nameError = 'Name must be 100 characters or less'
			return false
		}
		nameError = ''
		return true
	}

	// Handle form submission
	async function handleSubmit() {
		if (!validateName()) {
			return
		}

		try {
			isSubmitting = true

			if (isEditMode && group) {
				// Update existing group
				const result = await invoke<{ data?: ActivityGroup; error?: string }>(
					'update_activity_group',
					{
						request: {
							id: group.id,
							name: name.trim(),
							description: description.trim() || null,
						},
					}
				)

				if (result.error) {
					throw new Error(result.error)
				}

				if (result.data) {
					onSuccess(result.data)
					open = false
				}
			} else {
				// Create new group
				const result = await invoke<{ data?: ActivityGroup; error?: string }>(
					'create_activity_group',
					{
						request: {
							name: name.trim(),
							description: description.trim() || null,
						},
					}
				)

				if (result.error) {
					throw new Error(result.error)
				}

				if (result.data) {
					onSuccess(result.data)
					open = false
				}
			}
		} catch (error) {
			displayError(error)
		} finally {
			isSubmitting = false
		}
	}

	// Handle cancel
	function handleCancel() {
		open = false
		nameError = ''
	}
</script>

<Modal
	bind:open
	title={isEditMode ? 'Edit Activity Group' : 'Create Activity Group'}
	description={isEditMode
		? 'Update the name and description for this activity group.'
		: 'Create a new activity group to organize your activities.'}
	size="md"
	actions={[
		{
			label: 'Cancel',
			variant: 'secondary',
			onClick: handleCancel,
			disabled: isSubmitting,
		},
		{
			label: isEditMode ? 'Save Changes' : 'Create Group',
			variant: 'primary',
			onClick: handleSubmit,
			disabled: isSubmitting,
			loading: isSubmitting,
		},
	]}
>
	<form
		class="space-y-4"
		onsubmit={(e) => {
			e.preventDefault()
			handleSubmit()
		}}
	>
		<!-- Name field -->
		<Input
			bind:value={name}
			label="Name"
			placeholder="Enter group name"
			required
			error={nameError}
			disabled={isSubmitting}
			oninput={() => {
				if (nameError) validateName()
			}}
			onblur={validateName}
		/>

		<!-- Description field -->
		<div class="w-full">
			<label for="description" class="block text-sm font-medium text-gray-700 mb-1">
				Description
			</label>
			<textarea
				id="description"
				bind:value={description}
				placeholder="Enter a description (optional)"
				disabled={isSubmitting}
				rows="3"
				class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100 disabled:cursor-not-allowed"
			></textarea>
		</div>
	</form>
</Modal>
