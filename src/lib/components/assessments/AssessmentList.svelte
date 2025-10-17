<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { AssessmentType } from '$lib/bindings';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	let assessmentTypes: AssessmentType[] = [];
	let loading = true;
	let error = '';

	onMount(async () => {
		try {
			assessmentTypes = await invoke('get_assessment_types');
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	});

	function startAssessment(code: string) {
		window.location.href = `/assessments/${code}`;
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
			{#each assessmentTypes as assessment}
				<Card>
					<h2 class="text-xl font-semibold text-gray-800 mb-2">{assessment.name}</h2>
					<p class="text-gray-600 mb-4">
						{assessment.description || 'Mental health assessment'}
					</p>
					<div class="flex justify-between items-center text-sm text-gray-500 mb-4">
						<span>{assessment.question_count} questions</span>
						<span>Score: {assessment.min_score}-{assessment.max_score}</span>
					</div>
					<Button variant="primary" fullWidth on:click={() => startAssessment(assessment.code)}>
						Take Assessment
					</Button>
				</Card>
			{/each}
		</div>
	{/if}
</div>
