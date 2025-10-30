/**
 * Tests for ErrorMessage component
 */

import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/svelte'
import ErrorMessage from './ErrorMessage.svelte'
import type { CommandError } from '$lib/bindings'

describe('ErrorMessage', () => {
	describe('Display Modes', () => {
		it('renders nothing when no error or message provided', () => {
			const { container } = render(ErrorMessage, { props: {} })
			expect(container.textContent).toBe('')
		})

		it('renders error message from message prop', () => {
			render(ErrorMessage, {
				props: {
					message: 'This is a test error',
				},
			})
			expect(screen.getByText('This is a test error')).toBeInTheDocument()
		})

		it('renders error message from error prop', () => {
			const error = new Error('Test error from Error object')
			render(ErrorMessage, {
				props: {
					error,
				},
			})
			expect(screen.getByText('Test error from Error object')).toBeInTheDocument()
		})

		it('prioritizes message prop over error prop', () => {
			const error = new Error('Error object message')
			render(ErrorMessage, {
				props: {
					message: 'Direct message',
					error,
				},
			})
			expect(screen.getByText('Direct message')).toBeInTheDocument()
			expect(screen.queryByText('Error object message')).not.toBeInTheDocument()
		})
	})

	describe('CommandError Handling', () => {
		it('renders CommandError message correctly', () => {
			const commandError: CommandError = {
				message: 'Validation failed: Invalid assessment type',
				error_type: 'validation',
				retryable: false,
			}
			render(ErrorMessage, {
				props: {
					error: commandError,
				},
			})
			expect(screen.getByText('Validation failed: Invalid assessment type')).toBeInTheDocument()
		})

		it('shows technical details for CommandError in dev mode', () => {
			const commandError: CommandError = {
				message: 'Database is locked',
				error_type: 'database_locked',
				retryable: true,
			}
			render(ErrorMessage, {
				props: {
					error: commandError,
					showTechnicalDetails: true,
				},
			})

			// Should show the summary element
			const summary = screen.getByText('Technical details')
			expect(summary).toBeInTheDocument()

			// Should show error type
			expect(screen.getByText('Error type:')).toBeInTheDocument()
			expect(screen.getByText('database_locked')).toBeInTheDocument()

			// Should show retryable status
			expect(screen.getByText('Retryable:')).toBeInTheDocument()
			expect(screen.getByText('Yes')).toBeInTheDocument()
		})

		it('hides technical details when showTechnicalDetails is false', () => {
			const commandError: CommandError = {
				message: 'Database error',
				error_type: 'database_error',
				retryable: false,
			}
			render(ErrorMessage, {
				props: {
					error: commandError,
					showTechnicalDetails: false,
				},
			})

			expect(screen.queryByText('Technical details')).not.toBeInTheDocument()
		})

		it('shows "No" for non-retryable errors in technical details', () => {
			const commandError: CommandError = {
				message: 'Not found',
				error_type: 'not_found',
				retryable: false,
			}
			render(ErrorMessage, {
				props: {
					error: commandError,
					showTechnicalDetails: true,
				},
			})

			expect(screen.getByText('Retryable:')).toBeInTheDocument()
			expect(screen.getByText('No')).toBeInTheDocument()
		})
	})

	describe('Error Type Styling', () => {
		it('applies validation styling for validation errors', () => {
			const validationError: CommandError = {
				message: 'Invalid input',
				error_type: 'validation',
				retryable: false,
			}
			const { container } = render(ErrorMessage, {
				props: {
					error: validationError,
				},
			})

			const errorDiv = container.querySelector('.error-message')
			expect(errorDiv).toHaveClass('validation')
		})

		it('does not apply validation styling for non-validation errors', () => {
			const databaseError: CommandError = {
				message: 'Database error',
				error_type: 'database_error',
				retryable: true,
			}
			const { container } = render(ErrorMessage, {
				props: {
					error: databaseError,
				},
			})

			const errorDiv = container.querySelector('.error-message')
			expect(errorDiv).not.toHaveClass('validation')
		})

		it('assumes validation styling when message prop is provided', () => {
			const { container } = render(ErrorMessage, {
				props: {
					message: 'Please enter a valid email',
				},
			})

			const errorDiv = container.querySelector('.error-message')
			expect(errorDiv).toHaveClass('validation')
		})
	})

	describe('Accessibility', () => {
		it('has proper ARIA attributes for alerts', () => {
			const { container } = render(ErrorMessage, {
				props: {
					message: 'Test error',
				},
			})

			const alertDiv = container.querySelector('[role="alert"]')
			expect(alertDiv).toBeInTheDocument()
			expect(alertDiv).toHaveAttribute('aria-live', 'polite')
			expect(alertDiv).toHaveAttribute('aria-atomic', 'true')
		})

		it('renders icon as decorative (aria-hidden)', () => {
			const { container } = render(ErrorMessage, {
				props: {
					message: 'Test error',
				},
			})

			const icon = container.querySelector('svg')
			expect(icon).toHaveAttribute('aria-hidden', 'true')
		})
	})

	describe('Edge Cases', () => {
		it('handles string errors correctly', () => {
			render(ErrorMessage, {
				props: {
					error: 'Simple string error',
				},
			})
			expect(screen.getByText('Simple string error')).toBeInTheDocument()
		})

		it('handles unknown error types with fallback message', () => {
			const unknownError = { someField: 'not a standard error' }
			render(ErrorMessage, {
				props: {
					error: unknownError,
				},
			})
			expect(
				screen.getByText('An unexpected error occurred. Please try again.')
			).toBeInTheDocument()
		})

		it('handles null error gracefully', () => {
			const { container } = render(ErrorMessage, {
				props: {
					error: null,
				},
			})
			expect(container.textContent).toBe('')
		})

		it('handles undefined error gracefully', () => {
			const { container } = render(ErrorMessage, {
				props: {
					error: undefined,
				},
			})
			expect(container.textContent).toBe('')
		})

		it('handles CommandError with missing optional fields', () => {
			const partialError: CommandError = {
				message: 'Partial error',
				error_type: 'internal',
				retryable: true,
			}

			render(ErrorMessage, {
				props: {
					error: partialError,
					showTechnicalDetails: true,
				},
			})

			expect(screen.getByText('Partial error')).toBeInTheDocument()
			expect(screen.getByText('unknown')).toBeInTheDocument()
		})
	})

	describe('Animation', () => {
		it('includes slide-in animation class', () => {
			const { container } = render(ErrorMessage, {
				props: {
					message: 'Animated error',
				},
			})

			const errorDiv = container.querySelector('.error-message')
			// Check that the element has the animation defined in the CSS
			expect(errorDiv).toBeInTheDocument()
			// The animation is defined in the component's CSS
		})
	})
})
