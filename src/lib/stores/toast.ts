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
		 * @returns The toast ID
		 */
		show: (message: string, type: ToastType = 'info', duration: number = 5000): string => {
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
		 * Show a success toast
		 */
		success: (message: string, duration?: number): string => {
			return toastStore.show(message, 'success', duration)
		},
		/**
		 * Show an error toast
		 */
		error: (message: string, duration?: number): string => {
			return toastStore.show(message, 'error', duration)
		},
		/**
		 * Show an info toast
		 */
		info: (message: string, duration?: number): string => {
			return toastStore.show(message, 'info', duration)
		},
		/**
		 * Show a warning toast
		 */
		warning: (message: string, duration?: number): string => {
			return toastStore.show(message, 'warning', duration)
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
