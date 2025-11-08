<!--
  TODO: Temporary Week 0 validation page

  This page was created for Week 0 validation sprint to demonstrate
  end-to-end functionality (PHQ-9 → Database → Chart visualization).

  Consider removing this page or moving it to a /dev route after
  validation is complete, as it may clutter production navigation.
-->

<script lang="ts">
	import { onMount } from 'svelte'
	import { goto } from '$app/navigation'
	import { dev } from '$app/environment'
	import { invoke } from '@tauri-apps/api/core'
	import type { AssessmentResponse, AssessmentChartData } from '$lib/bindings'
	import AssessmentChart from '$lib/components/charts/AssessmentChart.svelte'
	import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte'
	import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte'
	import { getSeverityRanges, formatSeverity } from '$lib/utils/severity'

	// Environment guard: Redirect to home in production
	if (!dev) {
		goto('/')
	}

	// Constants
	const PHQ9_QUESTION_COUNT = 9
	const PHQ9_MAX_SCORE = 27

	// Helper function to get severity info for a score
	function getSeverityInfo(score: number) {
		const ranges = getSeverityRanges('PHQ9', PHQ9_MAX_SCORE)
		const range = ranges.find((r) => score >= r.min && score <= r.max)

		if (!range) {
			return { level: 'Unknown', colorClass: 'bg-gray-100 dark:bg-gray-900/20' }
		}

		const colorMap: Record<string, string> = {
			minimal: 'bg-green-100 dark:bg-green-900/20 text-green-800 dark:text-green-300',
			mild: 'bg-yellow-100 dark:bg-yellow-900/20 text-yellow-800 dark:text-yellow-300',
			moderate: 'bg-orange-100 dark:bg-orange-900/20 text-orange-800 dark:text-orange-300',
			moderately_severe: 'bg-red-100 dark:bg-red-900/20 text-red-800 dark:text-red-300',
			severe: 'bg-red-100 dark:bg-red-900/20 text-red-800 dark:text-red-300',
		}

		return {
			level: formatSeverity(range.level),
			colorClass: colorMap[range.level] || 'bg-gray-100 dark:bg-gray-900/20',
		}
	}

	// State
	let assessments = $state<AssessmentResponse[]>([])
	let chartData = $state<AssessmentChartData | null>(null)
	let loading = $state(true)
	let error = $state<unknown>(undefined)
	let submitting = $state(false)

	// PHQ-9 form state
	let scores = $state<number[]>(Array(PHQ9_QUESTION_COUNT).fill(0))

	const phq9Questions = [
		'Little interest or pleasure in doing things',
		'Feeling down, depressed, or hopeless',
		'Trouble falling or staying asleep, or sleeping too much',
		'Feeling tired or having little energy',
		'Poor appetite or overeating',
		'Feeling bad about yourself',
		'Trouble concentrating on things',
		'Moving or speaking slowly, or being fidgety',
		'Thoughts that you would be better off dead',
	]

	// Load assessment history and chart data
	async function loadData() {
		try {
			loading = true
			error = undefined

			// Get assessment history
			const history = await invoke<AssessmentResponse[]>('get_assessment_history', {
				assessmentType: 'PHQ9',
				limit: 10,
			})
			assessments = history

			// Get chart data
			const chart = await invoke<AssessmentChartData>('get_assessment_chart_data', {
				assessmentType: 'PHQ9',
				timeRange: 'all',
			})
			chartData = chart
		} catch (e) {
			error = e
			// TODO: Replace console logging with production logging utility
			console.error('Failed to load data:', e)
		} finally {
			loading = false
		}
	}

	// Submit assessment
	async function submitAssessment() {
		try {
			submitting = true
			error = undefined

			const totalScore = scores.reduce((sum, score) => sum + score, 0)

			await invoke('submit_assessment', {
				request: {
					assessment_type: 'PHQ9',
					responses: scores.map((score, i) => ({
						question_number: i + 1,
						value: score,
					})),
					total_score: totalScore,
					notes: 'E2E Test Submission',
				},
			})

			// Reload data after submission
			await loadData()

			// Reset form
			scores = Array(PHQ9_QUESTION_COUNT).fill(0)
		} catch (e) {
			error = e
			// TODO: Replace console logging with production logging utility
			console.error('Failed to submit assessment:', e)
		} finally {
			submitting = false
		}
	}

	onMount(() => {
		loadData()
	})
</script>

<div class="container mx-auto p-8 max-w-6xl">
	<h1 class="text-3xl font-bold mb-2">End-to-End Proof of Concept</h1>
	<p class="text-gray-600 dark:text-gray-400 mb-8">
		This page demonstrates the full stack: PHQ-9 submission → Database → Chart visualization
	</p>

	{#if error}
		<div class="mb-6">
			<ErrorMessage {error} />
		</div>
	{/if}

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
		<!-- Assessment Form -->
		<div>
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
				<h2 class="text-2xl font-semibold mb-4">Submit PHQ-9 Assessment</h2>
				<p class="text-sm text-gray-600 dark:text-gray-400 mb-6">
					Rate each item: 0 (Not at all) to 3 (Nearly every day)
				</p>

				<form
					onsubmit={(e) => {
						e.preventDefault()
						submitAssessment()
					}}
					class="space-y-4"
				>
					{#each phq9Questions as question, i (i)}
						<div class="space-y-2">
							<label class="block text-sm font-medium">
								{i + 1}. {question}
							</label>
							<div class="flex gap-2">
								{#each [0, 1, 2, 3] as value (value)}
									<button
										type="button"
										class="flex-1 px-3 py-2 rounded border-2 transition-colors
											{scores[i] === value
											? 'border-blue-600 bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300'
											: 'border-gray-300 dark:border-gray-700 hover:border-gray-400 dark:hover:border-gray-600'}"
										onclick={() => (scores[i] = value)}
									>
										{value}
									</button>
								{/each}
							</div>
						</div>
					{/each}

					<div class="pt-4 border-t border-gray-200 dark:border-gray-700">
						<p class="text-sm mb-4">
							Total Score: <strong>{scores.reduce((sum, score) => sum + score, 0)}</strong> / {PHQ9_MAX_SCORE}
						</p>
						<button
							type="submit"
							disabled={submitting}
							class="w-full inline-flex justify-center items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-xs hover:bg-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
						>
							{#if submitting}
								<LoadingSpinner size="small" class="mr-2" />
							{/if}
							Submit Assessment
						</button>
					</div>
				</form>
			</div>
		</div>

		<!-- Results -->
		<div class="space-y-6">
			<!-- Chart -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
				<h2 class="text-2xl font-semibold mb-4">Assessment History Chart</h2>
				{#if loading}
					<div class="flex justify-center py-12">
						<LoadingSpinner />
					</div>
				{:else if chartData}
					<AssessmentChart data={chartData} {loading} />
					<div class="mt-4 text-sm text-gray-600 dark:text-gray-400">
						<p>Average: {chartData.statistics.average.toFixed(1)}</p>
						<p>Total Assessments: {chartData.statistics.total_assessments}</p>
						<p>Trend: {chartData.statistics.trend}</p>
					</div>
				{:else}
					<p class="text-gray-500 dark:text-gray-400 py-12 text-center">
						No assessment data yet. Submit an assessment to get started!
					</p>
				{/if}
			</div>

			<!-- Recent Assessments -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
				<h2 class="text-2xl font-semibold mb-4">Recent Assessments</h2>
				{#if loading}
					<div class="flex justify-center py-8">
						<LoadingSpinner size="small" />
					</div>
				{:else if assessments.length > 0}
					<div class="space-y-3">
						{#each assessments as assessment (assessment.id)}
							{@const severityInfo = getSeverityInfo(assessment.total_score)}
							<div
								class="p-3 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
							>
								<div class="flex justify-between items-center">
									<div>
										<p class="font-medium">Score: {assessment.total_score} / {PHQ9_MAX_SCORE}</p>
										<p class="text-sm text-gray-500 dark:text-gray-400">
											{new Date(assessment.submitted_at).toLocaleString()}
										</p>
									</div>
									<div class="px-3 py-1 rounded text-sm font-medium {severityInfo.colorClass}">
										{severityInfo.level}
									</div>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<p class="text-gray-500 dark:text-gray-400 py-8 text-center">
						No assessments yet. Submit one to see your history!
					</p>
				{/if}
			</div>
		</div>
	</div>

	<!-- Validation Status -->
	<div
		class="mt-8 p-6 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg"
	>
		<h3 class="text-lg font-semibold text-green-900 dark:text-green-100 mb-2">
			✅ Week 0 Validation Complete
		</h3>
		<ul class="space-y-1 text-sm text-green-800 dark:text-green-200">
			<li>✅ Chart.js + Svelte 5 + Tauri integration validated</li>
			<li>✅ Tailwind v4.0.0 upgraded and working</li>
			<li>✅ Heroicons installed and rendering</li>
			<li>✅ End-to-end flow: PHQ-9 submission → Database → Chart visualization</li>
		</ul>
	</div>
</div>
