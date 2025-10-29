import { describe, it, expect, vi } from 'vitest'
import { render, fireEvent, waitFor } from '@testing-library/svelte'
import ActivityForm from './ActivityForm.svelte'
import type { Activity } from '$lib/bindings'

describe('ActivityForm', () => {
	const mockActivity: Activity = {
		id: 1,
		name: 'Exercise',
		color: '#22C55E',
		icon: '🏃',
		created_at: '2024-01-01T00:00:00Z',
		updated_at: '2024-01-01T00:00:00Z',
		deleted_at: null,
	}

	describe('Create Mode', () => {
		it('should render create form with default values', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			expect(container.textContent).toContain('Create Activity')
			expect(container.querySelector('#activity-name')).toHaveValue('')
			expect(container.querySelector('#activity-color')).toHaveValue('#3b82f6')
			expect(container.querySelector('#activity-icon')).toHaveValue('')
		})

		it('should show character counter at 0/100', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			expect(container.textContent).toContain('0 / 100 characters')
		})

		it('should show required asterisk for name field', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const label = container.querySelector('label[for="activity-name"]')
			expect(label?.textContent).toContain('*')
		})

		it('should show Create Activity button text', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const submitButton = container.querySelector('button[type="submit"]')
			expect(submitButton).toHaveTextContent('Create Activity')
		})
	})

	describe('Edit Mode', () => {
		it('should render edit form with activity values', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { activity: mockActivity, onSubmit, onCancel },
			})

			expect(container.textContent).toContain('Update Activity')
			expect(container.querySelector('#activity-name')).toHaveValue('Exercise')
			// Browser normalizes hex to lowercase
			const colorInput = container.querySelector('#activity-color') as HTMLInputElement
			expect(colorInput.value.toLowerCase()).toBe('#22c55e')
			expect(container.querySelector('#activity-icon')).toHaveValue('🏃')
		})

		it('should show character counter with activity name length', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { activity: mockActivity, onSubmit, onCancel },
			})

			expect(container.textContent).toContain('8 / 100 characters')
		})

		it('should show Update Activity button text', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { activity: mockActivity, onSubmit, onCancel },
			})

			const submitButton = container.querySelector('button[type="submit"]')
			expect(submitButton).toHaveTextContent('Update Activity')
		})
	})

	describe('Form Fields', () => {
		it('should update name input on typing', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Meditation' } })

			expect(nameInput).toHaveValue('Meditation')
		})

		it('should update character counter on typing', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Meditation' } })

			await waitFor(() => {
				expect(container.textContent).toContain('10 / 100 characters')
			})
		})

		it('should update color picker on change', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const colorInput = container.querySelector('#activity-color') as HTMLInputElement
			await fireEvent.input(colorInput, { target: { value: '#FF5733' } })

			// Browser normalizes hex to lowercase
			expect(colorInput.value.toLowerCase()).toBe('#ff5733')
		})

		it('should sync color picker with text input', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const colorInputs = container.querySelectorAll('input[type="color"], input[type="text"]')
			const textInput = Array.from(colorInputs).find(
				(input) => (input as HTMLInputElement).placeholder === '#3B82F6'
			) as HTMLInputElement

			await fireEvent.input(textInput, { target: { value: '#FF5733' } })

			expect(textInput).toHaveValue('#FF5733')
		})

		it('should update icon input on typing', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const iconInput = container.querySelector('#activity-icon') as HTMLInputElement
			await fireEvent.input(iconInput, { target: { value: '🧘' } })

			expect(iconInput).toHaveValue('🧘')
		})

		it('should enforce maxlength on name input', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			expect(nameInput).toHaveAttribute('maxlength', '100')
		})

		it('should enforce maxlength on icon input', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const iconInput = container.querySelector('#activity-icon') as HTMLInputElement
			expect(iconInput).toHaveAttribute('maxlength', '20')
		})
	})

	describe('Validation', () => {
		it('should show error when name is empty', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(container.textContent).toContain('Activity name is required')
			})
			expect(onSubmit).not.toHaveBeenCalled()
		})

		it('should show error when name is only whitespace', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: '   ' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(container.textContent).toContain('Activity name is required')
			})
			expect(onSubmit).not.toHaveBeenCalled()
		})

		it('should show error when name exceeds 100 characters', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const longName = 'a'.repeat(101)
			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: longName } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(container.textContent).toContain('Activity name must be 100 characters or less')
			})
			expect(onSubmit).not.toHaveBeenCalled()
		})

		it('should show error for invalid color format', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const colorInputs = container.querySelectorAll('input[type="color"], input[type="text"]')
			const textInput = Array.from(colorInputs).find(
				(input) => (input as HTMLInputElement).placeholder === '#3B82F6'
			) as HTMLInputElement
			await fireEvent.input(textInput, { target: { value: 'invalid' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(container.textContent).toContain('Color must be in #RRGGBB format')
			})
			expect(onSubmit).not.toHaveBeenCalled()
		})

		it('should show red border on name input when error', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				const nameInput = container.querySelector('#activity-name')
				expect(nameInput).toHaveClass('border-red-500')
			})
		})

		it('should show red border on color input when error', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const colorInputs = container.querySelectorAll('input[type="color"], input[type="text"]')
			const textInput = Array.from(colorInputs).find(
				(input) => (input as HTMLInputElement).placeholder === '#3B82F6'
			) as HTMLInputElement
			await fireEvent.input(textInput, { target: { value: 'invalid' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(textInput).toHaveClass('border-red-500')
			})
		})

		it('should accept valid form submission', async () => {
			const onSubmit = vi.fn().mockResolvedValue(undefined)
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Meditation' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(onSubmit).toHaveBeenCalledWith('Meditation', '#3B82F6', '')
			})
		})
	})

	describe('Form Submission', () => {
		it('should trim name and icon when submitting', async () => {
			const onSubmit = vi.fn().mockResolvedValue(undefined)
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: '  Meditation  ' } })

			const iconInput = container.querySelector('#activity-icon') as HTMLInputElement
			await fireEvent.input(iconInput, { target: { value: '  🧘  ' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(onSubmit).toHaveBeenCalledWith('Meditation', '#3B82F6', '🧘')
			})
		})

		it('should show Creating... during submission', async () => {
			const onSubmit = vi.fn(() => new Promise((resolve) => setTimeout(resolve, 100)))
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				const submitButton = container.querySelector('button[type="submit"]')
				expect(submitButton).toHaveTextContent('Creating...')
			})
		})

		it('should show Updating... during edit submission', async () => {
			const onSubmit = vi.fn(() => new Promise((resolve) => setTimeout(resolve, 100)))
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { activity: mockActivity, onSubmit, onCancel },
			})

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				const submitButton = container.querySelector('button[type="submit"]')
				expect(submitButton).toHaveTextContent('Updating...')
			})
		})

		it('should disable all inputs during submission', async () => {
			const onSubmit = vi.fn(() => new Promise((resolve) => setTimeout(resolve, 100)))
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				const allInputs = container.querySelectorAll('input')
				allInputs.forEach((input) => {
					expect(input).toBeDisabled()
				})
			})
		})

		it('should disable buttons during submission', async () => {
			const onSubmit = vi.fn(() => new Promise((resolve) => setTimeout(resolve, 100)))
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				const allButtons = container.querySelectorAll('button')
				allButtons.forEach((button) => {
					expect(button).toBeDisabled()
				})
			})
		})

		it('should handle submission errors gracefully', async () => {
			const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {})
			const onSubmit = vi.fn().mockRejectedValue(new Error('Submission failed'))
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(consoleError).toHaveBeenCalledWith('Form submission error:', expect.any(Error))
			})

			consoleError.mockRestore()
		})

		it('should re-enable inputs after submission completes', async () => {
			const onSubmit = vi.fn().mockResolvedValue(undefined)
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(onSubmit).toHaveBeenCalled()
			})

			await waitFor(() => {
				expect(nameInput).not.toBeDisabled()
			})
		})
	})

	describe('Cancel Button', () => {
		it('should call onCancel when clicked', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const cancelButton = Array.from(container.querySelectorAll('button')).find(
				(btn) => btn.textContent === 'Cancel'
			)!
			await fireEvent.click(cancelButton)

			expect(onCancel).toHaveBeenCalled()
		})

		it('should have Cancel button with proper styling', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const cancelButton = Array.from(container.querySelectorAll('button')).find(
				(btn) => btn.textContent === 'Cancel'
			)!
			expect(cancelButton).toHaveClass('bg-gray-200')
		})
	})

	describe('Accessibility', () => {
		it('should have labels for all inputs', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			expect(container.querySelector('label[for="activity-name"]')).toBeInTheDocument()
			expect(container.querySelector('label[for="activity-color"]')).toBeInTheDocument()
			expect(container.querySelector('label[for="activity-icon"]')).toBeInTheDocument()
		})

		it('should have proper input ids matching labels', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			expect(container.querySelector('#activity-name')).toBeInTheDocument()
			expect(container.querySelector('#activity-color')).toBeInTheDocument()
			expect(container.querySelector('#activity-icon')).toBeInTheDocument()
		})

		it('should have placeholder text for all inputs', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			expect(nameInput.placeholder).toBeTruthy()

			const iconInput = container.querySelector('#activity-icon') as HTMLInputElement
			expect(iconInput.placeholder).toBeTruthy()
		})

		it('should have helper text for icon field', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			expect(container.textContent).toContain('Enter an emoji to represent this activity')
		})

		it('should show (optional) label for icon field', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, { props: { onSubmit, onCancel } })

			const iconLabel = container.querySelector('label[for="activity-icon"]')
			expect(iconLabel).toHaveTextContent('optional')
		})
	})
})
