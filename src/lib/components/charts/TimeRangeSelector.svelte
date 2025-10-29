<script lang="ts">
	// T131: Time range selector component for chart filtering
	import type { TimeRange } from '$lib/bindings'

	interface Props {
		selected: TimeRange
		onchange?: (range: TimeRange) => void
	}

	let { selected = $bindable('week'), onchange }: Props = $props()

	const timeRanges: { value: TimeRange; label: string }[] = [
		{ value: 'week', label: 'Last Week' },
		{ value: 'month', label: 'Last Month' },
		{ value: 'quarter', label: 'Last 3 Months' },
		{ value: 'year', label: 'Last Year' },
		{ value: 'alltime', label: 'All Time' },
	]

	function handleChange(event: Event) {
		const target = event.target as HTMLSelectElement
		const newRange = target.value as TimeRange
		selected = newRange
		if (onchange) {
			onchange(newRange)
		}
	}
</script>

<div class="time-range-selector">
	<label for="time-range" class="block text-sm font-medium text-gray-700 mb-2"> Time Range </label>
	<select
		id="time-range"
		value={selected}
		onchange={handleChange}
		class="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-4 py-2 border"
	>
		{#each timeRanges as range (range.value)}
			<option value={range.value}>{range.label}</option>
		{/each}
	</select>
</div>

<style>
	.time-range-selector {
		width: 100%;
		max-width: 250px;
	}

	select {
		appearance: none;
		background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");
		background-position: right 0.5rem center;
		background-repeat: no-repeat;
		background-size: 1.5em 1.5em;
		padding-right: 2.5rem;
	}
</style>
