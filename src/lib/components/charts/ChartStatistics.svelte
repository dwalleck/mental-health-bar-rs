<script lang="ts">
	// T132: Chart statistics display component
	import type { ChartStatistics } from '$lib/bindings'

	interface Props {
		statistics: ChartStatistics
		title?: string
	}

	let { statistics, title = 'Statistics' }: Props = $props()

	function getTrendIcon(trend: string): string {
		switch (trend) {
			case 'improving':
				return '📉' // Downward trend (good for mental health assessments)
			case 'worsening':
				return '📈' // Upward trend (concerning)
			case 'stable':
				return '➡️' // Stable
			default:
				return ''
		}
	}

	function getTrendColor(trend: string): string {
		switch (trend) {
			case 'improving':
				return 'text-green-600'
			case 'worsening':
				return 'text-red-600'
			case 'stable':
				return 'text-gray-600'
			default:
				return 'text-gray-600'
		}
	}

	function getTrendLabel(trend: string): string {
		return trend.charAt(0).toUpperCase() + trend.slice(1)
	}
</script>

<div class="statistics-card bg-white rounded-lg shadow-sm border border-gray-200 p-6">
	<h3 class="text-lg font-semibold text-gray-900 mb-4">{title}</h3>

	<div class="grid grid-cols-2 gap-4">
		<div class="stat-item">
			<p class="text-sm text-gray-600">Minimum</p>
			<p class="text-2xl font-bold text-gray-900">{statistics.min.toFixed(1)}</p>
		</div>

		<div class="stat-item">
			<p class="text-sm text-gray-600">Maximum</p>
			<p class="text-2xl font-bold text-gray-900">{statistics.max.toFixed(1)}</p>
		</div>

		<div class="stat-item">
			<p class="text-sm text-gray-600">Average</p>
			<p class="text-2xl font-bold text-gray-900">{statistics.average.toFixed(1)}</p>
		</div>

		<div class="stat-item">
			<p class="text-sm text-gray-600">Total Assessments</p>
			<p class="text-2xl font-bold text-gray-900">{statistics.total_assessments}</p>
		</div>
	</div>

	<div class="mt-6 pt-6 border-t border-gray-200">
		<div class="flex items-center gap-3">
			<span class="text-3xl">{getTrendIcon(statistics.trend)}</span>
			<div>
				<p class="text-sm text-gray-600">Trend</p>
				<p class="text-lg font-semibold {getTrendColor(statistics.trend)}">
					{getTrendLabel(statistics.trend)}
				</p>
			</div>
		</div>
	</div>
</div>

<style>
	.stat-item {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
