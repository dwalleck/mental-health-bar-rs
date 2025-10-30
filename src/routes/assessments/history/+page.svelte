<script lang="ts">
	import { invokeWithRetry } from '$lib/utils/retry'
	import type { AssessmentResponse } from '$lib/bindings'
	import { formatSeverity } from '$lib/utils/severity'
	import { displayError } from '$lib/utils/errors'
	import Card from '$lib/components/ui/Card.svelte'

	let history = $state<AssessmentResponse[]>([])
	let loading = $state(true)
	let error = $state('')

	// Load assessment history on mount
	$effect(() => {
		async function loadHistory() {
			try {
				history = await invokeWithRetry('get_assessment_history', {
					assessmentTypeCode: null,
					fromDate: null,
					toDate: null,
					limit: null,
				})
			} catch (e) {
				const result = displayError(e)
				if (result.type === 'inline') {
					error = result.message || 'Failed to load history'
				}
			} finally {
				loading = false
			}
		}
		loadHistory()
	})

	function formatDate(dateString: string): string {
		const date = new Date(dateString)
		return date.toLocaleDateString() + ' ' + date.toLocaleTimeString()
	}

	function getSeverityColor(severity: string): string {
		const colors: Record<string, string> = {
			minimal: 'bg-green-100 text-green-800',
			mild: 'bg-yellow-100 text-yellow-800',
			moderate: 'bg-orange-100 text-orange-800',
			moderately_severe: 'bg-red-100 text-red-800',
			severe: 'bg-red-200 text-red-900',
		}
		return colors[severity] || 'bg-gray-100 text-gray-800'
	}
</script>

<div class="space-y-4">
	<div class="flex justify-between items-center">
		<h1 class="text-3xl font-bold text-gray-800">Assessment History</h1>
		<a href="/assessments" class="text-blue-600 hover:text-blue-800">Take New Assessment →</a>
	</div>

	{#if loading}
		<Card>
			<p class="text-gray-500">Loading history...</p>
		</Card>
	{:else if error}
		<Card>
			<p class="text-red-500">Error: {error}</p>
		</Card>
	{:else if history.length === 0}
		<Card>
			<p class="text-gray-600">
				No assessment history yet. Take your first assessment to get started!
			</p>
		</Card>
	{:else}
		<div class="space-y-3">
			{#each history as item (item.id)}
				<a
					href="/assessments/{item.assessment_type.code.toLowerCase()}/result/{item.id}"
					class="block hover:shadow-lg transition-shadow"
				>
					<Card padding="medium">
						<div class="flex justify-between items-start">
							<div class="flex-1">
								<h3 class="font-semibold text-gray-800">{item.assessment_type.name}</h3>
								<p class="text-sm text-gray-600 mt-1">{formatDate(item.completed_at)}</p>
							</div>
							<div class="text-right">
								<div class="text-2xl font-bold text-gray-800">{item.total_score}</div>
								<div class="text-xs px-2 py-1 rounded {getSeverityColor(item.severity_level)} mt-1">
									{formatSeverity(item.severity_level)}
								</div>
							</div>
						</div>
					</Card>
				</a>
			{/each}
		</div>
	{/if}
</div>
