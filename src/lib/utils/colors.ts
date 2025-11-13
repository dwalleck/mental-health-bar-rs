// T093: Mood color constants for consistent theming across the application

/**
 * Mood color mapping (1=Terrible to 7=Excellent)
 * Uses Tailwind CSS color classes for consistency
 */
export const MOOD_COLORS: Record<number, string> = {
	1: 'bg-red-500 text-white', // Terrible - Red
	2: 'bg-orange-500 text-white', // Very Bad - Dark Orange
	3: 'bg-orange-400 text-white', // Bad - Orange
	4: 'bg-yellow-500 text-white', // Ok - Yellow
	5: 'bg-lime-500 text-white', // Good - Light Green
	6: 'bg-green-500 text-white', // Very Good - Green
	7: 'bg-emerald-500 text-white', // Excellent - Bright Green
}

/**
 * Mood hover color variants for interactive elements
 */
export const MOOD_HOVER_COLORS: Record<number, string> = {
	1: 'hover:bg-red-600',
	2: 'hover:bg-orange-600',
	3: 'hover:bg-orange-500',
	4: 'hover:bg-yellow-600',
	5: 'hover:bg-lime-600',
	6: 'hover:bg-green-600',
	7: 'hover:bg-emerald-600',
}

/**
 * Mood text labels (1-7 scale)
 */
export const MOOD_LABELS: Record<number, string> = {
	1: 'Terrible',
	2: 'Very Bad',
	3: 'Bad',
	4: 'Ok',
	5: 'Good',
	6: 'Very Good',
	7: 'Excellent',
}

/**
 * Mood hex color codes (for use in charts and non-Tailwind contexts)
 */
export const MOOD_HEX_COLORS: Record<number, string> = {
	1: '#EF4444', // red-500
	2: '#F97316', // orange-500
	3: '#FB923C', // orange-400
	4: '#EAB308', // yellow-500
	5: '#84CC16', // lime-500
	6: '#22C55E', // green-500
	7: '#10B981', // emerald-500
}

/**
 * Get mood color class for a given rating
 * Returns neutral color for out-of-range values
 */
export function getMoodColor(rating: number): string {
	if (rating < 1 || rating > 7) {
		console.warn(`Invalid mood rating: ${rating}, using neutral default`)
		return MOOD_COLORS[4] // Ok is the middle value now
	}
	return MOOD_COLORS[rating]
}

/**
 * Get mood label for a given rating
 */
export function getMoodLabel(rating: number): string {
	return MOOD_LABELS[rating] || 'Unknown'
}

/**
 * Get mood hex color for a given rating
 */
export function getMoodHexColor(rating: number): string {
	return MOOD_HEX_COLORS[rating] || MOOD_HEX_COLORS[3]
}
