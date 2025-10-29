<script lang="ts">
	import type { AssessmentType } from '$lib/bindings'
	import {
		getSeverityRanges,
		getSeverityBgColor,
		formatSeverity,
		calculateSegmentWidth,
		type SeverityRange,
	} from '$lib/utils/severity'

	interface Props {
		assessmentType: AssessmentType
		score: number
		severityLevel: string
		showLabels?: boolean
		showScore?: boolean
	}

	let {
		assessmentType,
		score,
		severityLevel,
		showLabels = true,
		showScore = true,
	}: Props = $props()

	// Clamp score to valid range
	const clampedScore = $derived(Math.max(0, Math.min(score, assessmentType.max_score)))

	// Calculate percentage for progress bar
	const percentage = $derived((clampedScore / assessmentType.max_score) * 100)

	// Get severity ranges for this assessment type (uppercase the code for matching)
	const severityRanges = $derived(
		getSeverityRanges(assessmentType.code.toUpperCase(), assessmentType.max_score)
	)

	// Get color for current severity level (lowercase for matching)
	const currentSeverityColor = $derived(getSeverityBgColor(severityLevel.toLowerCase()))

	// Calculate aria label for accessibility
	const ariaLabel = $derived(
		`${assessmentType.name} score: ${clampedScore} out of ${assessmentType.max_score}, ${formatSeverity(severityLevel)} severity`
	)

	// Determine if a segment is the active/current one
	function isActiveSegment(range: SeverityRange): boolean {
		return range.level === severityLevel.toLowerCase()
	}

	// Calculate position and width for each segment
	function getSegmentStyle(range: SeverityRange): string {
		const startPercentage = (range.min / assessmentType.max_score) * 100
		const width = calculateSegmentWidth(range, assessmentType.max_score)
		return `left: ${startPercentage}%; width: ${width}%;`
	}
</script>

<div class="assessment-score-bar w-full space-y-2">
	<!-- Assessment name and score -->
	<div class="flex justify-between items-center">
		<h3 class="text-sm font-medium text-gray-700">{assessmentType.name}</h3>
		{#if showScore}
			<span class="text-sm font-semibold text-gray-900">
				{clampedScore} / {assessmentType.max_score}
			</span>
		{/if}
	</div>

	<!-- Progress bar container -->
	<div class="relative">
		<!-- Background segments showing severity ranges -->
		<div class="relative h-8 bg-gray-200 rounded-lg overflow-hidden">
			{#each severityRanges as range (range.level)}
				<div
					class="severity-segment absolute h-full transition-opacity {range.color} {isActiveSegment(
						range
					)
						? 'opacity-100 current active highlight'
						: 'opacity-30'}"
					style={getSegmentStyle(range)}
				></div>
			{/each}

			<!-- Progress bar overlay -->
			<div
				role="progressbar"
				aria-label={ariaLabel}
				aria-valuenow={clampedScore}
				aria-valuemin={assessmentType.min_score}
				aria-valuemax={assessmentType.max_score}
				class="absolute top-0 left-0 h-full {currentSeverityColor} transition-all duration-300 ease-in-out"
				style="width: {percentage}%"
			></div>
		</div>

		<!-- Severity labels below the bar (optional) -->
		{#if showLabels}
			<div class="relative mt-1">
				{#each severityRanges as range (range.level)}
					<div
						class="absolute text-xs {isActiveSegment(range)
							? 'font-semibold text-gray-900'
							: 'text-gray-500'}"
						style={getSegmentStyle(range)}
					>
						<span class="block text-center capitalize">
							{formatSeverity(range.level)}
						</span>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	.severity-segment {
		border-right: 1px solid rgba(255, 255, 255, 0.3);
	}

	.severity-segment:last-child {
		border-right: none;
	}
</style>
