<script lang="ts">
	// T129, T130, T132b, T181: Assessment chart component with threshold lines, empty state, and loading animations
	import { Chart, type ChartConfiguration } from 'chart.js'
	import type { AssessmentChartData } from '$lib/bindings'
	import { defaultChartOptions } from '$lib/utils/chart-config'
	import SkeletonLoader from '$lib/components/ui/SkeletonLoader.svelte'

	interface Props {
		data: AssessmentChartData | null
		loading?: boolean
	}

	let { data, loading = false }: Props = $props()

	let canvas = $state<HTMLCanvasElement>()
	let chart: Chart | null = null

	// T132b: Empty state check
	const hasInsufficientData = $derived(!data || !data.data_points || data.data_points.length < 2)

	// Reactive chart rendering with cleanup
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
			return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
		})

		const values = data.data_points.map((point) => point.value)

		// T130: Create threshold annotations
		// Using Record<string, any> here because chartjs-plugin-annotation types are incompatible
		// with our dynamic annotation object structure. The plugin expects an array or specific
		// annotation types, but we need a dynamic Record for threshold annotations.
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const annotations: Record<string, any> = {}
		data.thresholds.forEach((threshold, index) => {
			annotations[`threshold${index}`] = {
				type: 'line' as const,
				yMin: threshold.value,
				yMax: threshold.value,
				borderColor: threshold.color,
				borderWidth: 2,
				borderDash: [5, 5],
				label: {
					display: true,
					content: threshold.label,
					position: 'start' as const,
					backgroundColor: threshold.color,
					color: '#fff',
					padding: 4,
					font: {
						size: 11,
						weight: 'bold' as const,
					},
				},
			}
		})

		// Chart configuration
		const config: ChartConfiguration<'line'> = {
			type: 'line',
			data: {
				labels,
				datasets: [
					{
						label: `${data.assessment_type.name} Score`,
						data: values,
						borderColor: '#3B82F6',
						backgroundColor: 'rgba(59, 130, 246, 0.1)',
						borderWidth: 2,
						fill: true,
						tension: 0.2,
						pointRadius: 5,
						pointHoverRadius: 7,
						pointBackgroundColor: '#3B82F6',
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
						text: `${data.assessment_type.name} Trends`,
						font: {
							size: 18,
							weight: 'bold',
						},
						padding: 20,
					},
					annotation: {
						annotations,
					},
				},
				scales: {
					y: {
						...defaultChartOptions.scales?.y,
						min: data.assessment_type.min_score,
						max: data.assessment_type.max_score,
						title: {
							display: true,
							text: 'Score',
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
							text: 'Date',
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

		// Cleanup function for unmount
		return () => {
			if (chart) {
				chart.destroy()
				chart = null
			}
		}
	})
</script>

<div class="assessment-chart">
	{#if loading}
		<!-- T181: Loading skeleton with smooth transition -->
		<SkeletonLoader type="chart" />
	{:else if hasInsufficientData}
		<!-- T132b: Empty state for <2 data points -->
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
					d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
				/>
			</svg>
			<h3 class="text-lg font-semibold text-gray-700 dark:text-gray-300 mb-2">Not Enough Data</h3>
			<p class="text-gray-600 dark:text-gray-400 text-center max-w-md">
				Complete at least 2 assessments to view trends and patterns over time.
			</p>
		</div>
	{:else}
		<div class="chart-container">
			<canvas bind:this={canvas}></canvas>
		</div>
	{/if}
</div>

<style>
	.assessment-chart {
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
