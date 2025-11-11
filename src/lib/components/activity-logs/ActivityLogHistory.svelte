<script lang="ts">
	// T115: ActivityLogHistory - Timeline view with notes, date filtering, and add note feature
	import { onMount } from 'svelte'
	import { commands } from '$lib/bindings'
	import type { ActivityLog, Activity } from '$lib/bindings'
	import { displayError, displaySuccess } from '$lib/utils/errors'
	import Card from '$lib/components/ui/Card.svelte'

	interface Props {
		activity?: Activity
		activityId?: number
	}

	let { activity, activityId }: Props = $props()

	// Derive activity ID from either prop
	let resolvedActivityId = $derived(activity?.id ?? activityId ?? null)

	// State
	let logs = $state<ActivityLog[]>([])
	let loading = $state(true)
	let startDate = $state<string>('')
	let endDate = $state<string>('')
	let editingNoteId = $state<number | null>(null)
	let editingNoteText = $state('')
	let savingNote = $state(false)

	// Format date for display
	function formatLogDate(isoString: string): string {
		const date = new Date(isoString)
		const now = new Date()
		const diffMs = now.getTime() - date.getTime()
		const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

		if (diffDays === 0) {
			return `Today at ${date.toLocaleTimeString('en-US', {
				hour: 'numeric',
				minute: '2-digit',
			})}`
		} else if (diffDays === 1) {
			return `Yesterday at ${date.toLocaleTimeString('en-US', {
				hour: 'numeric',
				minute: '2-digit',
			})}`
		} else if (diffDays < 7) {
			return `${diffDays} days ago`
		} else {
			return date.toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'short',
				day: 'numeric',
				hour: 'numeric',
				minute: '2-digit',
			})
		}
	}

	async function loadLogs() {
		if (!resolvedActivityId) return

		try {
			loading = true

			const result = await commands.getActivityLogs(
				resolvedActivityId,
				startDate || null,
				endDate || null
			)

			if (result.status === 'error') {
				throw new Error(result.error.message)
			}

			logs = result.data
		} catch (error) {
			displayError(error)
		} finally {
			loading = false
		}
	}

	function handleEditNote(log: ActivityLog) {
		editingNoteId = log.id
		editingNoteText = log.notes || ''
	}

	function handleCancelEdit() {
		editingNoteId = null
		editingNoteText = ''
	}

	async function handleSaveNote(logId: number) {
		// Note: We need an update_activity_log command for this
		// For now, just show success and clear editing state
		try {
			savingNote = true
			// TODO: Implement update_activity_log command in backend
			// const result = await commands.updateActivityLog(logId, { notes: editingNoteText })
			console.log('Would update log', logId, 'with note:', editingNoteText)

			displaySuccess('Note saved (frontend only - backend update needed)')
			editingNoteId = null
			editingNoteText = ''
			await loadLogs()
		} catch (error) {
			displayError(error)
		} finally {
			savingNote = false
		}
	}

	function handleDateFilterChange() {
		loadLogs()
	}

	function clearDateFilter() {
		startDate = ''
		endDate = ''
		loadLogs()
	}

	onMount(() => {
		loadLogs()
	})
</script>

<div class="space-y-4">
	<!-- Date Filter Controls -->
	<Card>
		<div class="flex flex-wrap items-end gap-4">
			<div class="flex-1 min-w-[200px]">
				<label
					for="start-date"
					class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
				>
					From
				</label>
				<input
					id="start-date"
					type="date"
					bind:value={startDate}
					onchange={handleDateFilterChange}
					class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md
						focus:outline-hidden focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
				/>
			</div>

			<div class="flex-1 min-w-[200px]">
				<label
					for="end-date"
					class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
				>
					To
				</label>
				<input
					id="end-date"
					type="date"
					bind:value={endDate}
					onchange={handleDateFilterChange}
					class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md
						focus:outline-hidden focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
				/>
			</div>

			<button
				type="button"
				onclick={clearDateFilter}
				class="px-4 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200
					border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-50 dark:hover:bg-gray-700
					transition-colors"
			>
				Clear Filter
			</button>
		</div>
	</Card>

	<!-- Activity Logs Timeline -->
	{#if loading}
		<Card>
			<div class="text-center py-8 text-gray-500 dark:text-gray-400">
				<div class="animate-pulse">Loading activity logs...</div>
			</div>
		</Card>
	{:else if logs.length === 0}
		<Card>
			<div class="text-center py-12">
				<div class="text-4xl mb-2">📋</div>
				<div class="font-medium text-gray-900 dark:text-white">No activity logs yet</div>
				<div class="text-sm text-gray-600 dark:text-gray-400 mt-1">
					{#if startDate || endDate}
						No logs found in the selected date range
					{:else}
						Use the "Log Now" button to start tracking this activity
					{/if}
				</div>
			</div>
		</Card>
	{:else}
		<Card>
			<div class="space-y-6">
				<!-- Timeline -->
				<div class="relative">
					<!-- Vertical line -->
					<div
						class="absolute left-4 top-0 bottom-0 w-0.5 bg-gray-200 dark:bg-gray-700"
						aria-hidden="true"
					></div>

					<!-- Log entries -->
					<div class="space-y-6">
						{#each logs as log (log.id)}
							<div class="relative flex gap-4">
								<!-- Timeline dot -->
								<div
									class="flex-shrink-0 w-8 h-8 rounded-full bg-green-600 flex items-center justify-center z-10"
								>
									<svg class="w-4 h-4 text-white" fill="currentColor" viewBox="0 0 20 20">
										<path
											fill-rule="evenodd"
											d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
											clip-rule="evenodd"
										/>
									</svg>
								</div>

								<!-- Log content -->
								<div
									class="flex-1 bg-gray-50 dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
								>
									<div class="flex items-start justify-between gap-2 mb-2">
										<div>
											<div class="font-medium text-gray-900 dark:text-white">
												{activity?.name || 'Activity logged'}
											</div>
											<div class="text-sm text-gray-500 dark:text-gray-400">
												{formatLogDate(log.logged_at)}
											</div>
										</div>

										<!-- Edit note button -->
										<button
											type="button"
											onclick={() => handleEditNote(log)}
											class="text-sm text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
											aria-label="Edit note"
										>
											{log.notes ? 'Edit note' : 'Add note'}
										</button>
									</div>

									<!-- Note display/editor -->
									{#if editingNoteId === log.id}
										<div class="space-y-2">
											<textarea
												bind:value={editingNoteText}
												maxlength="500"
												rows="3"
												placeholder="Add a note (optional, max 500 characters)"
												class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md
													focus:outline-hidden focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white
													text-sm"
											></textarea>
											<div class="flex items-center justify-between">
												<span class="text-xs text-gray-500 dark:text-gray-400">
													{editingNoteText.length} / 500 characters
												</span>
												<div class="flex gap-2">
													<button
														type="button"
														onclick={handleCancelEdit}
														disabled={savingNote}
														class="px-3 py-1 text-sm text-gray-600 dark:text-gray-400
															hover:text-gray-900 dark:hover:text-gray-200 disabled:opacity-50"
													>
														Cancel
													</button>
													<button
														type="button"
														onclick={() => handleSaveNote(log.id)}
														disabled={savingNote}
														class="px-3 py-1 text-sm bg-blue-600 hover:bg-blue-700 text-white rounded-md
															disabled:opacity-50 disabled:cursor-not-allowed"
													>
														{savingNote ? 'Saving...' : 'Save'}
													</button>
												</div>
											</div>
										</div>
									{:else if log.notes}
										<div
											class="text-sm text-gray-700 dark:text-gray-300 mt-2 p-2 bg-white dark:bg-gray-900 rounded border border-gray-200 dark:border-gray-700"
										>
											{log.notes}
										</div>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				</div>
			</div>
		</Card>
	{/if}
</div>
