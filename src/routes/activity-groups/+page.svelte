<script lang="ts">
	import { onMount } from 'svelte'
	import { goto } from '$app/navigation'
	import { invoke } from '@tauri-apps/api/core'
	import { invokeWithRetry } from '$lib/utils/retry'
	import { displayError } from '$lib/utils/errors'
	import type { ActivityGroup } from '$lib/bindings'
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'
	import Modal from '$lib/components/ui/Modal.svelte'
	import ActivityGroupList from '$lib/components/activity-groups/ActivityGroupList.svelte'
	import ActivityGroupForm from '$lib/components/activity-groups/ActivityGroupForm.svelte'

	// Reactive state for activity groups
	let activityGroups = $state<ActivityGroup[]>([])
	let isLoading = $state(true)
	let showAddModal = $state(false)
	let showEditModal = $state(false)
	let showDeleteModal = $state(false)
	let selectedGroup = $state<ActivityGroup | undefined>(undefined)
	let groupToDelete = $state<ActivityGroup | undefined>(undefined)
	let isDeleting = $state(false)

	// Load activity groups on mount
	onMount(async () => {
		try {
			isLoading = true
			activityGroups = await invokeWithRetry('get_activity_groups')
		} catch (e) {
			displayError(e)
		} finally {
			isLoading = false
		}
	})

	// Handle successful group creation
	function handleGroupCreated(newGroup: ActivityGroup) {
		activityGroups = [...activityGroups, newGroup]
	}

	// Handle successful group update
	function handleGroupUpdated(updatedGroup: ActivityGroup) {
		activityGroups = activityGroups.map((g) => (g.id === updatedGroup.id ? updatedGroup : g))
	}

	// Event handlers for ActivityGroupList
	function handleEdit(group: ActivityGroup) {
		selectedGroup = group
		showEditModal = true
	}

	function handleDelete(group: ActivityGroup) {
		groupToDelete = group
		showDeleteModal = true
	}

	// Confirm and execute delete
	async function confirmDelete() {
		if (!groupToDelete) return

		try {
			isDeleting = true

			const result = await invoke<{ data?: null; error?: string }>('delete_activity_group', {
				groupId: groupToDelete.id,
			})

			if (result.error) {
				throw new Error(result.error)
			}

			// Remove deleted group from list
			activityGroups = activityGroups.filter((g) => g.id !== groupToDelete.id)
			showDeleteModal = false
			groupToDelete = undefined
		} catch (error) {
			displayError(error)
		} finally {
			isDeleting = false
		}
	}
</script>

<div class="space-y-6">
	<!-- Page Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-4xl font-bold text-gray-800 mb-2">Activity Groups</h1>
			<p class="text-lg text-gray-600">
				Organize your activities into groups for better tracking and insights.
			</p>
		</div>
		<Button variant="primary" onclick={() => (showAddModal = true)}>Add Group</Button>
	</div>

	<!-- Activity Groups List -->
	{#if isLoading}
		<Card>
			<div class="text-center py-8 text-gray-600">Loading activity groups...</div>
		</Card>
	{:else if activityGroups.length === 0}
		<Card>
			<div class="text-center py-12">
				<div class="text-gray-400 mb-4">
					<svg class="mx-auto h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
						/>
					</svg>
				</div>
				<h3 class="text-lg font-medium text-gray-900 mb-2">No Activity Groups Yet</h3>
				<p class="text-gray-600 mb-6">
					Get started by creating your first activity group to organize your activities.
				</p>
				<Button variant="primary" onclick={() => (showAddModal = true)}
					>Create Your First Group</Button
				>
			</div>
		</Card>
	{:else}
		<ActivityGroupList groups={activityGroups} onEdit={handleEdit} onDelete={handleDelete} />
	{/if}

	<!-- Back to Dashboard -->
	<div class="flex justify-start">
		<Button variant="secondary" onclick={() => goto('/')}>Back to Dashboard</Button>
	</div>
</div>

<!-- Add Group Modal -->
<ActivityGroupForm bind:open={showAddModal} onSuccess={handleGroupCreated} />

<!-- Edit Group Modal -->
<ActivityGroupForm bind:open={showEditModal} group={selectedGroup} onSuccess={handleGroupUpdated} />

<!-- Delete Confirmation Modal -->
<Modal
	bind:open={showDeleteModal}
	title="Delete Activity Group"
	description="Are you sure you want to delete '{groupToDelete?.name}'? This action cannot be undone."
	size="md"
	actions={[
		{
			label: 'Cancel',
			variant: 'secondary',
			onClick: () => {
				showDeleteModal = false
				groupToDelete = undefined
			},
			disabled: isDeleting,
		},
		{
			label: 'Delete',
			variant: 'danger',
			onClick: confirmDelete,
			disabled: isDeleting,
			loading: isDeleting,
		},
	]}
>
	{#snippet icon()}
		<svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
			/>
		</svg>
	{/snippet}

	<div class="mt-2">
		<p class="text-sm text-gray-600">
			<strong>Warning:</strong> All activities associated with this group will also be deleted due to
			CASCADE delete constraints.
		</p>
	</div>
</Modal>
