/**
 * Date/time formatting utilities for UI components.
 *
 * Shared helpers to avoid duplicated inline implementations.
 */

/**
 * Format a timestamp into a relative, user-friendly label.
 *
 * Examples:
 * - "Today at 3:45 PM"
 * - "Yesterday at 10:12 AM"
 * - "3 days ago"
 * - "Jan 5, 2025, 3:45 PM"
 *
 * Handles future dates (clock skew) by falling back to an absolute format.
 */
export function formatRelativeDate(isoString: string): string {
	const date = new Date(isoString)
	const now = new Date()

	if (Number.isNaN(date.getTime())) {
		return isoString
	}

	// Normalize to start of day for accurate day comparison
	const dateDay = new Date(date.getFullYear(), date.getMonth(), date.getDate())
	const nowDay = new Date(now.getFullYear(), now.getMonth(), now.getDate())
	const diffDays = Math.floor((nowDay.getTime() - dateDay.getTime()) / (1000 * 60 * 60 * 24))

	// Handle future dates (possible clock skew)
	if (diffDays < 0) {
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: 'numeric',
			minute: '2-digit',
		})
	}

	if (diffDays === 0) {
		return `Today at ${date.toLocaleTimeString('en-US', {
			hour: 'numeric',
			minute: '2-digit',
		})}`
	}

	if (diffDays === 1) {
		return `Yesterday at ${date.toLocaleTimeString('en-US', {
			hour: 'numeric',
			minute: '2-digit',
		})}`
	}

	if (diffDays < 7) {
		return `${diffDays} days ago`
	}

	return date.toLocaleDateString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
		hour: 'numeric',
		minute: '2-digit',
	})
}

/**
 * Simple, stable date-only format for metadata labels.
 *
 * Example:
 * - "Jan 5, 2025"
 */
export function formatSimpleDate(isoString: string): string {
	const date = new Date(isoString)

	if (Number.isNaN(date.getTime())) {
		return isoString
	}

	return date.toLocaleDateString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
	})
}
