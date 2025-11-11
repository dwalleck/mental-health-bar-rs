import { describe, it, expect, vi } from 'vitest'
import { render, fireEvent, waitFor } from '@testing-library/svelte'
import ActivityForm from './ActivityForm.svelte'
import type { Activity, ActivityGroup } from '$lib/bindings'

describe('ActivityForm', () => {
	const mockGroups: ActivityGroup[] = [
		{
			id: 1,
			name: 'Physical',
			description: 'Physical activities',
			created_at: '2024-01-01T00:00:00Z',
			deleted_at: null,
		},
		{
			id: 2,
			name: 'Mental',
			description: 'Mental activities',
			created_at: '2024-01-01T00:00:00Z',
			deleted_at: null,
		},
	]

	const mockActivity: Activity = {
		id: 1,
		group_id: 1,
		name: 'Exercise',
		color: '#22C55E',
		icon: '🏃',
		created_at: '2024-01-01T00:00:00Z',
		deleted_at: null,
	}

	describe('Create Mode', () => {
		it('should render create form with default values', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			expect(container.textContent).toContain('Create Activity')
			expect(container.querySelector('#activity-name')).toHaveValue('')
			expect(container.querySelector('#activity-color')).toHaveValue('#3b82f6')
		})

		it('should show character counter at 0/100', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			expect(container.textContent).toContain('0 / 100 characters')
		})

		it('should show required asterisk for name field', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const label = container.querySelector('label[for="activity-name"]')
			expect(label?.textContent).toContain('*')
		})

		it('should show Create Activity button text', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const submitButton = container.querySelector('button[type="submit"]')
			expect(submitButton).toHaveTextContent('Create Activity')
		})

		it('should render group selector with provided groups', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			expect(groupSelect).toBeInTheDocument()
			expect(groupSelect.options.length).toBe(3) // "Select a group..." + 2 groups
			expect(groupSelect.options[1].textContent).toBe('Physical')
			expect(groupSelect.options[2].textContent).toBe('Mental')
		})
	})

	describe('Edit Mode', () => {
		it('should render edit form with activity values', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { activity: mockActivity, groups: mockGroups, onSubmit, onCancel },
			})

			expect(container.textContent).toContain('Update Activity')
			expect(container.querySelector('#activity-name')).toHaveValue('Exercise')
			// Browser normalizes hex to lowercase
			const colorInput = container.querySelector('#activity-color') as HTMLInputElement
			expect(colorInput.value.toLowerCase()).toBe('#22c55e')
		})

		it('should show character counter with activity name length', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { activity: mockActivity, groups: mockGroups, onSubmit, onCancel },
			})

			expect(container.textContent).toContain('8 / 100 characters')
		})

		it('should show Update Activity button text', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { activity: mockActivity, groups: mockGroups, onSubmit, onCancel },
			})

			const submitButton = container.querySelector('button[type="submit"]')
			expect(submitButton).toHaveTextContent('Update Activity')
		})

		it('should pre-select the activity group', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { activity: mockActivity, groups: mockGroups, onSubmit, onCancel },
			})

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			expect(groupSelect.value).toBe('1')
		})
	})

	describe('Form Fields', () => {
		it('should update name input on typing', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Meditation' } })

			expect(nameInput).toHaveValue('Meditation')
		})

		it('should update character counter on typing', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Meditation' } })

			await waitFor(() => {
				expect(container.textContent).toContain('10 / 100 characters')
			})
		})

		it('should update color picker on change', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const colorInput = container.querySelector('#activity-color') as HTMLInputElement
			await fireEvent.input(colorInput, { target: { value: '#FF5733' } })

			// Browser normalizes hex to lowercase
			expect(colorInput.value.toLowerCase()).toBe('#ff5733')
		})

		it('should sync color picker with text input', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const colorInputs = container.querySelectorAll('input[type="color"], input[type="text"]')
			const textInput = Array.from(colorInputs).find(
				(input) => (input as HTMLInputElement).placeholder === '#3B82F6'
			) as HTMLInputElement

			await fireEvent.input(textInput, { target: { value: '#FF5733' } })

			expect(textInput).toHaveValue('#FF5733')
		})

		it('should update group selection', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			await fireEvent.change(groupSelect, { target: { value: '2' } })

			expect(groupSelect.value).toBe('2')
		})

		it('should enforce maxlength on name input', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			expect(nameInput).toHaveAttribute('maxlength', '100')
		})
	})

	describe('Validation', () => {
		it('should show error when name is empty', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

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
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

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
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

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

		it('should show error when no group is selected', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test Activity' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(container.textContent).toContain('Please select an activity group')
			})
			expect(onSubmit).not.toHaveBeenCalled()
		})

		it('should show error for invalid color format', async () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			await fireEvent.change(groupSelect, { target: { value: '1' } })

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
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

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
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			await fireEvent.change(groupSelect, { target: { value: '1' } })

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
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Meditation' } })

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			await fireEvent.change(groupSelect, { target: { value: '1' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(onSubmit).toHaveBeenCalledWith('Meditation', '#3B82F6', '', 1)
			})
		})
	})

	describe('Form Submission', () => {
		it('should trim name and icon when submitting', async () => {
			const onSubmit = vi.fn().mockResolvedValue(undefined)
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: '  Meditation  ' } })

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			await fireEvent.change(groupSelect, { target: { value: '1' } })

			// Find icon input by its label
			const iconLabel = Array.from(container.querySelectorAll('label')).find((label) =>
				label.textContent?.includes('Icon')
			)
			const iconInputId = iconLabel?.getAttribute('for')
			const iconInput = container.querySelector(`#${iconInputId}`) as HTMLInputElement
			await fireEvent.input(iconInput, { target: { value: '  🧘  ' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(onSubmit).toHaveBeenCalledWith('Meditation', '#3B82F6', '🧘', 1)
			})
		})

		it('should show Creating... during submission', async () => {
			const onSubmit = vi.fn(() => new Promise<void>((resolve) => setTimeout(resolve, 100)))
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			await fireEvent.change(groupSelect, { target: { value: '1' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				const submitButton = container.querySelector('button[type="submit"]')
				expect(submitButton).toHaveTextContent('Creating...')
			})
		})

		it('should show Updating... during edit submission', async () => {
			const onSubmit = vi.fn(() => new Promise<void>((resolve) => setTimeout(resolve, 100)))
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { activity: mockActivity, groups: mockGroups, onSubmit, onCancel },
			})

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				const submitButton = container.querySelector('button[type="submit"]')
				expect(submitButton).toHaveTextContent('Updating...')
			})
		})

		it('should disable all inputs during submission', async () => {
			const onSubmit = vi.fn(() => new Promise<void>((resolve) => setTimeout(resolve, 100)))
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			await fireEvent.change(groupSelect, { target: { value: '1' } })

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
			const onSubmit = vi.fn(() => new Promise<void>((resolve) => setTimeout(resolve, 100)))
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			await fireEvent.change(groupSelect, { target: { value: '1' } })

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
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			await fireEvent.change(groupSelect, { target: { value: '1' } })

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
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			await fireEvent.input(nameInput, { target: { value: 'Test' } })

			const groupSelect = container.querySelector('#activity-group') as HTMLSelectElement
			await fireEvent.change(groupSelect, { target: { value: '1' } })

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
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const cancelButton = Array.from(container.querySelectorAll('button')).find(
				(btn) => btn.textContent === 'Cancel'
			)!
			await fireEvent.click(cancelButton)

			expect(onCancel).toHaveBeenCalled()
		})

		it('should have Cancel button with proper styling', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const cancelButton = Array.from(container.querySelectorAll('button')).find(
				(btn) => btn.textContent === 'Cancel'
			)!
			expect(cancelButton).toHaveClass('bg-gray-200')
		})
	})

	describe('Accessibility', () => {
		it('should have labels for name, color, and group inputs', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			expect(container.querySelector('label[for="activity-name"]')).toBeInTheDocument()
			expect(container.querySelector('label[for="activity-color"]')).toBeInTheDocument()
			expect(container.querySelector('label[for="activity-group"]')).toBeInTheDocument()
		})

		it('should have proper input ids matching labels', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			expect(container.querySelector('#activity-name')).toBeInTheDocument()
			expect(container.querySelector('#activity-color')).toBeInTheDocument()
			expect(container.querySelector('#activity-group')).toBeInTheDocument()
		})

		it('should have icon picker with dynamic ID linked to label', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			// Find icon label by text content
			const iconLabel = Array.from(container.querySelectorAll('label')).find((label) =>
				label.textContent?.includes('Icon')
			)
			expect(iconLabel).toBeInTheDocument()

			// Verify the label has a for attribute
			const labelFor = iconLabel?.getAttribute('for')
			expect(labelFor).toBeTruthy()

			// Verify the input exists with matching ID
			const iconInput = container.querySelector(`#${labelFor}`)
			expect(iconInput).toBeInTheDocument()
		})

		it('should have placeholder text for name input', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			const nameInput = container.querySelector('#activity-name') as HTMLInputElement
			expect(nameInput.placeholder).toBeTruthy()
		})

		it('should have helper text for icon field', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			// Updated to match IconPicker's actual helper text
			expect(container.textContent).toContain('Enter a Heroicon name')
		})

		it('should show (optional) label for icon field', () => {
			const onSubmit = vi.fn()
			const onCancel = vi.fn()
			const { container } = render(ActivityForm, {
				props: { groups: mockGroups, onSubmit, onCancel },
			})

			// Find icon label by text content
			const iconLabel = Array.from(container.querySelectorAll('label')).find((label) =>
				label.textContent?.includes('Icon')
			)
			expect(iconLabel).toHaveTextContent('optional')
		})
	})
})
