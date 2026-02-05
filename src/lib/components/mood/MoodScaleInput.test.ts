import { describe, it, expect, vi } from 'vitest'
import { render, fireEvent } from '@testing-library/svelte'
import MoodScaleInput from './MoodScaleInput.svelte'

describe('MoodScaleInput', () => {
	describe('Rendering', () => {
		it('should render mood scale with all 7 buttons', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons).toHaveLength(7)
		})

		it('should render prompt text', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			expect(container.textContent).toContain('How are you feeling?')
		})

		it('should display all mood labels for 1-7 scale', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			expect(container.textContent).toContain('Terrible')
			expect(container.textContent).toContain('Very Bad')
			expect(container.textContent).toContain('Bad')
			expect(container.textContent).toContain('Ok')
			expect(container.textContent).toContain('Good')
			expect(container.textContent).toContain('Very Good')
			expect(container.textContent).toContain('Excellent')
		})
	})

	describe('Button States', () => {
		it('should mark button as pressed for current value', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[3]).toHaveAttribute('aria-pressed', 'true') // Index 3 = rating 4
		})

		it('should not mark other buttons as pressed', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[0]).toHaveAttribute('aria-pressed', 'false')
			expect(buttons[1]).toHaveAttribute('aria-pressed', 'false')
			expect(buttons[2]).toHaveAttribute('aria-pressed', 'false')
			expect(buttons[4]).toHaveAttribute('aria-pressed', 'false')
			expect(buttons[5]).toHaveAttribute('aria-pressed', 'false')
			expect(buttons[6]).toHaveAttribute('aria-pressed', 'false')
		})

		it('should add ring-4 styles to selected button', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 2, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[1]).toHaveClass('ring-4', 'ring-offset-2', 'ring-blue-500', 'scale-110')
		})

		it('should add opacity to unselected buttons', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 2, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[0]).toHaveClass('opacity-80')
			expect(buttons[2]).toHaveClass('opacity-80')
			expect(buttons[3]).toHaveClass('opacity-80')
			expect(buttons[4]).toHaveClass('opacity-80')
			expect(buttons[5]).toHaveClass('opacity-80')
			expect(buttons[6]).toHaveClass('opacity-80')
		})
	})

	describe('Mood Colors', () => {
		it('should have red color for rating 1 (Terrible)', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[0]).toHaveClass('bg-red-500')
		})

		it('should have dark orange color for rating 2 (Very Bad)', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[1]).toHaveClass('bg-orange-500')
		})

		it('should have orange color for rating 3 (Bad)', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[2]).toHaveClass('bg-orange-400')
		})

		it('should have yellow color for rating 4 (Ok)', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[3]).toHaveClass('bg-yellow-500')
		})

		it('should have lime color for rating 5 (Good)', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[4]).toHaveClass('bg-lime-500')
		})

		it('should have green color for rating 6 (Very Good)', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[5]).toHaveClass('bg-green-500')
		})

		it('should have emerald color for rating 7 (Excellent)', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[6]).toHaveClass('bg-emerald-500')
		})
	})

	describe('Click Interactions', () => {
		it('should call onChange when button clicked', async () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			await fireEvent.click(buttons[0])

			expect(onChange).toHaveBeenCalledWith(1)
		})

		it('should call onChange with correct rating for each button', async () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')

			await fireEvent.click(buttons[0])
			expect(onChange).toHaveBeenCalledWith(1)

			await fireEvent.click(buttons[1])
			expect(onChange).toHaveBeenCalledWith(2)

			await fireEvent.click(buttons[2])
			expect(onChange).toHaveBeenCalledWith(3)

			await fireEvent.click(buttons[3])
			expect(onChange).toHaveBeenCalledWith(4)

			await fireEvent.click(buttons[4])
			expect(onChange).toHaveBeenCalledWith(5)

			await fireEvent.click(buttons[5])
			expect(onChange).toHaveBeenCalledWith(6)

			await fireEvent.click(buttons[6])
			expect(onChange).toHaveBeenCalledWith(7)
		})

		it('should allow changing selection', async () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')

			await fireEvent.click(buttons[0])
			expect(onChange).toHaveBeenCalledWith(1)

			await fireEvent.click(buttons[6])
			expect(onChange).toHaveBeenCalledWith(7)
		})
	})

	describe('Keyboard Interactions', () => {
		it('should call onChange when Enter key pressed', async () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			await fireEvent.keyDown(buttons[0], { key: 'Enter' })

			expect(onChange).toHaveBeenCalledWith(1)
		})

		it('should call onChange when Space key pressed', async () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			await fireEvent.keyDown(buttons[3], { key: ' ' })

			expect(onChange).toHaveBeenCalledWith(4)
		})

		it('should navigate left with ArrowLeft key', async () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			await fireEvent.keyDown(buttons[3], { key: 'ArrowLeft' })

			expect(onChange).toHaveBeenCalledWith(3)
		})

		it('should navigate right with ArrowRight key', async () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			await fireEvent.keyDown(buttons[3], { key: 'ArrowRight' })

			expect(onChange).toHaveBeenCalledWith(5)
		})

		it('should not navigate left at minimum rating', async () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 1, onChange } })

			const buttons = container.querySelectorAll('button')
			await fireEvent.keyDown(buttons[0], { key: 'ArrowLeft' })

			expect(onChange).not.toHaveBeenCalled()
		})

		it('should not navigate right at maximum rating', async () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 7, onChange } })

			const buttons = container.querySelectorAll('button')
			await fireEvent.keyDown(buttons[6], { key: 'ArrowRight' })

			expect(onChange).not.toHaveBeenCalled()
		})

		it('should not call onChange for other keys', async () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			await fireEvent.keyDown(buttons[0], { key: 'Escape' })

			expect(onChange).not.toHaveBeenCalled()
		})
	})

	describe('Accessibility', () => {
		it('should have aria-label on each button', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			buttons.forEach((button) => {
				expect(button).toHaveAttribute('aria-label')
			})
		})

		it('should have descriptive aria-label for rating 1', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[0]).toHaveAttribute('aria-label', 'Rate your mood as Terrible (1 out of 7)')
		})

		it('should have descriptive aria-label for rating 7', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[6]).toHaveAttribute('aria-label', 'Rate your mood as Excellent (7 out of 7)')
		})

		it('should have aria-pressed attribute on all buttons', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			buttons.forEach((button) => {
				expect(button).toHaveAttribute('aria-pressed')
			})
		})

		it('should have focus ring-4 styles', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			buttons.forEach((button) => {
				expect(button).toHaveClass('focus:ring-4', 'focus:ring-offset-2', 'focus:ring-blue-500')
			})
		})

		it('should have tabindex="0" for keyboard navigation', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			buttons.forEach((button) => {
				expect(button).toHaveAttribute('tabindex', '0')
			})
		})
	})

	describe('Styling', () => {
		it('should have base button styling', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			buttons.forEach((button) => {
				expect(button).toHaveClass('mood-button', 'rounded-lg', 'font-medium', 'transition-all')
			})
		})

		it('should have white text color', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			buttons.forEach((button) => {
				expect(button).toHaveClass('text-white')
			})
		})

		it('should have flex layout for buttons', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttonContainer = container.querySelector('.flex.gap-2.flex-wrap')
			expect(buttonContainer).toBeInTheDocument()
		})
	})

	describe('Value Prop', () => {
		it('should accept different starting values', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 1, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[0]).toHaveAttribute('aria-pressed', 'true')
		})

		it('should render with provided value', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 7, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[6]).toHaveAttribute('aria-pressed', 'true')
		})

		it('should default to rating 4 (Ok) when no value provided', () => {
			const onChange = vi.fn()
			const { container } = render(MoodScaleInput, { props: { value: 4, onChange } })

			const buttons = container.querySelectorAll('button')
			expect(buttons[3]).toHaveAttribute('aria-pressed', 'true')
		})
	})
})
