<script lang="ts">
	// T4.1: Activity Report Card - Days/week display with bar chart
	import { Chart, type ChartConfiguration } from 'chart.js'
	import type { Activity, ActivityFrequency } from '$lib/bindings'
	import { defaultChartOptions } from '$lib/utils/chart-config'
	import SkeletonLoader from '$lib/components/ui/SkeletonLoader.svelte'

	interface Props {
		activity: Activity
		frequency: ActivityFrequency | null
		loading?: boolean
	}

	let { activity, frequency, loading = false }: Props = $props()

	let canvas = $state<HTMLCanvasElement>()
	let chart: Chart | null = null

	// Empty state check
	const hasData = $derived(frequency !== null && frequency.unique_days > 0)

	// Reactive chart rendering with cleanup
	$effect(() => {
		if (!canvas || !frequency || !hasData) {
			if (chart) {
				chart.destroy()
				chart = null
			}
			return
		}

		// Destroy existing chart
		if (chart) {
			chart.destroy()
		}

		// Prepare data for bar chart
		const labels = ['Days/Week']
		const targetValue = 7 // Maximum days per week
		const actualValue = Math.min(frequency.days_per_week, targetValue)

		// Chart configuration - single bar showing days/week
		const config: ChartConfiguration<'bar'> = {
			type: 'bar',
			data: {
				labels,
				datasets: [
					{
						label: 'Days per Week',
						data: [actualValue],
						backgroundColor:
							actualValue >= 5 ? '#10b981' : actualValue >= 3 ? '#f59e0b' : '#ef4444',
						borderColor: '#ffffff',
						borderWidth: 2,
						borderRadius: 8,
					},
				],
			},
			options: {
				...defaultChartOptions,
				indexAxis: 'y', // Horizontal bar
				plugins: {
					...defaultChartOptions.plugins,
					title: {
						display: false,
					},
					legend: {
						display: false,
					},
					tooltip: {
						...defaultChartOptions.plugins?.tooltip,
						callbacks: {
							label: function (context) {
								return `${context.parsed.x.toFixed(1)} days/week`
							},
						},
					},
				},
				scales: {
					x: {
						beginAtZero: true,
						max: 7,
						grid: {
							display: true,
							color: '#e5e7eb',
						},
						ticks: {
							stepSize: 1,
							callback: function (value) {
								return `${value} days`
							},
						},
					},
					y: {
						grid: {
							display: false,
						},
					},
				},
			},
		}

		// Create chart
		const ctx = canvas.getContext('2d')
		if (ctx) {
			chart = new Chart(ctx, config)
		}

		// Cleanup on destroy
		return () => {
			if (chart) {
				chart.destroy()
				chart = null
			}
		}
	})
</script>

<div class="activity-report-card bg-white rounded-lg shadow-sm border border-gray-200 p-6">
	<!-- Header -->
	<div class="mb-4">
		<div class="flex items-center gap-2 mb-1">
			{#if activity.icon}
				<span class="text-2xl">{activity.icon}</span>
			{/if}
			<h3 class="text-lg font-semibold text-gray-900 dark:text-white">{activity.name}</h3>
		</div>
		<p class="text-sm text-gray-600 dark:text-gray-400">Activity Frequency Report</p>
	</div>

	{#if loading}
		<SkeletonLoader height="200px" />
	{:else if !hasData}
		<!-- Empty State -->
		<div class="text-center py-12">
			<div class="text-4xl mb-2">📊</div>
			<p class="text-gray-600 dark:text-gray-400">No activity logs yet</p>
			<p class="text-sm text-gray-500 dark:text-gray-500 mt-1">
				Log this activity to see frequency data
			</p>
		</div>
	{:else if frequency}
		<!-- Chart -->
		<div class="mb-4" style="height: 100px;">
			<canvas bind:this={canvas}></canvas>
		</div>

		<!-- Statistics -->
		<div class="grid grid-cols-3 gap-4 pt-4 border-t border-gray-200 dark:border-gray-700">
			<div class="text-center">
				<p class="text-2xl font-bold text-blue-600 dark:text-blue-400">
					{frequency.days_per_week.toFixed(1)}
				</p>
				<p class="text-xs text-gray-600 dark:text-gray-400 mt-1">Days/Week</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-bold text-green-600 dark:text-green-400">
					{frequency.unique_days}
				</p>
				<p class="text-xs text-gray-600 dark:text-gray-400 mt-1">Unique Days</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-bold text-purple-600 dark:text-purple-400">
					{frequency.total_logs}
				</p>
				<p class="text-xs text-gray-600 dark:text-gray-400 mt-1">Total Logs</p>
			</div>
		</div>

		<!-- Interpretation -->
		<div class="mt-4 p-3 rounded-lg bg-gray-50 dark:bg-gray-800">
			<p class="text-xs text-gray-700 dark:text-gray-300">
				{#if frequency.days_per_week >= 5}
					<span class="font-semibold text-green-700 dark:text-green-400">✅ Excellent!</span>
					You're maintaining this activity consistently.
				{:else if frequency.days_per_week >= 3}
					<span class="font-semibold text-yellow-700 dark:text-yellow-400">👍 Good!</span>
					You're practicing this activity regularly.
				{:else if frequency.days_per_week >= 1}
					<span class="font-semibold text-orange-700 dark:text-orange-400">📈 Getting started!</span
					>
					Keep building this habit.
				{:else}
					<span class="font-semibold text-gray-700 dark:text-gray-400">💡 Tip:</span>
					Try to log this activity more frequently.
				{/if}
			</p>
		</div>
	{/if}
</div>
