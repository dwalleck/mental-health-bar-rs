<!--
  Modal Component with Tailwind UI patterns
  Features:
  - Multiple sizes
  - Backdrop click to close
  - Escape key to close
  - Focus trap
  - Smooth animations
  - Action buttons
  - Custom content slots
-->

<script lang="ts">
	import { fade, fly } from 'svelte/transition'
	import { onMount, onDestroy } from 'svelte'

	type ModalSize = 'sm' | 'md' | 'lg' | 'xl' | 'full'

	interface Props {
		open?: boolean
		title?: string
		description?: string
		size?: ModalSize
		closeOnBackdrop?: boolean
		closeOnEscape?: boolean
		showCloseButton?: boolean
		actions?: Array<{
			label: string
			variant?: 'primary' | 'secondary' | 'danger'
			onClick: () => void | Promise<void>
			disabled?: boolean
			loading?: boolean
		}>
		onclose?: () => void
		children?: import('svelte').Snippet
		icon?: import('svelte').Snippet
	}

	let {
		open = $bindable(false),
		title = '',
		description = '',
		size = 'md',
		closeOnBackdrop = true,
		closeOnEscape = true,
		showCloseButton = true,
		actions = [],
		onclose,
		children,
		icon
	}: Props = $props()

	// Size classes
	const sizeClasses: Record<ModalSize, string> = {
		sm: 'sm:max-w-sm',
		md: 'sm:max-w-lg',
		lg: 'sm:max-w-2xl',
		xl: 'sm:max-w-4xl',
		full: 'sm:max-w-7xl',
	}

	// Handle escape key
	function handleKeydown(event: KeyboardEvent) {
		if (closeOnEscape && event.key === 'Escape' && open) {
			close()
		}
	}

	// Handle backdrop click
	function handleBackdropClick() {
		if (closeOnBackdrop) {
			close()
		}
	}

	// Close modal
	function close() {
		open = false
		onclose?.()
	}

	// Focus trap
	let modalElement = $state<HTMLDivElement>()
	let previousActiveElement = $state<Element | null>(null)

	$effect(() => {
		if (open && modalElement) {
			previousActiveElement = document.activeElement
			const focusableElements = modalElement.querySelectorAll(
				'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
			)
			const firstElement = focusableElements[0] as HTMLElement
			const lastElement = focusableElements[focusableElements.length - 1] as HTMLElement

			firstElement?.focus()

			const trapFocus = (e: KeyboardEvent) => {
				if (e.key !== 'Tab') return

				if (e.shiftKey) {
					if (document.activeElement === firstElement) {
						lastElement?.focus()
						e.preventDefault()
					}
				} else {
					if (document.activeElement === lastElement) {
						firstElement?.focus()
						e.preventDefault()
					}
				}
			}

			modalElement.addEventListener('keydown', trapFocus)

			return () => {
				modalElement?.removeEventListener('keydown', trapFocus)
				;(previousActiveElement as HTMLElement)?.focus()
			}
		}
	})

	onMount(() => {
		if (typeof window !== 'undefined') {
			window.addEventListener('keydown', handleKeydown)
		}
	})

	onDestroy(() => {
		if (typeof window !== 'undefined') {
			window.removeEventListener('keydown', handleKeydown)
		}
	})

	// Get button classes based on variant
	function getButtonClass(variant?: 'primary' | 'secondary' | 'danger') {
		const classes: Record<'primary' | 'secondary' | 'danger', string> = {
			primary:
				'inline-flex justify-center rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600',
			secondary:
				'inline-flex justify-center rounded-md bg-white dark:bg-gray-800 px-3 py-2 text-sm font-semibold text-gray-900 dark:text-gray-100 shadow-sm ring-1 ring-inset ring-gray-300 dark:ring-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700',
			danger:
				'inline-flex justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600',
		}
		return classes[variant || 'secondary']
	}
</script>

{#if open}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-50 bg-gray-500 dark:bg-gray-900 bg-opacity-75 dark:bg-opacity-75 transition-opacity"
		transition:fade={{ duration: 150 }}
		onclick={handleBackdropClick}
		onkeydown={(e) => e.key === 'Enter' && handleBackdropClick()}
		role="button"
		tabindex="-1"
		aria-label="Close modal"
	></div>

	<!-- Modal -->
	<div class="fixed inset-0 z-50 w-screen overflow-y-auto">
		<div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
			<div
				bind:this={modalElement}
				class="relative transform overflow-hidden rounded-lg bg-white dark:bg-gray-800 px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full {sizeClasses[
					size
				]} sm:p-6"
				transition:fly={{ y: 50, duration: 300 }}
				role="dialog"
				aria-modal="true"
				aria-labelledby="modal-title"
				aria-describedby="modal-description"
			>
				<!-- Close button -->
				{#if showCloseButton}
					<div class="absolute right-0 top-0 hidden pr-4 pt-4 sm:block">
						<button
							type="button"
							class="rounded-md bg-white dark:bg-gray-800 text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
							onclick={close}
						>
							<span class="sr-only">Close</span>
							<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
							</svg>
						</button>
					</div>
				{/if}

				<!-- Icon slot -->
				<div class="sm:flex sm:items-start">
					{#if icon}
						<div
							class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900 sm:mx-0 sm:h-10 sm:w-10"
						>
							{@render icon()}
						</div>
					{/if}

					<div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left flex-1">
						<!-- Title -->
						{#if title}
							<h3 class="text-base font-semibold leading-6 text-gray-900 dark:text-white" id="modal-title">
								{title}
							</h3>
						{/if}

						<!-- Description -->
						{#if description}
							<div class="mt-2">
								<p class="text-sm text-gray-500 dark:text-gray-400" id="modal-description">
									{description}
								</p>
							</div>
						{/if}

						<!-- Content slot -->
						<div class="mt-4">
							{@render children?.()}
						</div>
					</div>
				</div>

				<!-- Actions -->
				{#if actions.length > 0}
					<div class="mt-5 sm:mt-4 sm:flex sm:flex-row-reverse gap-3">
						{#each actions as action (action.label)}
								<button
									type="button"
									class="{getButtonClass(action.variant)} disabled:opacity-50 disabled:cursor-not-allowed"
									onclick={action.onClick}
									disabled={action.disabled || action.loading}
								>
									{#if action.loading}
										<svg
											class="animate-spin -ml-1 mr-2 h-4 w-4"
											xmlns="http://www.w3.org/2000/svg"
											fill="none"
											viewBox="0 0 24 24"
										>
											<circle
												class="opacity-25"
												cx="12"
												cy="12"
												r="10"
												stroke="currentColor"
												stroke-width="4"
											></circle>
											<path
												class="opacity-75"
												fill="currentColor"
												d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
											></path>
										</svg>
									{/if}
									{action.label}
								</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}