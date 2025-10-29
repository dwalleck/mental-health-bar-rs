import { describe, it, expect, vi } from 'vitest'
import { render, fireEvent } from '@testing-library/svelte'
import Select from './Select.svelte'

describe('Select', () => {
	describe('Basic Rendering', () => {
		it('should render with default props', () => {
			const { container } = render(Select)

			const select = container.querySelector('select')
			expect(select).toBeInTheDocument()
		})

		it('should render with options', () => {
			const options = [
				{ value: '1', label: 'Option 1' },
				{ value: '2', label: 'Option 2' },
				{ value: '3', label: 'Option 3' },
			]
			const { container } = render(Select, { props: { options } })

			const select = container.querySelector('select')
			const optionElements = select?.querySelectorAll('option')

			// +1 for placeholder option
			expect(optionElements).toHaveLength(options.length + 1)
			expect(optionElements?.[1]).toHaveTextContent('Option 1')
			expect(optionElements?.[2]).toHaveTextContent('Option 2')
			expect(optionElements?.[3]).toHaveTextContent('Option 3')
		})

		it('should render option values correctly', () => {
			const options = [
				{ value: 'opt1', label: 'First Option' },
				{ value: 'opt2', label: 'Second Option' },
			]
			const { container } = render(Select, { props: { options } })

			const optionElements = container.querySelectorAll('option')

			expect(optionElements[1]).toHaveValue('opt1')
			expect(optionElements[2]).toHaveValue('opt2')
		})
	})

	describe('Label', () => {
		it('should not render label when undefined', () => {
			const { container } = render(Select)

			const label = container.querySelector('label')
			expect(label).not.toBeInTheDocument()
		})

		it('should render label when provided', () => {
			const { container } = render(Select, { props: { label: 'Test Label' } })

			const label = container.querySelector('label')
			expect(label).toBeInTheDocument()
			expect(label).toHaveTextContent('Test Label')
		})

		it('should have label styling', () => {
			const { container } = render(Select, { props: { label: 'Styled Label' } })

			const label = container.querySelector('label')
			expect(label).toHaveClass('block', 'text-sm', 'font-medium', 'text-gray-700', 'mb-1')
		})

		it('should show required indicator when required is true', () => {
			const { container } = render(Select, { props: { label: 'Required Field', required: true } })

			const requiredSpan = container.querySelector('.text-red-500')
			expect(requiredSpan).toBeInTheDocument()
			expect(requiredSpan).toHaveTextContent('*')
		})

		it('should not show required indicator when required is false', () => {
			const { container } = render(Select, { props: { label: 'Optional Field', required: false } })

			const labelSpan = container.querySelector('label .text-red-500')
			expect(labelSpan).not.toBeInTheDocument()
		})

		it('should associate label with select via id', () => {
			const customId = 'custom-select-id'
			const { container } = render(Select, {
				props: { label: 'Associated Label', id: customId },
			})

			const label = container.querySelector('label')
			const select = container.querySelector('select')
			expect(label).toHaveAttribute('for', customId)
			expect(select).toHaveAttribute('id', customId)
		})
	})

	describe('Placeholder', () => {
		it('should have default placeholder', () => {
			const { container } = render(Select)

			const placeholderOption = container.querySelector('option[value=""]')
			expect(placeholderOption).toBeInTheDocument()
			expect(placeholderOption).toHaveTextContent('Select an option')
		})

		it('should render custom placeholder when provided', () => {
			const { container } = render(Select, { props: { placeholder: 'Choose one...' } })

			const placeholderOption = container.querySelector('option[value=""]')
			expect(placeholderOption).toHaveTextContent('Choose one...')
		})

		it('should have placeholder option disabled', () => {
			const { container } = render(Select)

			const placeholderOption = container.querySelector('option[value=""]')
			expect(placeholderOption).toBeDisabled()
		})

		it('should select placeholder by default when value is empty', () => {
			const { container } = render(Select, { props: { value: '' } })

			const placeholderOption = container.querySelector('option[value=""]')
			expect(placeholderOption).toHaveAttribute('selected')
		})
	})

	describe('Required State', () => {
		it('should not be required by default', () => {
			const { container } = render(Select)

			const select = container.querySelector('select')
			expect(select).not.toBeRequired()
		})

		it('should be required when prop is true', () => {
			const { container } = render(Select, { props: { required: true } })

			const select = container.querySelector('select')
			expect(select).toBeRequired()
		})
	})

	describe('Disabled State', () => {
		it('should not be disabled by default', () => {
			const { container } = render(Select)

			const select = container.querySelector('select')
			expect(select).not.toBeDisabled()
		})

		it('should be disabled when prop is true', () => {
			const { container } = render(Select, { props: { disabled: true } })

			const select = container.querySelector('select')
			expect(select).toBeDisabled()
		})

		it('should have disabled styling', () => {
			const { container } = render(Select, { props: { disabled: true } })

			const select = container.querySelector('select')
			expect(select).toHaveClass('disabled:bg-gray-100', 'disabled:cursor-not-allowed')
		})
	})

	describe('Error State', () => {
		it('should not show error by default', () => {
			const { container } = render(Select)

			const error = container.querySelector('.text-red-500')
			expect(error).not.toBeInTheDocument()
		})

		it('should show error message when provided', () => {
			const { container } = render(Select, { props: { error: 'This field is required' } })

			const errorText = container.querySelector('p.text-red-500')
			expect(errorText).toBeInTheDocument()
			expect(errorText).toHaveTextContent('This field is required')
		})

		it('should apply error border styling when error is present', () => {
			const { container } = render(Select, { props: { error: 'Invalid selection' } })

			const select = container.querySelector('select')
			expect(select).toHaveClass('border-red-500')
		})

		it('should have normal border when no error', () => {
			const { container } = render(Select)

			const select = container.querySelector('select')
			expect(select).toHaveClass('border-gray-300')
		})
	})

	describe('Event Handling', () => {
		it('should fire onchange event when selection changes', async () => {
			const changeHandler = vi.fn()
			const options = [
				{ value: '1', label: 'Option 1' },
				{ value: '2', label: 'Option 2' },
			]
			const { container } = render(Select, {
				props: {
					options,
					onchange: changeHandler,
				},
			})

			const select = container.querySelector('select')!
			await fireEvent.change(select, { target: { value: '1' } })

			expect(changeHandler).toHaveBeenCalled()
		})
	})

	describe('Base Styling', () => {
		it('should have base select styling', () => {
			const { container } = render(Select)

			const select = container.querySelector('select')
			expect(select).toHaveClass(
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
			const { container } = render(Select)

			const wrapper = container.querySelector('div')
			expect(wrapper).toHaveClass('w-full')
		})
	})

	describe('ID Generation', () => {
		it('should generate unique id by default', () => {
			const { container: container1 } = render(Select)
			const { container: container2 } = render(Select)

			const select1 = container1.querySelector('select')
			const select2 = container2.querySelector('select')

			expect(select1).toHaveAttribute('id')
			expect(select2).toHaveAttribute('id')
			// IDs should be different
			expect(select1?.getAttribute('id')).not.toBe(select2?.getAttribute('id'))
		})

		it('should use custom id when provided', () => {
			const { container } = render(Select, { props: { id: 'my-custom-select-id' } })

			const select = container.querySelector('select')
			expect(select).toHaveAttribute('id', 'my-custom-select-id')
		})
	})

	describe('Props Combinations', () => {
		it('should combine label, required, and error', () => {
			const { container } = render(Select, {
				props: {
					label: 'Category',
					required: true,
					error: 'Invalid selection',
				},
			})

			const label = container.querySelector('label')
			const requiredSpan = label?.querySelector('.text-red-500')
			const select = container.querySelector('select')
			const errorText = container.querySelector('p.text-red-500')

			expect(label).toHaveTextContent('Category')
			expect(requiredSpan).toHaveTextContent('*')
			expect(select).toHaveClass('border-red-500')
			expect(errorText).toHaveTextContent('Invalid selection')
		})

		it('should combine options, placeholder, and disabled', () => {
			const options = [
				{ value: 'a', label: 'Option A' },
				{ value: 'b', label: 'Option B' },
			]
			const { container } = render(Select, {
				props: {
					options,
					placeholder: 'Pick one',
					disabled: true,
				},
			})

			const select = container.querySelector('select')
			const placeholderOption = container.querySelector('option[value=""]')
			const optionElements = container.querySelectorAll('option')

			expect(optionElements).toHaveLength(3) // placeholder + 2 options
			expect(placeholderOption).toHaveTextContent('Pick one')
			expect(select).toBeDisabled()
		})

		it('should work with all props combined', () => {
			const options = [
				{ value: 'option1', label: 'First' },
				{ value: 'option2', label: 'Second' },
			]
			const { container } = render(Select, {
				props: {
					options,
					label: 'Selection',
					placeholder: 'Choose...',
					required: true,
					disabled: false,
					error: undefined,
					id: 'combined-select',
				},
			})

			const label = container.querySelector('label')
			const select = container.querySelector('select')
			const placeholderOption = container.querySelector('option[value=""]')

			expect(label).toHaveTextContent('Selection')
			expect(select).toHaveAttribute('id', 'combined-select')
			expect(placeholderOption).toHaveTextContent('Choose...')
			expect(select).toBeRequired()
			expect(select).not.toBeDisabled()
		})
	})
})
