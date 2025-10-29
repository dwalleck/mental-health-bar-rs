import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { render, waitFor, fireEvent } from '@testing-library/svelte'
import AssessmentForm from './AssessmentForm.svelte'
import type { AssessmentQuestion, AssessmentResponse } from '$lib/bindings'

// Mock Tauri's invoke
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn(),
}))

// Mock SvelteKit's goto
vi.mock('$app/navigation', () => ({
	goto: vi.fn(),
}))

describe('AssessmentForm', () => {
	let invokeMock: ReturnType<typeof vi.fn>
	let gotoMock: ReturnType<typeof vi.fn>

	beforeEach(async () => {
		const { invoke } = await import('@tauri-apps/api/core')
		const { goto } = await import('$app/navigation')
		invokeMock = invoke as ReturnType<typeof vi.fn>
		gotoMock = goto as ReturnType<typeof vi.fn>

		invokeMock.mockClear()
		gotoMock.mockClear()
	})

	afterEach(() => {
		vi.clearAllMocks()
	})

	const mockQuestions: AssessmentQuestion[] = [
		{
			id: 1,
			assessment_type_code: 'PHQ9',
			number: 1,
			text: 'Little interest or pleasure in doing things',
			options: ['Not at all', 'Several days', 'More than half the days', 'Nearly every day'],
		},
		{
			id: 2,
			assessment_type_code: 'PHQ9',
			number: 2,
			text: 'Feeling down, depressed, or hopeless',
			options: ['Not at all', 'Several days', 'More than half the days', 'Nearly every day'],
		},
	]

	const mockSubmitResponse: AssessmentResponse = {
		id: 123,
		assessment_type_code: 'PHQ9',
		assessment_type: {
			code: 'PHQ9',
			name: 'PHQ-9',
			description: 'Depression screening',
			question_count: 2,
			min_score: 0,
			max_score: 8,
		},
		responses: [
			{ question_id: 1, question_text: 'Q1', score: 1 },
			{ question_id: 2, question_text: 'Q2', score: 2 },
		],
		total_score: 3,
		severity_level: 'minimal',
		completed_at: '2024-01-15T10:30:00Z',
		notes: null,
		created_at: '2024-01-15T10:25:00Z',
	}

	describe('Props', () => {
		it('should accept assessmentCode prop', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(invokeMock).toHaveBeenCalledWith('get_assessment_questions', {
					assessmentTypeCode: 'PHQ9',
				})
			})
		})
	})

	describe('Loading State', () => {
		it('should show loading message initially', () => {
			invokeMock.mockReturnValue(new Promise(() => {}))

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			expect(container.textContent).toContain('Loading assessment...')
		})
	})

	describe('Error State', () => {
		it('should display error message when fetch fails', async () => {
			invokeMock.mockRejectedValue(new Error('Failed to load'))

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Error: Error: Failed to load')
			})
		})
	})

	describe('Questions Display', () => {
		it('should display all questions', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Little interest or pleasure in doing things')
				expect(container.textContent).toContain('Feeling down, depressed, or hopeless')
			})
		})

		it('should display question numbers', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('1.')
				expect(container.textContent).toContain('2.')
			})
		})

		it('should display all options for each question', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
				expect(container.textContent).toContain('Several days')
				expect(container.textContent).toContain('More than half the days')
				expect(container.textContent).toContain('Nearly every day')
			})
		})
	})

	describe('Progress Tracking', () => {
		it('should show 0% progress initially', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Progress: 0/2 (0%)')
			})
		})

		it('should update progress when question answered', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			// Click first option of first question
			const buttons = container.querySelectorAll('button[type="button"]')
			await fireEvent.click(buttons[0])

			await waitFor(() => {
				expect(container.textContent).toContain('Progress: 1/2 (50%)')
			})
		})

		it('should show 100% when all questions answered', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			const buttons = container.querySelectorAll('button[type="button"]')

			// Answer first question (first option)
			await fireEvent.click(buttons[0])

			// Answer second question (first option of second set)
			await fireEvent.click(buttons[4]) // Skip 4 options from first question

			await waitFor(() => {
				expect(container.textContent).toContain('Progress: 2/2 (100%)')
			})
		})
	})

	describe('Answer Selection', () => {
		it('should mark selected option', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			const buttons = container.querySelectorAll('button[type="button"]')
			const firstOption = buttons[0]

			await fireEvent.click(firstOption)

			await waitFor(() => {
				expect(firstOption).toHaveAttribute('aria-checked', 'true')
			})
		})

		it('should allow changing answer', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			const buttons = container.querySelectorAll('button[type="button"]')

			// Select first option
			await fireEvent.click(buttons[0])

			await waitFor(() => {
				expect(buttons[0]).toHaveAttribute('aria-checked', 'true')
			})

			// Change to second option
			await fireEvent.click(buttons[1])

			await waitFor(() => {
				expect(buttons[0]).toHaveAttribute('aria-checked', 'false')
				expect(buttons[1]).toHaveAttribute('aria-checked', 'true')
			})
		})
	})

	describe('Notes Input', () => {
		it('should have notes textarea', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				const textarea = container.querySelector('textarea')
				expect(textarea).toBeInTheDocument()
			})
		})

		it('should allow entering notes', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				const textarea = container.querySelector('textarea')
				expect(textarea).toBeInTheDocument()
			})

			const textarea = container.querySelector('textarea')!
			await fireEvent.input(textarea, { target: { value: 'Feeling better today' } })

			expect(textarea).toHaveValue('Feeling better today')
		})

		it('should have placeholder text', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				const textarea = container.querySelector('textarea')
				expect(textarea).toHaveAttribute('placeholder', 'Any additional thoughts or context...')
			})
		})
	})

	describe('Form Validation', () => {
		it('should disable submit button when no questions answered', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				const submitButton = container.querySelector('button[type="submit"]')
				expect(submitButton).toBeDisabled()
			})
		})

		it('should disable submit button when some questions unanswered', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			// Answer only first question
			const buttons = container.querySelectorAll('button[type="button"]')
			await fireEvent.click(buttons[0])

			await waitFor(() => {
				const submitButton = container.querySelector('button[type="submit"]')
				expect(submitButton).toBeDisabled()
			})
		})

		it('should enable submit button when all questions answered', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			const buttons = container.querySelectorAll('button[type="button"]')

			// Answer both questions
			await fireEvent.click(buttons[0])
			await fireEvent.click(buttons[4])

			await waitFor(() => {
				const submitButton = container.querySelector('button[type="submit"]')
				expect(submitButton).not.toBeDisabled()
			})
		})
	})

	describe('Form Submission', () => {
		it('should submit assessment with responses', async () => {
			invokeMock.mockResolvedValueOnce(mockQuestions).mockResolvedValueOnce(mockSubmitResponse)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			const buttons = container.querySelectorAll('button[type="button"]')

			// Answer both questions
			await fireEvent.click(buttons[0]) // Question 1: option 0
			await fireEvent.click(buttons[5]) // Question 2: option 1

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(invokeMock).toHaveBeenCalledWith('submit_assessment', {
					request: {
						assessment_type_code: 'PHQ9',
						responses: [0, 1],
						notes: null,
					},
				})
			})
		})

		it('should include notes in submission', async () => {
			invokeMock.mockResolvedValueOnce(mockQuestions).mockResolvedValueOnce(mockSubmitResponse)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			// Answer questions
			const buttons = container.querySelectorAll('button[type="button"]')
			await fireEvent.click(buttons[0])
			await fireEvent.click(buttons[4])

			// Add notes
			const textarea = container.querySelector('textarea')!
			await fireEvent.input(textarea, { target: { value: 'Test notes' } })

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(invokeMock).toHaveBeenCalledWith('submit_assessment', {
					request: {
						assessment_type_code: 'PHQ9',
						responses: [0, 0],
						notes: 'Test notes',
					},
				})
			})
		})

		it('should navigate to results after successful submission', async () => {
			invokeMock.mockResolvedValueOnce(mockQuestions).mockResolvedValueOnce(mockSubmitResponse)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			const buttons = container.querySelectorAll('button[type="button"]')
			await fireEvent.click(buttons[0])
			await fireEvent.click(buttons[4])

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(gotoMock).toHaveBeenCalledWith('/assessments/PHQ9/result/123')
			})
		})

		it('should show submitting state during submission', async () => {
			invokeMock
				.mockResolvedValueOnce(mockQuestions)
				.mockImplementationOnce(() => new Promise(() => {}))

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			const buttons = container.querySelectorAll('button[type="button"]')
			await fireEvent.click(buttons[0])
			await fireEvent.click(buttons[4])

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(container.textContent).toContain('Submitting...')
			})
		})

		it('should show error if submission fails', async () => {
			invokeMock
				.mockResolvedValueOnce(mockQuestions)
				.mockRejectedValueOnce(new Error('Submission failed'))

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				expect(container.textContent).toContain('Not at all')
			})

			const buttons = container.querySelectorAll('button[type="button"]')
			await fireEvent.click(buttons[0])
			await fireEvent.click(buttons[4])

			const form = container.querySelector('form')!
			await fireEvent.submit(form)

			await waitFor(() => {
				expect(container.textContent).toContain('Error: Submission failed')
			})
		})
	})

	describe('Navigation Links', () => {
		it('should have back to assessments link', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				const link = container.querySelector('a[href="/assessments"]')
				expect(link).toBeInTheDocument()
				expect(link).toHaveTextContent('Back to Assessments')
			})
		})
	})

	describe('Accessibility', () => {
		it('should have progress bar with aria attributes', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				const progressBar = container.querySelector('[role="progressbar"]')
				expect(progressBar).toBeInTheDocument()
				expect(progressBar).toHaveAttribute('aria-valuenow', '0')
				expect(progressBar).toHaveAttribute('aria-valuemin', '0')
				expect(progressBar).toHaveAttribute('aria-valuemax', '100')
			})
		})

		it('should have radio group with aria-labelledby', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				const radioGroup = container.querySelector('[role="radiogroup"]')
				expect(radioGroup).toHaveAttribute('aria-labelledby', 'question-0-label')
			})
		})

		it('should have radio buttons with aria-checked', async () => {
			invokeMock.mockResolvedValue(mockQuestions)

			const { container } = render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } })

			await waitFor(() => {
				const radioButton = container.querySelector('[role="radio"]')
				expect(radioButton).toHaveAttribute('aria-checked', 'false')
			})
		})
	})
})
