import { describe, it, expect, beforeEach } from 'vitest'
import { render, fireEvent } from '@testing-library/svelte'
import { get } from 'svelte/store'
import ThemeToggle from './ThemeToggle.svelte'
import { theme } from '$lib/stores/theme'

describe('ThemeToggle', () => {
	beforeEach(() => {
		// Reset theme to light before each test
		theme.set('light')
	})

	describe('Basic Rendering', () => {
		it('should render toggle button', () => {
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')
			expect(button).toBeInTheDocument()
		})

		it('should have theme-toggle class', () => {
			const { container } = render(ThemeToggle)

			const button = container.querySelector('.theme-toggle')
			expect(button).toBeInTheDocument()
		})

		it('should have base styling classes', () => {
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')
			expect(button).toHaveClass('p-2', 'rounded-lg', 'transition-colors')
		})
	})

	describe('Icons', () => {
		it('should display sun icon for light mode', () => {
			theme.set('light')
			const { container } = render(ThemeToggle)

			const icon = container.querySelector('span')
			expect(icon).toHaveTextContent('☀️')
		})

		it('should display moon icon for dark mode', () => {
			theme.set('dark')
			const { container } = render(ThemeToggle)

			const icon = container.querySelector('span')
			expect(icon).toHaveTextContent('🌙')
		})

		it('should display computer icon for system mode', () => {
			theme.set('system')
			const { container } = render(ThemeToggle)

			const icon = container.querySelector('span')
			expect(icon).toHaveTextContent('💻')
		})

		it('should have aria-hidden on icon', () => {
			const { container } = render(ThemeToggle)

			const icon = container.querySelector('span')
			expect(icon).toHaveAttribute('aria-hidden', 'true')
		})
	})

	describe('Labels', () => {
		it('should have Light Mode label when in light mode', () => {
			theme.set('light')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')
			expect(button).toHaveAttribute('title', 'Light Mode')
			expect(button).toHaveAttribute('aria-label', 'Toggle theme (Light Mode)')
		})

		it('should have Dark Mode label when in dark mode', () => {
			theme.set('dark')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')
			expect(button).toHaveAttribute('title', 'Dark Mode')
			expect(button).toHaveAttribute('aria-label', 'Toggle theme (Dark Mode)')
		})

		it('should have System Mode label when in system mode', () => {
			theme.set('system')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')
			expect(button).toHaveAttribute('title', 'System Mode')
			expect(button).toHaveAttribute('aria-label', 'Toggle theme (System Mode)')
		})
	})

	describe('Theme Cycling', () => {
		it('should cycle from light to dark', async () => {
			theme.set('light')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')!
			await fireEvent.click(button)

			expect(get(theme)).toBe('dark')
		})

		it('should cycle from dark to system', async () => {
			theme.set('dark')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')!
			await fireEvent.click(button)

			expect(get(theme)).toBe('system')
		})

		it('should cycle from system to light', async () => {
			theme.set('system')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')!
			await fireEvent.click(button)

			expect(get(theme)).toBe('light')
		})

		it('should complete full cycle through all themes', async () => {
			theme.set('light')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')!

			// light -> dark
			await fireEvent.click(button)
			expect(get(theme)).toBe('dark')

			// dark -> system
			await fireEvent.click(button)
			expect(get(theme)).toBe('system')

			// system -> light
			await fireEvent.click(button)
			expect(get(theme)).toBe('light')
		})
	})

	describe('Icon Updates', () => {
		it('should update icon after clicking from light to dark', async () => {
			theme.set('light')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')!
			const icon = container.querySelector('span')

			expect(icon).toHaveTextContent('☀️')

			await fireEvent.click(button)

			expect(icon).toHaveTextContent('🌙')
		})

		it('should update icon after clicking from dark to system', async () => {
			theme.set('dark')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')!
			const icon = container.querySelector('span')

			expect(icon).toHaveTextContent('🌙')

			await fireEvent.click(button)

			expect(icon).toHaveTextContent('💻')
		})

		it('should update icon after clicking from system to light', async () => {
			theme.set('system')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')!
			const icon = container.querySelector('span')

			expect(icon).toHaveTextContent('💻')

			await fireEvent.click(button)

			expect(icon).toHaveTextContent('☀️')
		})
	})

	describe('Label Updates', () => {
		it('should update label after theme change', async () => {
			theme.set('light')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')!

			expect(button).toHaveAttribute('title', 'Light Mode')

			await fireEvent.click(button)

			expect(button).toHaveAttribute('title', 'Dark Mode')
		})

		it('should update aria-label after theme change', async () => {
			theme.set('light')
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')!

			expect(button).toHaveAttribute('aria-label', 'Toggle theme (Light Mode)')

			await fireEvent.click(button)

			expect(button).toHaveAttribute('aria-label', 'Toggle theme (Dark Mode)')
		})
	})

	describe('Accessibility', () => {
		it('should have aria-label attribute', () => {
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')
			expect(button).toHaveAttribute('aria-label')
		})

		it('should have title attribute for tooltip', () => {
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')
			expect(button).toHaveAttribute('title')
		})

		it('should have large icon size for clickability', () => {
			const { container } = render(ThemeToggle)

			const icon = container.querySelector('span')
			expect(icon).toHaveClass('text-2xl')
		})
	})

	describe('Styling', () => {
		it('should have hover styles', () => {
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')
			expect(button).toHaveClass('hover:bg-gray-100', 'dark:hover:bg-gray-700')
		})

		it('should have padding', () => {
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')
			expect(button).toHaveClass('p-2')
		})

		it('should have rounded corners', () => {
			const { container } = render(ThemeToggle)

			const button = container.querySelector('button')
			expect(button).toHaveClass('rounded-lg')
		})
	})
})
