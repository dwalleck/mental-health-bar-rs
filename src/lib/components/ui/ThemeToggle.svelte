<script lang="ts">
	// T185: Dark mode theme toggle button
	import { theme, type Theme } from '$lib/stores/theme'

	function toggleTheme() {
		theme.update((current) => {
			if (current === 'light') return 'dark'
			if (current === 'dark') return 'system'
			return 'light'
		})
	}

	function getIcon(themeValue: Theme): string {
		if (themeValue === 'light') return '☀️'
		if (themeValue === 'dark') return '🌙'
		return '💻'
	}

	function getLabel(themeValue: Theme): string {
		if (themeValue === 'light') return 'Light Mode'
		if (themeValue === 'dark') return 'Dark Mode'
		return 'System Mode'
	}
</script>

<button
	onclick={toggleTheme}
	class="theme-toggle p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
	aria-label="Toggle theme ({getLabel($theme)})"
	title={getLabel($theme)}
>
	<span class="text-2xl" aria-hidden="true">{getIcon($theme)}</span>
</button>

<style>
	.theme-toggle {
		border: 1px solid transparent;
	}

	.theme-toggle:hover {
		border-color: #e5e7eb;
	}

	:global(.dark) .theme-toggle:hover {
		border-color: #374151;
	}

	.theme-toggle:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}
</style>
