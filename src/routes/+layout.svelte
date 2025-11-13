<script lang="ts">
	import '../app.css'
	import { goto } from '$app/navigation'
	import ThemeToggle from '$lib/components/ui/ThemeToggle.svelte'
	import Toast from '$lib/components/ui/Toast.svelte'

	// Navigation items
	const navItems = [
		{ name: 'Dashboard', path: '/', icon: '🏠' },
		{ name: 'Assessments', path: '/assessments', icon: '📋' },
		{ name: 'Mood', path: '/mood', icon: '😊' },
		{ name: 'Charts', path: '/charts', icon: '📊' },
		{ name: 'Settings', path: '/settings', icon: '⚙️' },
		// Development/Testing pages
		{ name: 'E2E Test', path: '/e2e-test', icon: '🧪' },
	]

	// Get current path and determine if a nav item is active
	let currentPath = typeof window !== 'undefined' ? window.location.pathname : '/'

	function isActive(itemPath: string, currentPath: string): boolean {
		if (itemPath === '/') {
			return currentPath === '/'
		}
		return currentPath.startsWith(itemPath)
	}

	// T179: Handle notification clicks to navigate to assessment page
	$effect(() => {
		// Only set up notification listener in Tauri environment
		if (typeof window !== 'undefined' && '__TAURI__' in window) {
			// Store the plugin listener for cleanup
			type PluginListener = { unregister: () => Promise<void> }
			let listener: PluginListener | null = null

			// Set up the notification listener asynchronously
			import('@tauri-apps/plugin-notification').then(async ({ onAction }) => {
				// Listen for notification click actions (T179)
				// When a user clicks an assessment reminder notification,
				// extract the assessment type code from extra data and navigate
				listener = await onAction((notification) => {
					console.log('Notification action received:', notification)

					// The assessment type code is stored in the extra field
					if (notification.extra && typeof notification.extra === 'object') {
						const extra = notification.extra as Record<string, unknown>
						const assessmentTypeCode = extra.assessment_type_code

						if (typeof assessmentTypeCode === 'string') {
							const assessmentType = assessmentTypeCode.toLowerCase()
							console.log(`Navigating to assessment: ${assessmentType}`)
							// Navigate to the assessment page
							goto(`/assessments/${assessmentType}`)
						}
					}
				})
			})

			// Return cleanup function that will unregister the listener
			return () => {
				if (listener) {
					void listener.unregister()
				}
			}
		}
	})
</script>

<div class="flex h-screen bg-gray-50 dark:bg-gray-900">
	<!-- Sidebar -->
	<aside class="w-64 bg-white dark:bg-gray-800 shadow-md flex flex-col">
		<div class="p-6">
			<h1 class="text-2xl font-bold text-gray-800 dark:text-white">Mental Health Tracker</h1>
		</div>

		<nav class="mt-6 flex-1">
			{#each navItems as item (item.path)}
				<a
					href={item.path}
					data-sveltekit-preload-data="hover"
					class="flex items-center px-6 py-3 text-gray-700 dark:text-gray-300 hover:bg-blue-50 dark:hover:bg-blue-900 hover:text-blue-600 dark:hover:text-blue-400 transition-colors {isActive(
						item.path,
						currentPath
					)
						? 'bg-blue-50 dark:bg-blue-900 text-blue-600 dark:text-blue-400 border-r-4 border-blue-600'
						: ''}"
				>
					<span class="text-xl mr-3">{item.icon}</span>
					<span class="font-medium">{item.name}</span>
				</a>
			{/each}
		</nav>

		<!-- T185: Theme Toggle -->
		<div class="p-4 border-t border-gray-200 dark:border-gray-700">
			<div class="flex items-center justify-between">
				<span class="text-sm text-gray-600 dark:text-gray-400">Theme</span>
				<ThemeToggle />
			</div>
		</div>
	</aside>

	<!-- Main content area -->
	<main class="flex-1 overflow-y-auto">
		<div class="container mx-auto p-8">
			<slot />
		</div>
	</main>
</div>

<!-- Toast notifications -->
<Toast />

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family:
			-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
	}
</style>
