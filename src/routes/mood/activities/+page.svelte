<script lang="ts">
	// T114: /mood/activities route - Manage custom activities (CRUD operations)

	import { onMount } from 'svelte'
	import { invokeWithRetry } from '$lib/utils/retry'
	import Card from '$lib/components/ui/Card.svelte'
	import ActivityForm from '$lib/components/mood/ActivityForm.svelte'
	import ActivityList from '$lib/components/mood/ActivityList.svelte'
	import { displayError, displaySuccess } from '$lib/utils/errors'
	import type { Activity, ActivityGroup } from '$lib/bindings'

	let activities: Activity[] = $state([])
	let activityGroups: ActivityGroup[] = $state([])
	let loading = $state(true)
	let error = $state<string | null>(null)
	let showForm = $state(false)
	let editingActivity = $state<Activity | null>(null)

	// Load activities and groups on mount
	onMount(async () => {
		await Promise.all([loadActivities(), loadActivityGroups()])
	})

	async function loadActivities() {
		try {
			loading = true
			error = null
			activities = await invokeWithRetry('get_activities', { includeDeleted: false })
		} catch (e) {
			const result = displayError(e)
			if (result.type === 'inline') {
				error = result.message || 'Failed to load activities'
			}
		} finally {
			loading = false
		}
	}

	async function loadActivityGroups() {
		try {
			activityGroups = await invokeWithRetry('get_activity_groups')
		} catch (e) {
			const result = displayError(e)
			if (result.type === 'inline') {
				error = result.message || 'Failed to load activity groups'
			}
		}
	}

	function handleCreateNew() {
		editingActivity = null
		showForm = true
	}

	function handleEdit(activity: Activity) {
		editingActivity = activity
		showForm = true
	}

	function handleCancel() {
		showForm = false
		editingActivity = null
	}

	async function handleSubmit(name: string, color: string, icon: string, groupId: number) {
		try {
			error = null

			if (editingActivity) {
				// Update existing activity
				await invokeWithRetry('update_activity', {
					id: editingActivity.id,
					request: {
						name: name || null,
						color: color || null,
						icon: icon || null,
						group_id: groupId,
					},
				})
			} else {
				// Create new activity
				await invokeWithRetry('create_activity', {
					request: {
						name,
						color: color || null,
						icon: icon || null,
						group_id: groupId,
					},
				})
			}

			displaySuccess('Activity saved successfully!')
			await loadActivities()
			handleCancel()
		} catch (e) {
			const result = displayError(e)
			if (result.type === 'inline') {
				error = result.message || 'Failed to save activity'
			}
			throw e // Re-throw so form can handle it
		}
	}

	async function handleDelete(id: number) {
		try {
			error = null
			await invokeWithRetry('delete_activity', { id })
			displaySuccess('Activity deleted successfully!')
			await loadActivities()
		} catch (e) {
			const result = displayError(e)
			if (result.type === 'inline') {
				error = result.message || 'Failed to delete activity'
			}
		}
	}
</script>

<svelte:head>
	<title>Manage Activities - Mental Health Tracker</title>
</svelte:head>

<div class="max-w-6xl mx-auto">
	<div class="mb-6">
		<h1 class="text-3xl font-bold text-gray-900 dark:text-white">Manage Activities</h1>
		<p class="text-gray-600 dark:text-gray-400 mt-1">
			Create and organize custom activities to track with your mood check-ins.
		</p>
	</div>

	{#if error}
		<div class="mb-4 p-4 bg-red-100 border border-red-300 text-red-700 rounded-lg">
			<div class="font-semibold">Error</div>
			<div class="text-sm mt-1">{error}</div>
		</div>
	{/if}

	{#if activityGroups.length === 0 && !loading}
		<div class="mb-4 p-4 bg-yellow-100 border border-yellow-300 text-yellow-800 rounded-lg">
			<div class="font-semibold">No Activity Groups</div>
			<div class="text-sm mt-1">
				You need to create at least one activity group before you can add activities.
				<a
					href="/activity-groups"
					class="underline hover:no-underline font-medium"
					data-sveltekit-preload-data="hover"
				>
					Create a group now →
				</a>
			</div>
		</div>
	{/if}

	{#if showForm}
		<Card title={editingActivity ? 'Edit Activity' : 'Create New Activity'}>
			<ActivityForm
				activity={editingActivity}
				groups={activityGroups}
				onSubmit={handleSubmit}
				onCancel={handleCancel}
			/>
		</Card>
	{:else}
		<div class="mb-4">
			<button
				class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg shadow-xs transition-colors inline-flex items-center gap-2"
				onclick={handleCreateNew}
			>
				<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"
					></path>
				</svg>
				Create New Activity
			</button>
		</div>

		<Card title="Your Activities">
			<ActivityList {activities} {loading} onEdit={handleEdit} onDelete={handleDelete} />
		</Card>
	{/if}

	{#if !showForm}
		<div class="mt-6">
			<a
				href="/mood"
				data-sveltekit-preload-data="hover"
				class="inline-block px-6 py-3 bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-300 font-semibold rounded-lg transition-colors"
			>
				← Back to Mood Check-In
			</a>
		</div>
	{/if}
</div>
