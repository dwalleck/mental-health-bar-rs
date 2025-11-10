<script lang="ts">
	import { SvelteSet } from 'svelte/reactivity'
	import type { ActivityGroup } from '$lib/bindings'
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'

	// Props using Svelte 5 $props() rune
	let {
		groups,
		onEdit,
		onDelete,
	}: {
		groups: ActivityGroup[]
		onEdit: (group: ActivityGroup) => void
		onDelete: (group: ActivityGroup) => void
	} = $props()

	// State for expanded groups (for Task 3.3)
	// SvelteSet is already reactive - no need for $state wrapper
	let expandedGroupIds = new SvelteSet<number>()

	// Toggle group expansion
	function toggleExpand(groupId: number) {
		if (expandedGroupIds.has(groupId)) {
			expandedGroupIds.delete(groupId)
		} else {
			expandedGroupIds.add(groupId)
		}
	}

	// Format date helper with consistent locale and format
	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
		})
	}
</script>

<div class="space-y-4">
	{#each groups as group (group.id)}
		<Card>
			<div class="space-y-4">
				<!-- Group Header -->
				<div class="flex items-start justify-between">
					<div class="flex-1">
						<div class="flex items-center gap-2">
							<h3 class="text-lg font-semibold text-gray-800">{group.name}</h3>
							<button
								onclick={() => toggleExpand(group.id)}
								class="text-gray-500 hover:text-gray-700 transition-colors"
								aria-label={expandedGroupIds.has(group.id) ? 'Collapse group' : 'Expand group'}
							>
								<svg
									class="w-5 h-5 transition-transform {expandedGroupIds.has(group.id)
										? 'rotate-180'
										: ''}"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M19 9l-7 7-7-7"
									/>
								</svg>
							</button>
						</div>
						{#if group.description}
							<p class="text-sm text-gray-600 mt-1">{group.description}</p>
						{/if}
						<p class="text-xs text-gray-500 mt-2">
							Created {formatDate(group.created_at)}
						</p>
					</div>
					<div class="flex items-center gap-2">
						<Button variant="secondary" onclick={() => onEdit(group)}>Edit</Button>
						<Button variant="secondary" onclick={() => onDelete(group)}>Delete</Button>
					</div>
				</div>

				<!-- Expanded Content (placeholder for Task 3.3) -->
				{#if expandedGroupIds.has(group.id)}
					<div class="border-t border-gray-200 pt-4">
						<p class="text-sm text-gray-600">
							Activities for this group will be displayed here (Task 3.8)
						</p>
					</div>
				{/if}
			</div>
		</Card>
	{/each}
</div>
