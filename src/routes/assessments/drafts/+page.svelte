<script lang="ts">
	import { invokeWithRetry } from '$lib/utils/retry'
	import type { AssessmentResponse } from '$lib/bindings'
	import { displayError } from '$lib/utils/errors'
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'

	let drafts = $state<AssessmentResponse[]>([])
	let loading = $state(true)
	let error = $state('')

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
					<div class="flex justify-between items-start">
						<div class="flex-1">
							<h3 class="font-semibold text-gray-800">{draft.assessment_type.name}</h3>
							<p class="text-sm text-gray-600 mt-1">Saved: {formatDate(draft.completed_at)}</p>
							{#if draft.notes}
								<p class="text-sm text-gray-500 mt-2 italic">"{draft.notes}"</p>
							{/if}
						</div>
						<div class="text-right flex flex-col items-end gap-2">
							{@const progress = calculateProgress(draft.responses)}
							<div class="text-sm text-gray-600">
								Progress: {progress.answered}/{progress.total} questions
							</div>
							<div class="w-32 bg-gray-200 rounded-full h-2">
								<div
									class="bg-blue-600 h-2 rounded-full transition-all"
									style="width: {(progress.answered / progress.total) * 100}%"
								></div>
							</div>
							<Button
								variant="primary"
								size="small"
								onclick={() =>
									(window.location.href = `/assessments/${draft.assessment_type.code.toLowerCase()}?draft=${draft.id}`)}
							>
								Resume Draft
							</Button>
						</div>
					</div>
				</Card>
			{/each}
		</div>
	{/if}
</div>
