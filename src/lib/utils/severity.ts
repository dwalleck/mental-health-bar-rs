/**
 * Severity range utilities for assessment scoring
 */

export interface SeverityRange {
	level: string
	min: number
	max: number
	color: string
}

/**
 * Get severity color classes for Tailwind CSS
 */
export function getSeverityColor(severity: string): string {
	const colors: Record<string, string> = {
		minimal: 'bg-green-100 border-green-500 text-green-700',
		mild: 'bg-yellow-100 border-yellow-500 text-yellow-700',
		moderate: 'bg-orange-100 border-orange-500 text-orange-700',
		moderately_severe: 'bg-red-100 border-red-500 text-red-700',
		severe: 'bg-red-200 border-red-600 text-red-800',
	}
	return colors[severity] || 'bg-gray-100 border-gray-500 text-gray-700'
}

/**
 * Get background color class only (for progress bars)
 */
export function getSeverityBgColor(severity: string): string {
	const colors: Record<string, string> = {
		minimal: 'bg-green-500',
		mild: 'bg-yellow-500',
		moderate: 'bg-orange-500',
		moderately_severe: 'bg-red-500',
		severe: 'bg-red-600',
	}
	return colors[severity] || 'bg-gray-500'
}

/**
 * Get severity ranges for assessment types
 */
export function getSeverityRanges(assessmentCode: string, maxScore: number): SeverityRange[] {
	// PHQ-9: minimal (0-4), mild (5-9), moderate (10-14), moderately_severe (15-19), severe (20-27)
	if (assessmentCode === 'PHQ9') {
		return [
			{ level: 'minimal', min: 0, max: 4, color: 'bg-green-500' },
			{ level: 'mild', min: 5, max: 9, color: 'bg-yellow-500' },
			{ level: 'moderate', min: 10, max: 14, color: 'bg-orange-500' },
			{ level: 'moderately_severe', min: 15, max: 19, color: 'bg-red-500' },
			{ level: 'severe', min: 20, max: 27, color: 'bg-red-600' },
		]
	}

	// GAD-7: minimal (0-4), mild (5-9), moderate (10-14), severe (15-21)
	if (assessmentCode === 'GAD7') {
		return [
			{ level: 'minimal', min: 0, max: 4, color: 'bg-green-500' },
			{ level: 'mild', min: 5, max: 9, color: 'bg-yellow-500' },
			{ level: 'moderate', min: 10, max: 14, color: 'bg-orange-500' },
			{ level: 'severe', min: 15, max: 21, color: 'bg-red-600' },
		]
	}

	// CES-D: minimal (0-15), mild (16-21), moderate (22-36), severe (37-60)
	if (assessmentCode === 'CESD') {
		return [
			{ level: 'minimal', min: 0, max: 15, color: 'bg-green-500' },
			{ level: 'mild', min: 16, max: 21, color: 'bg-yellow-500' },
			{ level: 'moderate', min: 22, max: 36, color: 'bg-orange-500' },
			{ level: 'severe', min: 37, max: 60, color: 'bg-red-600' },
		]
	}

	// OASIS: minimal (0-7), moderate (8-14), severe (15-20)
	if (assessmentCode === 'OASIS') {
		return [
			{ level: 'minimal', min: 0, max: 7, color: 'bg-green-500' },
			{ level: 'moderate', min: 8, max: 14, color: 'bg-orange-500' },
			{ level: 'severe', min: 15, max: 20, color: 'bg-red-600' },
		]
	}

	// Default fallback
	return [{ level: 'minimal', min: 0, max: maxScore, color: 'bg-gray-500' }]
}

/**
 * Format severity level for display
 */
export function formatSeverity(severity: string): string {
	return severity
		.split('_')
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(' ')
}

/**
 * Calculate percentage width for a segment
 */
export function calculateSegmentWidth(range: SeverityRange, maxScore: number): number {
	const rangeSize = range.max - range.min + 1
	return (rangeSize / (maxScore + 1)) * 100
}
