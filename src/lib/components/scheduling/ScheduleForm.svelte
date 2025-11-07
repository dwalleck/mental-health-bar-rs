<script lang="ts">
	import {
		commands,
		type AssessmentType,
		type CreateScheduleRequest,
		type ScheduleFrequency,
	} from '$lib/bindings'
	import { invokeWithRetry } from '$lib/utils/retry'
	import { displayError, displaySuccess } from '$lib/utils/errors'
	import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte'

	let { onSuccess }: { onSuccess?: () => void } = $props()

	// Form state
	let assessmentTypes = $state<AssessmentType[]>([])
	let selectedAssessmentTypeId = $state<number>(0)
	let frequency = $state<ScheduleFrequency>('daily')
	let timeOfDay = $state('09:00')
	let dayOfWeek = $state<number | null>(null)
	let dayOfMonth = $state<number | null>(null)
	let loading = $state(false)
	let validationError = $state<unknown>(undefined)

	const daysOfWeek = [
		{ value: 0, label: 'Sunday' },
		{ value: 1, label: 'Monday' },
		{ value: 2, label: 'Tuesday' },
		{ value: 3, label: 'Wednesday' },
		{ value: 4, label: 'Thursday' },
		{ value: 5, label: 'Friday' },
		{ value: 6, label: 'Saturday' },
	]

	const daysOfMonth = Array.from({ length: 31 }, (_, i) => i + 1)

	// Load assessment types on mount
	$effect(() => {
		loadAssessmentTypes()
	})

	async function loadAssessmentTypes() {
		validationError = undefined
		try {
			const result = await commands.getAssessmentTypes()
			if (result.status === 'ok') {
				assessmentTypes = result.data
				if (assessmentTypes.length > 0) {
					selectedAssessmentTypeId = assessmentTypes[0].id
				}
			} else {
				// getAssessmentTypes is a query command - still returns string error
				const errorResult = displayError(result.error)
				if (errorResult.type === 'inline') {
					validationError = result.error
				}
			}
		} catch (e) {
			const errorResult = displayError(e)
			if (errorResult.type === 'inline') {
				validationError = e
			}
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault()
		loading = true
		validationError = undefined

		try {
			const request: CreateScheduleRequest = {
				assessment_type_id: selectedAssessmentTypeId,
				frequency,
				time_of_day: timeOfDay,
				day_of_week: frequency === 'weekly' || frequency === 'biweekly' ? dayOfWeek : null,
				day_of_month: frequency === 'monthly' ? dayOfMonth : null,
			}

			await invokeWithRetry('create_schedule', {
				request,
			})
			displaySuccess('Schedule created successfully!')
			// Reset form
			frequency = 'daily'
			timeOfDay = '09:00'
			dayOfWeek = null
			dayOfMonth = null
			if (assessmentTypes.length > 0) {
				selectedAssessmentTypeId = assessmentTypes[0].id
			}
			onSuccess?.()
		} catch (e) {
			const errorResult = displayError(e)
			if (errorResult.type === 'inline') {
				validationError = e
			}
		} finally {
			loading = false
		}
	}

	let needsDayOfWeek = $derived(frequency === 'weekly' || frequency === 'biweekly')
	let needsDayOfMonth = $derived(frequency === 'monthly')
</script>

<form onsubmit={handleSubmit} class="space-y-6 bg-white p-6 rounded-lg shadow-sm">
	<h2 class="text-2xl font-bold text-gray-900">Create Schedule</h2>

	<ErrorMessage error={validationError} />

	<div>
		<label for="assessment-type" class="block text-sm font-medium text-gray-700 mb-2">
			Assessment Type
		</label>
		<select
			id="assessment-type"
			bind:value={selectedAssessmentTypeId}
			class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-blue-500"
			required
		>
			{#each assessmentTypes as type (type.id)}
				<option value={type.id}>
					{type.name}
				</option>
			{/each}
		</select>
	</div>

	<div>
		<label for="frequency" class="block text-sm font-medium text-gray-700 mb-2"> Frequency </label>
		<select
			id="frequency"
			bind:value={frequency}
			class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-blue-500"
			required
		>
			<option value="daily">Daily</option>
			<option value="weekly">Weekly</option>
			<option value="biweekly">Biweekly</option>
			<option value="monthly">Monthly</option>
		</select>
	</div>

	<div>
		<label for="time" class="block text-sm font-medium text-gray-700 mb-2"> Time of Day </label>
		<input
			type="time"
			id="time"
			bind:value={timeOfDay}
			class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-blue-500"
			required
		/>
	</div>

	{#if needsDayOfWeek}
		<div>
			<label for="day-of-week" class="block text-sm font-medium text-gray-700 mb-2">
				Day of Week
			</label>
			<select
				id="day-of-week"
				bind:value={dayOfWeek}
				class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-blue-500"
				required
			>
				{#each daysOfWeek as day (day.value)}
					<option value={day.value}>
						{day.label}
					</option>
				{/each}
			</select>
		</div>
	{/if}

	{#if needsDayOfMonth}
		<div>
			<label for="day-of-month" class="block text-sm font-medium text-gray-700 mb-2">
				Day of Month
			</label>
			<select
				id="day-of-month"
				bind:value={dayOfMonth}
				class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-blue-500"
				required
			>
				{#each daysOfMonth as day (day)}
					<option value={day}>
						{day}
					</option>
				{/each}
			</select>
		</div>
	{/if}

	<button
		type="submit"
		disabled={loading}
		class="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-hidden focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:bg-blue-300 disabled:cursor-not-allowed"
	>
		{loading ? 'Creating...' : 'Create Schedule'}
	</button>
</form>
