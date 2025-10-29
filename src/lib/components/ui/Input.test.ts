import { describe, it, expect, vi } from 'vitest'
import { render, fireEvent } from '@testing-library/svelte'
import Input from './Input.svelte'

describe('Input', () => {
	describe('Basic Rendering', () => {
		it('should render with default props', () => {
			const { container } = render(Input)

			const input = container.querySelector('input')
			expect(input).toBeInTheDocument()
			expect(input).toHaveAttribute('type', 'text')
		})

		it('should render with all input types', () => {
			const types = ['text', 'number', 'email', 'password', 'date', 'time'] as const

			types.forEach((type) => {
				const { container } = render(Input, { props: { type } })
				const input = container.querySelector('input')
				expect(input).toHaveAttribute('type', type)
			})
		})
	})

	describe('Label', () => {
		it('should not render label when undefined', () => {
			const { container } = render(Input)

			const label = container.querySelector('label')
			expect(label).not.toBeInTheDocument()
		})

		it('should render label when provided', () => {
			const { container } = render(Input, { props: { label: 'Test Label' } })

			const label = container.querySelector('label')
			expect(label).toBeInTheDocument()
			expect(label).toHaveTextContent('Test Label')
		})

		it('should have label styling', () => {
			const { container } = render(Input, { props: { label: 'Styled Label' } })

			const label = container.querySelector('label')
			expect(label).toHaveClass('block', 'text-sm', 'font-medium', 'text-gray-700', 'mb-1')
		})

		it('should show required indicator when required is true', () => {
			const { container } = render(Input, { props: { label: 'Required Field', required: true } })

			const requiredSpan = container.querySelector('.text-red-500')
			expect(requiredSpan).toBeInTheDocument()
			expect(requiredSpan).toHaveTextContent('*')
		})

		it('should not show required indicator when required is false', () => {
			const { container } = render(Input, { props: { label: 'Optional Field', required: false } })

			// Error message might have text-red-500, so check specifically for the asterisk within label
			const labelSpan = container.querySelector('label .text-red-500')
			expect(labelSpan).not.toBeInTheDocument()
		})

		it('should associate label with input via id', () => {
			const customId = 'custom-input-id'
			const { container } = render(Input, { props: { label: 'Associated Label', id: customId } })

			const label = container.querySelector('label')
			const input = container.querySelector('input')
			expect(label).toHaveAttribute('for', customId)
			expect(input).toHaveAttribute('id', customId)
		})
	})

	describe('Placeholder', () => {
		it('should have no placeholder by default', () => {
			const { container } = render(Input)

			const input = container.querySelector('input')
			expect(input).toHaveAttribute('placeholder', '')
		})

		it('should render placeholder when provided', () => {
			const { container } = render(Input, { props: { placeholder: 'Enter text...' } })

			const input = container.querySelector('input')
			expect(input).toHaveAttribute('placeholder', 'Enter text...')
		})
	})

	describe('Required State', () => {
		it('should not be required by default', () => {
			const { container } = render(Input)

			const input = container.querySelector('input')
			expect(input).not.toBeRequired()
		})

		it('should be required when prop is true', () => {
			const { container } = render(Input, { props: { required: true } })

			const input = container.querySelector('input')
			expect(input).toBeRequired()
		})
	})

	describe('Disabled State', () => {
		it('should not be disabled by default', () => {
			const { container } = render(Input)

			const input = container.querySelector('input')
			expect(input).not.toBeDisabled()
		})

		it('should be disabled when prop is true', () => {
			const { container } = render(Input, { props: { disabled: true } })

			const input = container.querySelector('input')
			expect(input).toBeDisabled()
		})

		it('should have disabled styling', () => {
			const { container } = render(Input, { props: { disabled: true } })

			const input = container.querySelector('input')
			expect(input).toHaveClass('disabled:bg-gray-100', 'disabled:cursor-not-allowed')
		})
	})

	describe('Error State', () => {
		it('should not show error by default', () => {
			const { container } = render(Input)

			const error = container.querySelector('.text-red-500')
			expect(error).not.toBeInTheDocument()
		})

		it('should show error message when provided', () => {
			const { container } = render(Input, { props: { error: 'This field is required' } })

			const errorText = container.querySelector('p.text-red-500')
			expect(errorText).toBeInTheDocument()
			expect(errorText).toHaveTextContent('This field is required')
		})

		it('should apply error border styling when error is present', () => {
			const { container } = render(Input, { props: { error: 'Invalid input' } })

			const input = container.querySelector('input')
			expect(input).toHaveClass('border-red-500')
		})

		it('should have normal border when no error', () => {
			const { container } = render(Input)

			const input = container.querySelector('input')
			expect(input).toHaveClass('border-gray-300')
		})
	})

	describe('Event Handling', () => {
		it('should fire oninput event when typing', async () => {
			const inputHandler = vi.fn()
			const { container } = render(Input, {
				props: {
					oninput: inputHandler,
				},
			})

			const input = container.querySelector('input')!
			await fireEvent.input(input, { target: { value: 'test' } })

			expect(inputHandler).toHaveBeenCalled()
		})

		it('should fire onchange event when value changes', async () => {
			const changeHandler = vi.fn()
			const { container } = render(Input, {
				props: {
					onchange: changeHandler,
				},
			})

			const input = container.querySelector('input')!
			await fireEvent.change(input, { target: { value: 'changed' } })

			expect(changeHandler).toHaveBeenCalled()
		})

		it('should fire onblur event when input loses focus', async () => {
			const blurHandler = vi.fn()
			const { container } = render(Input, {
				props: {
					onblur: blurHandler,
				},
			})

			const input = container.querySelector('input')!
			await fireEvent.blur(input)

			expect(blurHandler).toHaveBeenCalled()
		})
	})

	describe('Base Styling', () => {
		it('should have base input styling', () => {
			const { container } = render(Input)

			const input = container.querySelector('input')
			expect(input).toHaveClass(
				'w-full',
				'px-4',
				'py-2',
				'border',
				'rounded-lg',
				'focus:outline-none',
				'focus:ring-2',
				'focus:ring-blue-500'
			)
		})

		it('should have full width container', () => {
			const { container } = render(Input)

			const wrapper = container.querySelector('div')
			expect(wrapper).toHaveClass('w-full')
		})
	})

	describe('ID Generation', () => {
		it('should generate unique id by default', () => {
			const { container: container1 } = render(Input)
			const { container: container2 } = render(Input)

			const input1 = container1.querySelector('input')
			const input2 = container2.querySelector('input')

			expect(input1).toHaveAttribute('id')
			expect(input2).toHaveAttribute('id')
			// IDs should be different
			expect(input1?.getAttribute('id')).not.toBe(input2?.getAttribute('id'))
		})

		it('should use custom id when provided', () => {
			const { container } = render(Input, { props: { id: 'my-custom-id' } })

			const input = container.querySelector('input')
			expect(input).toHaveAttribute('id', 'my-custom-id')
		})
	})

	describe('Props Combinations', () => {
		it('should combine label, required, and error', () => {
			const { container } = render(Input, {
				props: {
					label: 'Email',
					required: true,
					error: 'Invalid email',
				},
			})

			const label = container.querySelector('label')
			const requiredSpan = label?.querySelector('.text-red-500')
			const input = container.querySelector('input')
			const errorText = container.querySelector('p.text-red-500')

			expect(label).toHaveTextContent('Email')
			expect(requiredSpan).toHaveTextContent('*')
			expect(input).toHaveClass('border-red-500')
			expect(errorText).toHaveTextContent('Invalid email')
		})

		it('should combine type, placeholder, and disabled', () => {
			const { container } = render(Input, {
				props: {
					type: 'email',
					placeholder: 'user@example.com',
					disabled: true,
				},
			})

			const input = container.querySelector('input')
			expect(input).toHaveAttribute('type', 'email')
			expect(input).toHaveAttribute('placeholder', 'user@example.com')
			expect(input).toBeDisabled()
		})

		it('should work with all props combined', () => {
			const { container } = render(Input, {
				props: {
					type: 'password',
					label: 'Password',
					placeholder: 'Enter password',
					required: true,
					disabled: false,
					error: undefined,
					id: 'password-input',
				},
			})

			const label = container.querySelector('label')
			const input = container.querySelector('input')

			expect(label).toHaveTextContent('Password')
			expect(input).toHaveAttribute('type', 'password')
			expect(input).toHaveAttribute('placeholder', 'Enter password')
			expect(input).toBeRequired()
			expect(input).not.toBeDisabled()
			expect(input).toHaveAttribute('id', 'password-input')
		})
	})
})
