<script lang="ts">
	import { toastStore, type Toast, type ToastType } from '$lib/stores/toast'
	import { fly } from 'svelte/transition'
	import { browser } from '$app/environment'

	// Subscribe to toast store
	let toasts = $state<Toast[]>([])

	$effect(() => {
		const unsubscribe = toastStore.subscribe((state) => {
			toasts = state.toasts
		})

		return () => unsubscribe()
	})

	// Disable transitions in test environment
	const enableTransitions = browser && !import.meta.env.VITEST

	function getToastStyles(type: ToastType): string {
		const baseStyles =
			'flex items-start gap-3 p-4 rounded-lg shadow-lg border min-w-[300px] max-w-[500px]'

		const typeStyles = {
			success:
				'bg-green-50 dark:bg-green-900 border-green-200 dark:border-green-700 text-green-800 dark:text-green-100',
			error:
				'bg-red-50 dark:bg-red-900 border-red-200 dark:border-red-700 text-red-800 dark:text-red-100',
			warning:
				'bg-yellow-50 dark:bg-yellow-900 border-yellow-200 dark:border-yellow-700 text-yellow-800 dark:text-yellow-100',
			info: 'bg-blue-50 dark:bg-blue-900 border-blue-200 dark:border-blue-700 text-blue-800 dark:text-blue-100',
		}

		return `${baseStyles} ${typeStyles[type]}`
	}

	function getIconForType(type: ToastType): string {
		const icons = {
			success: '✓',
			error: '✕',
			warning: '⚠',
			info: 'ℹ',
		}
		return icons[type]
	}

	function getIconColorClass(type: ToastType): string {
		const colors = {
			success: 'text-green-600 dark:text-green-300',
			error: 'text-red-600 dark:text-red-300',
			warning: 'text-yellow-600 dark:text-yellow-300',
			info: 'text-blue-600 dark:text-blue-300',
		}
		return colors[type]
	}

	function dismiss(id: string) {
		toastStore.dismiss(id)
	}
</script>

<!-- Toast container - fixed position at top-right -->
<div
	class="fixed top-4 right-4 z-50 flex flex-col gap-2 pointer-events-none"
	role="region"
	aria-label="Notifications"
	aria-live="polite"
>
	{#each toasts as toast (toast.id)}
		{#if enableTransitions}
			<div
				class="pointer-events-auto"
				transition:fly={{ x: 300, duration: 300 }}
				role="alert"
				aria-atomic="true"
			>
				<div class={getToastStyles(toast.type)}>
					<!-- Icon -->
					<div
						class={`shrink-0 w-6 h-6 rounded-full flex items-center justify-center font-bold ${getIconColorClass(toast.type)}`}
					>
						{getIconForType(toast.type)}
					</div>

					<!-- Message -->
					<div class="flex-1 text-sm font-medium">
						{toast.message}
					</div>

					<!-- Close button -->
					<button
						onclick={() => dismiss(toast.id)}
						class="shrink-0 ml-2 text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300 transition-colors"
						aria-label="Dismiss notification"
					>
						<svg
							class="w-4 h-4"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M6 18L18 6M6 6l12 12"
							/>
						</svg>
					</button>
				</div>
			</div>
		{:else}
			<div class="pointer-events-auto" role="alert" aria-atomic="true">
				<div class={getToastStyles(toast.type)}>
					<!-- Icon -->
					<div
						class={`shrink-0 w-6 h-6 rounded-full flex items-center justify-center font-bold ${getIconColorClass(toast.type)}`}
					>
						{getIconForType(toast.type)}
					</div>

					<!-- Message -->
					<div class="flex-1 text-sm font-medium">
						{toast.message}
					</div>

					<!-- Close button -->
					<button
						onclick={() => dismiss(toast.id)}
						class="shrink-0 ml-2 text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300 transition-colors"
						aria-label="Dismiss notification"
					>
						<svg
							class="w-4 h-4"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M6 18L18 6M6 6l12 12"
							/>
						</svg>
					</button>
				</div>
			</div>
		{/if}
	{/each}
</div>
