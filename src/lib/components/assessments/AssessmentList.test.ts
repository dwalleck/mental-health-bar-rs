import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { render, waitFor, fireEvent } from '@testing-library/svelte'
import AssessmentList from './AssessmentList.svelte'
import type { AssessmentType } from '$lib/bindings'

// Mock Tauri's invoke
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn(),
}))

// Mock SvelteKit's goto
vi.mock('$app/navigation', () => ({
	goto: vi.fn(),
}))

describe('AssessmentList', () => {
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

	const mockAssessmentTypes: AssessmentType[] = [
		{
			id: 1,
			code: 'PHQ9',
			name: 'Patient Health Questionnaire-9',
			description: 'Depression screening',
			question_count: 9,
			min_score: 0,
			max_score: 27,
		},
		{
			id: 2,
			code: 'GAD7',
			name: 'Generalized Anxiety Disorder-7',
			description: 'Anxiety screening',
			question_count: 7,
			min_score: 0,
			max_score: 21,
		},
	]

	describe('Loading State', () => {
		it('should show loading message initially', () => {
			// Mock invoke to never resolve
			invokeMock.mockReturnValue(new Promise(() => {}))

			const { container } = render(AssessmentList)

			expect(container.textContent).toContain('Loading assessments...')
		})

		it('should not show assessment list while loading', () => {
			invokeMock.mockReturnValue(new Promise(() => {}))

			const { container } = render(AssessmentList)

			expect(container.querySelector('.grid')).not.toBeInTheDocument()
		})
	})

	describe('Success State', () => {
		it('should fetch and display assessment types', async () => {
			invokeMock.mockResolvedValue(mockAssessmentTypes)

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(invokeMock).toHaveBeenCalledWith('get_assessment_types')
			})

			await waitFor(() => {
				expect(container.textContent).toContain('Patient Health Questionnaire-9')
				expect(container.textContent).toContain('Generalized Anxiety Disorder-7')
			})
		})

		it('should display assessment descriptions', async () => {
			invokeMock.mockResolvedValue(mockAssessmentTypes)

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).toContain('Depression screening')
				expect(container.textContent).toContain('Anxiety screening')
			})
		})

		it('should display question counts', async () => {
			invokeMock.mockResolvedValue(mockAssessmentTypes)

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).toContain('9 questions')
				expect(container.textContent).toContain('7 questions')
			})
		})

		it('should display score ranges', async () => {
			invokeMock.mockResolvedValue(mockAssessmentTypes)

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).toContain('Score: 0-27')
				expect(container.textContent).toContain('Score: 0-21')
			})
		})

		it('should render Take Assessment buttons', async () => {
			invokeMock.mockResolvedValue(mockAssessmentTypes)

			const { container } = render(AssessmentList)

			await waitFor(() => {
				const buttons = container.querySelectorAll('button')
				expect(buttons.length).toBeGreaterThanOrEqual(2)

				const buttonTexts = Array.from(buttons).map((btn) => btn.textContent)
				const takeAssessmentButtons = buttonTexts.filter((text) =>
					text?.includes('Take Assessment')
				)
				expect(takeAssessmentButtons).toHaveLength(2)
			})
		})

		it('should hide loading message after data loads', async () => {
			invokeMock.mockResolvedValue(mockAssessmentTypes)

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).not.toContain('Loading assessments...')
			})
		})

		it('should use default description when none provided', async () => {
			const assessmentWithoutDescription: AssessmentType[] = [
				{
					id: 3,
					code: 'TEST',
					name: 'Test Assessment',
					description: null,
					question_count: 5,
					min_score: 0,
					max_score: 10,
				},
			]

			invokeMock.mockResolvedValue(assessmentWithoutDescription)

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).toContain('Mental health assessment')
			})
		})
	})

	describe('Error State', () => {
		it('should display error message when fetch fails', async () => {
			invokeMock.mockRejectedValue(new Error('Network error'))

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).toContain('Error: Error: Network error')
			})
		})

		it('should hide loading message on error', async () => {
			invokeMock.mockRejectedValue(new Error('Network error'))

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).not.toContain('Loading assessments...')
			})
		})

		it('should not show assessment grid on error', async () => {
			invokeMock.mockRejectedValue(new Error('Network error'))

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.querySelector('.grid')).not.toBeInTheDocument()
			})
		})
	})

	describe('Navigation', () => {
		it('should navigate to assessment page when button clicked', async () => {
			invokeMock.mockResolvedValue(mockAssessmentTypes)

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).toContain('Patient Health Questionnaire-9')
			})

			const buttons = container.querySelectorAll('button')
			const firstButton = buttons[0]

			await fireEvent.click(firstButton)

			await waitFor(() => {
				expect(gotoMock).toHaveBeenCalledWith('/assessments/PHQ9')
			})
		})

		it('should navigate to correct assessment based on button clicked', async () => {
			invokeMock.mockResolvedValue(mockAssessmentTypes)

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).toContain('Generalized Anxiety Disorder-7')
			})

			const buttons = container.querySelectorAll('button')
			const secondButton = buttons[1]

			await fireEvent.click(secondButton)

			await waitFor(() => {
				expect(gotoMock).toHaveBeenCalledWith('/assessments/GAD7')
			})
		})
	})

	describe('Header Content', () => {
		it('should display page title', async () => {
			invokeMock.mockResolvedValue([])

			const { container } = render(AssessmentList)

			await waitFor(() => {
				const heading = container.querySelector('h1')
				expect(heading).toHaveTextContent('Mental Health Assessments')
			})
		})

		it('should display page description', async () => {
			invokeMock.mockResolvedValue([])

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).toContain(
					'Choose an assessment to track your mental health over time'
				)
			})
		})

		it('should mention data privacy', async () => {
			invokeMock.mockResolvedValue([])

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).toContain('All data is stored locally')
				expect(container.textContent).toContain('privately')
			})
		})
	})

	describe('Layout', () => {
		it('should use grid layout for assessment cards', async () => {
			invokeMock.mockResolvedValue(mockAssessmentTypes)

			const { container } = render(AssessmentList)

			await waitFor(() => {
				const grid = container.querySelector('.grid')
				expect(grid).toBeInTheDocument()
				expect(grid).toHaveClass('gap-4', 'md:grid-cols-2')
			})
		})
	})

	describe('Empty State', () => {
		it('should handle empty assessment list', async () => {
			invokeMock.mockResolvedValue([])

			const { container } = render(AssessmentList)

			await waitFor(() => {
				expect(container.textContent).not.toContain('Loading assessments...')
			})

			// Grid should exist but be empty
			const grid = container.querySelector('.grid')
			expect(grid).toBeInTheDocument()
			expect(grid?.children.length).toBe(0)
		})
	})
})
