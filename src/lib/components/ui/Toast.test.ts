import { describe, it, expect, beforeEach } from 'vitest'
import { render, screen, waitForElementToBeRemoved } from '@testing-library/svelte'
import { toastStore } from '$lib/stores/toast'
import Toast from './Toast.svelte'

describe('Toast Component', () => {
	beforeEach(() => {
		toastStore.clear()
	})

	describe('Rendering', () => {
		it('should render without toasts', () => {
			const { container } = render(Toast)
			expect(container.querySelector('[role="region"]')).toBeInTheDocument()
		})

		it('should render a single toast', async () => {
			render(Toast)
			toastStore.info('Test message')

			const toast = await screen.findByText('Test message')
			expect(toast).toBeInTheDocument()
		})

		it('should render multiple toasts', async () => {
			render(Toast)
			toastStore.info('First message')
			toastStore.success('Second message')
			toastStore.error('Third message')

			expect(await screen.findByText('First message')).toBeInTheDocument()
			expect(await screen.findByText('Second message')).toBeInTheDocument()
			expect(await screen.findByText('Third message')).toBeInTheDocument()
		})
	})

	describe('Toast Types', () => {
		it('should display success toast with correct icon', async () => {
			render(Toast)
			toastStore.success('Success message')

			const toast = await screen.findByRole('alert')
			expect(toast).toHaveTextContent('✓')
			expect(toast).toHaveTextContent('Success message')
		})

		it('should display error toast with correct icon', async () => {
			render(Toast)
			toastStore.error('Error message')

			const toast = await screen.findByRole('alert')
			expect(toast).toHaveTextContent('✕')
			expect(toast).toHaveTextContent('Error message')
		})

		it('should display warning toast with correct icon', async () => {
			render(Toast)
			toastStore.warning('Warning message')

			const toast = await screen.findByRole('alert')
			expect(toast).toHaveTextContent('⚠')
			expect(toast).toHaveTextContent('Warning message')
		})

		it('should display info toast with correct icon', async () => {
			render(Toast)
			toastStore.info('Info message')

			const toast = await screen.findByRole('alert')
			expect(toast).toHaveTextContent('ℹ')
			expect(toast).toHaveTextContent('Info message')
		})
	})

	describe('Dismissal', () => {
		it('should have dismiss button for each toast', async () => {
			render(Toast)
			toastStore.info('Test message')

			const dismissButton = await screen.findByRole('button', { name: /dismiss notification/i })
			expect(dismissButton).toBeInTheDocument()
		})

		it('should remove toast when dismiss button is clicked', async () => {
			render(Toast)
			toastStore.info('Test message')

			const toast = await screen.findByText('Test message')
			expect(toast).toBeInTheDocument()

			const dismissButton = screen.getByRole('button', { name: /dismiss notification/i })
			dismissButton.click()

			// Wait for toast to be removed from DOM
			await waitForElementToBeRemoved(() => screen.queryByText('Test message'))
		})

		it('should only remove the specific toast when multiple are present', async () => {
			render(Toast)
			toastStore.info('First message')
			toastStore.info('Second message')

			// Both toasts should be present
			expect(await screen.findByText('First message')).toBeInTheDocument()
			expect(await screen.findByText('Second message')).toBeInTheDocument()

			// Get all dismiss buttons
			const dismissButtons = screen.getAllByRole('button', { name: /dismiss notification/i })
			expect(dismissButtons).toHaveLength(2)

			// Click the first dismiss button
			dismissButtons[0].click()

			// Wait for first to be removed
			await waitForElementToBeRemoved(() => screen.queryByText('First message'))

			// Second should still remain
			expect(screen.getByText('Second message')).toBeInTheDocument()
		})
	})

	describe('Accessibility', () => {
		it('should have proper ARIA attributes', async () => {
			render(Toast)
			toastStore.info('Test message')

			const container = screen.getByRole('region', { name: /notifications/i })
			expect(container).toHaveAttribute('aria-live', 'polite')

			const toast = await screen.findByRole('alert')
			expect(toast).toHaveAttribute('aria-atomic', 'true')
		})

		it('should have accessible dismiss buttons', async () => {
			render(Toast)
			toastStore.info('Test message')

			const dismissButton = await screen.findByRole('button', { name: /dismiss notification/i })
			expect(dismissButton).toHaveAttribute('aria-label', 'Dismiss notification')
		})
	})

	describe('Multiple Toast Types', () => {
		it('should display all toast types simultaneously', async () => {
			render(Toast)

			toastStore.success('Success!')
			toastStore.error('Error!')
			toastStore.warning('Warning!')
			toastStore.info('Info!')

			expect(await screen.findByText('Success!')).toBeInTheDocument()
			expect(screen.getByText('Error!')).toBeInTheDocument()
			expect(screen.getByText('Warning!')).toBeInTheDocument()
			expect(screen.getByText('Info!')).toBeInTheDocument()

			// Should have 4 alerts
			const alerts = screen.getAllByRole('alert')
			expect(alerts).toHaveLength(4)
		})

		it('should have unique icons for each type', async () => {
			render(Toast)

			toastStore.success('Success!')
			toastStore.error('Error!')
			toastStore.warning('Warning!')
			toastStore.info('Info!')

			const alerts = await screen.findAllByRole('alert')

			expect(alerts[0]).toHaveTextContent('✓')
			expect(alerts[1]).toHaveTextContent('✕')
			expect(alerts[2]).toHaveTextContent('⚠')
			expect(alerts[3]).toHaveTextContent('ℹ')
		})
	})
})
