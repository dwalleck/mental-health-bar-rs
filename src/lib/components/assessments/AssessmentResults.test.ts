import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { render, waitFor, fireEvent } from '@testing-library/svelte'
import AssessmentResults from './AssessmentResults.svelte'
import type { AssessmentResponse } from '$lib/bindings'

// Mock Tauri's invoke
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn(),
}))

// Mock SvelteKit's goto
vi.mock('$app/navigation', () => ({
	goto: vi.fn(),
}))

describe('AssessmentResults', () => {
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

	const mockAssessmentResponse: AssessmentResponse = {
		id: 1,
		assessment_type_code: 'PHQ9',
		assessment_type: {
			code: 'PHQ9',
			name: 'Patient Health Questionnaire-9',
			description: 'Depression screening',
			question_count: 9,
			min_score: 0,
			max_score: 27,
		},
		responses: [
			{ question_id: 1, question_text: 'Q1', score: 1 },
			{ question_id: 2, question_text: 'Q2', score: 2 },
		],
		total_score: 8,
		severity_level: 'mild',
		completed_at: '2024-01-15T10:30:00Z',
		notes: 'Feeling better this week',
		created_at: '2024-01-15T10:25:00Z',
	}

	describe('Props', () => {
		it('should accept assessmentId prop', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			render(AssessmentResults, { props: { assessmentId: 123 } })

			await waitFor(() => {
				expect(invokeMock).toHaveBeenCalledWith('get_assessment_response', { id: 123 })
			})
		})
	})

	describe('Loading State', () => {
		it('should show loading message initially', () => {
			invokeMock.mockReturnValue(new Promise(() => {}))

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			expect(container.textContent).toContain('Loading results...')
		})
	})

	describe('Error State', () => {
		it('should display error message when fetch fails', async () => {
			invokeMock.mockRejectedValue(new Error('Not found'))

			const { container } = render(AssessmentResults, { props: { assessmentId: 999 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Error: Error: Not found')
			})
		})

		it('should hide loading message on error', async () => {
			invokeMock.mockRejectedValue(new Error('Not found'))

			const { container } = render(AssessmentResults, { props: { assessmentId: 999 } })

			await waitFor(() => {
				expect(container.textContent).not.toContain('Loading results...')
			})
		})
	})

	describe('Success State', () => {
		it('should fetch and display assessment', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(invokeMock).toHaveBeenCalledWith('get_assessment_response', { id: 1 })
			})

			await waitFor(() => {
				expect(container.textContent).toContain('Assessment Complete')
			})
		})

		it('should display assessment type name', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Patient Health Questionnaire-9')
			})
		})

		it('should display total score', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('8')
				expect(container.textContent).toContain('out of 27')
			})
		})

		it('should display formatted severity level', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Mild')
			})
		})

		it('should display completed date', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Completed:')
			})
		})

		it('should display number of questions answered', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Questions Answered: 2')
			})
		})

		it('should display notes when provided', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Your Notes')
				expect(container.textContent).toContain('Feeling better this week')
			})
		})

		it('should not display notes section when notes are null', async () => {
			const responseWithoutNotes = { ...mockAssessmentResponse, notes: null }
			invokeMock.mockResolvedValue(responseWithoutNotes)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).not.toContain('Your Notes')
			})
		})

		it('should display disclaimer', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Disclaimer:')
				expect(container.textContent).toContain(
					'This assessment is a screening tool, not a diagnosis'
				)
			})
		})
	})

	describe('Severity Level Formatting', () => {
		it('should format minimal severity level', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'minimal' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Minimal')
			})
		})

		it('should format moderately_severe with spaces', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'moderately_severe' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Moderately Severe')
			})
		})
	})

	describe('Severity-Based Guidance', () => {
		it('should show minimal guidance for minimal severity', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'minimal' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('minimal symptoms')
				expect(container.textContent).toContain('Continue monitoring')
			})
		})

		it('should show mild guidance for mild severity', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'mild' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('mild symptoms')
				expect(container.textContent).toContain('self-care strategies')
			})
		})

		it('should show moderate guidance for moderate severity', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'moderate' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('moderate symptoms')
				expect(container.textContent).toContain('speaking with a mental health professional')
			})
		})

		it('should show severe guidance for moderately_severe', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'moderately_severe' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('significant symptoms')
				expect(container.textContent).toContain('strongly recommend consulting')
			})
		})

		it('should show severe guidance for severe', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'severe' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('significant symptoms')
				expect(container.textContent).toContain('strongly recommend consulting')
			})
		})
	})

	describe('Navigation Links', () => {
		it('should have back to assessments link', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				const link = container.querySelector('a[href="/assessments"]')
				expect(link).toBeInTheDocument()
				expect(link).toHaveTextContent('Back to Assessments')
			})
		})

		it('should have view history link', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				const link = container.querySelector('a[href="/assessments/history"]')
				expect(link).toBeInTheDocument()
				expect(link).toHaveTextContent('View History')
			})
		})
	})

	describe('Action Buttons', () => {
		it('should have Take Another Assessment button', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Take Another Assessment')
			})
		})

		it('should navigate to assessments page when Take Another clicked', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('Take Another Assessment')
			})

			const buttons = container.querySelectorAll('button')
			const takeAnotherButton = Array.from(buttons).find((btn) =>
				btn.textContent?.includes('Take Another Assessment')
			)

			expect(takeAnotherButton).toBeTruthy()
			await fireEvent.click(takeAnotherButton!)

			await waitFor(() => {
				expect(gotoMock).toHaveBeenCalledWith('/assessments')
			})
		})

		it('should have View Trends button', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('View Trends')
			})
		})

		it('should navigate to charts page when View Trends clicked', async () => {
			invokeMock.mockResolvedValue(mockAssessmentResponse)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				expect(container.textContent).toContain('View Trends')
			})

			const buttons = container.querySelectorAll('button')
			const viewTrendsButton = Array.from(buttons).find((btn) =>
				btn.textContent?.includes('View Trends')
			)

			expect(viewTrendsButton).toBeTruthy()
			await fireEvent.click(viewTrendsButton!)

			await waitFor(() => {
				expect(gotoMock).toHaveBeenCalledWith('/charts')
			})
		})
	})

	describe('Severity Colors', () => {
		it('should apply green color for minimal severity', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'minimal' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				const severityBadge = container.querySelector('.text-green-600')
				expect(severityBadge).toBeInTheDocument()
			})
		})

		it('should apply yellow color for mild severity', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'mild' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				const severityBadge = container.querySelector('.text-yellow-600')
				expect(severityBadge).toBeInTheDocument()
			})
		})

		it('should apply orange color for moderate severity', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'moderate' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				const severityBadge = container.querySelector('.text-orange-600')
				expect(severityBadge).toBeInTheDocument()
			})
		})

		it('should apply red color for moderately_severe', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'moderately_severe' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				const severityBadge = container.querySelector('.text-red-600')
				expect(severityBadge).toBeInTheDocument()
			})
		})

		it('should apply dark red color for severe', async () => {
			const response = { ...mockAssessmentResponse, severity_level: 'severe' }
			invokeMock.mockResolvedValue(response)

			const { container } = render(AssessmentResults, { props: { assessmentId: 1 } })

			await waitFor(() => {
				const severityBadge = container.querySelector('.text-red-700')
				expect(severityBadge).toBeInTheDocument()
			})
		})
	})
})
