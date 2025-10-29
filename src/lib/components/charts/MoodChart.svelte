<script lang="ts">
	// T147, T147b, T181: Mood chart component with empty state and loading animations
	import { onMount } from 'svelte'
	import { Chart, type ChartConfiguration } from 'chart.js'
	import type { MoodChartData } from '$lib/bindings'
	import { defaultChartOptions, moodColors } from '$lib/utils/chart-config'
	import SkeletonLoader from '$lib/components/ui/SkeletonLoader.svelte'

	interface Props {
		data: MoodChartData | null
		loading?: boolean
	}

	let { data, loading = false }: Props = $props()

	let canvas = $state<HTMLCanvasElement>()
	let chart: Chart | null = null

	// T147b: Empty state check
	const hasInsufficientData = $derived(!data || !data.data_points || data.data_points.length < 2)

	onMount(() => {
		return () => {
			// Cleanup chart on unmount
			if (chart) {
				chart.destroy()
			}
		}
	})

	// Reactive chart rendering
	$effect(() => {
		if (!canvas || !data || hasInsufficientData) {
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

		// Prepare data
		const labels = data.data_points.map((point) => {
			const date = new Date(point.timestamp)
			return date.toLocaleDateString('en-US', {
				month: 'short',
				day: 'numeric',
				hour: '2-digit',
				minute: '2-digit',
			})
		})

		const values = data.data_points.map((point) => point.value)

		// Create background colors based on mood rating
		const backgroundColors = values.map((value) => {
			const mood = Math.round(value) as 1 | 2 | 3 | 4 | 5
			return moodColors[mood] || moodColors[3]
		})

		// Chart configuration
		const config: ChartConfiguration<'line'> = {
			type: 'line',
			data: {
				labels,
				datasets: [
					{
						label: 'Mood Rating',
						data: values,
						borderColor: '#8B5CF6',
						backgroundColor: 'rgba(139, 92, 246, 0.1)',
						borderWidth: 2,
						fill: true,
						tension: 0.3,
						pointRadius: 6,
						pointHoverRadius: 8,
						pointBackgroundColor: backgroundColors,
						pointBorderColor: '#fff',
						pointBorderWidth: 2,
					},
				],
			},
			options: {
				...defaultChartOptions,
				plugins: {
					...defaultChartOptions.plugins,
					title: {
						display: true,
						text: 'Mood Trends',
						font: {
							size: 18,
							weight: 'bold',
						},
						padding: 20,
					},
					tooltip: {
						...defaultChartOptions.plugins?.tooltip,
						callbacks: {
							label: function (context) {
								const value = context.parsed.y
								if (value === null) return 'No data'
								const moodLabels = {
									1: 'Very Bad',
									2: 'Bad',
									3: 'Neutral',
									4: 'Good',
									5: 'Very Good',
								}
								const mood = Math.round(value) as 1 | 2 | 3 | 4 | 5
								return `Mood: ${value.toFixed(1)} (${moodLabels[mood] || 'N/A'})`
							},
						},
					},
				},
				scales: {
					y: {
						...defaultChartOptions.scales?.y,
						min: 1,
						max: 5,
						ticks: {
							stepSize: 1,
							callback: function (value) {
								const moodLabels = ['', 'Very Bad', 'Bad', 'Neutral', 'Good', 'Very Good']
								return moodLabels[value as number] || value
							},
						},
						title: {
							display: true,
							text: 'Mood Rating',
							font: {
								size: 14,
								weight: 'bold',
							},
						},
					},
					x: {
						...defaultChartOptions.scales?.x,
						title: {
							display: true,
							text: 'Date & Time',
							font: {
								size: 14,
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
	})
</script>

<div class="mood-chart">
	{#if loading}
		<!-- T181: Loading skeleton with smooth transition -->
		<SkeletonLoader type="chart" />
	{:else if hasInsufficientData}
		<!-- T147b: Empty state for <2 data points -->
		<div
			class="empty-state flex flex-col items-center justify-center h-64 bg-gray-50 dark:bg-gray-800 rounded-lg"
		>
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
					d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
				/>
			</svg>
			<h3 class="text-lg font-semibold text-gray-700 dark:text-gray-300 mb-2">Not Enough Data</h3>
			<p class="text-gray-600 dark:text-gray-400 text-center max-w-md">
				Log at least 2 mood check-ins to view patterns and trends over time.
			</p>
		</div>
	{:else}
		<div class="chart-container">
			<canvas bind:this={canvas}></canvas>
		</div>
	{/if}
</div>

<style>
	.mood-chart {
		width: 100%;
		padding: 1rem;
	}

	.chart-container {
		position: relative;
		width: 100%;
		height: 400px;
	}

	canvas {
		max-height: 400px;
	}
</style>
