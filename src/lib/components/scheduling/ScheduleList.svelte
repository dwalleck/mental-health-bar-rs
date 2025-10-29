<script lang="ts">
	import { commands, type AssessmentSchedule } from '$lib/bindings'
	import { onMount } from 'svelte'

	let { refresh = $bindable(0) }: { refresh?: number } = $props()

	let schedules = $state<AssessmentSchedule[]>([])
	let loading = $state(true)
	let error = $state<string | null>(null)

	$effect(() => {
		// Reload when refresh prop changes (skip initial mount since onMount handles that)
		if (refresh > 0) {
			loadSchedules()
		}
	})

	onMount(async () => {
		await loadSchedules()
	})

	async function loadSchedules() {
		loading = true
		error = null
		try {
			const result = await commands.getSchedules(false)
			if (result.status === 'ok') {
				schedules = result.data
			} else {
				error = result.error
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load schedules'
		} finally {
			loading = false
		}
	}

	async function toggleSchedule(schedule: AssessmentSchedule) {
		try {
			const result = await commands.updateSchedule(schedule.id, {
				enabled: !schedule.enabled,
				frequency: null,
				time_of_day: null,
				day_of_week: null,
				day_of_month: null,
			})
			if (result.status === 'ok') {
				await loadSchedules()
			} else {
				error = result.error
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to update schedule'
		}
	}

	async function deleteSchedule(id: number) {
		if (!confirm('Are you sure you want to delete this schedule?')) {
			return
		}
		try {
			const result = await commands.deleteSchedule(id)
			if (result.status === 'ok') {
				await loadSchedules()
			} else {
				error = result.error
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete schedule'
		}
	}

	function formatFrequency(freq: string): string {
		return freq.charAt(0).toUpperCase() + freq.slice(1)
	}

	function formatDayOfWeek(day: number | null): string {
		if (day === null) return ''
		const days = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday']
		return days[day]
	}

	function formatScheduleDescription(schedule: AssessmentSchedule): string {
		let desc = `${formatFrequency(schedule.frequency)} at ${schedule.time_of_day}`
		if (schedule.day_of_week !== null) {
			desc += ` on ${formatDayOfWeek(schedule.day_of_week)}`
		}
		if (schedule.day_of_month !== null) {
			desc += ` on day ${schedule.day_of_month}`
		}
		return desc
	}
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-bold text-gray-900">Assessment Schedules</h2>

	{#if error}
		<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
			{error}
		</div>
	{/if}

	{#if loading}
		<div class="text-center py-8">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
			<p class="mt-4 text-gray-600">Loading schedules...</p>
		</div>
	{:else if schedules.length === 0}
		<div class="bg-gray-50 border border-gray-200 rounded-lg p-8 text-center">
			<p class="text-gray-600">No schedules configured yet.</p>
			<p class="text-sm text-gray-500 mt-2">Create a schedule above to get started.</p>
		</div>
	{:else}
		<div class="space-y-3">
			{#each schedules as schedule (schedule.id)}
				<div
					class="bg-white border border-gray-200 rounded-lg p-4 flex items-center justify-between hover:shadow-md transition-shadow"
				>
					<div class="flex-1">
						<h3 class="text-lg font-semibold text-gray-900">
							{schedule.assessment_type_name}
						</h3>
						<p class="text-sm text-gray-600 mt-1">
							{formatScheduleDescription(schedule)}
						</p>
						{#if schedule.last_triggered_at}
							<p class="text-xs text-gray-500 mt-1">
								Last triggered: {new Date(schedule.last_triggered_at).toLocaleString()}
							</p>
						{/if}
					</div>

					<div class="flex items-center gap-3">
						<label class="relative inline-flex items-center cursor-pointer">
							<input
								type="checkbox"
								checked={schedule.enabled}
								onchange={() => toggleSchedule(schedule)}
								class="sr-only peer"
							/>
							<div
								class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"
							></div>
							<span class="ml-3 text-sm font-medium text-gray-700">
								{schedule.enabled ? 'Enabled' : 'Disabled'}
							</span>
						</label>

						<button
							onclick={() => deleteSchedule(schedule.id)}
							class="text-red-600 hover:text-red-800 font-medium text-sm"
							aria-label="Delete schedule"
						>
							Delete
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
