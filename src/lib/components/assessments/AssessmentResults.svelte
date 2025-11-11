<script lang="ts">
	import { goto } from '$app/navigation'
	import type { AssessmentResponse } from '$lib/bindings'
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'
	import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte'
	import { invokeWithRetry } from '$lib/utils/retry'
	import { displayError } from '$lib/utils/errors'

	let { assessmentId }: { assessmentId: number } = $props()

	let assessment = $state<AssessmentResponse | null>(null)
	let loading = $state(true)
	let loadError = $state<unknown>(undefined)

	$effect(() => {
		let isMounted = true

		async function fetchAssessment() {
			try {
				const response = await invokeWithRetry<AssessmentResponse>('get_assessment_response', {
					id: assessmentId,
				})

				if (!isMounted) return

				assessment = response
				loading = false
			} catch (e) {
				if (!isMounted) return

				const result = displayError(e)
				if (result.type === 'inline') {
					loadError = e
				}
				loading = false
			}
		}

		fetchAssessment()

		return () => {
			isMounted = false
		}
	})

	function getSeverityColor(severity: string): string {
		const colors: Record<string, string> = {
			minimal: 'text-green-600 bg-green-50',
			mild: 'text-yellow-600 bg-yellow-50',
			moderate: 'text-orange-600 bg-orange-50',
			moderately_severe: 'text-red-600 bg-red-50',
			severe: 'text-red-700 bg-red-100',
		}
		return colors[severity] || 'text-gray-600 bg-gray-50'
	}

	function formatSeverity(severity: string): string {
		return severity
			.split('_')
			.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
			.join(' ')
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString)
		return date.toLocaleString()
	}
</script>

<div class="max-w-2xl mx-auto space-y-6">
	<div class="flex justify-between items-center">
		<a href="/assessments" class="text-blue-600 hover:text-blue-800">← Back to Assessments</a>
		<a href="/assessments/history" class="text-blue-600 hover:text-blue-800">View History →</a>
	</div>

	{#if loading}
		<Card>
			<p class="text-gray-500">Loading results...</p>
		</Card>
	{:else if loadError}
		<Card>
			<ErrorMessage error={loadError} />
		</Card>
	{:else if assessment}
		<Card>
			<div class="text-center mb-6">
				<h1 class="text-3xl font-bold text-gray-800 mb-2">Assessment Complete</h1>
				<p class="text-gray-600">{assessment.assessment_type.name}</p>
			</div>

			<div class="bg-gray-50 rounded-lg p-6 mb-6">
				<div class="text-center">
					<div class="text-6xl font-bold text-gray-800 mb-2">{assessment.total_score}</div>
					<div class="text-sm text-gray-600 mb-4">
						out of {assessment.assessment_type.max_score}
					</div>
					<div
						class="inline-block px-4 py-2 rounded-lg {getSeverityColor(assessment.severity_level)}"
					>
						<span class="font-semibold">{formatSeverity(assessment.severity_level)}</span>
					</div>
				</div>
			</div>

			<div class="space-y-4">
				<div class="border-t pt-4">
					<h3 class="font-semibold text-gray-700 mb-2">Assessment Details</h3>
					<div class="text-sm text-gray-600 space-y-1">
						<p><span class="font-medium">Completed:</span> {formatDate(assessment.completed_at)}</p>
						<p>
							<span class="font-medium">Questions Answered:</span>
							{assessment.responses.length}
						</p>
					</div>
				</div>

				{#if assessment.notes}
					<div class="border-t pt-4">
						<h3 class="font-semibold text-gray-700 mb-2">Your Notes</h3>
						<p class="text-gray-600 whitespace-pre-wrap">{assessment.notes}</p>
					</div>
				{/if}

				<div class="border-t pt-4">
					<h3 class="font-semibold text-gray-700 mb-3">What This Means</h3>
					<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
						<p class="text-sm text-gray-700">
							{#if assessment.severity_level === 'minimal'}
								Your score indicates minimal symptoms. Continue monitoring your mental health.
							{:else if assessment.severity_level === 'mild'}
								Your score indicates mild symptoms. Consider self-care strategies and monitor
								changes.
							{:else if assessment.severity_level === 'moderate'}
								Your score indicates moderate symptoms. Consider speaking with a mental health
								professional.
							{:else if assessment.severity_level === 'moderately_severe' || assessment.severity_level === 'severe'}
								Your score indicates significant symptoms. We strongly recommend consulting with a
								mental health professional.
							{/if}
						</p>
						<p class="text-xs text-gray-600 mt-2">
							<strong>Disclaimer:</strong> This assessment is a screening tool, not a diagnosis. Always
							consult with a qualified mental health professional for proper evaluation and treatment.
						</p>
					</div>
				</div>
			</div>

			<div class="mt-6 flex gap-3">
				<Button variant="primary" fullWidth onclick={() => goto('/assessments')}>
					Take Another Assessment
				</Button>
				<Button variant="secondary" fullWidth onclick={() => goto('/charts')}>View Trends</Button>
			</div>
		</Card>
	{/if}
</div>
