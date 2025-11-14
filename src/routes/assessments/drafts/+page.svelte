<script lang="ts">
	import { invokeWithRetry } from '$lib/utils/retry'
	import type { AssessmentResponse } from '$lib/bindings'
	import { displayError, displaySuccess } from '$lib/utils/errors'
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'

	let drafts = $state<AssessmentResponse[]>([])
	let loading = $state(true)
	let error = $state('')
	let deletingId = $state<number | null>(null)

	// Load draft assessments on mount
	$effect(() => {
		async function loadDrafts() {
			try {
				drafts = await invokeWithRetry('get_draft_assessments', {})
			} catch (e) {
				const result = displayError(e)
				if (result.type === 'inline') {
					error = result.message || 'Failed to load drafts'
				}
			} finally {
				loading = false
			}
		}
		loadDrafts()
	})

	function formatDate(dateString: string): string {
		const date = new Date(dateString)
		return date.toLocaleDateString() + ' ' + date.toLocaleTimeString()
	}

	function calculateProgress(responses: number[]): { answered: number; total: number } {
		const answered = responses.filter((r) => r !== -1).length
		return { answered, total: responses.length }
	}

	async function handleDeleteDraft(draftId: number) {
		if (deletingId !== null) return // Prevent multiple deletions

		if (!confirm('Are you sure you want to delete this draft? This cannot be undone.')) {
			return
		}

		deletingId = draftId

		try {
			await invokeWithRetry('delete_assessment', { id: draftId })
			displaySuccess('Draft deleted successfully')
			// Remove from list
			drafts = drafts.filter((d) => d.id !== draftId)
		} catch (e) {
			const result = displayError(e)
			if (result.type === 'inline') {
				error = result.message || 'Failed to delete draft'
			}
		} finally {
			deletingId = null
		}
	}
</script>

<div class="space-y-4">
	<div class="flex justify-between items-center">
		<h1 class="text-3xl font-bold text-gray-800">Draft Assessments</h1>
		<a href="/assessments" class="text-blue-600 hover:text-blue-800">← Back to Assessments</a>
	</div>

	{#if loading}
		<Card>
			<p class="text-gray-500">Loading drafts...</p>
		</Card>
	{:else if error}
		<Card>
			<p class="text-red-500">Error: {error}</p>
		</Card>
	{:else if drafts.length === 0}
		<Card>
			<p class="text-gray-600">
				No draft assessments found. Start an assessment and save it as a draft to continue later.
			</p>
		</Card>
	{:else}
		<div class="space-y-3">
			{#each drafts as draft (draft.id)}
				<Card padding="medium">
					{@const progress = calculateProgress(draft.responses)}
					<div class="flex justify-between items-start">
						<div class="flex-1">
							<h3 class="font-semibold text-gray-800">{draft.assessment_type.name}</h3>
							<p class="text-sm text-gray-600 mt-1">Saved: {formatDate(draft.completed_at)}</p>
							{#if draft.notes}
								<p class="text-sm text-gray-500 mt-2 italic">"{draft.notes}"</p>
							{/if}
						</div>
						<div class="text-right flex flex-col items-end gap-2">
							<div class="text-sm text-gray-600">
								Progress: {progress.answered}/{progress.total} questions
							</div>
							<div class="w-32 bg-gray-200 rounded-full h-2">
								<div
									class="bg-blue-600 h-2 rounded-full transition-all"
									style="width: {(progress.answered / progress.total) * 100}%"
								></div>
							</div>
							<div class="flex gap-2">
								<Button
									variant="secondary"
									disabled={deletingId === draft.id}
									onclick={() => handleDeleteDraft(draft.id)}
								>
									{deletingId === draft.id ? 'Deleting...' : 'Delete'}
								</Button>
								<Button
									variant="primary"
									disabled={deletingId === draft.id}
									onclick={() =>
										(window.location.href = `/assessments/${draft.assessment_type.code.toLowerCase()}?draft=${draft.id}`)}
								>
									Resume Draft
								</Button>
							</div>
						</div>
					</div>
				</Card>
			{/each}
		</div>
	{/if}
</div>
