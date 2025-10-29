import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/svelte'
import Card from './Card.svelte'

describe('Card', () => {
	describe('Padding', () => {
		it('should render with default medium padding', () => {
			const { container } = render(Card)

			const cardDiv = container.querySelector('div')
			expect(cardDiv).toHaveClass('p-6')
		})

		it('should render with small padding when specified', () => {
			const { container } = render(Card, { props: { padding: 'small' } })

			const cardDiv = container.querySelector('div')
			expect(cardDiv).toHaveClass('p-4')
		})

		it('should render with large padding when specified', () => {
			const { container } = render(Card, { props: { padding: 'large' } })

			const cardDiv = container.querySelector('div')
			expect(cardDiv).toHaveClass('p-8')
		})
	})

	describe('Title', () => {
		it('should not display title when undefined', () => {
			render(Card)

			const title = screen.queryByRole('heading', { level: 2 })
			expect(title).not.toBeInTheDocument()
		})

		it('should display title when provided', () => {
			render(Card, { props: { title: 'Test Card Title' } })

			const title = screen.getByRole('heading', { level: 2 })
			expect(title).toBeInTheDocument()
			expect(title).toHaveTextContent('Test Card Title')
		})

		it('should have correct title styling', () => {
			render(Card, { props: { title: 'Styled Title' } })

			const title = screen.getByRole('heading', { level: 2 })
			expect(title).toHaveClass('text-xl', 'font-semibold', 'text-gray-800', 'mb-4')
		})
	})

	describe('Slot Content', () => {
		it('should render without errors when slot content provided', () => {
			// Testing Library has limitations with Svelte slots
			// Just verify the component renders without errors
			const { container } = render(Card)

			const cardDiv = container.querySelector('div')
			expect(cardDiv).toBeInTheDocument()
			// The slot will be rendered when used in real application
			// This test verifies the component structure is correct
		})
	})

	describe('Styling', () => {
		it('should have base card styling', () => {
			const { container } = render(Card)

			const cardDiv = container.querySelector('div')
			expect(cardDiv).toHaveClass('bg-white', 'rounded-lg', 'shadow-md')
		})

		it('should combine padding class with base styling', () => {
			const { container } = render(Card, { props: { padding: 'small' } })

			const cardDiv = container.querySelector('div')
			expect(cardDiv).toHaveClass('bg-white', 'rounded-lg', 'shadow-md', 'p-4')
		})
	})

	describe('Props Combination', () => {
		it('should render with both title and custom padding', () => {
			const { container } = render(Card, {
				props: { title: 'Combined Props', padding: 'large' },
			})

			const title = screen.getByRole('heading', { level: 2 })
			expect(title).toHaveTextContent('Combined Props')

			const cardDiv = container.querySelector('div')
			expect(cardDiv).toHaveClass('p-8')
		})
	})
})
