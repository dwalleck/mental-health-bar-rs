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
	// Single source of truth: store subscription handles all DOM updates
	theme.subscribe((value) => {
		localStorage.setItem('theme', value)
		const isDark = shouldUseDarkMode(value)

		if (isDark) {
			document.documentElement.classList.add('dark')
		} else {
			document.documentElement.classList.remove('dark')
		}
	})

	// Listen for system theme changes and trigger store update
	const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
	mediaQuery.addEventListener('change', (e) => {
		theme.update((t) => {
			if (t === 'system') {
				// Direct DOM update to bypass Svelte's equality check
				// (returning 'system' when value is already 'system' wouldn't trigger subscribers)
				if (e.matches) {
					document.documentElement.classList.add('dark')
				} else {
					document.documentElement.classList.remove('dark')
				}
				return 'system'
			}
			// If user has explicit light/dark preference, don't change it
			return t
		})
	})
}

export { theme, type Theme }
