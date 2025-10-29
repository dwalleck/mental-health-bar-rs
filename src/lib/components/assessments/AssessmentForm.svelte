<script lang="ts">
	import { goto } from '$app/navigation'
	import { invoke } from '@tauri-apps/api/core'
	import type {
		AssessmentQuestion,
		AssessmentResponse,
		SubmitAssessmentRequest,
	} from '$lib/bindings'
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'

	let { assessmentCode }: { assessmentCode: string } = $props()

	let questions = $state<AssessmentQuestion[]>([])
	let responses = $state<number[]>([])
	let notes = $state('')
	let loading = $state(true)
	let submitting = $state(false)
	let error = $state('')

	$effect(() => {
		let isMounted = true

		async function fetchQuestions() {
			try {
				const fetchedQuestions = await invoke<AssessmentQuestion[]>('get_assessment_questions', {
					assessmentTypeCode: assessmentCode,
				})

				if (!isMounted) return

				questions = fetchedQuestions
				responses = new Array(fetchedQuestions.length).fill(-1)
				loading = false
			} catch (e) {
				if (!isMounted) return

				error = String(e)
				loading = false
			}
		}

		fetchQuestions()

		return () => {
			isMounted = false
		}
	})

	async function handleSubmit(event: Event) {
		event.preventDefault()

		// Prevent double-submission
		if (submitting) {
			return
		}

		// Validate all questions answered
		if (responses.some((r) => r === -1)) {
			error = 'Please answer all questions'
			return
		}

		submitting = true
		error = ''

		try {
			const request: SubmitAssessmentRequest = {
				assessment_type_code: assessmentCode,
				responses: responses,
				notes: notes || null,
			}

			const result = await invoke<AssessmentResponse>('submit_assessment', { request })

			// Navigate to results
			await goto(`/assessments/${assessmentCode}/result/${result.id}`)
		} catch (e) {
			error = String(e)
		} finally {
			submitting = false
		}
	}

	function selectOption(questionIndex: number, optionIndex: number) {
		responses[questionIndex] = optionIndex
	}

	const progress = $derived(responses.filter((r) => r !== -1).length)
	const progressPercent = $derived((progress / questions.length) * 100)
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
		<div
			class="mb-4 bg-gray-200 rounded-full h-2"
			role="progressbar"
			aria-valuenow={Math.round(progressPercent)}
			aria-valuemin="0"
			aria-valuemax="100"
			aria-label="Assessment completion progress"
		>
			<div
				class="bg-blue-600 h-2 rounded-full transition-all"
				style="width: {progressPercent}%"
			></div>
		</div>

		<form onsubmit={handleSubmit} class="space-y-6">
			{#each questions as question, i (question.id)}
				<Card>
					<h3 id="question-{i}-label" class="font-medium text-gray-800 mb-3">
						{question.number}. {question.text}
					</h3>
					<div role="radiogroup" aria-labelledby="question-{i}-label" class="space-y-2">
						{#each question.options as option, optionIndex (optionIndex)}
							<button
								type="button"
								role="radio"
								aria-checked={responses[i] === optionIndex}
								class="w-full text-left px-4 py-3 border rounded-lg transition-colors {responses[
									i
								] === optionIndex
									? 'border-blue-600 bg-blue-50'
									: 'border-gray-300 hover:border-gray-400'}"
								onclick={() => selectOption(i, optionIndex)}
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
											<div class="w-2 h-2 bg-white rounded-full"></div>
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
				></textarea>
			</Card>

			{#if error && questions.length > 0}
				<div class="bg-red-50 border border-red-200 rounded-lg p-4">
					<p class="text-red-600">{error}</p>
				</div>
			{/if}

			<Button
				type="submit"
				variant="primary"
				fullWidth
				disabled={submitting || progress < questions.length}
			>
				{submitting ? 'Submitting...' : 'Submit Assessment'}
			</Button>
		</form>
	{/if}
</div>
