<script lang="ts">
	import { goto } from '$app/navigation'
	import { invokeWithRetry } from '$lib/utils/retry'
	import { displayError } from '$lib/utils/errors'
	import type { ActivityGroup } from '$lib/bindings'
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'

	// Reactive state for activity groups
	let activityGroups = $state<ActivityGroup[]>([])
	let isLoading = $state(true)

	// Load activity groups on mount
	$effect(() => {
		async function loadActivityGroups() {
			try {
				isLoading = true
				activityGroups = await invokeWithRetry('get_activity_groups')
			} catch (e) {
				displayError(e)
			} finally {
				isLoading = false
			}
		}
		loadActivityGroups()
	})
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
		<Button variant="primary" onclick={() => {}}>Add Group</Button>
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
				<Button variant="primary" onclick={() => {}}>Create Your First Group</Button>
			</div>
		</Card>
	{:else}
		<div class="space-y-4">
			{#each activityGroups as group (group.id)}
				<Card>
					<div class="flex items-center justify-between">
						<div>
							<h3 class="text-lg font-semibold text-gray-800">{group.name}</h3>
							{#if group.description}
								<p class="text-sm text-gray-600 mt-1">{group.description}</p>
							{/if}
							<p class="text-xs text-gray-500 mt-2">
								Created {new Date(group.created_at).toLocaleDateString()}
							</p>
						</div>
						<div class="flex items-center gap-2">
							<Button variant="outline" size="sm" onclick={() => {}}>Edit</Button>
							<Button variant="outline" size="sm" onclick={() => {}}>Delete</Button>
						</div>
					</div>
				</Card>
			{/each}
		</div>
	{/if}

	<!-- Back to Dashboard -->
	<div class="flex justify-start">
		<Button variant="outline" onclick={() => goto('/')}>Back to Dashboard</Button>
	</div>
</div>
