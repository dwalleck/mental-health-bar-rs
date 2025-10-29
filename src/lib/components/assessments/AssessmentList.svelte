<script lang="ts">
	import { goto } from '$app/navigation'
	import { invoke } from '@tauri-apps/api/core'
	import type { AssessmentType } from '$lib/bindings'
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'

	let assessmentTypes = $state<AssessmentType[]>([])
	let loading = $state(true)
	let error = $state('')

	$effect(() => {
		let isMounted = true

		async function fetchAssessmentTypes() {
			try {
				const types = await invoke<AssessmentType[]>('get_assessment_types')

				if (!isMounted) return

				assessmentTypes = types
				loading = false
			} catch (e) {
				if (!isMounted) return

				error = String(e)
				loading = false
			}
		}

		fetchAssessmentTypes()

		return () => {
			isMounted = false
		}
	})

	function startAssessment(code: string) {
		goto(`/assessments/${code}`)
	}
</script>

<div class="space-y-4">
	<h1 class="text-3xl font-bold text-gray-800">Mental Health Assessments</h1>
	<p class="text-gray-600">
		Choose an assessment to track your mental health over time. All data is stored locally and
		privately.
	</p>

	{#if loading}
		<p class="text-gray-500">Loading assessments...</p>
	{:else if error}
		<Card>
			<p class="text-red-500">Error: {error}</p>
		</Card>
	{:else}
		<div class="grid gap-4 md:grid-cols-2">
			{#each assessmentTypes as assessment (assessment.code)}
				<Card>
					<h2 class="text-xl font-semibold text-gray-800 mb-2">{assessment.name}</h2>
					<p class="text-gray-600 mb-4">
						{assessment.description || 'Mental health assessment'}
					</p>
					<div class="flex justify-between items-center text-sm text-gray-500 mb-4">
						<span>{assessment.question_count} questions</span>
						<span>Score: {assessment.min_score}-{assessment.max_score}</span>
					</div>
					<Button variant="primary" fullWidth onclick={() => startAssessment(assessment.code)}>
						Take Assessment
					</Button>
				</Card>
			{/each}
		</div>
	{/if}
</div>
