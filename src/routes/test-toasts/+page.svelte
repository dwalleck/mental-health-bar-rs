<script lang="ts">
	import { displaySuccess, displayError, displayWarning, displayInfo } from '$lib/utils/errors'
	import type { CommandError } from '$lib/bindings'

	function showSuccess() {
		displaySuccess('This is a success toast notification!')
	}

	function showError() {
		displayError(new Error('This is an error toast notification!'))
	}

	function showWarning() {
		displayWarning('This is a warning toast notification!')
	}

	function showInfo() {
		displayInfo('This is an info toast notification!')
	}

	function showValidationError() {
		// Simulate a validation error (should NOT show as toast per our logic)
		const error: CommandError = {
			message: 'Invalid input: Name is required',
			error_type: 'validation',
			retryable: false,
		}
		const result = displayError(error)
		alert(`Validation error handled as: ${result.type}\nMessage: ${result.message || 'N/A'}`)
	}

	function showRetryableError() {
		// Simulate a retryable database error (should show as toast)
		const error: CommandError = {
			message: 'Database is temporarily busy. Please try again.',
			error_type: 'database_locked',
			retryable: true,
		}
		displayError(error)
	}
</script>

<div class="container mx-auto p-8">
	<h1 class="text-3xl font-bold mb-8">Toast Notification Test Page</h1>

	<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
		<button
			on:click={showSuccess}
			class="p-4 bg-green-600 text-white rounded-sm hover:bg-green-700 transition"
		>
			Show Success Toast
		</button>

		<button
			on:click={showError}
			class="p-4 bg-red-600 text-white rounded-sm hover:bg-red-700 transition"
		>
			Show Error Toast
		</button>

		<button
			on:click={showWarning}
			class="p-4 bg-yellow-600 text-white rounded-sm hover:bg-yellow-700 transition"
		>
			Show Warning Toast
		</button>

		<button
			on:click={showInfo}
			class="p-4 bg-blue-600 text-white rounded-sm hover:bg-blue-700 transition"
		>
			Show Info Toast
		</button>

		<button
			on:click={showValidationError}
			class="p-4 bg-purple-600 text-white rounded-sm hover:bg-purple-700 transition"
		>
			Test Validation Error (Inline)
		</button>

		<button
			on:click={showRetryableError}
			class="p-4 bg-orange-600 text-white rounded-sm hover:bg-orange-700 transition"
		>
			Test Database Error (Toast)
		</button>
	</div>

	<div class="mt-12 p-6 bg-gray-100 dark:bg-gray-800 rounded-lg">
		<h2 class="text-xl font-semibold mb-4">How Toast Notifications Work in This App:</h2>
		<ul class="space-y-2 list-disc list-inside">
			<li>
				<strong>Success messages:</strong> Show as green toast notifications (e.g., "Assessment submitted
				successfully!")
			</li>
			<li>
				<strong>System errors:</strong> Show as red toast notifications (e.g., database errors, network
				issues)
			</li>
			<li>
				<strong>Validation errors:</strong> Show inline with forms using the ErrorMessage component (NOT
				as toasts)
			</li>
			<li><strong>Warnings:</strong> Show as yellow toast notifications</li>
			<li><strong>Info messages:</strong> Show as blue toast notifications</li>
		</ul>
	</div>

	<div class="mt-8 p-6 bg-blue-50 dark:bg-blue-900 rounded-lg">
		<h3 class="text-lg font-semibold mb-2">Migration Status:</h3>
		<p class="text-sm">
			✅ All components have been migrated to use the toast notification system. Every success
			action now shows a toast, and system errors are automatically displayed as toasts while
			validation errors show inline with forms.
		</p>
	</div>
</div>
