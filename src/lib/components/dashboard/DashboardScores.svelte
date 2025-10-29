<script lang="ts">
	import { goto } from '$app/navigation'
	import { invoke } from '@tauri-apps/api/core'
	import { SvelteMap } from 'svelte/reactivity'
	import type { AssessmentResponse } from '$lib/bindings'
	import { formatSeverity } from '$lib/utils/severity'
	import AssessmentScoreBar from './AssessmentScoreBar.svelte'
	import SkeletonLoader from '$lib/components/ui/SkeletonLoader.svelte'

	// Assessment type codes to fetch (UPPERCASE to match database)
	const ASSESSMENT_TYPES = ['PHQ9', 'GAD7', 'CESD', 'OASIS'] as const

	// Assessment type metadata (display names)
	const ASSESSMENT_METADATA: Record<string, { name: string }> = {
		PHQ9: { name: 'PHQ-9 (Depression)' },
		GAD7: { name: 'GAD-7 (Anxiety)' },
		CESD: { name: 'CES-D (Depression)' },
		OASIS: { name: 'OASIS (Anxiety)' },
	}

	// State management with Svelte 5 runes
	let assessments: SvelteMap<string, AssessmentResponse | null> = new SvelteMap()
	let loading = $state(true)
	let error = $state<string | null>(null)
	let failedAssessments = $state<string[]>([])

	// T213, T221: Fetch latest assessments for all types on mount using Svelte 5 $effect
	$effect(() => {
		async function fetchAssessments() {
			try {
				loading = true
				error = null
				failedAssessments = []

				// T221: Use Promise.all for parallel loading of all assessment types
				const results = await Promise.all(
					ASSESSMENT_TYPES.map(async (code) => {
						try {
							const result = await invoke<AssessmentResponse | null>('get_latest_assessment', {
								assessmentTypeCode: code,
							})
							return { code, data: result, failed: false }
						} catch (err) {
							// T225: Handle individual assessment failures gracefully
							console.error(`Failed to fetch assessment ${code}:`, err)
							return { code, data: null, failed: true }
						}
					})
				)

				// Populate the assessments map and track failures
				const newAssessments = new SvelteMap<string, AssessmentResponse | null>()
				const failed: string[] = []
				results.forEach(({ code, data, failed: hasFailed }) => {
					newAssessments.set(code, data)
					if (hasFailed) {
						failed.push(ASSESSMENT_METADATA[code].name)
					}
				})
				assessments = newAssessments
				failedAssessments = failed
			} catch (err) {
				console.error('Failed to fetch assessments:', err)
				error = 'Failed to load assessment data. Please try again.'
			} finally {
				loading = false
			}
		}

		fetchAssessments()
	})

	// T224: Navigate to chart view for a specific assessment type
	function navigateToChart(assessmentType: string) {
		// Convert to lowercase for URL (charts page handles toUpperCase conversion)
		goto(`/charts?type=${assessmentType.toLowerCase()}`)
	}
</script>

<div class="dashboard-scores">
	<!-- T225: Error handling with user-friendly messages -->
	{#if error}
		<div class="error-message bg-red-50 border border-red-200 rounded-lg p-4 mb-4">
			<p class="text-red-800 text-sm">{error}</p>
			<button
				class="mt-2 text-sm text-red-600 underline hover:text-red-800"
				onclick={() => window.location.reload()}
			>
				Retry
			</button>
		</div>
	{/if}

	<!-- Partial error warning for individual assessment failures -->
	{#if !error && failedAssessments.length > 0}
		<div
			class="error-message bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4 mb-4"
		>
			<p class="text-yellow-800 dark:text-yellow-200 text-sm font-medium">
				Some assessments could not be loaded
			</p>
			<p class="text-yellow-700 dark:text-yellow-300 text-sm mt-1">
				Failed to load: {failedAssessments.join(', ')}
			</p>
			<button
				class="mt-2 text-sm text-yellow-600 dark:text-yellow-400 underline hover:text-yellow-800 dark:hover:text-yellow-200"
				onclick={() => window.location.reload()}
			>
				Retry
			</button>
		</div>
	{/if}

	<!-- T220: Grid layout for all 4 assessment types -->
	<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
		{#each ASSESSMENT_TYPES as assessmentType (assessmentType)}
			<div class="assessment-item">
				{#if loading}
					<!-- T222: Show skeleton loader during fetch -->
					<SkeletonLoader type="card" count={1} />
				{:else}
					{@const assessment = assessments.get(assessmentType)}
					{#if assessment}
						<!-- Assessment has data - render clickable score bar -->
						<button
							type="button"
							class="w-full text-left p-4 bg-white rounded-lg border border-gray-200 hover:border-gray-300 hover:shadow-md transition-all cursor-pointer focus:outline-none focus:ring-2 focus:ring-blue-500"
							onclick={() => navigateToChart(assessmentType)}
							aria-label={`View ${ASSESSMENT_METADATA[assessmentType].name} chart`}
						>
							<div class="space-y-1">
								<AssessmentScoreBar
									assessmentType={assessment.assessment_type}
									score={assessment.total_score}
									severityLevel={assessment.severity_level}
									showLabels={false}
									showScore={false}
								/>
								<!-- Display score and severity -->
								<div
									class="text-xs text-center text-gray-600"
									data-testid="assessment-summary-{assessmentType.toLowerCase()}"
								>
									Score: {assessment.total_score} |
									<span data-severity>{formatSeverity(assessment.severity_level)}</span>
								</div>
							</div>
						</button>
					{:else}
						<!-- T223: "Not taken yet" empty state (not clickable) -->
						<div
							class="w-full p-4 bg-white rounded-lg border border-gray-200"
							aria-label={`${ASSESSMENT_METADATA[assessmentType].name} - Not taken yet`}
						>
							<div class="space-y-2">
								<h3 class="text-sm font-medium text-gray-700">
									{ASSESSMENT_METADATA[assessmentType].name}
								</h3>
								<p class="text-sm text-gray-500">Not taken yet</p>
							</div>
						</div>
					{/if}
				{/if}
			</div>
		{/each}
	</div>
</div>

<style>
	.dashboard-scores {
		width: 100%;
	}

	.error-message {
		animation: fadeIn 0.3s ease-in;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Ensure score bars have proper spacing */
	:global(.assessment-score-bar) {
		width: 100%;
	}
</style>
