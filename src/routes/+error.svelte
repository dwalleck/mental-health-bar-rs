<script lang="ts">
	// T180: Comprehensive error boundary for the application
	import { page } from '$app/stores'
	import Card from '$lib/components/ui/Card.svelte'

	// SvelteKit provides error details through the page store
	$: error = $page.error
	$: status = $page.status

	function getErrorTitle(status: number): string {
		switch (status) {
			case 404:
				return 'Page Not Found'
			case 500:
				return 'Internal Error'
			case 503:
				return 'Service Unavailable'
			default:
				return 'Something Went Wrong'
		}
	}

	function getErrorMessage(status: number): string {
		switch (status) {
			case 404:
				return "The page you're looking for doesn't exist or has been moved."
			case 500:
				return 'An unexpected error occurred. Please try again or restart the application.'
			case 503:
				return 'The service is temporarily unavailable. Please try again in a moment.'
			default:
				return 'An unexpected error occurred. Please try refreshing the page.'
		}
	}

	function handleGoHome() {
		window.location.href = '/'
	}

	function handleRefresh() {
		window.location.reload()
	}
</script>

<svelte:head>
	<title>Error {status} - Mental Health Tracker</title>
</svelte:head>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900 flex items-center justify-center p-4">
	<div class="max-w-2xl w-full">
		<Card padding="large">
			<div class="text-center">
				<!-- Error Icon -->
				<div class="mb-6">
					<svg
						class="w-24 h-24 mx-auto text-red-500"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
						/>
					</svg>
				</div>

				<!-- Error Status -->
				<div class="text-6xl font-bold text-gray-800 dark:text-gray-100 mb-2">{status}</div>

				<!-- Error Title -->
				<h1 class="text-3xl font-bold text-gray-800 dark:text-gray-100 mb-4">
					{getErrorTitle(status)}
				</h1>

				<!-- Error Message -->
				<p class="text-gray-600 dark:text-gray-300 mb-6 text-lg">
					{getErrorMessage(status)}
				</p>

				<!-- Technical Details (expandable) -->
				{#if error?.message}
					<details class="mb-6 text-left">
						<summary
							class="cursor-pointer text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 font-medium"
						>
							Show Technical Details
						</summary>
						<div class="mt-4 p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
							<pre
								class="text-sm text-gray-700 dark:text-gray-300 whitespace-pre-wrap break-words">{String(
									error.message
								)}</pre>
						</div>
					</details>
				{/if}

				<!-- Action Buttons -->
				<div class="flex flex-col sm:flex-row gap-3 justify-center">
					<button
						onclick={handleGoHome}
						class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg shadow-sm transition-colors inline-flex items-center justify-center gap-2"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
							/>
						</svg>
						Go to Dashboard
					</button>

					<button
						onclick={handleRefresh}
						class="px-6 py-3 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-200 font-semibold rounded-lg shadow-sm transition-colors inline-flex items-center justify-center gap-2"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
							/>
						</svg>
						Refresh Page
					</button>
				</div>

				<!-- Help Text -->
				<div class="mt-8 pt-6 border-t border-gray-200 dark:border-gray-700">
					<p class="text-sm text-gray-500 dark:text-gray-400">
						If this problem persists, please try restarting the application.
					</p>
				</div>
			</div>
		</Card>
	</div>
</div>

<style>
	/* Ensure error page works even if global styles fail to load */
	:global(body) {
		margin: 0;
		font-family:
			-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
	}
</style>
