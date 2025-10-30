// Toast notification store for non-blocking user feedback
import { writable } from 'svelte/store'

export type ToastType = 'success' | 'error' | 'info' | 'warning'

export interface Toast {
	id: string
	message: string
	type: ToastType
	duration: number // milliseconds, 0 = no auto-dismiss
}

interface ToastStore {
	toasts: Toast[]
}

function createToastStore() {
	const { subscribe, update } = writable<ToastStore>({ toasts: [] })

	return {
		subscribe,
		/**
		 * Add a new toast notification
		 * @param message - The message to display
		 * @param type - The type of toast (success, error, info, warning)
		 * @param duration - How long to show the toast in ms (default: 5000, 0 = manual dismiss only)
		 * @param deduplicate - If true, prevent duplicate messages of the same type (default: true)
		 * @returns The toast ID (or existing toast ID if deduplicated)
		 */
		show: (
			message: string,
			type: ToastType = 'info',
			duration: number = 5000,
			deduplicate: boolean = true
		): string => {
			// Check for existing toast with same message and type (deduplication)
			let existingToast: Toast | undefined
			update((state) => {
				if (deduplicate) {
					existingToast = state.toasts.find((t) => t.message === message && t.type === type)
				}
				return state
			})

			// If duplicate found, return existing ID without creating new toast
			if (existingToast) {
				return existingToast.id
			}

			const id = `toast-${Date.now()}-${Math.random()}`
			const toast: Toast = { id, message, type, duration }

			update((state) => ({
				toasts: [...state.toasts, toast],
			}))

			// Auto-dismiss if duration > 0
			if (duration > 0) {
				setTimeout(() => {
					toastStore.dismiss(id)
				}, duration)
			}

			return id
		},
		/**
		 * Show a success toast (with deduplication by default)
		 * @param message - The message to display
		 * @param duration - How long to show the toast in ms
		 * @param deduplicate - If true, prevent duplicate messages (default: true)
		 */
		success: (message: string, duration?: number, deduplicate?: boolean): string => {
			return toastStore.show(message, 'success', duration, deduplicate)
		},
		/**
		 * Show an error toast (with deduplication by default)
		 * @param message - The message to display
		 * @param duration - How long to show the toast in ms
		 * @param deduplicate - If true, prevent duplicate messages (default: true)
		 */
		error: (message: string, duration?: number, deduplicate?: boolean): string => {
			return toastStore.show(message, 'error', duration, deduplicate)
		},
		/**
		 * Show an info toast (with deduplication by default)
		 * @param message - The message to display
		 * @param duration - How long to show the toast in ms
		 * @param deduplicate - If true, prevent duplicate messages (default: true)
		 */
		info: (message: string, duration?: number, deduplicate?: boolean): string => {
			return toastStore.show(message, 'info', duration, deduplicate)
		},
		/**
		 * Show a warning toast (with deduplication by default)
		 * @param message - The message to display
		 * @param duration - How long to show the toast in ms
		 * @param deduplicate - If true, prevent duplicate messages (default: true)
		 */
		warning: (message: string, duration?: number, deduplicate?: boolean): string => {
			return toastStore.show(message, 'warning', duration, deduplicate)
		},
		/**
		 * Dismiss a specific toast by ID
		 */
		dismiss: (id: string) => {
			update((state) => ({
				toasts: state.toasts.filter((t) => t.id !== id),
			}))
		},
		/**
		 * Clear all toasts
		 */
		clear: () => {
			update(() => ({ toasts: [] }))
		},
	}
}

export const toastStore = createToastStore()
