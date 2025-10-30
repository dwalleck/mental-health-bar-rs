<!--
  ErrorMessage Component

  Displays inline validation errors with consistent styling.
  For validation errors only - system errors should use toast notifications.

  Usage:
  <ErrorMessage {error} />
  <ErrorMessage message="Custom error message" />
-->

<script lang="ts">
	import { dev } from '$app/environment'
	import { formatUserError, isValidationError } from '$lib/utils/errors'
	import { isCommandError } from '$lib/utils/types'
	import type { CommandError } from '$lib/bindings'

	// Props - accept either an error object or a message string
	export let error: unknown = undefined
	export let message: string | undefined = undefined
	export let showTechnicalDetails: boolean = dev // Show technical details in dev mode by default

	// Computed values
	$: errorMessage = message || (error ? formatUserError(error) : '')
	$: isValidation = error ? isValidationError(error) : true // Assume validation if message provided
	$: commandError = error && isCommandError(error) ? (error as CommandError) : null
	$: errorType = commandError?.error_type || null
	$: hasRetryable = commandError?.retryable !== undefined
</script>

{#if errorMessage}
	<div
		class="error-message"
		class:validation={isValidation}
		role="alert"
		aria-live="polite"
		aria-atomic="true"
	>
		<svg
			class="icon"
			width="16"
			height="16"
			viewBox="0 0 16 16"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
			aria-hidden="true"
		>
			<path
				d="M8 1.5L1 14h14L8 1.5z"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linejoin="round"
			/>
			<path
				d="M8 6v3.5M8 11.5v.5"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
			/>
		</svg>

		<div class="content">
			<p class="message">{errorMessage}</p>

			{#if showTechnicalDetails && commandError}
				<details class="technical-details">
					<summary>Technical details</summary>
					<div class="details-content">
						{#if errorType}
							<div class="detail-item">
								<span class="label">Error type:</span>
								<code>{errorType}</code>
							</div>
						{/if}
						{#if hasRetryable}
							<div class="detail-item">
								<span class="label">Retryable:</span>
								<code>{commandError.retryable ? 'Yes' : 'No'}</code>
							</div>
						{/if}
					</div>
				</details>
			{/if}
		</div>
	</div>
{/if}

<style>
	.error-message {
		display: flex;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		margin: 0.5rem 0;
		border-radius: 0.375rem;
		background-color: hsl(10 100% 95%);
		border: 1px solid hsl(10 100% 85%);
		color: hsl(10 100% 25%);
		font-size: 0.875rem;
		line-height: 1.25rem;
		animation: slideIn 0.2s ease-out;
	}

	/* Validation errors use warning colors (yellow) */
	.error-message.validation {
		background-color: hsl(45 100% 95%);
		border-color: hsl(45 100% 80%);
		color: hsl(45 100% 25%);
	}

	/* Dark mode support */
	:global(.dark) .error-message {
		background-color: hsl(10 100% 15%);
		border-color: hsl(10 100% 25%);
		color: hsl(10 100% 85%);
	}

	:global(.dark) .error-message.validation {
		background-color: hsl(45 100% 10%);
		border-color: hsl(45 100% 20%);
		color: hsl(45 100% 85%);
	}

	.icon {
		flex-shrink: 0;
		margin-top: 0.125rem;
	}

	.content {
		flex: 1;
		min-width: 0;
	}

	.message {
		margin: 0;
		word-wrap: break-word;
	}

	.technical-details {
		margin-top: 0.5rem;
		font-size: 0.75rem;
		opacity: 0.8;
	}

	.technical-details summary {
		cursor: pointer;
		user-select: none;
		font-weight: 500;
		margin-bottom: 0.25rem;
	}

	.technical-details summary:hover {
		opacity: 1;
	}

	.details-content {
		margin-top: 0.5rem;
		padding: 0.5rem;
		background-color: rgba(0, 0, 0, 0.05);
		border-radius: 0.25rem;
		font-family: 'Courier New', Courier, monospace;
	}

	:global(.dark) .details-content {
		background-color: rgba(255, 255, 255, 0.05);
	}

	.detail-item {
		display: flex;
		gap: 0.5rem;
		margin: 0.25rem 0;
		align-items: baseline;
	}

	.detail-item .label {
		font-weight: 500;
		font-family: inherit;
		min-width: 80px;
	}

	.detail-item code {
		font-size: 0.75rem;
		word-break: break-all;
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(-0.25rem);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Responsive adjustments */
	@media (max-width: 640px) {
		.error-message {
			padding: 0.625rem 0.875rem;
			font-size: 0.8125rem;
		}

		.icon {
			width: 14px;
			height: 14px;
		}
	}
</style>
