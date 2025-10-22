<script lang="ts">
	import '../app.css'

	// Navigation items
	const navItems = [
		{ name: 'Dashboard', path: '/', icon: '🏠' },
		{ name: 'Assessments', path: '/assessments', icon: '📋' },
		{ name: 'Mood', path: '/mood', icon: '😊' },
		{ name: 'Charts', path: '/charts', icon: '📊' },
		{ name: 'Settings', path: '/settings', icon: '⚙️' },
	]

	// Get current path and determine if a nav item is active
	let currentPath = typeof window !== 'undefined' ? window.location.pathname : '/'

	function isActive(itemPath: string, currentPath: string): boolean {
		if (itemPath === '/') {
			return currentPath === '/'
		}
		return currentPath.startsWith(itemPath)
	}
</script>

<div class="flex h-screen bg-gray-50">
	<!-- Sidebar -->
	<aside class="w-64 bg-white shadow-md">
		<div class="p-6">
			<h1 class="text-2xl font-bold text-gray-800">Mental Health Tracker</h1>
		</div>

		<nav class="mt-6">
			{#each navItems as item (item.path)}
				<a
					href={item.path}
					data-sveltekit-preload-data="hover"
					class="flex items-center px-6 py-3 text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition-colors {isActive(
						item.path,
						currentPath
					)
						? 'bg-blue-50 text-blue-600 border-r-4 border-blue-600'
						: ''}"
				>
					<span class="text-xl mr-3">{item.icon}</span>
					<span class="font-medium">{item.name}</span>
				</a>
			{/each}
		</nav>
	</aside>

	<!-- Main content area -->
	<main class="flex-1 overflow-y-auto">
		<div class="container mx-auto p-8">
			<slot />
		</div>
	</main>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family:
			-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
	}
</style>
