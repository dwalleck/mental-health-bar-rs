<script lang="ts">
	import { goto } from '$app/navigation'
	import { invokeWithRetry } from '$lib/utils/retry'
	import { displayError } from '$lib/utils/errors'
	import type { AssessmentType, AssessmentResponse } from '$lib/bindings'
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'
	import DashboardScores from '$lib/components/dashboard/DashboardScores.svelte'

	let assessmentTypes = $state<AssessmentType[]>([])
	let recentCount = $state(0)

	// Load dashboard data on mount
	$effect(() => {
		async function loadDashboardData() {
			try {
				assessmentTypes = await invokeWithRetry('get_assessment_types')
				const history = await invokeWithRetry<AssessmentResponse[]>('get_assessment_history', {
					assessmentTypeCode: null,
					fromDate: null,
					toDate: null,
					limit: 10,
				})
				recentCount = history.length
			} catch (e) {
				displayError(e)
			}
		}
		loadDashboardData()
	})
</script>

<div class="space-y-6">
	<div>
		<h1 class="text-4xl font-bold text-gray-800 mb-2">Welcome to Mental Health Tracker</h1>
		<p class="text-lg text-gray-600">
			Track your mental health journey with evidence-based assessments and visualizations.
		</p>
	</div>

	<div class="grid gap-4 md:grid-cols-3">
		<Card>
			<div class="text-center">
				<div class="text-4xl font-bold text-blue-600">{assessmentTypes.length}</div>
				<div class="text-sm text-gray-600 mt-1">Available Assessments</div>
			</div>
		</Card>

		<Card>
			<div class="text-center">
				<div class="text-4xl font-bold text-green-600">{recentCount}</div>
				<div class="text-sm text-gray-600 mt-1">Completed Assessments</div>
			</div>
		</Card>

		<Card>
			<div class="text-center">
				<div class="text-4xl font-bold text-purple-600">100%</div>
				<div class="text-sm text-gray-600 mt-1">Privacy (Local Only)</div>
			</div>
		</Card>
	</div>

	<!-- Latest Assessment Scores Section -->
	<Card title="Latest Assessment Scores">
		<div class="text-sm text-gray-600 mb-4">
			Click on any assessment to view detailed charts and history
		</div>
		<DashboardScores />
	</Card>

	<div class="grid gap-6 md:grid-cols-2">
		<Card title="Quick Start">
			<div class="space-y-3">
				<p class="text-gray-600">Get started with mental health tracking:</p>
				<ol class="list-decimal list-inside space-y-2 text-gray-700">
					<li>Choose an assessment (PHQ-9, GAD-7, CES-D, or OASIS)</li>
					<li>Answer all questions honestly</li>
					<li>Review your score and severity level</li>
					<li>Track your progress over time with charts</li>
				</ol>
				<Button variant="primary" fullWidth onclick={() => goto('/assessments')}>
					Take Your First Assessment
				</Button>
			</div>
		</Card>

		<Card title="Available Assessments">
			<div class="space-y-3">
				{#each assessmentTypes as assessment (assessment.id)}
					<div class="border-l-4 border-blue-500 pl-3">
						<div class="font-semibold text-gray-800">{assessment.code}</div>
						<div class="text-sm text-gray-600">{assessment.name}</div>
					</div>
				{/each}
			</div>
		</Card>
	</div>

	<Card>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
			<h3 class="font-semibold text-blue-800 mb-2">Privacy & Data Security</h3>
			<p class="text-sm text-blue-700">
				All your data is stored <strong>locally on your device</strong>. No information is sent to
				external servers. Your mental health data remains completely private and under your control.
			</p>
		</div>
	</Card>

	<Card>
		<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
			<h3 class="font-semibold text-yellow-800 mb-2">Important Disclaimer</h3>
			<p class="text-sm text-yellow-700">
				These assessments are <strong>screening tools</strong>, not diagnostic instruments. They
				help monitor your mental health over time but cannot replace professional evaluation. If
				you're experiencing significant distress, please consult with a qualified mental health
				professional.
			</p>
		</div>
	</Card>
</div>
