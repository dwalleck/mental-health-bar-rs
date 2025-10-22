// T093: Mood color constants for consistent theming across the application

/**
 * Mood color mapping (1=Very Bad to 5=Very Good)
 * Uses Tailwind CSS color classes for consistency
 */
export const MOOD_COLORS: Record<number, string> = {
	1: 'bg-red-500 text-white', // Very Bad - Red
	2: 'bg-orange-500 text-white', // Bad - Orange
	3: 'bg-yellow-500 text-white', // Neutral - Yellow
	4: 'bg-lime-500 text-white', // Good - Lime
	5: 'bg-green-500 text-white', // Very Good - Green
}

/**
 * Mood hover color variants for interactive elements
 */
export const MOOD_HOVER_COLORS: Record<number, string> = {
	1: 'hover:bg-red-600',
	2: 'hover:bg-orange-600',
	3: 'hover:bg-yellow-600',
	4: 'hover:bg-lime-600',
	5: 'hover:bg-green-600',
}

/**
 * Mood text labels
 */
export const MOOD_LABELS: Record<number, string> = {
	1: 'Very Bad',
	2: 'Bad',
	3: 'Neutral',
	4: 'Good',
	5: 'Very Good',
}

/**
 * Mood hex color codes (for use in charts and non-Tailwind contexts)
 */
export const MOOD_HEX_COLORS: Record<number, string> = {
	1: '#EF4444', // red-500
	2: '#F97316', // orange-500
	3: '#EAB308', // yellow-500
	4: '#84CC16', // lime-500
	5: '#22C55E', // green-500
}

/**
 * Get mood color class for a given rating
 * Returns neutral color for out-of-range values
 */
export function getMoodColor(rating: number): string {
	if (rating < 1 || rating > 5) {
		console.warn(`Invalid mood rating: ${rating}, using neutral default`)
		return MOOD_COLORS[3]
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
