<!--
  Example component demonstrating the new error handling patterns:
  - Validation errors show inline using ErrorMessage component
  - System errors show as toast notifications
  - Retry logic for transient errors
-->

<script lang="ts">
	import { invokeWithRetry } from '$lib/utils/retry'
	import { displayError, displaySuccess } from '$lib/utils/errors'
	import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte'
	import type { CommandError } from '$lib/bindings'

	let validationError: unknown = undefined
	let isSubmitting = false

	// Example 1: Submit with inline validation error
	async function handleSubmitWithValidation() {
		isSubmitting = true
		validationError = undefined

		try {
			// Simulate an API call that might fail with validation
			await invokeWithRetry('submit_assessment', {
				request: {
					assessment_type_code: 'INVALID', // This will trigger a validation error
					responses: [],
					notes: null,
				},
			})

			displaySuccess('Assessment submitted successfully!')
		} catch (error) {
			// Use the new displayError function to determine how to show the error
			const result = displayError(error)

			if (result.type === 'inline') {
				// Validation error - show inline
				validationError = error
			}
			// System errors are automatically shown as toast by displayError
		} finally {
			isSubmitting = false
		}
	}

	// Example 2: Trigger a system error (shows as toast)
	async function handleSystemError() {
		// Simulate a CommandError with database lock (retryable)
		const error: CommandError = {
			message: 'Database is temporarily busy. Please try again.',
			error_type: 'database_locked',
			retryable: true,
		}

		// This will show as a toast notification
		displayError(error)
	}

	// Example 3: Manual success message
	function handleSuccess() {
		displaySuccess('Operation completed successfully!')
	}
</script>

<div class="example-container">
	<h2>Error Handling Examples</h2>

	<section>
		<h3>1. Validation Error (Inline Display)</h3>
		<form on:submit|preventDefault={handleSubmitWithValidation}>
			<div class="form-group">
				<label for="assessment">Assessment Type</label>
				<input id="assessment" type="text" value="INVALID" readonly />

				<!-- Validation errors show inline with the form -->
				<ErrorMessage error={validationError} />
			</div>

			<button type="submit" disabled={isSubmitting}>
				{isSubmitting ? 'Submitting...' : 'Submit (Will Show Validation Error)'}
			</button>
		</form>
	</section>

	<section>
		<h3>2. System Error (Toast Display)</h3>
		<button on:click={handleSystemError}>Trigger Database Error (Shows Toast)</button>
		<p class="hint">System errors automatically show as toast notifications</p>
	</section>

	<section>
		<h3>3. Success Message</h3>
		<button on:click={handleSuccess}>Show Success Message</button>
		<p class="hint">Success messages also show as toast notifications</p>
	</section>

	<section>
		<h3>How It Works</h3>
		<ul>
			<li>
				<strong>Validation Errors:</strong> Show inline with the form using the ErrorMessage component
			</li>
			<li>
				<strong>System Errors:</strong> Automatically display as toast notifications
			</li>
			<li>
				<strong>Retry Logic:</strong> Database lock and transient errors are automatically retried
			</li>
			<li>
				<strong>CommandError:</strong> Structured errors from the backend include error_type and retryable
				flag
			</li>
		</ul>
	</section>
</div>

<style>
	.example-container {
		max-width: 600px;
		margin: 2rem auto;
		padding: 1.5rem;
		background: white;
		border-radius: 0.5rem;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	:global(.dark) .example-container {
		background: #1f2937;
	}

	h2 {
		margin-bottom: 2rem;
		color: #111827;
	}

	:global(.dark) h2 {
		color: #f3f4f6;
	}

	section {
		margin-bottom: 2rem;
		padding-bottom: 2rem;
		border-bottom: 1px solid #e5e7eb;
	}

	:global(.dark) section {
		border-bottom-color: #374151;
	}

	section:last-child {
		border-bottom: none;
	}

	h3 {
		margin-bottom: 1rem;
		color: #374151;
		font-size: 1.125rem;
	}

	:global(.dark) h3 {
		color: #d1d5db;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	label {
		display: block;
		margin-bottom: 0.5rem;
		color: #374151;
		font-weight: 500;
	}

	:global(.dark) label {
		color: #d1d5db;
	}

	input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		background: white;
		color: #111827;
	}

	:global(.dark) input {
		background: #374151;
		border-color: #4b5563;
		color: #f3f4f6;
	}

	button {
		padding: 0.5rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	button:hover:not(:disabled) {
		background: #2563eb;
	}

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.hint {
		margin-top: 0.5rem;
		color: #6b7280;
		font-size: 0.875rem;
		font-style: italic;
	}

	:global(.dark) .hint {
		color: #9ca3af;
	}

	ul {
		margin-top: 1rem;
		padding-left: 1.5rem;
		list-style-type: disc;
	}

	li {
		margin-bottom: 0.5rem;
		color: #374151;
	}

	:global(.dark) li {
		color: #d1d5db;
	}

	strong {
		color: #111827;
	}

	:global(.dark) strong {
		color: #f3f4f6;
	}
</style>
