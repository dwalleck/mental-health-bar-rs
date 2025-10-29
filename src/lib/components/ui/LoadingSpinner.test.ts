import { describe, it, expect, vi } from 'vitest'
import { render } from '@testing-library/svelte'
import LoadingSpinner from './LoadingSpinner.svelte'

// Mock svelte/transition to avoid element.animate issues in jsdom
vi.mock('svelte/transition', () => ({
	fade: () => ({}),
}))

describe('LoadingSpinner', () => {
	describe('Default Rendering', () => {
		it('should render with default props', () => {
			const { container } = render(LoadingSpinner)

			const spinner = container.querySelector('.loading-spinner')
			expect(spinner).toBeInTheDocument()
		})

		it('should have default loading text', () => {
			const { container } = render(LoadingSpinner)

			expect(container.textContent).toContain('Loading...')
		})

		it('should render SVG spinner', () => {
			const { container } = render(LoadingSpinner)

			const svg = container.querySelector('svg')
			expect(svg).toBeInTheDocument()
			expect(svg).toHaveClass('animate-spin', 'text-blue-600')
		})

		it('should have role status for accessibility', () => {
			const { container } = render(LoadingSpinner)

			const spinner = container.querySelector('[role="status"]')
			expect(spinner).toBeInTheDocument()
		})

		it('should have aria-live polite', () => {
			const { container } = render(LoadingSpinner)

			const spinner = container.querySelector('[aria-live="polite"]')
			expect(spinner).toBeInTheDocument()
		})

		it('should have aria-label', () => {
			const { container } = render(LoadingSpinner)

			const spinner = container.querySelector('[aria-label]')
			expect(spinner).toHaveAttribute('aria-label', 'Loading...')
		})
	})

	describe('Size Variants', () => {
		it('should render small size', () => {
			const { container } = render(LoadingSpinner, { props: { size: 'small' } })

			const svg = container.querySelector('svg')
			expect(svg).toHaveClass('w-5', 'h-5')
		})

		it('should render medium size by default', () => {
			const { container } = render(LoadingSpinner)

			const svg = container.querySelector('svg')
			expect(svg).toHaveClass('w-8', 'h-8')
		})

		it('should render medium size when specified', () => {
			const { container } = render(LoadingSpinner, { props: { size: 'medium' } })

			const svg = container.querySelector('svg')
			expect(svg).toHaveClass('w-8', 'h-8')
		})

		it('should render large size', () => {
			const { container } = render(LoadingSpinner, { props: { size: 'large' } })

			const svg = container.querySelector('svg')
			expect(svg).toHaveClass('w-12', 'h-12')
		})

		it('should apply small text size for small spinner', () => {
			const { container } = render(LoadingSpinner, { props: { size: 'small' } })

			const text = container.querySelector('p')
			expect(text).toHaveClass('text-sm')
		})

		it('should apply medium text size for medium spinner', () => {
			const { container } = render(LoadingSpinner, { props: { size: 'medium' } })

			const text = container.querySelector('p')
			expect(text).toHaveClass('text-base')
		})

		it('should apply large text size for large spinner', () => {
			const { container } = render(LoadingSpinner, { props: { size: 'large' } })

			const text = container.querySelector('p')
			expect(text).toHaveClass('text-lg')
		})
	})

	describe('Custom Text', () => {
		it('should display custom text', () => {
			const { container } = render(LoadingSpinner, { props: { text: 'Please wait...' } })

			expect(container.textContent).toContain('Please wait...')
		})

		it('should update aria-label with custom text', () => {
			const { container } = render(LoadingSpinner, { props: { text: 'Saving data...' } })

			const spinner = container.querySelector('[role="status"]')
			expect(spinner).toHaveAttribute('aria-label', 'Saving data...')
		})

		it('should hide text when empty string provided', () => {
			const { container } = render(LoadingSpinner, { props: { text: '' } })

			const text = container.querySelector('p')
			expect(text).not.toBeInTheDocument()
		})

		it('should have text styling', () => {
			const { container } = render(LoadingSpinner, { props: { text: 'Loading...' } })

			const text = container.querySelector('p')
			expect(text).toHaveClass('text-gray-600', 'dark:text-gray-400', 'font-medium')
		})
	})

	describe('Centering', () => {
		it('should not center by default', () => {
			const { container } = render(LoadingSpinner)

			const spinner = container.querySelector('.loading-spinner')
			expect(spinner).not.toHaveClass('flex', 'items-center', 'justify-center')
		})

		it('should center when center prop is true', () => {
			const { container } = render(LoadingSpinner, { props: { center: true } })

			const spinner = container.querySelector('.loading-spinner')
			expect(spinner).toHaveClass('flex', 'items-center', 'justify-center')
		})
	})

	describe('SVG Attributes', () => {
		it('should have aria-hidden on SVG', () => {
			const { container } = render(LoadingSpinner)

			const svg = container.querySelector('svg')
			expect(svg).toHaveAttribute('aria-hidden', 'true')
		})

		it('should have correct viewBox', () => {
			const { container } = render(LoadingSpinner)

			const svg = container.querySelector('svg')
			expect(svg).toHaveAttribute('viewBox', '0 0 24 24')
		})

		it('should have circle element', () => {
			const { container } = render(LoadingSpinner)

			const circle = container.querySelector('circle')
			expect(circle).toBeInTheDocument()
			expect(circle).toHaveAttribute('cx', '12')
			expect(circle).toHaveAttribute('cy', '12')
			expect(circle).toHaveAttribute('r', '10')
		})

		it('should have path element', () => {
			const { container } = render(LoadingSpinner)

			const path = container.querySelector('path')
			expect(path).toBeInTheDocument()
			expect(path).toHaveClass('opacity-75')
		})
	})

	describe('Layout', () => {
		it('should have flex column layout for spinner content', () => {
			const { container } = render(LoadingSpinner)

			const content = container.querySelector('.flex.flex-col')
			expect(content).toBeInTheDocument()
			expect(content).toHaveClass('items-center', 'gap-3')
		})
	})

	describe('Props Combinations', () => {
		it('should combine all props correctly', () => {
			const { container } = render(LoadingSpinner, {
				props: {
					size: 'large',
					text: 'Processing...',
					center: true,
				},
			})

			const spinner = container.querySelector('.loading-spinner')
			expect(spinner).toHaveClass('flex', 'items-center', 'justify-center')

			const svg = container.querySelector('svg')
			expect(svg).toHaveClass('w-12', 'h-12')

			const text = container.querySelector('p')
			expect(text).toHaveTextContent('Processing...')
			expect(text).toHaveClass('text-lg')
		})
	})
})
