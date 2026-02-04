<script lang="ts">
	// T4.2: Activity Trend Chart - % change visualization with arrow indicators
	import { Chart, type ChartConfiguration } from 'chart.js'
	import type { Activity, ActivityTrend } from '$lib/bindings'
	import { defaultBarChartOptions } from '$lib/utils/chart-config'
	import SkeletonLoader from '$lib/components/ui/SkeletonLoader.svelte'

	interface Props {
		activity: Activity
		trend: ActivityTrend | null
		loading?: boolean
		periodLabel?: string // e.g., "7 days", "30 days"
	}

	let { activity, trend, loading = false, periodLabel = '7 days' }: Props = $props()

	let canvas = $state<HTMLCanvasElement>()
	let chart: Chart | null = null

	// Empty state check
	const hasData = $derived(
		trend !== null && (trend.current_period_days > 0 || trend.previous_period_days > 0)
	)

	// Trend visualization helpers
	const trendIcon = $derived.by(() => {
		if (!trend) return '→'
		switch (trend.trend) {
			case 'Improving':
				return '↗️'
			case 'Declining':
				return '↘️'
			case 'Stable':
				return '→'
			default:
				return '→'
		}
	})

	const trendColor = $derived.by(() => {
		if (!trend) return '#6b7280'
		switch (trend.trend) {
			case 'Improving':
				return '#10b981'
			case 'Declining':
				return '#ef4444'
			case 'Stable':
				return '#f59e0b'
			default:
				return '#6b7280'
		}
	})

	const trendLabel = $derived.by(() => {
		if (!trend) return 'No Data'
		switch (trend.trend) {
			case 'Improving':
				return 'Improving'
			case 'Declining':
				return 'Declining'
			case 'Stable':
				return 'Stable'
			default:
				return 'Unknown'
		}
	})

	// Reactive chart rendering with cleanup
	$effect(() => {
		if (!canvas || !trend || !hasData) {
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

		// Prepare data for comparison bar chart
		const labels = ['Previous Period', 'Current Period']
		const data = [trend.previous_period_days, trend.current_period_days]

		// Chart configuration - comparison bars
		const config: ChartConfiguration<'bar'> = {
			type: 'bar',
			data: {
				labels,
				datasets: [
					{
						label: 'Days Active',
						data,
						backgroundColor: [
							'#94a3b8', // Previous period - gray
							trendColor, // Current period - color based on trend
						],
						borderColor: '#ffffff',
						borderWidth: 2,
						borderRadius: 8,
					},
				],
			},
			options: {
				...defaultBarChartOptions,
				plugins: {
					...defaultBarChartOptions.plugins,
					title: {
						display: false,
					},
					legend: {
						display: false,
					},
					tooltip: {
						...defaultBarChartOptions.plugins?.tooltip,
						callbacks: {
							label: function (context) {
								return `${context.parsed.y} days`
							},
						},
					},
				},
				scales: {
					y: {
						beginAtZero: true,
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
					x: {
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

<div class="activity-trend-chart bg-white rounded-lg shadow-sm border border-gray-200 p-6">
	<!-- Header -->
	<div class="mb-4">
		<div class="flex items-center gap-2 mb-1">
			{#if activity.icon}
				<span class="text-2xl">{activity.icon}</span>
			{/if}
			<h3 class="text-lg font-semibold text-gray-900 dark:text-white">{activity.name}</h3>
		</div>
		<p class="text-sm text-gray-600 dark:text-gray-400">Trend Analysis ({periodLabel})</p>
	</div>

	{#if loading}
		<SkeletonLoader type="chart" />
	{:else if !hasData}
		<!-- Empty State -->
		<div class="text-center py-12">
			<div class="text-4xl mb-2">📈</div>
			<p class="text-gray-600 dark:text-gray-400">Not enough data for trend analysis</p>
			<p class="text-sm text-gray-500 dark:text-gray-500 mt-1">
				Need at least 2 periods of activity logs
			</p>
		</div>
	{:else if trend}
		<!-- Trend Indicator -->
		<div class="flex items-center justify-center mb-6 p-4 rounded-lg bg-gray-50 dark:bg-gray-800">
			<div class="text-center">
				<div class="flex items-center justify-center gap-2 mb-2">
					<span class="text-4xl">{trendIcon}</span>
					<span class="text-3xl font-bold" style="color: {trendColor}">
						{trend.change_percentage > 0 ? '+' : ''}{trend.change_percentage.toFixed(1)}%
					</span>
				</div>
				<p class="text-sm text-gray-600 dark:text-gray-400">
					<span class="font-semibold" style="color: {trendColor}">{trendLabel}</span>
					{#if trend.change_days !== 0}
						({trend.change_days > 0 ? '+' : ''}{trend.change_days} days)
					{/if}
				</p>
			</div>
		</div>

		<!-- Chart -->
		<div class="mb-4" style="height: 200px;">
			<canvas bind:this={canvas}></canvas>
		</div>

		<!-- Statistics -->
		<div class="grid grid-cols-2 gap-4 pt-4 border-t border-gray-200 dark:border-gray-700">
			<div class="text-center">
				<p class="text-sm text-gray-600 dark:text-gray-400 mb-1">Previous Period</p>
				<p class="text-2xl font-bold text-gray-500 dark:text-gray-400">
					{trend.previous_period_days}
				</p>
				<p class="text-xs text-gray-500 dark:text-gray-500">days</p>
			</div>
			<div class="text-center">
				<p class="text-sm text-gray-600 dark:text-gray-400 mb-1">Current Period</p>
				<p class="text-2xl font-bold" style="color: {trendColor}">
					{trend.current_period_days}
				</p>
				<p class="text-xs text-gray-500 dark:text-gray-500">days</p>
			</div>
		</div>

		<!-- Interpretation -->
		<div class="mt-4 p-3 rounded-lg" style="background-color: {trendColor}15">
			<p class="text-xs" style="color: {trendColor}">
				{#if trend.trend === 'Improving'}
					<span class="font-semibold">🎉 Great progress!</span>
					You're doing this activity {Math.abs(trend.change_percentage).toFixed(0)}% more often than
					before.
				{:else if trend.trend === 'Declining'}
					<span class="font-semibold">💡 Needs attention</span>
					Activity frequency has decreased by {Math.abs(trend.change_percentage).toFixed(0)}%.
					Consider setting a goal to get back on track.
				{:else}
					<span class="font-semibold">✨ Consistent!</span>
					You're maintaining a steady frequency for this activity.
				{/if}
			</p>
		</div>
	{/if}
</div>
