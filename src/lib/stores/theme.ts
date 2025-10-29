// T185: Dark mode theme store with localStorage persistence
import { writable } from 'svelte/store'
import { browser } from '$app/environment'

type Theme = 'light' | 'dark' | 'system'

// Get initial theme from localStorage or default to system
function getInitialTheme(): Theme {
	if (!browser) return 'system'

	const stored = localStorage.getItem('theme')
	if (stored === 'light' || stored === 'dark' || stored === 'system') {
		return stored
	}

	return 'system'
}

// Check if dark mode should be active
function shouldUseDarkMode(theme: Theme): boolean {
	if (theme === 'dark') return true
	if (theme === 'light') return false

	// System preference
	if (browser && window.matchMedia) {
		return window.matchMedia('(prefers-color-scheme: dark)').matches
	}

	return false
}

// Create the store
const theme = writable<Theme>(getInitialTheme())

// Subscribe to changes and update DOM + localStorage
if (browser) {
	theme.subscribe((value) => {
		localStorage.setItem('theme', value)
		const isDark = shouldUseDarkMode(value)

		if (isDark) {
			document.documentElement.classList.add('dark')
		} else {
			document.documentElement.classList.remove('dark')
		}
	})

	// Listen for system theme changes
	const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
	mediaQuery.addEventListener('change', () => {
		theme.update((t) => {
			// Only update if using system theme
			if (t === 'system') {
				const isDark = shouldUseDarkMode(t)
				if (isDark) {
					document.documentElement.classList.add('dark')
				} else {
					document.documentElement.classList.remove('dark')
				}
			}
			return t
		})
	})

	// Set initial theme on load
	const initialTheme = getInitialTheme()
	const isDark = shouldUseDarkMode(initialTheme)
	if (isDark) {
		document.documentElement.classList.add('dark')
	} else {
		document.documentElement.classList.remove('dark')
	}
}

export { theme, type Theme }
