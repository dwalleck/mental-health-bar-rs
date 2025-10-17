<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { AssessmentQuestion, SubmitAssessmentRequest } from '$lib/bindings';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	export let assessmentCode: string;

	let questions: AssessmentQuestion[] = [];
	let responses: number[] = [];
	let notes = '';
	let loading = true;
	let submitting = false;
	let error = '';

	onMount(async () => {
		try {
			questions = await invoke('get_assessment_questions', {
				assessmentTypeCode: assessmentCode
			});
			responses = new Array(questions.length).fill(-1);
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	});

	async function handleSubmit() {
		// Validate all questions answered
		if (responses.some((r) => r === -1)) {
			error = 'Please answer all questions';
			return;
		}

		submitting = true;
		error = '';

		try {
			const request: SubmitAssessmentRequest = {
				assessment_type_code: assessmentCode,
				responses: responses,
				notes: notes || null
			};

			const result = await invoke('submit_assessment', { request });

			// Navigate to results
			window.location.href = `/assessments/${assessmentCode}/result/${result.id}`;
		} catch (e) {
			error = String(e);
		} finally {
			submitting = false;
		}
	}

	function selectOption(questionIndex: number, optionIndex: number) {
		responses[questionIndex] = optionIndex;
	}

	$: progress = responses.filter((r) => r !== -1).length;
	$: progressPercent = (progress / questions.length) * 100;
</script>

<div class="max-w-3xl mx-auto space-y-6">
	<div class="flex justify-between items-center">
		<a href="/assessments" class="text-blue-600 hover:text-blue-800">← Back to Assessments</a>
		<div class="text-sm text-gray-600">
			Progress: {progress}/{questions.length} ({Math.round(progressPercent)}%)
		</div>
	</div>

	{#if loading}
		<Card>
			<p class="text-gray-500">Loading assessment...</p>
		</Card>
	{:else if error && questions.length === 0}
		<Card>
			<p class="text-red-500">Error: {error}</p>
		</Card>
	{:else}
		<div class="mb-4 bg-gray-200 rounded-full h-2">
			<div class="bg-blue-600 h-2 rounded-full transition-all" style="width: {progressPercent}%" />
		</div>

		<form on:submit|preventDefault={handleSubmit} class="space-y-6">
			{#each questions as question, i}
				<Card>
					<h3 class="font-medium text-gray-800 mb-3">
						{question.number}. {question.text}
					</h3>
					<div class="space-y-2">
						{#each question.options as option, optionIndex}
							<button
								type="button"
								class="w-full text-left px-4 py-3 border rounded-lg transition-colors {responses[i] ===
								optionIndex
									? 'border-blue-600 bg-blue-50'
									: 'border-gray-300 hover:border-gray-400'}"
								on:click={() => selectOption(i, optionIndex)}
							>
								<div class="flex items-center">
									<div
										class="w-5 h-5 rounded-full border-2 mr-3 flex items-center justify-center {responses[
											i
										] === optionIndex
											? 'border-blue-600 bg-blue-600'
											: 'border-gray-400'}"
									>
										{#if responses[i] === optionIndex}
											<div class="w-2 h-2 bg-white rounded-full" />
										{/if}
									</div>
									<span class="text-gray-700">{option}</span>
								</div>
							</button>
						{/each}
					</div>
				</Card>
			{/each}

			<Card title="Additional Notes (Optional)">
				<textarea
					bind:value={notes}
					class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
					rows="4"
					placeholder="Any additional thoughts or context..."
				/>
			</Card>

			{#if error && questions.length > 0}
				<div class="bg-red-50 border border-red-200 rounded-lg p-4">
					<p class="text-red-600">{error}</p>
				</div>
			{/if}

			<Button type="submit" variant="primary" fullWidth disabled={submitting || progress < questions.length}>
				{submitting ? 'Submitting...' : 'Submit Assessment'}
			</Button>
		</form>
	{/if}
</div>
