<script lang="ts">
	// T114: ActivityLogButton - Quick log button for each activity
	import { commands } from '$lib/bindings'
	import type { Activity } from '$lib/bindings'
	import { displayError, displaySuccess } from '$lib/utils/errors'

	interface Props {
		activity: Activity
		onLogCreated?: () => void
		size?: 'small' | 'medium' | 'large'
	}

	let { activity, onLogCreated, size = 'medium' }: Props = $props()

	let isLogging = $state(false)

	async function handleQuickLog() {
		try {
			isLogging = true

			const result = await commands.logActivity({
				activity_id: activity.id,
				logged_at: null, // null = current timestamp on backend
				notes: null,
			})

			if (result.status === 'error') {
				throw new Error(result.error.message)
			}

			displaySuccess(`Logged ${activity.name}`)
			onLogCreated?.()
		} catch (error) {
			displayError(error)
		} finally {
			isLogging = false
		}
	}

	// Size variants
	const sizeClasses = {
		small: 'px-2 py-1 text-xs',
		medium: 'px-3 py-2 text-sm',
		large: 'px-4 py-3 text-base',
	}

	const iconSizes = {
		small: 'w-3 h-3',
		medium: 'w-4 h-4',
		large: 'w-5 h-5',
	}
</script>

<button
	type="button"
	onclick={handleQuickLog}
	disabled={isLogging}
	class="inline-flex items-center gap-2 bg-green-600 hover:bg-green-700 text-white font-medium rounded-md
		disabled:opacity-50 disabled:cursor-not-allowed transition-colors {sizeClasses[size]}"
	aria-label="Log {activity.name}"
	title="Quick log {activity.name}"
>
	{#if isLogging}
		<!-- Loading spinner -->
		<svg
			class="animate-spin {iconSizes[size]}"
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
		>
			<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
			></circle>
			<path
				class="opacity-75"
				fill="currentColor"
				d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
			></path>
		</svg>
		<span>Logging...</span>
	{:else}
		<!-- Plus icon -->
		<svg class={iconSizes[size]} fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"
			></path>
		</svg>
		<span>Log Now</span>
	{/if}
</button>
