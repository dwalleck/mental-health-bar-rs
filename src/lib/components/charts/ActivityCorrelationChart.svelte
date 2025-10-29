<script lang="ts">
	// T148: Activity correlation chart component (horizontal bar chart)
	import { Chart, type ChartConfiguration } from 'chart.js'
	import type { ActivityMoodData } from '$lib/bindings'
	import { defaultBarChartOptions, moodColors } from '$lib/utils/chart-config'

	interface Props {
		data: ActivityMoodData[]
		loading?: boolean
	}

	let { data, loading = false }: Props = $props()

	let canvas = $state<HTMLCanvasElement>()
	let chart: Chart | null = null

	const hasData = $derived(data && data.length > 0)

	// Reactive chart rendering with cleanup
	$effect(() => {
		if (!canvas || !data || data.length === 0) {
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

		// Sort by average mood (highest first)
		const sortedData = [...data].sort((a, b) => b.average_mood - a.average_mood)

		// Prepare data
		const labels = sortedData.map((item) => item.activity.name)
		const values = sortedData.map((item) => item.average_mood)

		// Helper function to convert hex color to RGBA
		const hexToRgba = (hex: string, alpha: number = 1): string => {
			const r = parseInt(hex.slice(1, 3), 16)
			const g = parseInt(hex.slice(3, 5), 16)
			const b = parseInt(hex.slice(5, 7), 16)
			return `rgba(${r}, ${g}, ${b}, ${alpha})`
		}

		// Create background colors based on mood rating
		const backgroundColors = values.map((value) => {
			const mood = Math.round(value) as 1 | 2 | 3 | 4 | 5
			const hexColor = moodColors[mood] || moodColors[3]
			return hexToRgba(hexColor, 0.7)
		})

		// Create border colors with higher opacity
		const borderColors = values.map((value) => {
			const mood = Math.round(value) as 1 | 2 | 3 | 4 | 5
			const hexColor = moodColors[mood] || moodColors[3]
			return hexToRgba(hexColor, 1)
		})

		// Chart configuration
		const config: ChartConfiguration<'bar'> = {
			type: 'bar',
			data: {
				labels,
				datasets: [
					{
						label: 'Average Mood',
						data: values,
						backgroundColor: backgroundColors,
						borderColor: borderColors,
						borderWidth: 1,
					},
				],
			},
			options: {
				...defaultBarChartOptions,
				plugins: {
					...defaultBarChartOptions.plugins,
					title: {
						display: true,
						text: 'Activity Mood Correlations',
						font: {
							size: 16,
							weight: 'bold',
						},
						padding: 16,
					},
					tooltip: {
						callbacks: {
							label: function (context) {
								const value = context.parsed.x
								if (value === null || value === undefined) return 'No data'
								const moodLabels: Record<number, string> = {
									1: 'Very Bad',
									2: 'Bad',
									3: 'Neutral',
									4: 'Good',
									5: 'Very Good',
								}
								const mood = Math.round(value) as 1 | 2 | 3 | 4 | 5
								return `Avg Mood: ${value.toFixed(2)} (${moodLabels[mood] || 'N/A'})`
							},
						},
					},
				},
				scales: {
					x: {
						min: 1,
						max: 5,
						ticks: {
							stepSize: 1,
							callback: function (value) {
								const moodLabels: Record<number, string> = {
									1: 'Very Bad',
									2: 'Bad',
									3: 'Neutral',
									4: 'Good',
									5: 'Very Good',
								}
								return moodLabels[value as number] || value
							},
						},
						title: {
							display: true,
							text: 'Average Mood Rating',
							font: {
								size: 12,
								weight: 'bold',
							},
						},
						grid: {
							color: 'rgba(0, 0, 0, 0.1)',
						},
					},
					y: {
						grid: {
							display: false,
						},
						title: {
							display: true,
							text: 'Activity',
							font: {
								size: 12,
								weight: 'bold',
							},
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

		// Cleanup function for unmount
		return () => {
			if (chart) {
				chart.destroy()
				chart = null
			}
		}
	})
</script>

<div class="activity-correlation-chart">
	{#if loading}
		<div class="flex items-center justify-center h-64">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-green-600"></div>
		</div>
	{:else if !hasData}
		<div class="empty-state flex flex-col items-center justify-center h-64 bg-gray-50 rounded-lg">
			<svg
				class="w-16 h-16 text-gray-400 mb-4"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
				/>
			</svg>
			<h3 class="text-lg font-semibold text-gray-700 mb-2">No Activity Data</h3>
			<p class="text-gray-600 text-center max-w-md">
				Log mood check-ins with activities to see which activities correlate with better or worse
				moods.
			</p>
		</div>
	{:else}
		<div class="chart-container">
			<canvas bind:this={canvas}></canvas>
		</div>
	{/if}
</div>

<style>
	.activity-correlation-chart {
		width: 100%;
		padding: 1rem;
	}

	.chart-container {
		position: relative;
		width: 100%;
		min-height: 300px;
	}

	canvas {
		max-height: 500px;
	}
</style>
