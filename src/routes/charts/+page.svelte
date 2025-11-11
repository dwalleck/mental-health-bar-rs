<script lang="ts">
	// T133, T149, T150: Charts route with assessment and mood visualizations
	import { page } from '$app/stores'
	import { invokeWithRetry } from '$lib/utils/retry'
	import { displayError } from '$lib/utils/errors'
	import type {
		AssessmentType,
		AssessmentChartData,
		MoodChartData,
		TimeRange,
		Activity,
		ActivityFrequency,
		ActivityTrend,
	} from '$lib/bindings'
	import { commands } from '$lib/bindings'
	import AssessmentChart from '$lib/components/charts/AssessmentChart.svelte'
	import ChartStatistics from '$lib/components/charts/ChartStatistics.svelte'
	import TimeRangeSelector from '$lib/components/charts/TimeRangeSelector.svelte'
	import MoodChart from '$lib/components/charts/MoodChart.svelte'
	import ActivityCorrelationChart from '$lib/components/charts/ActivityCorrelationChart.svelte'
	import ActivityReportCard from '$lib/components/charts/ActivityReportCard.svelte'
	import ActivityTrendChart from '$lib/components/charts/ActivityTrendChart.svelte'
	import GoalProgressDashboard from '$lib/components/charts/GoalProgressDashboard.svelte'

	// Assessment state
	let assessmentTypes: AssessmentType[] = $state([])
	let selectedType: string = $state('PHQ9')
	let selectedTimeRange: TimeRange = $state('month')
	let chartData: AssessmentChartData | null = $state(null)
	let loading: boolean = $state(false)
	let error: string = $state('')

	// Mood state
	let moodTimeRange: TimeRange = $state('month')
	let moodChartData: MoodChartData | null = $state(null)
	let moodLoading: boolean = $state(false)
	let moodError: string = $state('')

	// Tab state
	let activeTab: 'assessments' | 'mood' | 'activities' = $state('assessments')

	// Activities state (T4.4)
	let activities: Activity[] = $state([])
	let selectedActivityId: number | null = $state(null)
	let activityTimeRange: TimeRange = $state('month')
	let activityFrequency: ActivityFrequency | null = $state(null)
	let activityTrend: ActivityTrend | null = $state(null)
	let activitiesLoading: boolean = $state(false)
	let activitiesError: string = $state('')

	// Load chart data on mount
	$effect(() => {
		async function initializeCharts() {
			// T230: Support URL query parameter for assessment type
			const typeParam = $page.url.searchParams.get('type')
			if (typeParam) {
				selectedType = typeParam.toUpperCase()
			}

			await loadAssessmentTypes()
			await loadChartData()
			await loadMoodChartData()
		}
		initializeCharts()
	})

	async function loadAssessmentTypes() {
		try {
			assessmentTypes = await invokeWithRetry<AssessmentType[]>('get_assessment_types')
			// Only set default if selectedType hasn't been set (e.g., from URL parameter)
			if (assessmentTypes.length > 0 && !selectedType) {
				selectedType = assessmentTypes[0].code
			}
		} catch (err) {
			displayError(err)
			error = 'Failed to load assessment types'
		}
	}

	async function loadChartData() {
		if (!selectedType) return

		loading = true
		error = ''

		try {
			chartData = await invokeWithRetry<AssessmentChartData>('get_assessment_chart_data', {
				code: selectedType,
				timeRange: selectedTimeRange,
				fromDate: null,
				toDate: null,
			})
		} catch (err) {
			displayError(err)
			error = 'Failed to load chart data'
			chartData = null
		} finally {
			loading = false
		}
	}

	async function handleTypeChange(event: Event) {
		const target = event.target as HTMLSelectElement
		selectedType = target.value
		await loadChartData()
	}

	async function handleTimeRangeChange(range: TimeRange) {
		selectedTimeRange = range
		await loadChartData()
	}

	// T149, T150: Mood chart data loading
	async function loadMoodChartData() {
		moodLoading = true
		moodError = ''

		try {
			moodChartData = await invokeWithRetry<MoodChartData>('get_mood_chart_data', {
				timeRange: moodTimeRange,
				fromDate: null,
				toDate: null,
				groupByActivity: true,
			})
		} catch (err) {
			displayError(err)
			moodError = 'Failed to load mood chart data'
			moodChartData = null
		} finally {
			moodLoading = false
		}
	}

	async function handleMoodTimeRangeChange(range: TimeRange) {
		moodTimeRange = range
		await loadMoodChartData()
	}

	// T4.4: Activities reporting data loading
	async function loadActivities() {
		try {
			const result = await commands.getActivities()
			if (result.status === 'error') {
				throw new Error(result.error.message)
			}
			activities = result.data
			if (activities.length > 0 && !selectedActivityId) {
				selectedActivityId = activities[0].id
				await loadActivityReports()
			}
		} catch (err) {
			displayError(err)
			activitiesError = 'Failed to load activities'
		}
	}

	async function loadActivityReports() {
		if (!selectedActivityId) return

		activitiesLoading = true
		activitiesError = ''

		try {
			// Calculate date range based on timeRange
			 
			const endDate = new Date()
			// eslint-disable-next-line svelte/prefer-svelte-reactivity
			let startDate = new Date()

			switch (activityTimeRange) {
				case 'week':
					startDate.setDate(endDate.getDate() - 7)
					break
				case 'month':
					startDate.setMonth(endDate.getMonth() - 1)
					break
				case 'quarter':
					startDate.setMonth(endDate.getMonth() - 3)
					break
				case 'year':
					startDate.setFullYear(endDate.getFullYear() - 1)
					break
			}

			// Load frequency data
			const frequencyResult = await commands.getActivityFrequency(
				selectedActivityId,
				startDate.toISOString(),
				endDate.toISOString()
			)
			if (frequencyResult.status === 'ok') {
				activityFrequency = frequencyResult.data
			}

			// Load trend data
			const periodDays = Math.ceil(
				(endDate.getTime() - startDate.getTime()) / (1000 * 60 * 60 * 24)
			)
			const trendResult = await commands.getActivityTrend(
				selectedActivityId,
				periodDays,
				endDate.toISOString()
			)
			if (trendResult.status === 'ok') {
				activityTrend = trendResult.data
			}
		} catch (err) {
			displayError(err)
			activitiesError = 'Failed to load activity reports'
			activityFrequency = null
			activityTrend = null
		} finally {
			activitiesLoading = false
		}
	}

	async function handleActivityChange(event: Event) {
		const target = event.target as HTMLSelectElement
		selectedActivityId = parseInt(target.value)
		await loadActivityReports()
	}

	async function handleActivityTimeRangeChange(range: TimeRange) {
		activityTimeRange = range
		await loadActivityReports()
	}

	// Load activities when tab becomes active
	$effect(() => {
		if (activeTab === 'activities' && activities.length === 0) {
			loadActivities()
		}
	})
</script>

<svelte:head>
	<title>Data Visualization - Mental Health Tracker</title>
</svelte:head>

<div class="charts-page container mx-auto px-4 py-8 max-w-7xl">
	<div class="mb-8">
		<h1 class="text-3xl font-bold text-gray-900 mb-2">Data Visualization</h1>
		<p class="text-gray-600">Track your mental health trends over time</p>
	</div>

	<!-- Tabs -->
	<div class="tabs mb-6">
		<button
			class="tab-button {activeTab === 'assessments' ? 'active' : ''}"
			onclick={() => (activeTab = 'assessments')}
		>
			📊 Assessment Trends
		</button>
		<button
			class="tab-button {activeTab === 'mood' ? 'active' : ''}"
			onclick={() => (activeTab = 'mood')}
		>
			😊 Mood Patterns
		</button>
		<button
			class="tab-button {activeTab === 'activities' ? 'active' : ''}"
			onclick={() => (activeTab = 'activities')}
		>
			🎯 Activity Reports
		</button>
	</div>

	<!-- Assessment Charts Tab -->
	{#if activeTab === 'assessments'}
		{#if error}
			<div class="bg-red-50 border border-red-200 text-red-800 rounded-lg p-4 mb-6">
				<p class="font-semibold">Error</p>
				<p>{error}</p>
			</div>
		{/if}

		<!-- Assessment Selection and Filters -->
		<div class="controls-section bg-white rounded-lg shadow-xs border border-gray-200 p-6 mb-6">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<!-- Assessment Type Selector -->
				<div>
					<label for="assessment-type" class="block text-sm font-medium text-gray-700 mb-2">
						Assessment Type
					</label>
					<select
						id="assessment-type"
						value={selectedType}
						onchange={handleTypeChange}
						class="block w-full rounded-md border-gray-300 shadow-xs focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-4 py-2 border"
					>
						{#each assessmentTypes as type (type.id)}
							<option value={type.code}>{type.name}</option>
						{/each}
					</select>
				</div>

				<!-- Time Range Selector -->
				<TimeRangeSelector selected={selectedTimeRange} onchange={handleTimeRangeChange} />
			</div>
		</div>

		<!-- Chart Display -->
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
			<!-- Chart -->
			<div class="lg:col-span-2">
				<div class="bg-white rounded-lg shadow-xs border border-gray-200">
					<AssessmentChart data={chartData} {loading} />
				</div>
			</div>

			<!-- Statistics -->
			<div class="lg:col-span-1">
				{#if chartData && chartData.data_points.length >= 2}
					<ChartStatistics statistics={chartData.statistics} title="Assessment Statistics" />
				{/if}
			</div>
		</div>

		<!-- Info Section -->
		<div class="mt-8 bg-blue-50 border border-blue-200 rounded-lg p-6">
			<h3 class="text-lg font-semibold text-blue-900 mb-2">Understanding Your Charts</h3>
			<ul class="space-y-2 text-blue-800 text-sm">
				<li>
					<strong>Lower scores are better</strong> - For mental health assessments, decreasing trends
					indicate improvement
				</li>
				<li>
					<strong>Threshold lines</strong> - Dashed lines show clinical severity boundaries
				</li>
				<li>
					<strong>Trend analysis</strong> - Compares your first and last assessment to show overall direction
				</li>
			</ul>
		</div>
	{/if}

	<!-- Mood Charts Tab (T149, T150) -->
	{#if activeTab === 'mood'}
		{#if moodError}
			<div class="bg-red-50 border border-red-200 text-red-800 rounded-lg p-4 mb-6">
				<p class="font-semibold">Error</p>
				<p>{moodError}</p>
			</div>
		{/if}

		<!-- Mood Time Range Selector -->
		<div class="controls-section bg-white rounded-lg shadow-xs border border-gray-200 p-6 mb-6">
			<TimeRangeSelector selected={moodTimeRange} onchange={handleMoodTimeRangeChange} />
		</div>

		<!-- Mood Chart -->
		<div class="bg-white rounded-lg shadow-xs border border-gray-200 mb-6">
			<MoodChart data={moodChartData} loading={moodLoading} />
		</div>

		<!-- Mood Statistics -->
		{#if moodChartData && moodChartData.data_points.length >= 2}
			<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
				<div class="bg-white rounded-lg shadow-xs border border-gray-200 p-6">
					<h3 class="text-lg font-semibold text-gray-900 mb-4">Mood Statistics</h3>
					<div class="grid grid-cols-2 gap-4">
						<div>
							<p class="text-sm text-gray-600">Average Mood</p>
							<p class="text-2xl font-bold text-gray-900">
								{moodChartData.statistics.average.toFixed(1)}
							</p>
						</div>
						<div>
							<p class="text-sm text-gray-600">Median Mood</p>
							<p class="text-2xl font-bold text-gray-900">
								{moodChartData.statistics.median.toFixed(1)}
							</p>
						</div>
						<div>
							<p class="text-sm text-gray-600">Total Check-ins</p>
							<p class="text-2xl font-bold text-gray-900">
								{moodChartData.statistics.total_checkins}
							</p>
						</div>
						<div>
							<p class="text-sm text-gray-600">Per Day</p>
							<p class="text-2xl font-bold text-gray-900">
								{moodChartData.statistics.checkins_per_day.toFixed(1)}
							</p>
						</div>
					</div>
				</div>

				<!-- Activity Correlation Chart (T150) -->
				<div class="bg-white rounded-lg shadow-xs border border-gray-200">
					<ActivityCorrelationChart data={moodChartData.activity_breakdown} loading={moodLoading} />
				</div>
			</div>
		{/if}

		<!-- Info Section -->
		<div class="mt-8 bg-purple-50 border border-purple-200 rounded-lg p-6">
			<h3 class="text-lg font-semibold text-purple-900 mb-2">Understanding Mood Patterns</h3>
			<ul class="space-y-2 text-purple-800 text-sm">
				<li>
					<strong>Mood trends</strong> - Track how your mood changes over time and identify patterns
				</li>
				<li>
					<strong>Activity correlations</strong> - See which activities are associated with better or
					worse moods
				</li>
				<li>
					<strong>Higher ratings are better</strong> - 5 is Very Good, 1 is Very Bad
				</li>
			</ul>
		</div>
	{/if}

	<!-- Activity Reports Tab (T4.4) -->
	{#if activeTab === 'activities'}
		{#if activitiesError}
			<div class="bg-red-50 border border-red-200 text-red-800 rounded-lg p-4 mb-6">
				<p class="font-semibold">Error</p>
				<p>{activitiesError}</p>
			</div>
		{/if}

		{#if activities.length === 0 && !activitiesLoading}
			<!-- Empty State -->
			<div class="bg-white rounded-lg shadow-xs border border-gray-200 p-12 text-center">
				<div class="text-4xl mb-4">🎯</div>
				<h3 class="text-lg font-medium text-gray-900 mb-2">No Activities Yet</h3>
				<p class="text-gray-600 mb-6">
					Create activities and log them to see frequency reports and trends
				</p>
				<a
					href="/mood/activities"
					class="inline-block px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
				>
					Go to Activities
				</a>
			</div>
		{:else}
			<!-- Activity Selection and Filters -->
			<div class="controls-section bg-white rounded-lg shadow-xs border border-gray-200 p-6 mb-6">
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<!-- Activity Selector -->
					<div>
						<label for="activity-select" class="block text-sm font-medium text-gray-700 mb-2">
							Select Activity
						</label>
						<select
							id="activity-select"
							value={selectedActivityId || ''}
							onchange={handleActivityChange}
							class="block w-full rounded-md border-gray-300 shadow-xs focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-4 py-2 border"
						>
							{#each activities as activity (activity.id)}
								<option value={activity.id}>
									{activity.icon ? `${activity.icon} ` : ''}{activity.name}
								</option>
							{/each}
						</select>
					</div>

					<!-- Time Range Selector -->
					<TimeRangeSelector
						selected={activityTimeRange}
						onchange={handleActivityTimeRangeChange}
					/>
				</div>
			</div>

			{#if selectedActivityId}
				{@const selectedActivity = activities.find((a) => a.id === selectedActivityId)}
				{#if selectedActivity}
					<!-- Activity Reports Grid -->
					<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
						<!-- Frequency Report Card -->
						<ActivityReportCard
							activity={selectedActivity}
							frequency={activityFrequency}
							loading={activitiesLoading}
						/>

						<!-- Trend Chart -->
						<ActivityTrendChart
							activity={selectedActivity}
							trend={activityTrend}
							loading={activitiesLoading}
							periodLabel={activityTimeRange === 'week'
								? '7 days'
								: activityTimeRange === 'month'
									? '30 days'
									: activityTimeRange === 'quarter'
										? '90 days'
										: '365 days'}
						/>
					</div>
				{/if}
			{/if}

			<!-- Goal Progress Dashboard -->
			<div class="mb-6">
				<GoalProgressDashboard {activities} />
			</div>

			<!-- Info Section -->
			<div class="mt-8 bg-green-50 border border-green-200 rounded-lg p-6">
				<h3 class="text-lg font-semibold text-green-900 mb-2">Understanding Activity Reports</h3>
				<ul class="space-y-2 text-green-800 text-sm">
					<li>
						<strong>Days/Week</strong> - Shows how frequently you perform this activity
					</li>
					<li>
						<strong>Trend Analysis</strong> - Compares your current period to the previous period of
						the same length
					</li>
					<li>
						<strong>Goal Progress</strong> - Track all your active goals in one place
					</li>
					<li>
						<strong>Color coding</strong> - Green (improving), Yellow (stable), Red (declining)
					</li>
				</ul>
			</div>
		{/if}
	{/if}
</div>

<style>
	.tabs {
		display: flex;
		gap: 0.5rem;
		border-bottom: 2px solid #e5e7eb;
	}

	.tab-button {
		padding: 0.75rem 1.5rem;
		font-size: 1rem;
		font-weight: 500;
		color: #6b7280;
		background: transparent;
		border: none;
		border-bottom: 3px solid transparent;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tab-button:hover {
		color: #1f2937;
		border-bottom-color: #d1d5db;
	}

	.tab-button.active {
		color: #3b82f6;
		border-bottom-color: #3b82f6;
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
